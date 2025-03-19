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
    async fn put_bytes(&self, cf: &str, key: &[u8], value: &[u8]) -> Result<()>;

    /// 値を取得
    async fn get_bytes(&self, cf: &str, key: &[u8]) -> Result<Option<Vec<u8>>>;

    /// キーを削除
    async fn delete_bytes(&self, cf: &str, key: &[u8]) -> Result<()>;

    /// プレフィックスでスキャン
    async fn scan_prefix_bytes(&self, cf: &str, prefix: &[u8]) -> Result<Vec<(Vec<u8>, Vec<u8>)>>;

    /// バッチ書き込み
    async fn batch_write_bytes(&self, cf: &str, pairs: &[(&[u8], &[u8])]) -> Result<()>;

    /// スナップショットを作成
    async fn create_snapshot(&self, path: &Path) -> Result<()>;

    /// スナップショットから復元
    async fn restore_from_snapshot(&self, path: &Path) -> Result<()>;
}

/// 型付きストレージ拡張
#[async_trait]
pub trait TypedStorage: StorageEngine {
    /// 型付きの値を保存
    async fn put<K, V>(&self, cf: &str, key: K, value: &V) -> Result<()>
    where
        K: AsRef<[u8]> + Send + Sync,
        V: Serialize + Send + Sync,
    {
        let value_bytes = bincode::serialize(value)?;
        self.put_bytes(cf, key.as_ref(), &value_bytes).await
    }

    /// 型付きの値を取得
    async fn get<K, V>(&self, cf: &str, key: K) -> Result<Option<V>>
    where
        K: AsRef<[u8]> + Send + Sync,
        V: DeserializeOwned + Send + Sync,
    {
        if let Some(bytes) = self.get_bytes(cf, key.as_ref()).await? {
            Ok(Some(bincode::deserialize(&bytes)?))
        } else {
            Ok(None)
        }
    }

    /// 型付きのキーを削除
    async fn delete<K>(&self, cf: &str, key: K) -> Result<()>
    where
        K: AsRef<[u8]> + Send + Sync,
    {
        self.delete_bytes(cf, key.as_ref()).await
    }

    /// 型付きのプレフィックスでスキャン
    async fn scan_prefix<K, V>(&self, cf: &str, prefix: K) -> Result<Vec<(Vec<u8>, V)>>
    where
        K: AsRef<[u8]> + Send + Sync,
        V: DeserializeOwned + Send + Sync,
    {
        let pairs = self.scan_prefix_bytes(cf, prefix.as_ref()).await?;
        let mut result = Vec::with_capacity(pairs.len());
        for (key, value) in pairs {
            result.push((key, bincode::deserialize(&value)?));
        }
        Ok(result)
    }

    /// 型付きのバッチ書き込み
    async fn batch_write<K, V>(&self, cf: &str, pairs: &[(K, V)]) -> Result<()>
    where
        K: AsRef<[u8]> + Send + Sync,
        V: Serialize + Send + Sync,
    {
        let mut bytes_pairs: Vec<(&[u8], &[u8])> = Vec::with_capacity(pairs.len());
        let mut value_bytes_vec = Vec::with_capacity(pairs.len());

        for (key, value) in pairs {
            let value_bytes = bincode::serialize(value)?;
            value_bytes_vec.push(value_bytes);
        }

        let mut refs = Vec::with_capacity(pairs.len());
        for (i, (key, _)) in pairs.iter().enumerate() {
            refs.push((key.as_ref(), value_bytes_vec[i].as_slice()));
        }

        self.batch_write_bytes(cf, &refs).await
    }
}

/// カラムファミリー名の定義
pub const CF_SHARD_STATE: &str = "shard_state";
pub const CF_TRANSACTIONS: &str = "transactions";
pub const CF_METADATA: &str = "metadata";

/// キープレフィックスの定義
pub const PREFIX_SHARD: &[u8] = b"shard/";
pub const PREFIX_TX: &[u8] = b"tx/";
pub const PREFIX_META: &[u8] = b"meta/";