//! Agent Laboratory - A research paper analysis and ML experimentation framework
//! 
//! This library provides tools for automated research paper analysis,
//! machine learning experimentation, and scientific workflow automation.

pub mod agents;
pub mod utils;
pub mod tools;
pub mod llm;

// Re-export commonly used items
pub use agents::Agent;
pub use utils::{AgentError, AgentResult};
pub use llm::{LLMClient, LLMConfig, LLMResponse};