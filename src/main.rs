use anyhow::Result;
use console::style;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod cli;
mod dev;
mod i18n;
mod blockchain;
mod sustainable;

use cli::{AppOptions, AppState, InteractiveConsole, ServerManager};
use dev::TestNodeManager;
use i18n::LocaleConfig;

#[tokio::main]
async fn main() -> Result<()> {
    // コマンドラインオプションの解析
    let options = AppOptions::new();
    
    // ロギングレベルの設定
    let log_level = match options.log_level.as_deref() {
        Some("debug") => Level::DEBUG,
        Some("warn") => Level::WARN,
        Some("error") => Level::ERROR,
        Some("trace") => Level::TRACE,
        _ => Level::INFO,
    };

    // ロギングの設定
    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    // 開発モードの場合はテストノードを起動
    if options.dev {
        println!("{}", style("\n🧪 Development Mode: Starting Test Nodes").yellow().bold());
        println!("{}", style("⚠️  This mode is for development and testing only!").red());
        println!();

        let mut node_manager = TestNodeManager::new(options.base_port, options.data_dir.into());
        
        // テストノードを追加
        for i in 1..=options.nodes {
            node_manager.add_node(i)?;
        }

        // テストノードを起動
        node_manager.start_nodes(options.nodes).await?;

        // Ctrl+Cを待機
        let (tx, rx) = tokio::sync::oneshot::channel();
        ctrlc::set_handler(move || {
            let _ = tx.send(());
        })?;

        println!("\n{}", style("Press Ctrl+C to stop all nodes").cyan());
        let _ = rx.await;
        
        println!("\n{}", style("Stopping all test nodes...").yellow());
        node_manager.stop_nodes().await?;
        println!("{}", style("✨ All test nodes stopped successfully!").green());
        return Ok(());
    }

    // ポート設定（標準的なポートを優先）
    let api_preferred_ports = vec![8001, 3001, 5001, 8081, 9001, 50128];
    let frontend_preferred_ports = vec![8000, 3000, 5000, 8080, 9000, 55560];
    
    // 使用可能なポートを見つける関数
    let find_available_port = |preferred_ports: &[u16]| -> u16 {
        for &port in preferred_ports {
            // ポートが使用可能かチェック
            match std::net::TcpListener::bind(format!("0.0.0.0:{}", port)) {
                Ok(listener) => {
                    // リスナーをドロップして、ポートを解放
                    drop(listener);
                    return port;
                },
                Err(_) => continue,
            }
        }
        // すべてのポートが使用中の場合はランダムなポートを使用
        let listener = std::net::TcpListener::bind("0.0.0.0:0").expect("Failed to bind to random port");
        let addr = listener.local_addr().expect("Failed to get local address");
        drop(listener);
        addr.port()
    };
    
    let api_port = options.api_port.unwrap_or_else(|| find_available_port(&api_preferred_ports));
    let frontend_port = options.frontend_port.unwrap_or_else(|| find_available_port(&frontend_preferred_ports));

    // アプリケーション状態の初期化
    let app_state = AppState {
        api_port,
        frontend_port,
        locale: LocaleConfig::new("en"), // デフォルトは英語
        api_url: format!("http://localhost:{}", api_port),
        frontend_url: format!("http://localhost:{}", frontend_port),
    };

    // サーバーマネージャーの初期化と起動
    let server_manager = ServerManager::new(
        api_port,
        frontend_port,
        options.api_only,
        options.frontend_only,
        options.fast,
        options.release,
    );
    server_manager.start_servers().await?;

    // 持続可能なブロックチェーンのデモを実行
    if options.sustainable_demo {
        sustainable::run_demo().await?;
    }

    // インタラクティブコンソールの起動
    InteractiveConsole::run(&app_state).await?;

    // 終了処理
    println!("\n{}", style("Shutting down services...").yellow());
    server_manager.stop_servers()?;
    println!("{}", style("✨ Thank you for using Rustorium!").green().bold());

    Ok(())
}