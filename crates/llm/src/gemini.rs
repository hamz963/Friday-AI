use crate::{LlmProvider, LlmRequest, LlmResponse};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub struct GeminiProvider {
    pub api_key: String,
    pub model: String,
    pub client: reqwest::Client,
}

#[derive(Serialize)]
struct GeminiPart<'a> {
    text: &'a str,
}

#[derive(Serialize)]
struct GeminiContent<'a> {
    parts: Vec<GeminiPart<'a>>,
}

#[derive(Serialize)]
struct GeminiRequest<'a> {
    contents: Vec<GeminiContent<'a>>,
}

#[derive(Deserialize)]
struct GeminiPartResp {
    text: Option<String>,
}

#[derive(Deserialize)]
struct GeminiContentResp {
    parts: Vec<GeminiPartResp>,
}

#[derive(Deserialize)]
struct GeminiCandidate {
    content: GeminiContentResp,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<GeminiCandidate>,
}

impl GeminiProvider {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        Self {
            api_key,
            model: model.unwrap_or_else(|| "gemini-2.5-flash".to_string()),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LlmProvider for GeminiProvider {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, Box<dyn Error + Send + Sync>> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model, self.api_key
        );

        let contents: Vec<GeminiContent> = request.messages.iter().map(|m| {
            GeminiContent {
                parts: vec![GeminiPart { text: &m.content }],
            }
        }).collect();

        let payload = GeminiRequest { contents };

        let res = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await?;

        if !res.status().is_success() {
            let err_text = res.text().await.unwrap_or_default();
            return Err(format!("Gemini API error: {}", err_text).into());
        }

        let resp_json: GeminiResponse = res.json().await?;
        let output_text = resp_json.candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .and_then(|p| p.text.clone())
            .unwrap_or_default();

        Ok(LlmResponse { content: output_text })
    }
}
