use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::utils::AgentResult;
use crate::llm::LLMClient;

/// Common configuration for all agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub model: String,
    pub temperature: f32,
    pub max_tokens: usize,
    pub api_key: String,
}

/// Core trait that all agents must implement
#[async_trait]
pub trait Agent {
    /// Initialize a new agent with configuration and shared LLM client
    fn new_with_client(config: AgentConfig, llm_client: Arc<Box<dyn LLMClient>>) -> Self where Self: Sized;
    
    /// Process a task and return results
    async fn process(&self, input: &str) -> AgentResult<String>;
    
    /// Get agent's configuration
    fn config(&self) -> &AgentConfig;
    
    /// Update agent's configuration
    fn update_config(&mut self, config: AgentConfig);
}

// Module declarations
pub mod research;
pub mod paper;
pub mod factory;

// Re-export key types
pub use factory::AgentFactory;