use std::sync::Arc;
use anyhow::Result;
use tokio::sync::Mutex;
use crate::llm::{LLMClient, LLMConfig, TokenCount, DeepSeekClient};
use super::{Agent, AgentConfig};
use super::research::ResearchAgent;
use super::paper::PaperAgent;

/// Factory for creating agents with shared resources
pub struct AgentFactory {
    llm_client: Arc<Box<dyn LLMClient>>,
    token_counter: Arc<Mutex<TokenCount>>,
    base_config: AgentConfig,
}

impl AgentFactory {
    pub fn new(api_key: String, config: AgentConfig) -> Result<Self> {
        let token_counter = Arc::new(Mutex::new(TokenCount::new()));
        
        let llm_config = LLMConfig {
            temperature: Some(config.temperature),
            max_tokens: Some(config.max_tokens),
            model: config.model.clone(),
            version: "1.5".to_string(),
        };

        let llm_client: Box<dyn LLMClient> = Box::new(DeepSeekClient::new(
            api_key,
            llm_config,
            token_counter.clone()
        ));

        Ok(Self {
            llm_client: Arc::new(llm_client),
            token_counter,
            base_config: config,
        })
    }

    pub fn create_research_agent(&self) -> ResearchAgent {
        ResearchAgent::new_with_client(
            self.base_config.clone(),
            self.llm_client.clone()
        )
    }

    pub fn create_paper_agent(&self) -> PaperAgent {
        PaperAgent::new_with_client(
            self.base_config.clone(),
            self.llm_client.clone()
        )
    }

    pub fn token_counter(&self) -> Arc<Mutex<TokenCount>> {
        self.token_counter.clone()
    }
}