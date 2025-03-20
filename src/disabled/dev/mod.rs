use crate::core::{
    avalanche::AvalancheParams,
    engine::RustoriumEngine,
    storage::{rocksdb::RocksDBStorage, ShardStateManager},
};
use crate::network::P2PNetwork;
use anyhow::Result;
use libp2p::identity::Keypair;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;
use tracing::{info, warn};

/// テストノードの設定
#[derive(Debug, Clone)]
pub struct TestNodeConfig {
    /// ノードID（1-10）
    pub node_id: u8,
    /// データディレクトリ
    pub data_dir: PathBuf,
    /// APIポート
    pub api_port: u16,
    /// フロントエンドポート
    pub frontend_port: u16,
    /// P2Pポート
    pub p2p_port: u16,
    /// ブートストラップノードのアドレス（オプション）
    pub bootstrap: Option<String>,
}

/// テストノードの管理
pub struct TestNodeManager {
    nodes: Vec<TestNodeConfig>,
    base_port: u16,
    data_dir: PathBuf,
}

impl TestNodeManager {
    /// 新しいテストノード管理を作成
    pub fn new(base_port: u16, data_dir: PathBuf) -> Self {
        Self {
            nodes: Vec::new(),
            base_port,
            data_dir,
        }
    }

    /// テストノードを追加
    pub fn add_node(&mut self, node_id: u8) -> Result<()> {
        let node_config = TestNodeConfig {
            node_id,
            data_dir: self.data_dir.join(format!("node{}", node_id)),
            api_port: self.base_port + (node_id as u16 * 3),
            frontend_port: self.base_port + (node_id as u16 * 3) + 1,
            p2p_port: self.base_port + (node_id as u16 * 3) + 2,
            bootstrap: if node_id == 1 {
                None
            } else {
                Some(format!(
                    "/ip4/127.0.0.1/tcp/{}/p2p/{}",
                    self.base_port + 2,
                    "QmBootstrapNode" // TODO: 実際のPeerIDを使用
                ))
            },
        };

        std::fs::create_dir_all(&node_config.data_dir)?;
        self.nodes.push(node_config);
        Ok(())
    }

    /// 複数のテストノードを起動
    pub async fn start_nodes(&self, node_count: u8) -> Result<()> {
        info!("Starting {} test nodes for development...", node_count);
        info!("⚠️  This is a development feature and should not be used in production!");

        for node in &self.nodes {
            if node.node_id > node_count {
                continue;
            }

            info!("Starting node {} with:", node.node_id);
            info!("  📡 API: http://localhost:{}", node.api_port);
            info!("  🌐 Frontend: http://localhost:{}", node.frontend_port);
            info!("  🔗 P2P: /ip4/127.0.0.1/tcp/{}", node.p2p_port);

            // ストレージの初期化
            let storage = Arc::new(RocksDBStorage::new(&node.data_dir.join("db"))?);
            let shard_manager = Arc::new(ShardStateManager::new(storage.clone()));

            // P2Pネットワークの初期化
            let keypair = Keypair::generate_ed25519();
            let peer_id = keypair.public().to_peer_id();
            info!("  🆔 Peer ID: {}", peer_id);

            let listen_addr = format!("/ip4/0.0.0.0/tcp/{}", node.p2p_port)
                .parse()
                .expect("Invalid multiaddr");

            let mut network = P2PNetwork::new(keypair).await?;

            // ブートストラップノードに接続
            if let Some(bootstrap) = &node.bootstrap {
                info!("  🔌 Connecting to bootstrap node: {}", bootstrap);
                if let Err(e) = network.connect_to_peer(&bootstrap.parse()?).await {
                    warn!("Failed to connect to bootstrap node: {}", e);
                }
            }

            // Avalancheパラメータの設定
            let params = AvalancheParams {
                sample_size: 20,
                threshold: 0.8,
                max_rounds: 10,
            };

            // エンジンの初期化
            let network = Arc::new(RwLock::new(network));
            let _engine = RustoriumEngine::new(Arc::clone(&network), shard_manager, params);

            // ネットワークの起動
            let network_handle = {
                let mut network = network.write().await;
                network.start(listen_addr).await?
            };

            tokio::spawn(async move {
                if let Err(e) = network_handle.await {
                    warn!("Node {} network error: {}", node.node_id, e);
                }
            });
        }

        info!("\n✨ All test nodes started successfully!");
        info!("💡 Press Ctrl+C to stop all nodes");

        Ok(())
    }

    /// すべてのノードを停止
    pub async fn stop_nodes(&self) -> Result<()> {
        info!("Stopping all test nodes...");
        // TODO: 各ノードのクリーンな停止処理を実装
        Ok(())
    }
}