//! Rustorium Core - モジュラーブロックチェーンのコアインターフェース

pub mod network;
pub mod consensus;
pub mod storage;
pub mod runtime;
pub mod types;

pub use network::NetworkModule;
pub use consensus::ConsensusModule;
pub use storage::StorageModule;
pub use runtime::RuntimeModule;
pub use types::*;

/// モジュールの共通インターフェース
#[async_trait::async_trait]
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone)]
pub struct ModuleMetrics {
    /// メトリクス収集時刻
    pub timestamp: std::time::SystemTime,
    /// メトリクス
    pub metrics: std::collections::HashMap<String, f64>,
}

/// モジュールの設定
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ModuleConfig {
    /// モジュール名
    pub name: String,
    /// モジュールの設定
    pub config: std::collections::HashMap<String, serde_json::Value>,
}
