use std::sync::Arc;
use anyhow::Result;
use tracing::{info, error};
use crate::{
    config::NodeConfig,
    web::WebServer,
    core::{
        storage::RocksDBStorage,
        network::P2PNetwork,
    },
};

/// サービスマネージャー
pub struct ServiceManager {
    config: NodeConfig,
    storage: Option<Arc<RocksDBStorage>>,
    network: Option<Arc<P2PNetwork>>,
    web_server: Option<WebServer>,
}

impl ServiceManager {
    /// 新しいサービスマネージャーを作成
    pub fn new(config: NodeConfig) -> Self {
        Self {
            config,
            storage: None,
            network: None,
            web_server: None,
        }
    }

    /// サービスを起動
    pub async fn start(&mut self) -> Result<()> {
        // ストレージエンジンを初期化
        info!("Initializing storage engine...");
        let storage = Arc::new(RocksDBStorage::new(
            self.config.storage.path.to_str().unwrap()
        )?);
        self.storage = Some(storage.clone());

        // P2Pネットワークを初期化
        info!("Initializing P2P network...");
        let keypair = libp2p::identity::Keypair::generate_ed25519();
        let network = Arc::new(P2PNetwork::new(keypair).await?);
        self.network = Some(network.clone());

        // Web UIサーバーを起動
        if self.config.web.enabled {
            info!("Starting Web UI server...");
            let web_server = WebServer::new(
                self.config.network.port + self.config.web.port_offset,
                self.config.clone(),
            );

            let web_server_clone = web_server.clone();
            self.web_server = Some(web_server);

            tokio::spawn(async move {
                if let Err(e) = web_server_clone.run().await {
                    error!("Web server error: {}", e);
                }
            });

            // サーバーの起動を待機
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            info!("Web UI server started");
        }

        Ok(())
    }

    /// サービスを停止
    pub async fn stop(&mut self) -> Result<()> {
        info!("Stopping services...");
        
        // 各サービスを停止
        if let Some(web_server) = self.web_server.take() {
            info!("Stopping Web UI server...");
            web_server.shutdown();
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }

        if let Some(network) = self.network.take() {
            info!("Stopping P2P network...");
            drop(network);
        }

        if let Some(storage) = self.storage.take() {
            info!("Stopping storage engine...");
            drop(storage);
        }

        info!("All services stopped");
        Ok(())
    }
}