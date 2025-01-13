use std::sync::Arc;
use async_trait::async_trait;
use serde_json::json;
use tokio::sync::Mutex;
use tracing::{debug, info, warn, error};
use reqwest::{Client, header};
use exponential_backoff::Backoff;
use tiktoken_rs::cl100k_base;

use crate::utils::{AgentError, AgentResult};
use super::{LLMClient, LLMConfig, LLMResponse, TokenCount, Message};

const DEEPSEEK_API_URL: &str = "https://api.deepseek.com/v1/chat/completions";
const MAX_RETRIES: u32 = 5;
const INITIAL_TIMEOUT_MS: u64 = 1000;

pub struct DeepSeekClient {
    config: LLMConfig,
    client: Client,
    token_counter: Arc<Mutex<TokenCount>>,
}

#[async_trait]
impl LLMClient for DeepSeekClient {
    fn new(api_key: String, config: LLMConfig, token_counter: Arc<Mutex<TokenCount>>) -> Self {
        let mut headers = header::HeaderMap::new();
        
        // Remove any existing Bearer prefix and add it consistently
        let clean_key = api_key.trim_start_matches("Bearer ").trim();
        let auth_header = format!("Bearer {}", clean_key);
        
        // Debug logging
        info!("API Key format: {}", if clean_key.starts_with("sk-") {
            "starts with sk-"
        } else {
            "does not start with sk-"
        });
        debug!("API URL: {}", DEEPSEEK_API_URL);
        
        // Add authorization header
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&auth_header)
                .expect("Invalid API key format"),
        );
        
        // Add content type header
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        Self {
            config,
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .default_headers(headers)
                .build()
                .expect("Failed to create HTTP client"),
            token_counter,
        }
    }

    async fn complete(&self, system: &str, prompt: &str) -> AgentResult<LLMResponse> {
        debug!("Sending prompt to DeepSeek: {}", prompt);

        // Count input tokens
        let bpe = cl100k_base().unwrap();
        let input_tokens = bpe.encode_with_special_tokens(&format!("{}{}", system, prompt)).len();

        let backoff = Backoff::new(
            MAX_RETRIES,
            std::time::Duration::from_millis(INITIAL_TIMEOUT_MS),
            Some(std::time::Duration::from_secs(30))
        );
        let mut last_error = None;
        let mut attempt = 0;

        while let Some(duration) = backoff.next(attempt) {
            attempt += 1;
            let messages = vec![
                Message { 
                    role: "system".into(), 
                    content: system.into() 
                },
                Message { 
                    role: "user".into(), 
                    content: prompt.into() 
                },
            ];

            // Match the exact request format from the working implementation
            let request = json!({
                "model": "deepseek-chat",
                "messages": messages,
                "temperature": 0.7
            });

            // Log request details
            debug!("DeepSeek Request:\nURL: {}\nAuth Header: Bearer sk-...\nRequest body: {:?}", 
                   DEEPSEEK_API_URL,
                   request);

            match self.client
                .post(DEEPSEEK_API_URL)
                .json(&request)
                .send()
                .await
            {
                Ok(response) => {
                    if !response.status().is_success() {
                        let status = response.status();
                        let headers = response.headers().clone();
                        let body = response.text().await.unwrap_or_default();
                        
                        // Log full response information
                        error!("DeepSeek API Error Response:\n\
                               Status: {}\n\
                               Headers: {}\n\
                               Body: {}\n\
                               Request URL: {}\n\
                               Auth Format: Bearer sk-...",
                            status,
                            headers.iter()
                                  .map(|(k, v)| format!("{}: {}", k, v.to_str().unwrap_or("invalid")))
                                  .collect::<Vec<_>>()
                                  .join("\n"),
                            body,
                            DEEPSEEK_API_URL
                        );

                        let error = format!("DeepSeek API error: {} - {}", status, body);
                        last_error = Some(AgentError::Api(error));
                        tokio::time::sleep(duration).await;
                        continue;
                    }

                    match response.json::<serde_json::Value>().await {
                        Ok(response_data) => {
                            let text = response_data["choices"][0]["message"]["content"]
                                .as_str()
                                .ok_or_else(|| AgentError::Api("Missing response text".into()))?
                                .to_string();

                            let tokens_used = response_data["usage"]["total_tokens"]
                                .as_u64()
                                .ok_or_else(|| AgentError::Api("Missing token count".into()))? as usize;

                            // Update token counter
                            let mut counter = self.token_counter.lock().await;
                            counter.add_tokens(self.model_name().as_str(), input_tokens, tokens_used - input_tokens);
                            info!("Current cost: ${:.4}", counter.get_cost());

                            return Ok(LLMResponse { text, tokens_used });
                        }
                        Err(e) => {
                            let error = format!("Failed to parse DeepSeek response: {}", e);
                            warn!("{}", error);
                            last_error = Some(AgentError::Api(error));
                            tokio::time::sleep(duration).await;
                            continue;
                        }
                    }
                }
                Err(e) => {
                    let error = format!("DeepSeek request failed: {}", e);
                    warn!("{}", error);
                    last_error = Some(AgentError::Api(error));
                    tokio::time::sleep(duration).await;
                    continue;
                }
            }
        }

        Err(last_error.unwrap_or_else(|| AgentError::Api("Max retries exceeded".into())))
    }

    fn model_name(&self) -> String {
        "deepseek-chat".into()
    }
}