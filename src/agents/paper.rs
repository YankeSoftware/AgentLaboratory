use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::utils::AgentResult;
use crate::llm::LLMClient;
use super::{Agent, AgentConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperAgentConfig {
    #[serde(flatten)]
    pub base: AgentConfig,
    pub analysis_depth: AnalysisDepth,
    pub extract_code: bool,
    pub extract_math: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AnalysisDepth {
    #[serde(rename = "quick")]
    Quick,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "deep")]
    Deep,
}

pub struct PaperAgent {
    config: PaperAgentConfig,
    llm_client: Arc<Box<dyn LLMClient>>,
}

#[async_trait]
impl Agent for PaperAgent {
    fn new_with_client(base_config: AgentConfig, llm_client: Arc<Box<dyn LLMClient>>) -> Self {
        Self {
            config: PaperAgentConfig {
                base: base_config,
                analysis_depth: AnalysisDepth::Normal,
                extract_code: true,
                extract_math: true,
            },
            llm_client,
        }
    }

    async fn process(&self, paper_content: &str) -> AgentResult<String> {
        // Extract paper sections
        let sections = self.extract_sections(paper_content).await?;
        let mut analysis = format!("\n=== Paper Analysis ===\n\nSections Found: {}\n", sections.len());

        // Analyze each section with LLM
        let system_prompt = match self.config.analysis_depth {
            AnalysisDepth::Quick => "Provide a brief summary of this paper section.",
            AnalysisDepth::Normal => "Analyze this paper section, highlighting key points and methodology.",
            AnalysisDepth::Deep => "Provide a comprehensive analysis of this section, including methodology, results, and implications.",
        };

        for (i, section) in sections.iter().enumerate() {
            let response = self.llm_client.complete(system_prompt, section).await?;
            analysis.push_str(&format!("\nSection {}: {}\n", i + 1, response.text));
        }

        // Extract and analyze code if requested
        if self.config.extract_code {
            let code_snippets = self.extract_code_snippets(paper_content).await?;
            if !code_snippets.is_empty() {
                analysis.push_str("\n=== Code Analysis ===\n");
                for (i, snippet) in code_snippets.iter().enumerate() {
                    let response = self.llm_client
                        .complete(
                            "Analyze this code snippet and explain its purpose:",
                            snippet
                        ).await?;
                    analysis.push_str(&format!("\nSnippet {}: {}\n{}\n", i + 1, snippet, response.text));
                }
            }
        }

        // Extract and analyze mathematical formulas if requested
        if self.config.extract_math {
            let formulas = self.extract_math_formulas(paper_content).await?;
            if !formulas.is_empty() {
                analysis.push_str("\n=== Mathematical Analysis ===\n");
                for (i, formula) in formulas.iter().enumerate() {
                    let response = self.llm_client
                        .complete(
                            "Explain this mathematical formula and its significance:",
                            formula
                        ).await?;
                    analysis.push_str(&format!("\nFormula {}: {}\n{}\n", i + 1, formula, response.text));
                }
            }
        }

        Ok(analysis)
    }

    fn config(&self) -> &AgentConfig {
        &self.config.base
    }

    fn update_config(&mut self, new_config: AgentConfig) {
        self.config.base = new_config;
    }
}

impl PaperAgent {
    pub fn with_depth(mut self, depth: AnalysisDepth) -> Self {
        self.config.analysis_depth = depth;
        self
    }

    pub fn extract_code(mut self, enable: bool) -> Self {
        self.config.extract_code = enable;
        self
    }

    pub fn extract_math(mut self, enable: bool) -> Self {
        self.config.extract_math = enable;
        self
    }

    pub async fn extract_sections(&self, content: &str) -> AgentResult<Vec<String>> {
        // Common section headers in research papers
        let section_headers = [
            "Abstract", "Introduction", "Background", "Related Work",
            "Methods", "Methodology", "Implementation", "Approach",
            "Results", "Evaluation", "Discussion", "Conclusion",
            "Future Work", "References"
        ];

        let mut sections = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        let mut current_section = String::new();
        let mut current_header = "";

        for &line in &lines {
            let line_trimmed = line.trim();
            
            // Check if this line is a section header
            let is_header = section_headers.iter().any(|&header| {
                line_trimmed.to_lowercase().contains(&header.to_lowercase()) &&
                (line_trimmed.len() <= header.len() + 5) // Allow for some numbering/formatting
            });

            if is_header {
                // Save previous section if it exists
                if !current_section.is_empty() {
                    sections.push(format!("=== {} ===\n{}", current_header, current_section.trim()));
                }
                current_header = line_trimmed;
                current_section.clear();
            } else {
                if !line_trimmed.is_empty() {
                    current_section.push_str(line);
                    current_section.push('\n');
                }
            }
        }

        // Add the last section
        if !current_section.is_empty() {
            sections.push(format!("=== {} ===\n{}", current_header, current_section.trim()));
        }

        // If no sections were found, treat the whole content as one section
        if sections.is_empty() {
            sections.push(format!("=== Full Content ===\n{}", content.trim()));
        }

        Ok(sections)
    }

