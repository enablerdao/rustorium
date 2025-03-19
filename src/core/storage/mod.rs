pub mod shard;
pub mod state;

use anyhow::Result;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::path::Path;

/// ストレージエンジンのトレイト
#[async_trait]
pub trait StorageEngine: Send + Sync {
    /// キーバリューペアを保存
    async fn put<K, V>(&self, cf: &str, key: K, value: V) -> Result<()>
    where
        K: AsRef<[u8]> + Send + Sync,
        V: Serialize + Send + Sync;

    /// 値を取得
    async fn get<K, V>(&self, cf: &str, key: K) -> Result<Option<V>>
    where
        K: AsRef<[u8]> + Send + Sync,
        V: DeserializeOwned + Send + Sync;

    /// キーを削除
    async fn delete<K>(&self, cf: &str, key: K) -> Result<()>
    where
        K: AsRef<[u8]> + Send + Sync;

    /// プレフィックスでスキャン
    async fn scan_prefix<K, V>(&self, cf: &str, prefix: K) -> Result<Vec<(Vec<u8>, V)>>
    where
        K: AsRef<[u8]> + Send + Sync,
        V: DeserializeOwned + Send + Sync;

    /// バッチ書き込み
    async fn batch_write<K, V>(&self, cf: &str, pairs: Vec<(K, V)>) -> Result<()>
    where
        K: AsRef<[u8]> + Send + Sync,
        V: Serialize + Send + Sync;

    /// スナップショットを作成
    async fn create_snapshot(&self, path: &Path) -> Result<()>;

    /// スナップショットから復元
    async fn restore_from_snapshot(&self, path: &Path) -> Result<()>;
}

/// カラムファミリー名の定義
pub const CF_SHARD_STATE: &str = "shard_state";
pub const CF_TRANSACTIONS: &str = "transactions";
pub const CF_METADATA: &str = "metadata";

/// キープレフィックスの定義
pub const PREFIX_SHARD: &[u8] = b"shard/";
pub const PREFIX_TX: &[u8] = b"tx/";
pub const PREFIX_META: &[u8] = b"meta/";