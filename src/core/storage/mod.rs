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

    /// トランザクションを保存
    async fn put_transaction(&self, tx: &Transaction) -> Result<()> {
        let tx_bytes = bincode::serialize(tx)?;
        self.put_bytes(CF_TRANSACTIONS, tx.id.as_bytes(), &tx_bytes).await
    }

    /// トランザクションを取得
    async fn get_transaction(&self, tx_id: &TxId) -> Result<Option<Transaction>> {
        if let Some(tx_bytes) = self.get_bytes(CF_TRANSACTIONS, tx_id.as_bytes()).await? {
            Ok(Some(bincode::deserialize(&tx_bytes)?))
        } else {
            Ok(None)
        }
    }

    /// トランザクションを削除
    async fn delete_transaction(&self, tx_id: &TxId) -> Result<()> {
        self.delete_bytes(CF_TRANSACTIONS, tx_id.as_bytes()).await
    }

    /// トランザクションをバッチ保存
    async fn batch_write_transactions(&self, txs: &[&Transaction]) -> Result<()> {
        let mut pairs = Vec::with_capacity(txs.len());
        let mut tx_bytes_vec = Vec::with_capacity(txs.len());

        for tx in txs {
            let tx_bytes = bincode::serialize(tx)?;
            tx_bytes_vec.push(tx_bytes);
            pairs.push((tx.id.as_bytes(), tx_bytes_vec.last().unwrap().as_slice()));
        }

        self.batch_write_bytes(CF_TRANSACTIONS, &pairs).await
    }
}

/// 型付きストレージ拡張
#[async_trait]
pub trait TypedStorage: StorageEngine {
    /// 型付きの値を保存
    async fn put<K, V>(&self, cf: &str, key: K, value: &V) -> Result<()>
    where
        K: AsRef<[u8]> + Send + Sync,
        V: Serialize + Send + Sync;

    /// 型付きの値を取得
    async fn get<K, V>(&self, cf: &str, key: K) -> Result<Option<V>>
    where
        K: AsRef<[u8]> + Send + Sync,
        V: DeserializeOwned + Send + Sync;

    /// 型付きのキーを削除
    async fn delete<K>(&self, cf: &str, key: K) -> Result<()>
    where
        K: AsRef<[u8]> + Send + Sync;

    /// 型付きのプレフィックスでスキャン
    async fn scan_prefix<K, V>(&self, cf: &str, prefix: K) -> Result<Vec<(Vec<u8>, V)>>
    where
        K: AsRef<[u8]> + Send + Sync,
        V: DeserializeOwned + Send + Sync;

    /// 型付きのバッチ書き込み
    async fn batch_write<K, V>(&self, cf: &str, pairs: &[(K, &V)]) -> Result<()>
    where
        K: AsRef<[u8]> + Send + Sync,
        V: Serialize + Send + Sync;
}

/// カラムファミリー名の定義
pub const CF_SHARD_STATE: &str = "shard_state";
pub const CF_TRANSACTIONS: &str = "transactions";
pub const CF_METADATA: &str = "metadata";

/// キープレフィックスの定義
pub const PREFIX_SHARD: &[u8] = b"shard/";
pub const PREFIX_TX: &[u8] = b"tx/";
pub const PREFIX_META: &[u8] = b"meta/";