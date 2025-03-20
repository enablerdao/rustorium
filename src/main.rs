use anyhow::Result;
use clap::Parser;
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
use tracing::{info, warn, error, Level};
use tracing_subscriber::fmt;
use console::style;

#[derive(Parser)]
#[clap(name = "rustorium", about = "Next-generation blockchain platform")]
struct Opts {
    /// 設定ファイルのパス
    #[clap(long, default_value = "/etc/rustorium/config.toml")]
    config: String,

    /// データディレクトリ
    #[clap(long, default_value = "/var/lib/rustorium")]
    data_dir: String,

    /// ベースポート番号
    #[clap(long, default_value = "9070")]
    port: u16,

    /// 開発モード
    #[clap(long)]
    dev: bool,

    /// インタラクティブモードを無効化
    #[clap(long)]
    no_interactive: bool,

    /// ログレベル
    #[clap(long, default_value = "info")]
    log_level: String,

    /// メトリクスを有効化
    #[clap(long)]
    metrics: bool,

    /// ワーカー数
    #[clap(long, default_value = "1")]
    workers: usize,

    /// デバッグモード
    #[clap(long)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // コマンドライン引数の解析
    let opts = Opts::parse();

    // ロギングの設定
    let log_level = match opts.log_level.to_lowercase().as_str() {
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    let subscriber = fmt::fmt()
        .with_max_level(log_level)
        .with_target(opts.debug)
        .with_thread_ids(opts.debug)
        .with_file(opts.debug)
        .with_line_number(opts.debug)
        .with_thread_names(opts.debug)
        .with_level(true)
        .with_ansi(true)
        .pretty()
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    // 開発モードのログ
    if opts.dev {
        info!("Running in development mode");
        info!("Data directory: {}", opts.data_dir);
        info!("Base port: {}", opts.port);
    }

    // 設定の読み込みと更新
    let mut config = if opts.dev {
        NodeConfig::development()
    } else {
        NodeConfig::from_file(&opts.config)?
    };

    // 設定の更新
    config.node.data_dir = opts.data_dir.into();
    config.network.port = opts.port;
    config.web.enabled = true;

    // ディレクトリの作成
    tokio::fs::create_dir_all(&config.node.data_dir).await?;
    tokio::fs::create_dir_all(&config.storage.path).await?;

    info!("Initializing storage...");
    // ストレージの設定と初期化
    let storage_config = StorageConfig {
        path: config.storage.path.to_string_lossy().to_string(),
        max_size: 1024 * 1024 * 1024 * 1024, // 1TB
        compression_enabled: true,
        encryption_enabled: true,
        replication_factor: 3,
    };
    let storage = Arc::new(RedbStorage::new(storage_config)?);

    info!("Initializing network...");
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

    info!("Initializing AI optimizer...");
    // AI最適化エンジンの初期化
    let ai_optimizer = Arc::new(Mutex::new(AiOptimizer::new()));

    // 最適化タスクの開始
    if !opts.dev {
        let ai_optimizer_clone = ai_optimizer.clone();
        tokio::spawn(async move {
            loop {
                let mut optimizer = ai_optimizer_clone.lock().await;
                if let Err(e) = optimizer.optimize_system().await {
                    error!("AI optimization error: {}", e);
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            }
        });
    }

    info!("Starting services...");
    // サービスマネージャーを作成して起動
    let mut service_manager = ServiceManager::new(config.clone());
    service_manager.set_storage(storage);
    service_manager.set_ai_optimizer(ai_optimizer);
    service_manager.start().await?;

    info!("Rustorium node started successfully!");
    info!("API endpoint: http://localhost:{}", config.network.port);
    info!("Web UI: http://localhost:{}", config.network.port + 1);
    info!("WebSocket: ws://localhost:{}", config.network.port + 2);

    // メトリクスの有効化
    if opts.metrics {
        info!("Metrics enabled at http://localhost:{}/metrics", config.network.port);
    }

    // インタラクティブコンソールを起動（--no-interactiveが指定されていない場合）
    if !opts.no_interactive {
        InteractiveConsole::run(&service_manager).await?;
    } else {
        info!("Running in non-interactive mode. Press Ctrl+C to stop.");
        tokio::signal::ctrl_c().await?;
        println!("\n{}", style("Received Ctrl+C, shutting down...").dim());
    }

    // シャットダウン処理
    info!("Shutting down services...");
    service_manager.stop().await?;
    info!("Shutdown complete.");

    Ok(())
}