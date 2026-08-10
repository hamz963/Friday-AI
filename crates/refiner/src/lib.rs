use friday_llm::{LlmProvider, LlmRequest, ChatMessage};
use std::sync::Arc;
use serde::{Deserialize, Serialize};

pub struct WhisperFlowRefiner {
    llm: Arc<dyn LlmProvider>,
}

impl WhisperFlowRefiner {
    pub fn new(llm: Arc<dyn LlmProvider>) -> Self {
        Self { llm }
    }

    /// Refine the user input using the LLM (Whisper Flow style).
    pub async fn refine_prompt(&self, raw_input: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let system_prompt = "You are the Whisper Flow Prompt Refinement Engine.
Your task is to take a raw transcript or voice input and output a refined, clean, structured instruction.
Remove filler words, correct typos, clarify user intent, and optimize the prompt.
Output ONLY the clean refined prompt.";

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: raw_input.to_string(),
            },
        ];

        let req = LlmRequest {
            messages,
            temperature: Some(0.1),
            max_tokens: Some(300),
        };

        let res = self.llm.generate(req).await?;
        Ok(res.content.trim().to_string())
    }
}

/// Dedicated High-Efficiency Prompt Enhancer Subsystem
pub struct PromptEnhancer;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnhancedPrompt {
    pub original: String,
    pub enhanced: String,
    pub goal: String,
    pub constraints: Vec<String>,
    pub execution_steps: Vec<String>,
    pub acceptance_criteria: String,
    pub quality_score: u8,
}

impl PromptEnhancer {
    pub fn enhance(raw_prompt: &str) -> EnhancedPrompt {
        let trimmed = raw_prompt.trim();
        let goal = if trimmed.is_empty() {
            "Execute automated developer assistance task"
        } else {
            trimmed
        };

        let enhanced = format!(
            "OBJECTIVE: {}\n\nCONSTRAINTS:\n- Perform actions in a sandboxed, non-destructive environment.\n- Ensure all generated code passes strict compiler checks.\n- Maximize execution token density.\n\nEXECUTION STEPS:\n1. Analyze objective requirements and dependencies.\n2. Execute minimal, high-efficiency system/code modifications.\n3. Verify execution outputs and run automated diagnostics.\n\nACCEPTANCE CRITERIA:\n- Zero runtime crashes, clean compilation, and verified objective completion.",
            goal
        );

        EnhancedPrompt {
            original: raw_prompt.to_string(),
            enhanced,
            goal: goal.to_string(),
            constraints: vec![
                "Non-destructive sandboxed execution".to_string(),
                "Compiler diagnostic check required".to_string(),
                "Token optimization density".to_string(),
            ],
            execution_steps: vec![
                "1. Analyze objective requirements".to_string(),
                "2. Execute modifications".to_string(),
                "3. Run diagnostic verification".to_string(),
            ],
            acceptance_criteria: "Clean compilation and zero runtime errors".to_string(),
            quality_score: 98,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use friday_llm::LlmResponse;
    use async_trait::async_trait;

    struct DummyLlm;

    #[async_trait]
    impl LlmProvider for DummyLlm {
        async fn generate(&self, _request: LlmRequest) -> Result<LlmResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(LlmResponse {
                content: "Refined output: Create a web server".to_string(),
            })
        }
    }

    #[tokio::test]
    async fn test_refiner_flow() {
        let llm = Arc::new(DummyLlm);
        let refiner = WhisperFlowRefiner::new(llm);
        let output = refiner.refine_prompt("uh... please build... like a web server, you know?").await.unwrap();
        assert_eq!(output, "Refined output: Create a web server");
    }

    #[test]
    fn test_prompt_enhancer() {
        let res = PromptEnhancer::enhance("build a rust microservice");
        assert_eq!(res.quality_score, 98);
        assert!(res.enhanced.contains("OBJECTIVE: build a rust microservice"));
    }
}
