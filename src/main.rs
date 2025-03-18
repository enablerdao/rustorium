use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // ロギングの設定
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    // 既存のプロセスをクリーンアップ
    info!("Cleaning up any existing processes...");
    let _ = Command::new("pkill")
        .args(["-f", "target/debug/standalone_api"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    
    let _ = Command::new("pkill")
        .args(["-f", "target/debug/web"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
        
    let _ = Command::new("pkill")
        .args(["-f", "target/debug/rustorium"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // APIサーバーを起動
    info!("Starting API server...");
    let api_port = 50128;
    let _api_process = Command::new("cargo")
        .current_dir("api")
        .args(["run"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;
    
    info!("API server starting on port: {}", api_port);
    
    // 少し待機
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    
    // WebUIサーバーを起動
    info!("Starting Web UI server...");
    let web_port = 55560;
    let _web_process = Command::new("cargo")
        .current_dir("web")
        .args(["run"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;
    
    info!("Web UI server starting on port: {}", web_port);
    
    // 少し待機
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    
    info!("All services started!");
    info!("API server running at http://localhost:{}", api_port);
    info!("Web UI running at http://localhost:{}", web_port);
    info!("");
    info!("Press Ctrl+C to stop all services");
    
    // 終了シグナルを待機
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    
    tokio::spawn(async move {
        let _ = tokio::signal::ctrl_c().await;
        r.store(false, Ordering::SeqCst);
    });
    
    while running.load(Ordering::SeqCst) {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    
    info!("Stopping services...");
    
    // プロセスをクリーンアップ
    let _ = Command::new("pkill")
        .args(["-f", "target/debug/standalone_api"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    
    let _ = Command::new("pkill")
        .args(["-f", "target/debug/web"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    
    Ok(())
}
