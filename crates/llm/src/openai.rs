use crate::{ChatMessage, LlmProvider, LlmRequest, LlmResponse};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub struct OpenAiProvider {
    pub api_key: String,
    pub model: String,
    pub endpoint: String,
    pub client: reqwest::Client,
}

#[derive(Serialize)]
struct OpenAiRequest<'a> {
    model: &'a str,
    messages: &'a [ChatMessage],
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
}

#[derive(Deserialize)]
struct OpenAiChoice {
    message: ChatMessage,
}

#[derive(Deserialize)]
struct OpenAiResponse {
    choices: Vec<OpenAiChoice>,
}

impl OpenAiProvider {
    pub fn new(api_key: String, model: Option<String>, endpoint: Option<String>) -> Self {
        Self {
            api_key,
            model: model.unwrap_or_else(|| "gpt-4o-mini".to_string()),
            endpoint: endpoint.unwrap_or_else(|| "https://api.openai.com/v1".to_string()),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LlmProvider for OpenAiProvider {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/chat/completions", self.endpoint);
        let payload = OpenAiRequest {
            model: &self.model,
            messages: &request.messages,
            temperature: request.temperature,
            max_tokens: request.max_tokens,
        };

        let res = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&payload)
            .send()
            .await?;

        if !res.status().is_success() {
            let err_text = res.text().await.unwrap_or_default();
            return Err(format!("OpenAI API Error: {}", err_text).into());
        }

        let resp: OpenAiResponse = res.json().await?;
        let content = resp
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        Ok(LlmResponse { content })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openai_provider_init() {
        let provider = OpenAiProvider::new("test_key".to_string(), None, None);
        assert_eq!(provider.model, "gpt-4o-mini");
        assert_eq!(provider.endpoint, "https://api.openai.com/v1");
    }
}
