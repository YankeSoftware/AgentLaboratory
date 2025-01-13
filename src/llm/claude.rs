use async_trait::async_trait;
use serde_json::json;
use tracing::{debug, info};
use reqwest::Client;
use crate::utils::{AgentError, AgentResult};
use super::{LLMClient, LLMConfig, LLMResponse};

const CLAUDE_API_URL: &str = "https://api.anthropic.com/v1/messages";
const CLAUDE_MODEL: &str = "claude-3-opus-20240229";

pub struct ClaudeClient {
    api_key: String,
    config: LLMConfig,
    client: Client,
}

#[async_trait]
impl LLMClient for ClaudeClient {
    fn new(api_key: String, config: LLMConfig) -> Self {
        Self {
            api_key,
            config,
            client: Client::new(),
        }
    }

    async fn complete(&self, prompt: &str) -> AgentResult<LLMResponse> {
        debug!("Sending prompt to Claude: {}", prompt);

        let request = json!({
            "model": CLAUDE_MODEL,
            "messages": [{
                "role": "user",
                "content": prompt
            }],
            "max_tokens": self.config.max_tokens,
            "temperature": self.config.temperature
        });

        let response = self.client
            .post(CLAUDE_API_URL)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&request)
            .send()
            .await
            .map_err(|e| AgentError::Api(format!("Claude request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AgentError::Api(format!(
                "Claude API error: {} - {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        let response_data = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| AgentError::Api(format!("Failed to parse Claude response: {}", e)))?;

        // Extract response text and token usage
        let text = response_data["content"][0]["text"]
            .as_str()
            .ok_or_else(|| AgentError::Api("Missing response text".into()))?
            .to_string();

        let tokens_used = response_data["usage"]["total_tokens"]
            .as_u64()
            .ok_or_else(|| AgentError::Api("Missing token count".into()))? as usize;

        info!("Claude response received: {} tokens used", tokens_used);
        Ok(LLMResponse { text, tokens_used })
    }

    fn model_name(&self) -> String {
        CLAUDE_MODEL.into()
    }
}