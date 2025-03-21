//! RocksDBストレージモジュール

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use rocksdb::{DB, Options};
use rustorium_core::{
    Module, ModuleConfig, ModuleStatus, ModuleMetrics,
    storage::{StorageModule, StorageOperation},
};
use async_trait::async_trait;
use tracing::info;

/// RocksDBストレージモジュール
pub struct RocksDBModule {
    /// 設定
    config: ModuleConfig,
    /// ステータス
    status: ModuleStatus,
    /// RocksDBインスタンス
    db: Option<DB>,
    /// キャッシュ
    cache: Arc<RwLock<HashMap<Vec<u8>, Vec<u8>>>>,
}

impl RocksDBModule {
    /// 新しいRocksDBストレージモジュールを作成
    pub fn new(config: ModuleConfig) -> Self {
        Self {
            config,
            status: ModuleStatus::Uninitialized,
            db: None,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// RocksDBインスタンスの作成
    fn create_db(&self) -> anyhow::Result<DB> {
        let mut opts = Options::default();
        opts.create_if_missing(true);

        // 設定値の取得
        let path = self.config.config
            .get("path")
            .and_then(|v| v.as_str())
            .unwrap_or("data/rocksdb");

        Ok(DB::open(&opts, path)?)
    }
}

#[async_trait]
impl Module for RocksDBModule {
    async fn init(&mut self) -> anyhow::Result<()> {
        info!("Initializing RocksDB storage module...");
        self.db = Some(self.create_db()?);
        self.status = ModuleStatus::Initialized;
        info!("RocksDB storage module initialized");
        Ok(())
    }

    async fn start(&mut self) -> anyhow::Result<()> {
        info!("Starting RocksDB storage module...");
        self.status = ModuleStatus::Running;
        info!("RocksDB storage module started");
        Ok(())
    }

    async fn stop(&mut self) -> anyhow::Result<()> {
        info!("Stopping RocksDB storage module...");
        if let Some(db) = self.db.take() {
            drop(db);
        }
        self.status = ModuleStatus::Stopped;
        info!("RocksDB storage module stopped");
        Ok(())
    }

    async fn status(&self) -> anyhow::Result<ModuleStatus> {
        Ok(self.status.clone())
    }

    async fn metrics(&self) -> anyhow::Result<ModuleMetrics> {
        let mut metrics = HashMap::new();
        metrics.insert("cache_size".to_string(), self.cache.read().await.len() as f64);
        
        Ok(ModuleMetrics {
            timestamp: std::time::SystemTime::now(),
            metrics,
        })
    }
}

#[async_trait]
impl StorageModule for RocksDBModule {
    async fn put(&mut self, key: Vec<u8>, value: Vec<u8>) -> anyhow::Result<()> {
        if let Some(db) = &self.db {
            db.put(key.clone(), value.clone())?;
            self.cache.write().await.insert(key, value);
            info!("Put key: {}", hex::encode(&key));
        }
        Ok(())
    }

    async fn get(&self, key: &[u8]) -> anyhow::Result<Option<Vec<u8>>> {
        // まずキャッシュを確認
        if let Some(value) = self.cache.read().await.get(key) {
            return Ok(Some(value.clone()));
        }

        // キャッシュになければRocksDBから取得
        if let Some(db) = &self.db {
            if let Some(value) = db.get(key)? {
                self.cache.write().await.insert(key.to_vec(), value.clone());
                Ok(Some(value))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    async fn delete(&mut self, key: &[u8]) -> anyhow::Result<()> {
        if let Some(db) = &self.db {
            db.delete(key)?;
            self.cache.write().await.remove(key);
            info!("Deleted key: {}", hex::encode(key));
        }
        Ok(())
    }

    async fn batch(&mut self, operations: Vec<StorageOperation>) -> anyhow::Result<()> {
        if let Some(db) = &self.db {
            let mut batch = rocksdb::WriteBatch::default();
            let mut cache = self.cache.write().await;

            for op in operations {
                match op {
                    StorageOperation::Put { key, value } => {
                        batch.put(&key, &value);
                        cache.insert(key, value);
                    }
                    StorageOperation::Delete { key } => {
                        batch.delete(&key);
                        cache.remove(&key);
                    }
                }
            }

            db.write(batch)?;
        }
        Ok(())
    }

    async fn snapshot(&self) -> anyhow::Result<Vec<u8>> {
        // TODO: 実装
        unimplemented!()
    }

    async fn restore(&mut self, snapshot: Vec<u8>) -> anyhow::Result<()> {
        // TODO: 実装
        unimplemented!()
    }
}
