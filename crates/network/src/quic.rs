//! QUICネットワークモジュール

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use quinn::{Endpoint, ServerConfig, TransportConfig};
use rustorium_core::{
    Module, ModuleConfig, ModuleStatus, ModuleMetrics,
    network::NetworkModule,
};
use async_trait::async_trait;
use tracing::info;

/// QUICネットワークモジュール
pub struct QuicNetworkModule {
    /// エンドポイント
    endpoint: Option<Endpoint>,
    /// ピア接続
    peers: Arc<RwLock<HashMap<SocketAddr, quinn::Connection>>>,
    /// 設定
    config: ModuleConfig,
    /// ステータス
    status: ModuleStatus,
}

impl QuicNetworkModule {
    /// 新しいQUICネットワークモジュールを作成
    pub fn new(config: ModuleConfig) -> Self {
        Self {
            endpoint: None,
            peers: Arc::new(RwLock::new(HashMap::new())),
            config,
            status: ModuleStatus::Uninitialized,
        }
    }

    /// トランスポート設定の作成
    fn create_transport_config(&self) -> TransportConfig {
        let mut transport = TransportConfig::default();
        
        // 設定値の取得
        let max_concurrent_uni_streams = self.config.config
            .get("max_concurrent_uni_streams")
            .and_then(|v| v.as_u64())
            .unwrap_or(100);
            
        let initial_rtt = self.config.config
            .get("initial_rtt_ms")
            .and_then(|v| v.as_u64())
            .unwrap_or(100);
            
        transport
            .max_concurrent_uni_streams(max_concurrent_uni_streams as u32)
            .initial_rtt(std::time::Duration::from_millis(initial_rtt));
            
        transport
    }

    /// サーバー設定の作成
    fn create_server_config(&self) -> anyhow::Result<ServerConfig> {
        let transport = Arc::new(self.create_transport_config());
        
        let mut server_config = ServerConfig::default();
        server_config.transport = transport;
        
        // TODO: TLS証明書の設定
        
        Ok(server_config)
    }
}

#[async_trait]
impl Module for QuicNetworkModule {
    async fn init(&mut self) -> anyhow::Result<()> {
        info!("Initializing QUIC network module...");
        
        let server_config = self.create_server_config()?;
        
        let bind_addr = self.config.config
            .get("bind_addr")
            .and_then(|v| v.as_str())
            .unwrap_or("0.0.0.0:0")
            .parse()?;
            
        self.endpoint = Some(Endpoint::server(server_config, bind_addr)?);
        self.status = ModuleStatus::Initialized;
        
        info!("QUIC network module initialized");
        Ok(())
    }

    async fn start(&mut self) -> anyhow::Result<()> {
        info!("Starting QUIC network module...");
        
        if let Some(endpoint) = &self.endpoint {
            let peers = self.peers.clone();
            
            // 接続受付ループの開始
            tokio::spawn(async move {
                while let Some(conn) = endpoint.accept().await {
                    let remote_addr = conn.remote_address();
                    info!("New connection from {}", remote_addr);
                    
                    if let Ok(connection) = conn.await {
                        peers.write().await.insert(remote_addr, connection);
                    }
                }
            });
            
            self.status = ModuleStatus::Running;
            info!("QUIC network module started");
        }
        
        Ok(())
    }

    async fn stop(&mut self) -> anyhow::Result<()> {
        info!("Stopping QUIC network module...");
        
        if let Some(endpoint) = self.endpoint.take() {
            endpoint.close(0u32.into(), b"Shutting down");
        }
        
        self.peers.write().await.clear();
        self.status = ModuleStatus::Stopped;
        
        info!("QUIC network module stopped");
        Ok(())
    }

    async fn status(&self) -> anyhow::Result<ModuleStatus> {
        Ok(self.status.clone())
    }

    async fn metrics(&self) -> anyhow::Result<ModuleMetrics> {
        let peers = self.peers.read().await;
        
        let mut metrics = HashMap::new();
        metrics.insert("connected_peers".to_string(), peers.len() as f64);
        
        Ok(ModuleMetrics {
            timestamp: std::time::SystemTime::now(),
            metrics,
        })
    }
}

#[async_trait]
impl NetworkModule for QuicNetworkModule {
    async fn connect(&mut self, addr: SocketAddr) -> anyhow::Result<()> {
        if let Some(endpoint) = &self.endpoint {
            let connection = endpoint.connect(addr, "localhost")?.await?;
            self.peers.write().await.insert(addr, connection);
            info!("Connected to {}", addr);
        }
        
        Ok(())
    }

    async fn disconnect(&mut self, addr: SocketAddr) -> anyhow::Result<()> {
        if let Some(connection) = self.peers.write().await.remove(&addr) {
            connection.close(0u32.into(), b"Disconnecting");
            info!("Disconnected from {}", addr);
        }
        
        Ok(())
    }

    async fn send(&self, addr: SocketAddr, data: Vec<u8>) -> anyhow::Result<()> {
        if let Some(connection) = self.peers.read().await.get(&addr) {
            let mut send = connection.open_uni().await?;
            send.write_all(&data).await?;
            send.finish().await?;
            info!("Sent {} bytes to {}", data.len(), addr);
        }
        
        Ok(())
    }

    async fn receive(&self) -> anyhow::Result<(SocketAddr, Vec<u8>)> {
        // TODO: 実装
        unimplemented!()
    }

    async fn peers(&self) -> anyhow::Result<Vec<SocketAddr>> {
        Ok(self.peers.read().await.keys().copied().collect())
    }
}