    pub async fn extract_code_snippets(&self, content: &str) -> AgentResult<Vec<String>> {
        if !self.config.extract_code {
            return Ok(vec![]);
        }

        let mut snippets = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut in_code_block = false;
        let mut current_snippet = String::new();

        for &line in &lines {
            let line_trimmed = line.trim();

            // Check for code block markers
            if line_trimmed.starts_with("```") || line_trimmed.starts_with("~~~") {
                if in_code_block {
                    // End of code block
                    if !current_snippet.trim().is_empty() {
                        snippets.push(current_snippet.trim().to_string());
                    }
                    current_snippet.clear();
                }
                in_code_block = !in_code_block;
                continue;
            }

            // Check for indented code blocks (4 spaces or tab)
            let is_indented = line.starts_with("    ") || line.starts_with('\t');

            if in_code_block || is_indented {
                current_snippet.push_str(line);
                current_snippet.push('\n');
            }
        }

        // Add any remaining snippet
        if !current_snippet.trim().is_empty() {
            snippets.push(current_snippet.trim().to_string());
        }

        // Also look for potential inline code snippets
        let inline_code_patterns = [
            "def ", "class ", "fn ", "import ", "from ", "use ",
            "public class", "public static", "function ", "var ", "let ",
            "#include", "int ", "void ", "struct ", "impl "
        ];

        for &line in &lines {
            for pattern in inline_code_patterns.iter() {
                if line.trim().starts_with(pattern) && !snippets.contains(&line.trim().to_string()) {
                    snippets.push(line.trim().to_string());
                }
            }
        }

        Ok(snippets)
    }

    pub async fn extract_math_formulas(&self, content: &str) -> AgentResult<Vec<String>> {
        if !self.config.extract_math {
            return Ok(vec![]);
        }

        let mut formulas = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut in_equation = false;
        let mut current_formula = String::new();

        for &line in &lines {
            let line_trimmed = line.trim();

            // Check for LaTeX equation environments
            if line_trimmed.contains("\\begin{equation}") || line_trimmed.contains("\\[") {
                in_equation = true;
                continue;
            }
            if line_trimmed.contains("\\end{equation}") || line_trimmed.contains("\\]") {
                in_equation = false;
                if !current_formula.trim().is_empty() {
                    formulas.push(current_formula.trim().to_string());
                    current_formula.clear();
                }
                continue;
            }

            // Check for inline math delimiters
            if !in_equation {
                let mut formula = String::new();
                let mut in_inline_math = false;

                for (i, c) in line_trimmed.chars().enumerate() {
                    if i < line_trimmed.len() - 1 {
                        let next_c = line_trimmed.chars().nth(i + 1).unwrap();
                        if c == '$' && next_c != '$' {
                            in_inline_math = !in_inline_math;
                            continue;
                        }
                        if c == '$' && next_c == '$' {
                            in_inline_math = !in_inline_math;
                            continue;
                        }
                    }

                    if in_inline_math {
                        formula.push(c);
                    }
                }

                if !formula.is_empty() {
                    formulas.push(formula);
                }
            } else {
                current_formula.push_str(line_trimmed);
                current_formula.push('\n');
            }
        }

        // Also look for potential mathematical expressions without delimiters
        let math_patterns = [
            "=", "+", "-", "*", "/", "^", "\\sum", "\\prod", "\\int",
            "\\lim", "\\inf", "\\sup", "\\max", "\\min", "\\frac",
            "\\sqrt", "\\alpha", "\\beta", "\\gamma", "\\theta"
        ];

        for &line in &lines {
            let line_trimmed = line.trim();
            if math_patterns.iter().any(|&p| line_trimmed.contains(p)) &&
               !formulas.contains(&line_trimmed.to_string()) {
                formulas.push(line_trimmed.to_string());
            }
        }

        Ok(formulas)
    }
}