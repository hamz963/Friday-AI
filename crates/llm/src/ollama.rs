use crate::{ChatMessage, LlmProvider, LlmRequest, LlmResponse};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub struct OllamaProvider {
    pub endpoint: String,
    pub model: String,
    pub client: reqwest::Client,
}

#[derive(Serialize)]
struct OllamaRequest<'a> {
    model: &'a str,
    messages: &'a [ChatMessage],
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaMessage {
    content: String,
}

#[derive(Deserialize)]
struct OllamaResponse {
    message: OllamaMessage,
}

impl OllamaProvider {
    pub fn new(model: Option<String>, endpoint: Option<String>) -> Self {
        Self {
            model: model.unwrap_or_else(|| "llama3.2".to_string()),
            endpoint: endpoint.unwrap_or_else(|| "http://localhost:11434".to_string()),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LlmProvider for OllamaProvider {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/api/chat", self.endpoint);
        let payload = OllamaRequest {
            model: &self.model,
            messages: &request.messages,
            stream: false,
        };

        let res = self.client.post(&url).json(&payload).send().await?;
        if !res.status().is_success() {
            let err_text = res.text().await.unwrap_or_default();
            return Err(format!("Ollama API Error: {}", err_text).into());
        }

        let resp: OllamaResponse = res.json().await?;
        Ok(LlmResponse {
            content: resp.message.content,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ollama_provider_init() {
        let provider = OllamaProvider::new(None, None);
        assert_eq!(provider.model, "llama3.2");
        assert_eq!(provider.endpoint, "http://localhost:11434");
    }
}
