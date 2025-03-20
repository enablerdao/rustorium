use anyhow::Result;
use rustorium::{
    cli::console::InteractiveConsole,
    config::NodeConfig,
    services::ServiceManager,
    core::{
        storage::redb_storage::{RedbStorage, StorageConfig},
        network::quic::{QuicNetwork, NetworkConfig},
        ai::AiOptimizer,
    },
};

use std::sync::Arc;
use tokio::sync::Mutex;
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

    // ストレージの設定と初期化
    let storage_config = StorageConfig {
        path: config.storage.path.to_string_lossy().to_string(),
        max_size: 1024 * 1024 * 1024 * 1024, // 1TB
        compression_enabled: true,
        encryption_enabled: true,
        replication_factor: 3,
    };
    let storage = Arc::new(RedbStorage::new(storage_config)?);

    // ネットワークの設定と初期化
    let network_config = NetworkConfig {
        listen_addr: format!("0.0.0.0:{}", config.network.port).parse()?,
        bootstrap_nodes: config.network.bootstrap_nodes.clone(),
        max_concurrent_streams: 1000,
        keep_alive_interval: std::time::Duration::from_secs(10),
        handshake_timeout: std::time::Duration::from_secs(10),
        idle_timeout: std::time::Duration::from_secs(30),
    };
    let network = Arc::new(QuicNetwork::new(network_config).await?);

    // AI最適化エンジンの初期化
    let ai_optimizer = Arc::new(Mutex::new(AiOptimizer::new()));

    // 最適化タスクの開始
    let ai_optimizer_clone = ai_optimizer.clone();
    tokio::spawn(async move {
        loop {
            let mut optimizer = ai_optimizer_clone.lock().await;
            if let Err(e) = optimizer.optimize_system().await {
                tracing::error!("AI optimization error: {}", e);
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    });

    // サービスマネージャーを作成して起動
    let mut service_manager = ServiceManager::new(config.clone());
    service_manager.set_storage(storage);
    service_manager.set_ai_optimizer(ai_optimizer);
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