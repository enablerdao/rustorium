pub mod shard;
pub mod state;
pub mod rocksdb;

use anyhow::Result;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::path::Path;

/// ストレージエンジンのトレイト
#[async_trait]
pub trait StorageEngine: Send + Sync + 'static {
    /// キーバリューペアを保存
    async fn put_bytes(&self, cf: &str, key: &[u8], value: &[u8]) -> Result<()>;

    /// 値を取得
    async fn get_bytes(&self, cf: &str, key: &[u8]) -> Result<Option<Vec<u8>>>;

    /// キーを削除
    async fn delete_bytes(&self, cf: &str, key: &[u8]) -> Result<()>;

    /// プレフィックスでスキャン
    async fn scan_prefix_bytes(&self, cf: &str, prefix: &[u8]) -> Result<Vec<(Vec<u8>, Vec<u8>)>>;

    /// バッチ書き込み
    async fn batch_write_bytes(&self, cf: &str, pairs: Vec<(Vec<u8>, Vec<u8>)>) -> Result<()>;

    /// スナップショットを作成
    async fn create_snapshot(&self, path: &Path) -> Result<()>;

    /// スナップショットから復元
    async fn restore_from_snapshot(&self, path: &Path) -> Result<()>;
}

/// 型付きストレージ拡張
#[async_trait]
pub trait TypedStorage: StorageEngine {
    /// シリアライズ可能な値を取得
    async fn get<T: DeserializeOwned + Send + Sync>(&self, cf: &str, key: &[u8]) -> Result<Option<T>> {
        let bytes = self.get_bytes(cf, key).await?;
        match bytes {
            Some(bytes) => Ok(Some(bincode::deserialize(&bytes)?)),
            None => Ok(None),
        }
    }

    /// シリアライズ可能な値を設定
    async fn put<T: Serialize + Send + Sync>(&self, cf: &str, key: &[u8], value: &T) -> Result<()> {
        let bytes = bincode::serialize(value)?;
        self.put_bytes(cf, key, &bytes).await
    }

    /// プレフィックスに一致するキーと値のペアを取得
    async fn scan_prefix<T: DeserializeOwned + Send + Sync>(
        &self,
        cf: &str,
        prefix: &[u8],
    ) -> Result<Vec<(Vec<u8>, T)>> {
        let pairs = self.scan_prefix_bytes(cf, prefix).await?;
        pairs
            .into_iter()
            .map(|(key, value)| Ok((key, bincode::deserialize(&value)?)))
            .collect()
    }

    /// 複数のキーと値のペアを一括で書き込み
    async fn batch_write<T: Serialize + Send + Sync>(
        &self,
        cf: &str,
        pairs: Vec<(Vec<u8>, T)>,
    ) -> Result<()> {
        let pairs: Result<Vec<_>> = pairs
            .into_iter()
            .map(|(key, value)| Ok((key, bincode::serialize(&value)?)))
            .collect();
        self.batch_write_bytes(cf, pairs?).await
    }
}

// StorageEngineを実装するすべての型に対してTypedStorageを自動実装
impl<T: ?Sized + StorageEngine> TypedStorage for T {}

/// カラムファミリー名の定義
pub const CF_SHARD_STATE: &str = "shard_state";
pub const CF_TRANSACTIONS: &str = "transactions";
pub const CF_METADATA: &str = "metadata";

/// キープレフィックスの定義
pub const PREFIX_SHARD: &[u8] = b"shard/";
pub const PREFIX_TX: &[u8] = b"tx/";
pub const PREFIX_META: &[u8] = b"meta/";