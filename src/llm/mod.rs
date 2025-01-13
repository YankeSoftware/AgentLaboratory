use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use crate::utils::AgentResult;

// Cost maps for token counting (taken from Python implementation)
lazy_static::lazy_static! {
    static ref COST_MAP_IN: HashMap<&'static str, f64> = {
        let mut m = HashMap::new();
        m.insert("gpt-4o", 2.50 / 1_000_000.0);
        m.insert("gpt-4o-mini", 0.150 / 1_000_000.0);
        m.insert("o1-preview", 15.00 / 1_000_000.0);
        m.insert("o1-mini", 3.00 / 1_000_000.0);
        m.insert("claude-3-5-sonnet", 3.00 / 1_000_000.0);
        m.insert("deepseek-chat", 1.00 / 1_000_000.0);
        m.insert("o1", 15.00 / 1_000_000.0);
        m
    };

    static ref COST_MAP_OUT: HashMap<&'static str, f64> = {
        let mut m = HashMap::new();
        m.insert("gpt-4o", 10.00 / 1_000_000.0);
        m.insert("gpt-4o-mini", 0.6 / 1_000_000.0);
        m.insert("o1-preview", 60.00 / 1_000_000.0);
        m.insert("o1-mini", 12.00 / 1_000_000.0);
        m.insert("claude-3-5-sonnet", 12.00 / 1_000_000.0);
        m.insert("deepseek-chat", 5.00 / 1_000_000.0);
        m.insert("o1", 60.00 / 1_000_000.0);
        m
    };
}

// Token counting maps
#[derive(Default)]
pub struct TokenCount {
    pub tokens_in: HashMap<String, usize>,
    pub tokens_out: HashMap<String, usize>,
}

impl TokenCount {
    pub fn new() -> Self {
        Self {
            tokens_in: HashMap::new(),
            tokens_out: HashMap::new(),
        }
    }

    pub fn add_tokens(&mut self, model: &str, input_tokens: usize, output_tokens: usize) {
        *self.tokens_in.entry(model.to_string()).or_insert(0) += input_tokens;
        *self.tokens_out.entry(model.to_string()).or_insert(0) += output_tokens;
    }

    pub fn get_cost(&self) -> f64 {
        let mut total = 0.0;
        
        for (model, tokens) in &self.tokens_in {
            if let Some(cost) = COST_MAP_IN.get(model.as_str()) {
                total += *cost * *tokens as f64;
            }
        }

        for (model, tokens) in &self.tokens_out {
            if let Some(cost) = COST_MAP_OUT.get(model.as_str()) {
                total += *cost * *tokens as f64;
            }
        }

        total
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub temperature: Option<f32>,
    pub max_tokens: Option<usize>,
    pub model: String,
    pub version: String,
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            temperature: None,
            max_tokens: None,
            model: "o1-mini".to_string(),
            version: "1.5".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMResponse {
    pub text: String,
    pub tokens_used: usize,
}

#[async_trait]
pub trait LLMClient: Send + Sync {
    /// Initialize a new LLM client
    fn new(api_key: String, config: LLMConfig, token_counter: Arc<Mutex<TokenCount>>) -> Self where Self: Sized;
    
    /// Get a completion from the LLM
    async fn complete(&self, system: &str, prompt: &str) -> AgentResult<LLMResponse>;
    
    /// Get model's name/identifier
    fn model_name(&self) -> String;
}

mod deepseek;

pub use deepseek::DeepSeekClient;