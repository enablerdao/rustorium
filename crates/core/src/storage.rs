//! GQT Core - ストレージモジュールインターフェース

use crate::{Module, ModuleConfig};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// ストレージ操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageOperation {
    /// データの保存
    Put { key: Vec<u8>, value: Vec<u8> },
    /// データの削除
    Delete { key: Vec<u8> },
}

/// ストレージモジュールのインターフェース
#[async_trait]
pub trait StorageModule: Module {
    /// データの保存
    async fn put(&mut self, key: Vec<u8>, value: Vec<u8>) -> anyhow::Result<()>;
    /// データの取得
    async fn get(&self, key: &[u8]) -> anyhow::Result<Option<Vec<u8>>>;
    /// データの削除
    async fn delete(&mut self, key: &[u8]) -> anyhow::Result<()>;
    /// バッチ操作
    async fn batch(&mut self, operations: Vec<StorageOperation>) -> anyhow::Result<()>;
    /// スナップショットの作成
    async fn snapshot(&self) -> anyhow::Result<Vec<u8>>;
    /// スナップショットからの復元
    async fn restore(&mut self, snapshot: Vec<u8>) -> anyhow::Result<()>;
}

/// ストレージモジュールのファクトリ
pub trait StorageModuleFactory {
    /// ストレージモジュールの作成
    fn create(config: ModuleConfig) -> Box<dyn StorageModule>;
}
