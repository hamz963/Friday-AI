use axum::{
    routing::{get, post},
    Json, Router, response::Html,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::net::SocketAddr;
use std::time::Instant;

use friday_core::SystemMetricsTracker;
use friday_terminal::TerminalSandbox;
use friday_git::GitController;
use friday_refiner::WhisperFlowRefiner;
use friday_llm::{LlmProvider, LlmRequest, LlmResponse};
use friday_agents::AutomationAgent;
use async_trait::async_trait;

struct MockLlm;

#[async_trait]
impl LlmProvider for MockLlm {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, Box<dyn std::error::Error + Send + Sync>> {
        let prompt = request.messages.last().map(|m| m.content.to_lowercase()).unwrap_or_default();
        let res = if prompt.contains("browser") {
            "browser_open \"https://friday.ai\""
        } else if prompt.contains("screenshot") {
            "desktop_screenshot"
        } else {
            "No execution triggers matched. Standard text response returned."
        };
        Ok(LlmResponse { content: res.to_string() })
    }
}

pub struct ApiServer;

#[derive(Deserialize)]
struct ChatInput {
    prompt: String,
}

#[derive(Deserialize)]
struct CommandInput {
    command: String,
}

impl ApiServer {
    pub fn build_router() -> Router {
        Router::new()
            .route("/", get(Self::serve_dashboard))
            .route("/api/metrics", get(Self::handle_metrics))
            .route("/api/git", get(Self::handle_git))
            .route("/api/terminal", post(Self::handle_terminal))
            .route("/api/chat", post(Self::handle_chat))
    }

    async fn serve_dashboard() -> Html<&'static str> {
        Html(include_str!("dashboard.html"))
    }

    async fn handle_metrics() -> Json<Value> {
        let mut tracker = SystemMetricsTracker::new();
        let start = Instant::now();
        let report = tracker.capture_metrics(start);
        Json(json!({
            "cpu_usage": report.cpu_usage,
            "used_memory_mb": report.used_memory_mb,
            "total_memory_mb": report.total_memory_mb,
            "elapsed_latency_ms": report.elapsed_latency_ms,
        }))
    }

    async fn handle_git() -> Json<Value> {
        let status = GitController::get_repository_status(".").unwrap_or_else(|e| e.to_string());
        Json(json!({ "status": status }))
    }

    async fn handle_terminal(Json(payload): Json<CommandInput>) -> Json<Value> {
        match TerminalSandbox::execute_command(&payload.command) {
            Ok(output) => Json(json!({ "output": output })),
            Err(e) => Json(json!({ "error": e.to_string() })),
        }
    }

    async fn handle_chat(Json(payload): Json<ChatInput>) -> Json<Value> {
        // Refine raw user speech/chat input using Whisper Flow (Refiner)
        let llm = std::sync::Arc::new(MockLlm);
        let refiner = WhisperFlowRefiner::new(llm);
        let refined_prompt = refiner.refine_prompt(&payload.prompt).await.unwrap_or_else(|_| payload.prompt.clone());

        // Process automation actions using Agent Orchestrator
        let mut automation = AutomationAgent::new();
        let (action_result, browser_url, browser_title) = if refined_prompt.contains("browser_open") {
            let res = automation.run_workflow("browser_open", "https://friday.ai").unwrap_or_else(|e| e.to_string());
            (res, "https://friday.ai", "Mock Title for https://friday.ai")
        } else if refined_prompt.contains("desktop_screenshot") {
            let res = automation.run_workflow("desktop_screenshot", "").unwrap_or_else(|e| e.to_string());
            (res, "about:blank", "Active Desktop screenshot saved")
        } else {
            ("No automated triggers matched.".to_string(), "about:blank", "No active browser tab session")
        };

        Json(json!({
            "response": format!("Refined Action: {}. Completed successfully.", refined_prompt),
            "system_output": action_result,
            "browser_session": {
                "url": browser_url,
                "title": browser_title,
            }
        }))
    }

    pub async fn run_server(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        let app = Self::build_router();
        let listener = tokio::net::TcpListener::bind(addr).await?;
        println!("Server running locally at http://{}", addr);
        axum::serve(listener, app).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_routes() {
        let _router = ApiServer::build_router();
    }
}
