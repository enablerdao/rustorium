//! GQT - 量子的高速ブロックチェーンプラットフォーム

use clap::Parser;
use gqt_core::{Config, Node};
use gqt_network::{NetworkModule, NetworkModuleFactory};
use gqt_consensus::{ConsensusModule, ConsensusModuleFactory};
use gqt_storage::{StorageModule, StorageModuleFactory};
use gqt_runtime::{RuntimeModule, RuntimeModuleFactory};
use std::path::PathBuf;
use tracing_subscriber::{fmt, EnvFilter};

/// GQTノードのコマンドライン引数
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

    // ノードの作成と起動
    let mut node = Node::new(
        config.into(),
        network,
        consensus,
        storage,
        runtime,
    );

    // シグナルハンドラの設定
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::mpsc::channel(1);
    ctrlc::set_handler(move || {
        let _ = shutdown_tx.try_send(());
    })?;

    // ノードの初期化と起動
    node.init().await?;
    node.start().await?;

    // シャットダウン待機
    shutdown_rx.recv().await;

    // ノードの停止
    node.stop().await?;

    Ok(())
}
