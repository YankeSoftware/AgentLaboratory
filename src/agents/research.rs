use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::utils::AgentResult;
use crate::llm::LLMClient;
use crate::tools::arxiv::ArxivClient;
use super::{Agent, AgentConfig};

/// Configuration specific to research agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchAgentConfig {
    #[serde(flatten)]
    pub base: AgentConfig,
    pub research_style: ResearchStyle,
    pub max_papers: usize,
    pub min_citation_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResearchStyle {
    #[serde(rename = "broad")]
    Broad,
    #[serde(rename = "deep")]
    Deep,
    #[serde(rename = "hybrid")]
    Hybrid,
}

pub struct ResearchAgent {
    config: ResearchAgentConfig,
    llm_client: Arc<Box<dyn LLMClient>>,
    arxiv_client: ArxivClient,
}

#[async_trait]
impl Agent for ResearchAgent {
    fn new_with_client(base_config: AgentConfig, llm_client: Arc<Box<dyn LLMClient>>) -> Self {
        Self {
            config: ResearchAgentConfig {
                base: base_config.clone(),
                research_style: ResearchStyle::Hybrid,
                max_papers: 10,
                min_citation_count: 5,
            },
            llm_client,  // Use the shared client
            arxiv_client: ArxivClient::new(),
        }
    }

    async fn process(&self, query: &str) -> AgentResult<String> {
        let mut results = String::new();

        // Phase 1: Literature Review
        results.push_str(&format!("\n=== Phase 1: Literature Review ===\n"));
        let papers = self.search_papers(query).await?;
        results.push_str(&format!("Found {} relevant papers:\n", papers.len()));
        for paper in papers {
            results.push_str(&format!("- {}\n", paper));
        }

        // Phase 2: Experimentation
        results.push_str("\n=== Phase 2: Experimentation ===\n");
        let experiment_plan = match self.config.research_style {
            ResearchStyle::Deep => {
                "1. In-depth analysis of key papers\n\
                 2. Implementation of core methods\n\
                 3. Comparative evaluation\n"
            },
            ResearchStyle::Broad => {
                "1. Survey of multiple approaches\n\
                 2. Meta-analysis of results\n\
                 3. Synthesis of findings\n"
            },
            ResearchStyle::Hybrid => {
                "1. Initial broad survey\n\
                 2. Deep dive into promising approaches\n\
                 3. Selective implementation and evaluation\n"
            }
        };
        results.push_str(&format!("Experiment Plan:\n{}", experiment_plan));

        // Phase 3: Report Writing
        results.push_str("\n=== Phase 3: Report Writing ===\n");
        // Will be implemented by paper agent later
        results.push_str("Report generation in progress...\n");

        Ok(results)
    }

    fn config(&self) -> &AgentConfig {
        &self.config.base
    }

    fn update_config(&mut self, new_config: AgentConfig) {
        self.config.base = new_config;
    }
}

impl ResearchAgent {
    pub fn with_style(mut self, style: ResearchStyle) -> Self {
        self.config.research_style = style;
        self
    }

    pub fn with_max_papers(mut self, max_papers: usize) -> Self {
        self.config.max_papers = max_papers;
        self
    }

    pub fn with_min_citations(mut self, min_citations: usize) -> Self {
        self.config.min_citation_count = min_citations;
        self
    }

    pub async fn search_papers(&self, query: &str) -> AgentResult<Vec<String>> {
        // Search papers using ArXiv API
        let arxiv_papers = self.arxiv_client.find_papers_by_str(query, self.config.max_papers).await?;
        
        // Use LLM to analyze and filter papers
        let system = format!(
            "You are a research assistant tasked with analyzing papers relevant to: {}\n\
             For each paper, provide a title and 1-2 sentence summary.\n\
             Focus on papers that have: \n\
             1. Direct relevance to the query\n\
             2. Novel methodologies or findings\n\
             3. Recent publication dates when possible\n",
            query
        );

        let mut papers = Vec::new();
        for paper in arxiv_papers {
            // Create detailed prompt for each paper
            let prompt = format!(
                "Please analyze this paper:\nTitle: {}\nSummary: {}\nPublished: {}\nCategories: {}\n",
                paper.title,
                paper.summary,
                paper.published,
                paper.categories.join(", ")
            );

            let response = self.llm_client.complete(&system, &prompt).await?;
            papers.push(format!(
                "Title: {}\nSummary: {}\nArXiv ID: {}\nPublished: {}\nCategories: {}\n",
                paper.title,
                response.text.trim(),
                paper.paper_id,
                paper.published,
                paper.categories.join(", ")
            ));
        }

        Ok(papers)
    }

    pub async fn analyze_paper(&self, paper_id: &str) -> AgentResult<String> {
        // Retrieve full paper text
        let paper_text = self.arxiv_client.retrieve_full_paper_text(paper_id).await?;
        
        // Create analysis prompt
        let system = format!(
            "You are a research assistant tasked with analyzing a scientific paper.\n\
             Please provide a comprehensive analysis including:\n\
             1. Key findings and contributions\n\
             2. Methodology and approach\n\
             3. Important equations or algorithms\n\
             4. Experimental results and validity\n\
             5. Potential applications and limitations\n"
        );

        let prompt = format!("Please analyze this paper:\n{}", paper_text);
        let response = self.llm_client.complete(&system, &prompt).await?;
        
        Ok(format!("Analysis of paper {}\n{}", paper_id, response.text))
    }
}