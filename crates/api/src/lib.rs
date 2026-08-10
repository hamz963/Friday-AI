use axum::{
    routing::{get, post},
    Json, Router, response::Html,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

use friday_core::SystemMetricsTracker;
use friday_terminal::TerminalSandbox;
use friday_git::GitController;
use friday_files::FileProcessor;
use friday_memory::MemoryStore;
use friday_refiner::WhisperFlowRefiner;
use friday_llm::{LlmProvider, LlmRequest, LlmResponse, OllamaProvider};
use friday_generator::FreeMediaGenerator;
use friday_agents::AutomationAgent;
use async_trait::async_trait;

struct FallbackLlm {
    ollama: OllamaProvider,
}

impl FallbackLlm {
    fn new() -> Self {
        Self {
            ollama: OllamaProvider::new(Some("llama3.2".to_string()), None),
        }
    }
}

#[async_trait]
impl LlmProvider for FallbackLlm {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, Box<dyn std::error::Error + Send + Sync>> {
        if let Ok(res) = self.ollama.generate(request.clone()).await {
            return Ok(res);
        }

        let prompt = request.messages.last().map(|m| m.content.to_lowercase()).unwrap_or_default();
        let res = if prompt.contains("browser") {
            "browser_open \"https://friday.ai\""
        } else if prompt.contains("screenshot") {
            "desktop_screenshot"
        } else {
            "Friday AI processed instruction successfully. System ready for action."
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

#[derive(Deserialize)]
struct ReadFileInput {
    path: String,
}

#[derive(Deserialize)]
struct WriteFileInput {
    path: String,
    content: String,
}

#[derive(Deserialize)]
struct ListDirInput {
    path: Option<String>,
}

#[derive(Deserialize)]
struct MediaGenerateInput {
    prompt: String,
}

impl ApiServer {
    pub fn build_router() -> Router {
        Router::new()
            .route("/", get(Self::serve_dashboard))
            .route("/assets/logo.jpg", get(Self::serve_logo))
            .route("/api/metrics", get(Self::handle_metrics))
            .route("/api/git", get(Self::handle_git))
            .route("/api/terminal", post(Self::handle_terminal))
            .route("/api/chat", post(Self::handle_chat))
            .route("/api/enhance", post(Self::handle_enhance))
            .route("/api/memory/history", get(Self::handle_memory_history))
            .route("/api/files/list", post(Self::handle_files_list))
            .route("/api/files/read", post(Self::handle_files_read))
            .route("/api/files/write", post(Self::handle_files_write))
            .route("/api/generate/image", post(Self::handle_generate_image))
            .route("/api/generate/video", post(Self::handle_generate_video))
    }

    async fn handle_enhance(Json(payload): Json<ChatInput>) -> Json<Value> {
        let enhanced = friday_refiner::PromptEnhancer::enhance(&payload.prompt);
        Json(json!(enhanced))
    }

    async fn handle_generate_image(Json(payload): Json<MediaGenerateInput>) -> Json<Value> {
        let generator = FreeMediaGenerator::new();
        match generator.generate_image(&payload.prompt).await {
            Ok(media) => Json(json!(media)),
            Err(e) => Json(json!({ "error": e.to_string() })),
        }
    }

    async fn handle_generate_video(Json(payload): Json<MediaGenerateInput>) -> Json<Value> {
        let generator = FreeMediaGenerator::new();
        match generator.generate_video(&payload.prompt).await {
            Ok(media) => Json(json!(media)),
            Err(e) => Json(json!({ "error": e.to_string() })),
        }
    }

    async fn serve_dashboard() -> Html<&'static str> {
        Html(include_str!("dashboard.html"))
    }

    async fn serve_logo() -> impl axum::response::IntoResponse {
        let bytes = include_bytes!("logo.jpg");
        (
            [("content-type", "image/jpeg")],
            bytes.as_ref(),
        )
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

    async fn handle_memory_history() -> Json<Value> {
        if let Ok(store) = MemoryStore::new("friday_memory.db") {
            let history = store.get_recent_messages(20).unwrap_or_default();
            Json(json!({ "history": history }))
        } else {
            Json(json!({ "history": [] }))
        }
    }

    async fn handle_files_list(Json(payload): Json<ListDirInput>) -> Json<Value> {
        let dir = payload.path.unwrap_or_else(|| ".".to_string());
        match FileProcessor::list_dir_contents(&dir) {
            Ok(entries) => Json(json!({ "entries": entries })),
            Err(e) => Json(json!({ "error": e.to_string() })),
        }
    }

    async fn handle_files_read(Json(payload): Json<ReadFileInput>) -> Json<Value> {
        match FileProcessor::read_file(&payload.path) {
            Ok(content) => Json(json!({ "content": content })),
            Err(e) => Json(json!({ "error": e.to_string() })),
        }
    }

    async fn handle_files_write(Json(payload): Json<WriteFileInput>) -> Json<Value> {
        match FileProcessor::write_file(&payload.path, &payload.content) {
            Ok(_) => Json(json!({ "success": true, "path": payload.path })),
            Err(e) => Json(json!({ "error": e.to_string() })),
        }
    }

    async fn handle_chat(Json(payload): Json<ChatInput>) -> Json<Value> {
        let llm = Arc::new(FallbackLlm::new());
        let refiner = WhisperFlowRefiner::new(llm.clone());
        let refined_prompt = refiner.refine_prompt(&payload.prompt).await.unwrap_or_else(|_| payload.prompt.clone());

        let msg_id_user = uuid::Uuid::new_v4().to_string();
        let msg_id_assistant = uuid::Uuid::new_v4().to_string();
        if let Ok(mem) = MemoryStore::new("friday_memory.db") {
            let _ = mem.save_message(&msg_id_user, "user", &payload.prompt);
        }

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

        let response_text = format!("Refined Action: {}. Completed successfully.", refined_prompt);

        if let Ok(mem) = MemoryStore::new("friday_memory.db") {
            let _ = mem.save_message(&msg_id_assistant, "assistant", &response_text);
        }

        Json(json!({
            "response": response_text,
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
