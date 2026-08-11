use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskCategory {
    Coding,
    ArchitectureReasoning,
    DeepResearch,
    DocumentGeneration,
    UIAnalysisVision,
    DeterministicOperation, // Skip LLM!
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelProfile {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub supports_vision: bool,
    pub supports_coding: bool,
    pub supports_reasoning: bool,
    pub is_local: bool,
    pub cost_per_1k_tokens: f32,
    pub latency_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtaskRoutingDecision {
    pub subtask_id: String,
    pub category: TaskCategory,
    pub selected_model: ModelProfile,
    pub estimated_cost: f32,
    pub is_deterministic: bool,
}

pub struct ModelOptimizer {
    pub available_models: Vec<ModelProfile>,
    pub daily_budget: f32,
    pub current_spent: f32,
}

impl ModelOptimizer {
    pub fn new(daily_budget: f32) -> Self {
        let models = vec![
            ModelProfile {
                id: "claude-3-7-sonnet".to_string(),
                name: "Claude 3.7 Sonnet".to_string(),
                provider: "Anthropic".to_string(),
                supports_vision: true,
                supports_coding: true,
                supports_reasoning: true,
                is_local: false,
                cost_per_1k_tokens: 0.003,
                latency_ms: 350,
            },
            ModelProfile {
                id: "gpt-4o".to_string(),
                name: "GPT-4o Flagship".to_string(),
                provider: "OpenAI".to_string(),
                supports_vision: true,
                supports_coding: true,
                supports_reasoning: true,
                is_local: false,
                cost_per_1k_tokens: 0.0025,
                latency_ms: 280,
            },
            ModelProfile {
                id: "deepseek-r1".to_string(),
                name: "DeepSeek R1 Reasoning".to_string(),
                provider: "DeepSeek".to_string(),
                supports_vision: false,
                supports_coding: true,
                supports_reasoning: true,
                is_local: false,
                cost_per_1k_tokens: 0.0005,
                latency_ms: 450,
            },
            ModelProfile {
                id: "ollama-llama3".to_string(),
                name: "Ollama Llama 3 8B (Local)".to_string(),
                provider: "Ollama".to_string(),
                supports_vision: false,
                supports_coding: true,
                supports_reasoning: false,
                is_local: true,
                cost_per_1k_tokens: 0.0,
                latency_ms: 80,
            },
        ];

        Self {
            available_models: models,
            daily_budget,
            current_spent: 0.0,
        }
    }

    pub fn classify_task(&self, prompt: &str) -> TaskCategory {
        let lower = prompt.to_lowercase();
        if (lower.contains("rename") && lower.contains("file")) || lower.contains("calculate") || lower.contains("math") {
            TaskCategory::DeterministicOperation
        } else if lower.contains("vision") || lower.contains("screenshot") || lower.contains("ui") {
            TaskCategory::UIAnalysisVision
        } else if lower.contains("research") || lower.contains("search") || lower.contains("paper") {
            TaskCategory::DeepResearch
        } else if lower.contains("architect") || lower.contains("design system") || lower.contains("plan") {
            TaskCategory::ArchitectureReasoning
        } else {
            TaskCategory::Coding
        }
    }

    pub fn route_subtask(&self, subtask_id: &str, prompt: &str) -> SubtaskRoutingDecision {
        let category = self.classify_task(prompt);

        if category == TaskCategory::DeterministicOperation {
            return SubtaskRoutingDecision {
                subtask_id: subtask_id.to_string(),
                category,
                selected_model: self.available_models[3].clone(), // Ollama Local 0 token cost
                estimated_cost: 0.0,
                is_deterministic: true,
            };
        }

        let model = match category {
            TaskCategory::ArchitectureReasoning => &self.available_models[2], // DeepSeek R1
            TaskCategory::UIAnalysisVision => &self.available_models[0],       // Claude 3.7 Sonnet
            TaskCategory::DeepResearch => &self.available_models[1],           // GPT-4o
            _ => &self.available_models[0],                                    // Claude 3.7
        };

        SubtaskRoutingDecision {
            subtask_id: subtask_id.to_string(),
            category,
            selected_model: model.clone(),
            estimated_cost: model.cost_per_1k_tokens * 2.0,
            is_deterministic: false,
        }
    }
}
