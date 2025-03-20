use anyhow::Result;
use rustorium::{
    cli::console::InteractiveConsole,
    config::NodeConfig,
    services::ServiceManager,
};

use tracing::{info, Level};
use tracing_subscriber::fmt;
use console::style;

#[tokio::main]
async fn main() -> Result<()> {
    // コマンドライン引数の解析
    let args = rustorium::cli::options::AppOptions::new();

    // ロギングの設定
    let subscriber = fmt::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .with_thread_names(false)
        .with_level(true)
        .with_ansi(true)
        .pretty()
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    // 設定の読み込みと更新
    let mut config = NodeConfig::default();
    config.update_from_args(&args);

    // ノードの役割を自動判定
    config.detect_role();

    // ストレージディレクトリの作成
    tokio::fs::create_dir_all(&config.node.data_dir).await?;
    tokio::fs::create_dir_all(&config.storage.path).await?;

    // サービスマネージャーを作成して起動
    let mut service_manager = ServiceManager::new(config.clone());
    service_manager.start().await?;

    // インタラクティブコンソールを起動（--no-interactiveが指定されていない場合）
    if !args.no_interactive {
        InteractiveConsole::run(&service_manager).await?;
    } else {
        // バックグラウンドモードの場合は、Ctrl+Cを待機
        tokio::signal::ctrl_c().await?;
        println!("\n{}", style("Received Ctrl+C, shutting down...").dim());
    }

    // シャットダウン処理
    info!("Shutting down...");
    service_manager.stop().await?;

    Ok(())
}