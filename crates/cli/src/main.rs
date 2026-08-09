use friday_core::{AppConfig, detect_hardware};
use friday_api::ApiServer;
use std::net::SocketAddr;
use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Booting Friday AI Operating System ===");

    // 1. Detect hardware & settings
    let hw = detect_hardware();
    println!("Hardware Detected: Platform={}, CPU={}, RAM={:.1}GB", hw.platform, hw.cpu_brand, hw.ram_gb);

    let config = AppConfig::default();
    println!("Loaded Settings: wake_word='{}', model='{}'", config.voice.wake_word, config.llm.model);

    // 2. Build local server address
    let port = 8080;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    
    // 3. Open browser dashboard automatically
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
        println!("Opening Developer Control Dashboard in your default browser...");
        let url = format!("http://127.0.0.1:{}", port);
        
        let _ = if cfg!(target_os = "windows") {
            Command::new("cmd").args(&["/C", "start", &url]).status()
        } else if cfg!(target_os = "macos") {
            Command::new("open").arg(&url).status()
        } else {
            Command::new("xdg-open").arg(&url).status()
        };
    });

    // 4. Run Axum server
    ApiServer::run_server(addr).await?;

    Ok(())
}
