//! Rustorium - 量子的高速ブロックチェーンプラットフォーム

use clap::Parser;
use rustorium_core::{Config, Node};
use rustorium_network::{NetworkModule, NetworkModuleFactory};
use rustorium_consensus::{ConsensusModule, ConsensusModuleFactory};
use rustorium_storage::{StorageModule, StorageModuleFactory};
use rustorium_runtime::{RuntimeModule, RuntimeModuleFactory};
use rustorium_api::{ApiServer, ApiConfig, ApiState};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;
use tracing_subscriber::{fmt, EnvFilter};

/// Rustoriumノードのコマンドライン引数
#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// 設定ファイルのパス
    #[clap(short, long)]
    config: Option<PathBuf>,

    /// 開発モードで起動
    #[clap(long)]
    dev: bool,

    /// ノードID
    #[clap(long)]
    node_id: Option<String>,

    /// リッスンアドレス
    #[clap(long)]
    listen_addr: Option<String>,

    /// 外部アドレス
    #[clap(long)]
    external_addr: Option<String>,

    /// ブートストラップノード
    #[clap(long)]
    bootstrap_nodes: Vec<String>,

    /// データディレクトリ
    #[clap(long)]
    data_dir: Option<PathBuf>,

    /// Web UIのポート
    #[clap(long, default_value = "9070")]
    web_port: u16,

    /// REST APIのポート
    #[clap(long, default_value = "9071")]
    api_port: u16,

    /// WebSocketのポート
    #[clap(long, default_value = "9072")]
    ws_port: u16,

    /// ログレベル
    #[clap(long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // コマンドライン引数の解析
    let args = Args::parse();

    // ロギングの設定
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&args.log_level));

    fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    // 設定の読み込み
    let mut config = if let Some(path) = args.config {
        Config::load(path)?
    } else if args.dev {
        Config::default()
    } else {
        anyhow::bail!("Configuration file is required in production mode");
    };

    // コマンドライン引数で設定を上書き
    if let Some(node_id) = args.node_id {
        config.node_id = node_id;
    }
    if let Some(listen_addr) = args.listen_addr {
        config.network.listen_addr = listen_addr;
    }
    if let Some(external_addr) = args.external_addr {
        config.network.external_addr = external_addr;
    }
    if !args.bootstrap_nodes.is_empty() {
        config.network.bootstrap_nodes = args.bootstrap_nodes;
    }
    if let Some(data_dir) = args.data_dir {
        config.storage.data_dir = data_dir.to_string_lossy().into_owned();
    }

    // モジュールの作成
    let network = NetworkModuleFactory::create(config.network.clone());
    let consensus = ConsensusModuleFactory::create(config.consensus.clone());
    let storage = StorageModuleFactory::create(config.storage.clone());
    let runtime = RuntimeModuleFactory::create(config.runtime.clone());

    // APIサーバーの設定
    let api_config = ApiConfig {
        web_port: args.web_port,
        api_port: args.api_port,
        ws_port: args.ws_port,
        static_dir: PathBuf::from("frontend/dist"),
    };

    // APIサーバーの状態
    let api_state = Arc::new(ApiState {
        network: Arc::new(RwLock::new(network)),
        consensus: Arc::new(RwLock::new(consensus)),
        storage: Arc::new(RwLock::new(storage)),
        runtime: Arc::new(RwLock::new(runtime)),
    });

    // APIサーバーの作成
    let api_server = ApiServer::new(api_state.clone(), api_config);

    // シグナルハンドラの設定
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::mpsc::channel(1);
    ctrlc::set_handler(move || {
        let _ = shutdown_tx.try_send(());
    })?;

    // APIサーバーの起動
    tokio::spawn(async move {
        if let Err(e) = api_server.start().await {
            tracing::error!("API server error: {}", e);
        }
    });

    // シャットダウン待機
    shutdown_rx.recv().await;

    Ok(())
}
