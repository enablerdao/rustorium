use std::sync::Arc;
use anyhow::Result;
use tracing::{info, error};
use crate::{
    config::NodeConfig,
    web::WebServer,
    core::{
        storage::redb_storage::RedbStorage,
        network::P2PNetwork,
        ai::AiOptimizer,
    },
};
use tokio::sync::Mutex;

/// サービスマネージャー
pub struct ServiceManager {
    config: NodeConfig,
    storage: Option<Arc<RedbStorage>>,
    network: Option<Arc<P2PNetwork>>,
    web_server: Option<WebServer>,
    ai_optimizer: Option<Arc<Mutex<AiOptimizer>>>,
}

impl ServiceManager {
    /// 新しいサービスマネージャーを作成
    pub fn new(config: NodeConfig) -> Self {
        Self {
            config,
            storage: None,
            network: None,
            web_server: None,
            ai_optimizer: None,
        }
    }

    /// ストレージエンジンを設定
    pub fn set_storage(&mut self, storage: Arc<RedbStorage>) {
        self.storage = Some(storage);
    }

    /// AI最適化エンジンを設定
    pub fn set_ai_optimizer(&mut self, optimizer: Arc<Mutex<AiOptimizer>>) {
        self.ai_optimizer = Some(optimizer);
    }

    /// 設定を取得
    pub fn config(&self) -> &NodeConfig {
        &self.config
    }

    /// ピア数を取得
    pub async fn get_peer_count(&self) -> u32 {
        // TODO: 実際のP2Pネットワークからピア数を取得
        0
    }

    /// 平均レイテンシーを取得
    pub async fn get_average_latency(&self) -> u32 {
        if let Some(optimizer) = &self.ai_optimizer {
            let metrics = optimizer.lock().await.get_network_metrics().await;
            metrics.average_latency as u32
        } else {
            0
        }
    }

    /// ブロック数を取得
    pub async fn get_block_count(&self) -> u64 {
        if let Some(storage) = &self.storage {
            storage.get_block_count().await.unwrap_or(0)
        } else {
            0
        }
    }

    /// サービスを起動
    pub async fn start(&mut self) -> Result<()> {
        // データディレクトリを作成
        tokio::fs::create_dir_all(&self.config.node.data_dir).await?;

        // ストレージエンジンの初期化確認
        if let Some(storage) = &self.storage {
            info!("Storage engine initialized");
        } else {
            let storage_path = if self.config.storage.path.as_os_str().is_empty() {
                self.config.node.data_dir.join("storage")
            } else {
                self.config.storage.path.clone()
            };
            tokio::fs::create_dir_all(&storage_path).await?;
            let storage = Arc::new(RedbStorage::new(storage_path.to_str().unwrap())?);
            self.storage = Some(storage);
            info!("Storage engine initialized");
        }

        // AI最適化エンジンの初期化確認
        if let Some(optimizer) = &self.ai_optimizer {
            info!("AI optimization engine initialized");
        } else {
            let optimizer = Arc::new(Mutex::new(AiOptimizer::new()));
            self.ai_optimizer = Some(optimizer);
            info!("AI optimization engine initialized");
        }

        // P2Pネットワークを初期化
        info!("Initializing P2P network...");
        let keypair = libp2p::identity::Keypair::generate_ed25519();
        let network = Arc::new(P2PNetwork::new(keypair).await?);
        self.network = Some(network.clone());

        // Web UIサーバーを起動
        if self.config.web.enabled {
            info!("Starting Web UI server...");

            // ダッシュボード
            let web_server = WebServer::new(
                self.config.network.port,  // 9070
                self.config.clone(),
            );
            self.web_server = Some(web_server.clone());
            tokio::spawn(async move {
                if let Err(e) = web_server.run().await {
                    error!("Web server error: {}", e);
                }
            });

            // APIサーバー
            let api_server = WebServer::new(
                self.config.network.port + 1,  // 9071
                self.config.clone(),
            );
            tokio::spawn(async move {
                if let Err(e) = api_server.run().await {
                    error!("API server error: {}", e);
                }
            });

            // WebSocketサーバー
            let ws_server = WebServer::new(
                self.config.network.port + 2,  // 9072
                self.config.clone(),
            );
            tokio::spawn(async move {
                if let Err(e) = ws_server.run().await {
                    error!("WebSocket server error: {}", e);
                }
            });

            // サーバーの起動を待機（実際のリクエストで確認する方が望ましい）
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

        if let Some(optimizer) = self.ai_optimizer.take() {
            info!("Stopping AI optimization engine...");
            // 最適化エンジンの適切な停止処理
            let mut opt = optimizer.lock().await;
            opt.shutdown().await?;
        }

        if let Some(storage) = self.storage.take() {
            info!("Stopping storage engine...");
            // ストレージエンジンの適切な停止処理
            storage.shutdown().await?;
        }

        info!("All services stopped");
        Ok(())
    }

    // ストレージエンジンへのアクセス
    pub fn storage(&self) -> Option<&Arc<RedbStorage>> {
        self.storage.as_ref()
    }

    // AI最適化エンジンへのアクセス
    pub fn ai_optimizer(&self) -> Option<&Arc<Mutex<AiOptimizer>>> {
        self.ai_optimizer.as_ref()
    }
}
