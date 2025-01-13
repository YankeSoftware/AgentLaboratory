use anyhow::{anyhow, Result};
use exponential_backoff::Backoff;
use pdf_extract as pdfext;
use quick_xml::events::Event;
use quick_xml::Reader;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tempfile::NamedTempFile;
use tokio::time::Duration;
use tracing::{debug, info};

const MAX_QUERY_LENGTH: usize = 300;
const MAX_RETRIES: u32 = 3;
const INITIAL_RETRY_DELAY: Duration = Duration::from_secs(2);

/// Represents an ArXiv paper's metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArxivPaper {
    pub title: String,
    pub summary: String,
    pub published: String,
    pub categories: Vec<String>,
    pub paper_id: String,
    pub pdf_url: String,
}

/// Client for interacting with the ArXiv API
pub struct ArxivClient {
    client: Client,
}

impl ArxivClient {
    /// Create a new ArXiv client
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Process query string to fit within MAX_QUERY_LENGTH while preserving information
    fn process_query(&self, query: &str) -> String {
        if query.len() <= MAX_QUERY_LENGTH {
            return query.to_string();
        }

        let words: Vec<&str> = query.split_whitespace().collect();
        let mut processed_query = Vec::new();
        let mut current_length = 0;

        for word in words {
            if current_length + word.len() + 1 <= MAX_QUERY_LENGTH {
                processed_query.push(word);
                current_length += word.len() + 1;
            } else {
                break;
            }
        }

        processed_query.join(" ")
    }

