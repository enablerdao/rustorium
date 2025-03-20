//! Rustorium Core
//! 
//! このクレートはRustoriumの中核機能を提供します。

use anyhow::Result;
use thiserror::Error;
use tracing::{info, warn, error};

pub mod types;
pub mod transaction;
pub mod block;
pub mod state;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("トランザクションエラー: {0}")]
    TransactionError(String),
    
    #[error("ブロックエラー: {0}")]
    BlockError(String),
    
    #[error("ステートエラー: {0}")]
    StateError(String),
}

/// Rustoriumのコアエンジン
pub struct RustoriumCore {
    network: rustorium_network::NetworkManager,
    consensus: rustorium_consensus::ConsensusEngine,
    storage: rustorium_storage::StorageEngine,
    api: rustorium_api::ApiServer,
}

impl RustoriumCore {
    /// 新しいRustoriumインスタンスを作成
    pub async fn new() -> Result<Self> {
        info!("Initializing Rustorium Core...");
        
        let network = rustorium_network::NetworkManager::new().await?;
        let consensus = rustorium_consensus::ConsensusEngine::new().await?;
        let storage = rustorium_storage::StorageEngine::new().await?;
        let api = rustorium_api::ApiServer::new().await?;
        
        Ok(Self {
            network,
            consensus,
            storage,
            api,
        })
    }
    
    /// ノードを起動
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting Rustorium node...");
        
        // ネットワークの初期化
        self.network.start().await?;
        
        // コンセンサスの開始
        self.consensus.start().await?;
        
        // APIサーバーの起動
        self.api.start().await?;
        
        info!("Rustorium node started successfully");
        Ok(())
    }
    
    /// ノードを停止
    pub async fn stop(&mut self) -> Result<()> {
        info!("Stopping Rustorium node...");
        
        // APIサーバーの停止
        self.api.stop().await?;
        
        // コンセンサスの停止
        self.consensus.stop().await?;
        
        // ネットワークの停止
        self.network.stop().await?;
        
        info!("Rustorium node stopped successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_core_lifecycle() -> Result<()> {
        let mut core = RustoriumCore::new().await?;
        
        // 起動テスト
        core.start().await?;
        
        // 停止テスト
        core.stop().await?;
        
        Ok(())
    }
}
