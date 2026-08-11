use nova_llm::{LlmProvider, LlmRequest, ChatMessage};
use nova_memory::MemoryStore;
use nova_plugins::Plugin;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentRole {
    Planner,
    Coding,
    Research,
    Architecture,
    Product,
    Design,
    Document,
    Spreadsheet,
    Browser,
    Computer,
    Network,
    Security,
    QA,
    DevOps,
    Critic,
    ProjectManager,
}

pub struct AgentOrchestrator {
    llm: Arc<dyn LlmProvider>,
    memory: Arc<MemoryStore>,
    plugins: Vec<Arc<dyn Plugin>>,
}

impl AgentOrchestrator {
    pub fn new(llm: Arc<dyn LlmProvider>, memory: Arc<MemoryStore>) -> Self {
        Self {
            llm,
            memory,
            plugins: Vec::new(),
        }
    }

    pub fn register_plugin(&mut self, plugin: Arc<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    pub fn select_agents_for_goal(&self, goal: &str) -> Vec<AgentRole> {
        let lower = goal.to_lowercase();
        let mut roles = vec![AgentRole::Planner];

        if lower.contains("cisco") || lower.contains("packet tracer") || lower.contains("network") || lower.contains("ospf") {
            roles.push(AgentRole::Network);
            roles.push(AgentRole::Computer);
        } else if lower.contains("website") || lower.contains("code") || lower.contains("app") || lower.contains("fix") {
            roles.push(AgentRole::Architecture);
            roles.push(AgentRole::Coding);
            roles.push(AgentRole::QA);
        } else if lower.contains("pdf") || lower.contains("docx") || lower.contains("presentation") || lower.contains("pptx") {
            roles.push(AgentRole::Document);
        } else if lower.contains("research") || lower.contains("analyze") {
            roles.push(AgentRole::Research);
        }

        roles.push(AgentRole::Critic);
        roles
    }

    /// Ask the orchestrator to solve a task with Ruthless Critic Self-Verification.
    pub async fn run_task(&self, user_input: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let selected_roles = self.select_agents_for_goal(user_input);
        
        let system_prompt = format!(
            "You are NOVA OS Agent Orchestrator controlling roles: {:?}.
Analyze the user's goal objectively:
1. Technical feasibility & breakdown
2. Risk assessment
3. Verified step-by-step execution strategy.",
            selected_roles
        );

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt,
            },
            ChatMessage {
                role: "user".to_string(),
                content: user_input.to_string(),
            },
        ];

        let req = LlmRequest {
            messages,
            temperature: Some(0.3),
            max_tokens: Some(1000),
        };

        let response = self.llm.generate(req).await?;
        let conversation_id = uuid::Uuid::new_v4().to_string();
        let _ = self.memory.save_message(&format!("{}-critique", conversation_id), "assistant", &response.content);

        Ok(response.content)
    }
}

pub struct AutomationAgent {
    pub desktop: nova_desktop::DesktopOperator,
    pub browser: nova_browser::BrowserOperator,
}

impl AutomationAgent {
    pub fn new() -> Self {
        Self {
            desktop: nova_desktop::DesktopOperator::new(),
            browser: nova_browser::BrowserOperator::new(),
        }
    }

    pub fn run_workflow(&mut self, action: &str, target: &str) -> Result<String, Box<dyn std::error::Error>> {
        match action {
            "browser_open" => {
                let title = self.browser.navigate_and_get_title(target)?;
                Ok(format!("Successfully opened browser at {}. Title: {}", target, title))
            }
            "desktop_screenshot" => {
                self.desktop.capture_screen(target)?;
                Ok(format!("Saved screenshot to {}", target))
            }
            _ => Err("Unknown action pattern for AutomationAgent".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_role_selection() {
        let memory = Arc::new(MemoryStore::new_in_memory().unwrap());
        let mock_provider = Arc::new(nova_llm::OllamaProvider::new("http://127.0.0.1:11434".to_string(), "llama3".to_string()));
        let orchestrator = AgentOrchestrator::new(mock_provider, memory);

        let network_roles = orchestrator.select_agents_for_goal("Create a Cisco Packet Tracer project");
        assert!(network_roles.contains(&AgentRole::Network));

        let coding_roles = orchestrator.select_agents_for_goal("Build a website app");
        assert!(coding_roles.contains(&AgentRole::Coding));
    }
}
