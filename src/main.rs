use std::sync::Arc;
use anyhow::Result;
use console::style;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod cli;
mod core;
mod dev;
mod i18n;
mod blockchain;
mod sustainable;
mod network;

use cli::{AppOptions, AppState, InteractiveConsole};
use core::{
    dag::{DAGManager, Transaction, TxId},
    avalanche::AvalancheEngine,
    sharding::ShardManager,
    storage::{RocksDBStorage, StorageEngine},
};
use dev::TestNodeManager;
use i18n::LocaleConfig;
use network::P2PNetwork;

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

        let mut node_manager = TestNodeManager::new(options.base_port, options.data_dir.clone().into());
        
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

    // ノードの初期化
    info!("Initializing node...");

    // ストレージエンジンの初期化
    let storage = Arc::new(RocksDBStorage::new(&options.data_dir.clone().into())?);

    // DAGマネージャーの初期化
    let dag_manager = Arc::new(DAGManager::new(storage.clone()));

    // P2Pネットワークの初期化
    let network = Arc::new(P2PNetwork::new(options.keypair.clone()).await?);

    // Avalancheコンセンサスエンジンの初期化
    let avalanche = Arc::new(AvalancheEngine::new(
        Default::default(),
        network.clone(),
    ));

    // シャードマネージャーの初期化
    let shard_manager = Arc::new(ShardManager::new(
        storage.clone(),
        network.clone(),
    ));

    // トークンマネージャーの初期化
    let token_manager = Arc::new(TokenManager::new(storage.clone()));

    // APIサーバーの初期化
    let api_config = ApiConfig {
        host: options.api_host.unwrap_or_else(|| "127.0.0.1".to_string()),
        rest_port: options.api_port.unwrap_or(8001),
        ws_port: options.ws_port.unwrap_or(8002),
        graphql_port: options.graphql_port.unwrap_or(8003),
        cors_origin: options.cors_origin.unwrap_or_else(|| "*".to_string()),
        rate_limit: options.rate_limit.unwrap_or(1000),
    };

    let api_server = ApiServer::new(
        api_config.clone(),
        token_manager.clone(),
        dag_manager.clone(),
        shard_manager.clone(),
    );

    // アプリケーション状態の初期化
    let app_state = AppState {
        api_port: api_config.rest_port,
        ws_port: api_config.ws_port,
        graphql_port: api_config.graphql_port,
        locale: LocaleConfig::new("en"), // デフォルトは英語
        api_url: format!("http://localhost:{}", api_config.rest_port),
        ws_url: format!("ws://localhost:{}", api_config.ws_port),
        graphql_url: format!("http://localhost:{}", api_config.graphql_port),
        dag_manager: dag_manager.clone(),
        avalanche: avalanche.clone(),
        shard_manager: shard_manager.clone(),
        token_manager: token_manager.clone(),
        network: network.clone(),
    };

    // APIサーバーの起動
    info!("Starting API servers...");
    api_server.start().await?;

    // P2Pネットワークの起動
    info!("Starting P2P network...");
    network.start(options.p2p_addr.clone()).await?;

    // シャードマネージャーの起動
    info!("Starting shard manager...");
    shard_manager.start().await?;

    // 持続可能なブロックチェーンのデモを実行
    if options.sustainable_demo {
        sustainable::run_demo().await?;
    }

    // インタラクティブコンソールの起動
    InteractiveConsole::run(&app_state).await?;

    // 終了処理
    println!("\n{}", style("Shutting down services...").yellow());
    
    // シャードマネージャーの停止
    shard_manager.stop().await?;
    
    // P2Pネットワークの停止
    network.stop().await?;
    
    // APIサーバーの停止
    api_server.stop().await?;
    
    println!("{}", style("✨ Thank you for using Rustorium!").green().bold());

    Ok(())
}