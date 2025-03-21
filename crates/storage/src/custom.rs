//! カスタムストレージモジュール

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use rustorium_core::{
    Module, ModuleConfig, ModuleStatus, ModuleMetrics,
    storage::{StorageModule, StorageOperation},
};
use async_trait::async_trait;
use tracing::info;

/// カスタムストレージモジュール
pub struct CustomStorageModule {
    /// 設定
    config: ModuleConfig,
    /// ステータス
    status: ModuleStatus,
    /// インメモリストレージ
    storage: Arc<RwLock<HashMap<Vec<u8>, Vec<u8>>>>,
}

impl CustomStorageModule {
    /// 新しいカスタムストレージモジュールを作成
    pub fn new(config: ModuleConfig) -> Self {
        Self {
            config,
            status: ModuleStatus::Uninitialized,
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl Module for CustomStorageModule {
    async fn init(&mut self) -> anyhow::Result<()> {
        info!("Initializing custom storage module...");
        self.status = ModuleStatus::Initialized;
        info!("Custom storage module initialized");
        Ok(())
    }

    async fn start(&mut self) -> anyhow::Result<()> {
        info!("Starting custom storage module...");
        self.status = ModuleStatus::Running;
        info!("Custom storage module started");
        Ok(())
    }

    async fn stop(&mut self) -> anyhow::Result<()> {
        info!("Stopping custom storage module...");
        self.status = ModuleStatus::Stopped;
        info!("Custom storage module stopped");
        Ok(())
    }

    async fn status(&self) -> anyhow::Result<ModuleStatus> {
        Ok(self.status.clone())
    }

    async fn metrics(&self) -> anyhow::Result<ModuleMetrics> {
        let mut metrics = HashMap::new();
        metrics.insert("storage_size".to_string(), self.storage.read().await.len() as f64);
        
        Ok(ModuleMetrics {
            timestamp: std::time::SystemTime::now(),
            metrics,
        })
    }
}

#[async_trait]
impl StorageModule for CustomStorageModule {
    async fn put(&mut self, key: Vec<u8>, value: Vec<u8>) -> anyhow::Result<()> {
        self.storage.write().await.insert(key.clone(), value);
        info!("Put key: {}", hex::encode(&key));
        Ok(())
    }

    async fn get(&self, key: &[u8]) -> anyhow::Result<Option<Vec<u8>>> {
        Ok(self.storage.read().await.get(key).cloned())
    }

    async fn delete(&mut self, key: &[u8]) -> anyhow::Result<()> {
        self.storage.write().await.remove(key);
        info!("Deleted key: {}", hex::encode(key));
        Ok(())
    }

    async fn batch(&mut self, operations: Vec<StorageOperation>) -> anyhow::Result<()> {
        let mut storage = self.storage.write().await;
        for op in operations {
            match op {
                StorageOperation::Put { key, value } => {
                    storage.insert(key, value);
                }
                StorageOperation::Delete { key } => {
                    storage.remove(&key);
                }
            }
        }
        Ok(())
    }

    async fn snapshot(&self) -> anyhow::Result<Vec<u8>> {
        let storage = self.storage.read().await;
        Ok(bincode::serialize(&*storage)?)
    }

    async fn restore(&mut self, snapshot: Vec<u8>) -> anyhow::Result<()> {
        let storage: HashMap<Vec<u8>, Vec<u8>> = bincode::deserialize(&snapshot)?;
        *self.storage.write().await = storage;
        Ok(())
    }
}