    /// Search papers by query string
    pub async fn find_papers_by_str(&self, query: &str, limit: usize) -> Result<Vec<ArxivPaper>> {
        let processed_query = self.process_query(query);
        debug!("Processed query: {}", processed_query);
        
        let encoded_query = url::form_urlencoded::byte_serialize(processed_query.as_bytes()).collect::<String>();
        let search_url = format!(
            "http://export.arxiv.org/api/query?search_query=abs:{}&start=0&max_results={}",
            encoded_query, limit
        );
        debug!("ArXiv API URL: {}", search_url);

        let mut backoff = Backoff::new(MAX_RETRIES, INITIAL_RETRY_DELAY, None);
        let mut attempt = 0;

        while let Some(delay) = backoff.next(attempt) {
            match self.client.get(&search_url).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        let xml = response.text().await?;
                        debug!("ArXiv API response: {}", xml);
                        return self.parse_arxiv_response(&xml);
                    }
                }
                Err(e) => {
                    if attempt >= MAX_RETRIES - 1 {
                        return Err(anyhow!("Failed to fetch ArXiv results: {}", e));
                    }
                }
            }
            attempt += 1;
            tokio::time::sleep(delay).await;
        }

        Err(anyhow!("Max retries exceeded for ArXiv search"))
    }

    /// Parse ArXiv API XML response into ArxivPaper structs
    fn parse_arxiv_response(&self, xml: &str) -> Result<Vec<ArxivPaper>> {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);
        
        let mut papers = Vec::new();
        let mut current_paper: Option<ArxivPaper> = None;
        let mut current_field = String::new();
        let mut buf = Vec::new();


        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name().as_ref() {
                        b"entry" => {
                            current_paper = Some(ArxivPaper {
                                title: String::new(),
                                summary: String::new(),
                                published: String::new(),
                                categories: Vec::new(),
                                paper_id: String::new(),
                                pdf_url: String::new(),
                            });
                        }
                        b"title" | b"summary" | b"published" | b"id" => {
                            if !e.attributes().any(|attr| {
                                attr.map(|a| a.key.as_ref() == b"type").unwrap_or(false)
                            }) {
                                current_field = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                            }
                        }
                        b"category" | b"term" => {
                            if let Some(paper) = current_paper.as_mut() {
                                // Look for the term attribute which contains the category
                                for attr in e.attributes() {
                                    if let Ok(attr) = attr {
                                        if attr.key.as_ref() == b"term" {
                                            if let Ok(category) = String::from_utf8(attr.value.into_owned()) {
                                                debug!("Found category: {}", category);
                                                paper.categories.push(category);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::Text(e)) => {
                    if let Some(paper) = current_paper.as_mut() {
                        let text = e.unescape().unwrap_or_default().into_owned();
                        match current_field.as_str() {
                            "title" => paper.title = text.trim().to_string(),
                            "summary" => paper.summary = text.trim().to_string(),
                            "published" => {
                                paper.published = text.split('T').next()
                                    .unwrap_or(&text)
                                    .trim()
                                    .to_string();
                            },
                            "id" => {
                                // Extract paper ID from the full URL
                                if let Some(id) = text.split('/').last() {
                                    let id = id.trim();
                                    paper.paper_id = id.to_string();
                                    paper.pdf_url = format!("https://arxiv.org/pdf/{}.pdf", id);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Ok(Event::End(ref e)) => {
                    match e.name().as_ref() {
                        b"entry" => {
                            if let Some(paper) = current_paper.take() {
                                // Only add papers that have all required fields
                                if !paper.title.is_empty() && !paper.paper_id.is_empty() {
                                    papers.push(paper);
                                }
                            }
                        }
                        _ => {}
                    }
                    current_field.clear();
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(anyhow!("Error parsing XML: {}", e)),
                _ => {}
            }
        }

        info!("Successfully parsed {} papers from ArXiv response", papers.len());
        Ok(papers)
    }

    /// Download and extract text from a paper's PDF
    pub async fn retrieve_full_paper_text(&self, paper_id: &str) -> Result<String> {
        let pdf_url = format!("https://arxiv.org/pdf/{}.pdf", paper_id);
        
        // Download PDF to temp file
        let response = self.client.get(&pdf_url).send().await?;
        let temp_file = NamedTempFile::new()?;
        let pdf_path = temp_file.path().to_owned();
        
        let bytes = response.bytes().await?;
        fs::write(&pdf_path, &bytes)?;

        // Extract text from PDF
        let text = self.extract_text_from_pdf(&pdf_path)?;
        
        // Clean up temp file
        fs::remove_file(pdf_path)?;
        
        Ok(text)
    }

    /// Extract text content from a PDF file
    fn extract_text_from_pdf(&self, pdf_path: &PathBuf) -> Result<String> {
        // Extract text using pdf-extract
        let text = pdfext::extract_text(pdf_path)
            .map_err(|e| anyhow!("Failed to extract text from PDF: {}", e))?;
        
        // Split into pages and format
        let pages: Vec<&str> = text.split('\x0C').collect();
        let mut formatted = String::new();
        
        for (i, page) in pages.iter().enumerate() {
            // Clean and format the page text
            let clean_text = page.trim()
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>()
                .join("\n");
            
            if !clean_text.is_empty() {
                formatted.push_str(&format!("\n=== Page {} ===\n\n", i + 1));
                formatted.push_str(&clean_text);
                formatted.push_str("\n");
            }
        }

        if formatted.is_empty() {
            return Err(anyhow!("No text content found in PDF"));
        }

        Ok(formatted)
    }
}

impl Default for ArxivClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::sleep;
    use tracing_subscriber::{self, fmt::format::FmtSpan};

    fn init_tracing() {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("debug")
            .with_span_events(FmtSpan::FULL)
            .try_init();
    }

    #[tokio::test]
    async fn test_process_query() {
        init_tracing();
        let client = ArxivClient::new();
        
        // Test short query remains unchanged
        let short_query = "quantum computing";
        assert_eq!(client.process_query(short_query), short_query);

        // Test long query gets truncated appropriately
        let long_query = "a".repeat(MAX_QUERY_LENGTH + 100);
        let processed = client.process_query(&long_query);
        assert!(processed.len() <= MAX_QUERY_LENGTH);

        // Test space normalization
        let spaced_query = "machine   learning   neural   networks";
        let processed = client.process_query(spaced_query);
        assert_eq!(processed.split_whitespace().collect::<Vec<_>>(), 
                  ["machine", "learning", "neural", "networks"]);
    }

    #[tokio::test]
    async fn test_find_papers_quantum_computing() {
        init_tracing();
        let client = ArxivClient::new();
        let papers = client.find_papers_by_str("quantum computing", 5).await.unwrap();
        
        assert!(!papers.is_empty());
        assert!(papers.len() <= 5);
        
        // Verify paper structure
        let paper = &papers[0];
        assert!(!paper.title.is_empty());
        assert!(!paper.summary.is_empty());
        assert!(!paper.published.is_empty());
        assert!(!paper.categories.is_empty());
        assert!(!paper.paper_id.is_empty());
        assert!(paper.pdf_url.starts_with("https://arxiv.org/pdf/"));
        assert!(paper.pdf_url.ends_with(".pdf"));

        // Rate limiting
        sleep(Duration::from_secs(3)).await;
    }

    #[tokio::test]
    async fn test_find_papers_ml() {
        init_tracing();
        let client = ArxivClient::new();
        let papers = client.find_papers_by_str("machine learning", 3).await.unwrap();
        
        assert!(!papers.is_empty());
        assert!(papers.len() <= 3);
        
        // Check ML specific categories
        let has_ml_papers = papers.iter().any(|p| 
            p.categories.iter().any(|c| 
                c.contains("cs.LG") || c.contains("stat.ML")
            )
        );
        assert!(has_ml_papers, "Should find papers in ML categories");

        // Rate limiting
        sleep(Duration::from_secs(3)).await;
    }

    #[tokio::test]
    async fn test_retrieve_paper_text() {
        init_tracing();
        let client = ArxivClient::new();

        // Use well-known papers that are unlikely to disappear
        let papers = [
            ("quant-ph/0512258", "quantum"), // Quantum computing paper from 2005
            ("0704.0001", "physics"),       // Physics paper from 2007
        ];

        for (paper_id, expected_keyword) in papers {
            debug!("Testing paper {}", paper_id);
            let text = client.retrieve_full_paper_text(paper_id).await.unwrap();
            
            assert!(!text.is_empty(), "Paper text should not be empty");
            assert!(text.contains("=== Page 1 ==="), "Should contain page markers");
            assert!(
                text.to_lowercase().contains(expected_keyword), 
                "Should find expected keyword '{}' in paper {}", 
                expected_keyword, 
                paper_id
            );

            // Rate limiting
            sleep(Duration::from_secs(3)).await;
        }
    }
}
