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
        .args(["-f", "target/debug/api"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    
    let _ = Command::new("pkill")
        .args(["-f", "target/debug/frontend"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    
    // 自分自身のプロセスIDを取得して除外する
    let current_pid = std::process::id();
    info!("Current process ID: {}", current_pid);
    
    // 自分自身以外のrustoriumプロセスを終了
    let _ = Command::new("sh")
        .arg("-c")
        .arg(format!("ps -ef | grep target/debug/rustorium | grep -v {} | grep -v grep | awk '{{print $2}}' | xargs -r kill", current_pid))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // APIサーバーを起動
    info!("Starting API server...");
    let api_port = 50128;
    let _api_process = Command::new("cargo")
        .current_dir("api")
        .args(["run", "--bin", "api"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;
    
    info!("API server starting on port: {}", api_port);
    
    // 少し待機
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    
    // フロントエンドサーバーを起動
    info!("Starting Frontend server...");
    let frontend_port = 55560;
    let _frontend_process = Command::new("cargo")
        .current_dir("frontend")
        .args(["run", "--bin", "frontend"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;
    
    info!("Frontend server starting on port: {}", frontend_port);
    
    // 少し待機
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    
    info!("All services started!");
    info!("API server running at http://localhost:{}", api_port);
    info!("Frontend running at http://localhost:{}", frontend_port);
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
        .args(["-f", "target/debug/api"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    
    let _ = Command::new("pkill")
        .args(["-f", "target/debug/frontend"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    
    Ok(())
}
