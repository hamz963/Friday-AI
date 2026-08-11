use crate::{ChatMessage, LlmProvider, LlmRequest, LlmResponse};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub struct AnthropicProvider {
    pub api_key: String,
    pub model: String,
    pub client: reqwest::Client,
}

#[derive(Serialize)]
struct AnthropicMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Serialize)]
struct AnthropicRequest<'a> {
    model: &'a str,
    max_tokens: u32,
    messages: Vec<AnthropicMessage<'a>>,
}

#[derive(Deserialize)]
struct AnthropicContentBlock {
    text: Option<String>,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContentBlock>,
}

impl AnthropicProvider {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        Self {
            api_key,
            model: model.unwrap_or_else(|| "claude-3-5-sonnet-20241022".to_string()),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LlmProvider for AnthropicProvider {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, Box<dyn Error + Send + Sync>> {
        let url = "https://api.anthropic.com/v1/messages";
        
        let messages: Vec<AnthropicMessage> = request.messages.iter().map(|m| {
            let role = if m.role == "user" { "user" } else { "assistant" };
            AnthropicMessage {
                role,
                content: &m.content,
            }
        }).collect();

        let payload = AnthropicRequest {
            model: &self.model,
            max_tokens: request.max_tokens.unwrap_or(4096),
            messages,
        };

        let res = self.client
            .post(url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if !res.status().is_success() {
            let err_text = res.text().await.unwrap_or_default();
            return Err(format!("Anthropic API error: {}", err_text).into());
        }

        let resp_json: AnthropicResponse = res.json().await?;
        let output_text = resp_json.content
            .first()
            .and_then(|c| c.text.clone())
            .unwrap_or_default();

        Ok(LlmResponse { content: output_text })
    }
}
