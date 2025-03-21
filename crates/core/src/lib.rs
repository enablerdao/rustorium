//! GQT Core - 量子的高速ブロックチェーンのコアモジュール

pub mod network;
pub mod consensus;
pub mod storage;
pub mod runtime;
pub mod types;
pub mod metrics;
pub mod config;

pub use network::NetworkModule;
pub use consensus::ConsensusModule;
pub use storage::StorageModule;
pub use runtime::RuntimeModule;
pub use types::*;
pub use metrics::*;
pub use config::*;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// モジュールの共通インターフェース
#[async_trait]
pub trait Module: Send + Sync {
    /// モジュールの初期化
    async fn init(&mut self) -> anyhow::Result<()>;
    /// モジュールの開始
    async fn start(&mut self) -> anyhow::Result<()>;
    /// モジュールの停止
    async fn stop(&mut self) -> anyhow::Result<()>;
    /// モジュールのステータス取得
    async fn status(&self) -> anyhow::Result<ModuleStatus>;
    /// モジュールのメトリクス取得
    async fn metrics(&self) -> anyhow::Result<ModuleMetrics>;
}

/// モジュールのステータス
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModuleStatus {
    /// 初期化前
    Uninitialized,
    /// 初期化済み
    Initialized,
    /// 起動中
    Running,
    /// 停止中
    Stopped,
    /// エラー発生
    Error(String),
}

/// モジュールのメトリクス
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleMetrics {
    /// メトリクス収集時刻
    pub timestamp: SystemTime,
    /// メトリクス
    pub metrics: HashMap<String, f64>,
}

/// モジュールの設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleConfig {
    /// モジュール名
    pub name: String,
    /// モジュールの設定
    pub config: HashMap<String, serde_json::Value>,
}

/// GQTノードの設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    /// ノードID
    pub node_id: String,
    /// ネットワーク設定
    pub network: ModuleConfig,
    /// コンセンサス設定
    pub consensus: ModuleConfig,
    /// ストレージ設定
    pub storage: ModuleConfig,
    /// ランタイム設定
    pub runtime: ModuleConfig,
    /// API設定
    pub api: ModuleConfig,
}

/// GQTノード
pub struct Node {
    /// ノードの設定
    config: NodeConfig,
    /// ネットワークモジュール
    network: Box<dyn NetworkModule>,
    /// コンセンサスモジュール
    consensus: Box<dyn ConsensusModule>,
    /// ストレージモジュール
    storage: Box<dyn StorageModule>,
    /// ランタイムモジュール
    runtime: Box<dyn RuntimeModule>,
}

impl Node {
    /// 新しいGQTノードを作成
    pub fn new(
        config: NodeConfig,
        network: Box<dyn NetworkModule>,
        consensus: Box<dyn ConsensusModule>,
        storage: Box<dyn StorageModule>,
        runtime: Box<dyn RuntimeModule>,
    ) -> Self {
        Self {
            config,
            network,
            consensus,
            storage,
            runtime,
        }
    }

    /// ノードを初期化
    pub async fn init(&mut self) -> anyhow::Result<()> {
        tracing::info!("Initializing GQT node...");
        
        // 各モジュールを初期化
        self.storage.init().await?;
        self.runtime.init().await?;
        self.consensus.init().await?;
        self.network.init().await?;

        tracing::info!("GQT node initialized");
        Ok(())
    }

    /// ノードを開始
    pub async fn start(&mut self) -> anyhow::Result<()> {
        tracing::info!("Starting GQT node...");
        
        // 各モジュールを開始
        self.storage.start().await?;
        self.runtime.start().await?;
        self.consensus.start().await?;
        self.network.start().await?;

        tracing::info!("GQT node started");
        Ok(())
    }

    /// ノードを停止
    pub async fn stop(&mut self) -> anyhow::Result<()> {
        tracing::info!("Stopping GQT node...");
        
        // 各モジュールを停止（逆順）
        self.network.stop().await?;
        self.consensus.stop().await?;
        self.runtime.stop().await?;
        self.storage.stop().await?;

        tracing::info!("GQT node stopped");
        Ok(())
    }

    /// ノードのステータスを取得
    pub async fn status(&self) -> anyhow::Result<HashMap<String, ModuleStatus>> {
        let mut status = HashMap::new();
        status.insert("network".to_string(), self.network.status().await?);
        status.insert("consensus".to_string(), self.consensus.status().await?);
        status.insert("storage".to_string(), self.storage.status().await?);
        status.insert("runtime".to_string(), self.runtime.status().await?);
        Ok(status)
    }

    /// ノードのメトリクスを取得
    pub async fn metrics(&self) -> anyhow::Result<HashMap<String, ModuleMetrics>> {
        let mut metrics = HashMap::new();
        metrics.insert("network".to_string(), self.network.metrics().await?);
        metrics.insert("consensus".to_string(), self.consensus.metrics().await?);
        metrics.insert("storage".to_string(), self.storage.metrics().await?);
        metrics.insert("runtime".to_string(), self.runtime.metrics().await?);
        Ok(metrics)
    }
}
