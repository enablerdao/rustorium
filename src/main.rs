use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use std::env;

mod blockchain;
mod sustainable;

// コマンドラインオプションの定義
#[derive(Debug, Clone)]
struct AppOptions {
    api_only: bool,
    frontend_only: bool,
    fast: bool,
    release: bool,
    sustainable_demo: bool,
}

impl AppOptions {
    fn new() -> Self {
        Self {
            api_only: false,
            frontend_only: false,
            fast: false,
            release: false,
            sustainable_demo: false,
        }
    }

    fn parse_args() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut options = Self::new();
        
        for arg in args.iter().skip(1) {
            match arg.as_str() {
                "--api-only" => options.api_only = true,
                "--frontend-only" => options.frontend_only = true,
                "--fast" => options.fast = true,
                "--release" => options.release = true,
                "--sustainable-demo" => options.sustainable_demo = true,
                "-h" | "--help" => {
                    print_help();
                    std::process::exit(0);
                }
                _ => {
                    eprintln!("Unknown option: {}", arg);
                    print_help();
                    std::process::exit(1);
                }
            }
        }
        
        options
    }
}

fn print_help() {
    println!("Rustorium - ブロックチェーンプラットフォーム");
    println!();
    println!("使用方法:");
    println!("  cargo run [オプション]");
    println!();
    println!("オプション:");
    println!("  --api-only         APIサーバーのみを起動");
    println!("  --frontend-only    フロントエンドサーバーのみを起動");
    println!("  --fast             高速起動モード（最適化レベル低）");
    println!("  --release          リリースモードで起動（最適化レベル高）");
    println!("  --sustainable-demo 持続可能なブロックチェーン機能のデモを実行");
    println!("  -h, --help         このヘルプメッセージを表示");
}

#[tokio::main]
async fn main() -> Result<()> {
    // コマンドラインオプションの解析
    let options = AppOptions::parse_args();
    
    // ロギングの設定
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    // 既存のプロセスをクリーンアップ
    info!("Cleaning up any existing processes...");
    
    // APIプロセスのクリーンアップ
    if !options.frontend_only {
        let _ = Command::new("pkill")
            .args(["-f", "target/debug/api"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
            
        let _ = Command::new("pkill")
            .args(["-f", "target/release/api"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    
    // フロントエンドプロセスのクリーンアップ
    if !options.api_only {
        let _ = Command::new("pkill")
            .args(["-f", "target/debug/frontend"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
            
        let _ = Command::new("pkill")
            .args(["-f", "target/release/frontend"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    
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
        
    let _ = Command::new("sh")
        .arg("-c")
        .arg(format!("ps -ef | grep target/release/rustorium | grep -v {} | grep -v grep | awk '{{print $2}}' | xargs -r kill", current_pid))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // 起動モードに応じたコマンドを構築
    let api_port = 50128;
    let frontend_port = 55560;
    
    let cargo_command = if options.release {
        "cargo run --release"
    } else if options.fast {
        "cargo run --profile fast-dev"
    } else {
        "cargo run"
    };
    
    // APIサーバーを起動（フロントエンドのみモードでない場合）
    if !options.frontend_only {
        info!("Starting API server...");
        let api_args = cargo_command.split_whitespace().collect::<Vec<&str>>();
        let _api_process = Command::new(api_args[0])
            .current_dir("api")
            .args(&api_args[1..])
            .args(["--bin", "api"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;
        
        info!("API server starting on port: {}", api_port);
        
        // 少し待機
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    }
    
    // フロントエンドサーバーを起動（APIのみモードでない場合）
    if !options.api_only {
        info!("Starting Frontend server...");
        let frontend_args = cargo_command.split_whitespace().collect::<Vec<&str>>();
        let _frontend_process = Command::new(frontend_args[0])
            .current_dir("frontend")
            .args(&frontend_args[1..])
            .args(["--bin", "frontend"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;
        
        info!("Frontend server starting on port: {}", frontend_port);
        
        // 少し待機
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    }
    
    info!("All services started!");
    
    if !options.frontend_only {
        info!("API server running at http://localhost:{}", api_port);
    }
    
    if !options.api_only {
        info!("Frontend running at http://localhost:{}", frontend_port);
    }
    
    info!("");
    info!("Press Ctrl+C to stop all services");
    
    // 持続可能なブロックチェーン機能のデモを実行
    if options.sustainable_demo {
        info!("Running sustainable blockchain demo...");
        
        // 持続可能なブロックチェーンマネージャーを初期化
        let manager = sustainable::SustainableBlockchainManager::new();
        
        // デモ用の初期化
        manager.initialize_demo();
        
        // コンセンサスステータスの表示
        let consensus_status = manager.get_consensus_status();
        info!("=== コンセンサスステータス ===");
        info!("バリデーター数: {}", consensus_status.validator_count);
        info!("総ステーク量: {}", consensus_status.total_stake);
        info!("リソース効率: {:.2}", consensus_status.resource_efficiency);
        info!("現在の報酬レート: {}", consensus_status.current_reward_rate);
        
        // 負荷シミュレーション
        info!("=== 負荷シミュレーション ===");
        manager.simulate_load(5);
        
        // スケーリングステータスの表示
        let scaling_status = manager.get_scaling_status();
        info!("=== スケーリングステータス ===");
        info!("現在のシャード数: {}", scaling_status.current_shards);
        info!("現在のノード数: {}", scaling_status.current_nodes);
        info!("CPU使用率: {:.2}", scaling_status.cpu_usage);
        info!("メモリ使用率: {:.2}", scaling_status.memory_usage);
        info!("TPS: {:.2}", scaling_status.tps);
        info!("最後のスケーリング: {}", scaling_status.last_scaling);
        info!("次のスケーリング予定: {}", scaling_status.next_scaling);
        
        info!("持続可能なブロックチェーン機能のデモを終了します");
        return Ok(());
    }

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
    if !options.frontend_only {
        let target_dir = if options.release {
            "target/release/api"
        } else {
            "target/debug/api"
        };
        
        let _ = Command::new("pkill")
            .args(["-f", target_dir])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    
    if !options.api_only {
        let target_dir = if options.release {
            "target/release/frontend"
        } else {
            "target/debug/frontend"
        };
        
        let _ = Command::new("pkill")
            .args(["-f", target_dir])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    
    Ok(())
}
