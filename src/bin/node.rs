use anyhow::Result;
use libp2p::identity::Keypair;
use rustorium::{
    core::{
        avalanche::AvalancheParams,
        engine::RustoriumEngine,
        storage::{rocksdb::RocksDBStorage, ShardStateManager},
    },
    network::P2PNetwork,
};
use std::{path::PathBuf, sync::Arc};
use structopt::StructOpt;
use tokio::sync::RwLock;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(StructOpt, Debug)]
#[structopt(name = "rustorium-node")]
struct Opt {
    /// ノードID（1-10）
    #[structopt(short, long)]
    node_id: u8,

    /// データディレクトリ
    #[structopt(short, long, parse(from_os_str))]
    data_dir: PathBuf,

    /// P2Pポート
    #[structopt(short, long)]
    port: u16,

    /// ブートストラップノード
    #[structopt(long)]
    bootstrap: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    // ロギングの設定
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_names(true)
        .with_level(true)
        .compact()
        .build();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting node {}", opt.node_id);

    // データディレクトリの作成
    let node_dir = opt.data_dir.join(format!("node{}", opt.node_id));
    std::fs::create_dir_all(&node_dir)?;

    // ストレージエンジンの初期化
    let storage = Arc::new(RocksDBStorage::new(&node_dir.join("db"))?);
    let shard_manager = Arc::new(ShardStateManager::new(storage.clone()));

    // キーペアの生成
    let keypair = Keypair::generate_ed25519();
    let peer_id = keypair.public().to_peer_id();
    info!("Node peer ID: {}", peer_id);

    // P2Pネットワークの初期化
    let listen_addr = format!("/ip4/0.0.0.0/tcp/{}", opt.port)
        .parse()
        .expect("Invalid multiaddr");
    let mut network = P2PNetwork::new(keypair).await?;

    // ブートストラップノードに接続
    if let Some(bootstrap) = opt.bootstrap {
        info!("Connecting to bootstrap node: {}", bootstrap);
        network.connect_to_peer(&bootstrap.parse()?).await?;
    }

    // Avalancheパラメータの設定
    let params = AvalancheParams {
        sample_size: 20,
        threshold: 0.8,
        max_rounds: 10,
    };

    // エンジンの初期化
    let network = Arc::new(RwLock::new(network));
    let engine = RustoriumEngine::new(
        Arc::clone(&network),
        shard_manager,
        params,
    );

    // ネットワークの起動
    let network_handle = {
        let mut network = network.write().await;
        network.start(listen_addr).await?
    };

    // シグナルハンドリング
    let (tx, rx) = tokio::sync::oneshot::channel();
    ctrlc::set_handler(move || {
        let _ = tx.send(());
    })?;

    // 終了を待機
    rx.await?;
    info!("Shutting down node {}", opt.node_id);

    Ok(())
}