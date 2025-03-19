use super::{StorageEngine, CF_METADATA, CF_TRANSACTIONS, PREFIX_META, PREFIX_TX};
use crate::core::dag::{Transaction, TxId};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// グローバルステート
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalState {
    pub last_block_height: u64,
    pub total_transactions: u64,
    pub last_updated: DateTime<Utc>,
    pub network_version: String,
}

/// ステートマネージャー
pub struct StateManager {
    storage: Arc<dyn StorageEngine>,
}

impl StateManager {
    pub fn new(storage: Arc<dyn StorageEngine>) -> Self {
        Self { storage }
    }

    /// トランザクションを保存
    pub async fn save_transaction(&self, tx: Transaction) -> Result<()> {
        let key = self.make_tx_key(&tx.id);
        self.storage.put(CF_TRANSACTIONS, key, tx).await
    }

    /// トランザクションを取得
    pub async fn get_transaction(&self, tx_id: &TxId) -> Result<Option<Transaction>> {
        let key = self.make_tx_key(tx_id);
        self.storage.get(CF_TRANSACTIONS, key).await
    }

    /// トランザクションをバッチ保存
    pub async fn save_transactions(&self, txs: Vec<Transaction>) -> Result<()> {
        let pairs: Vec<_> = txs
            .into_iter()
            .map(|tx| (self.make_tx_key(&tx.id), tx))
            .collect();
        self.storage.batch_write(CF_TRANSACTIONS, pairs).await
    }

    /// グローバルステートを更新
    pub async fn update_global_state(&self, state: GlobalState) -> Result<()> {
        let key = self.make_meta_key(b"global_state");
        self.storage.put(CF_METADATA, key, state).await
    }

    /// グローバルステートを取得
    pub async fn get_global_state(&self) -> Result<Option<GlobalState>> {
        let key = self.make_meta_key(b"global_state");
        self.storage.get(CF_METADATA, key).await
    }

    /// メタデータを保存
    pub async fn save_metadata<V: Serialize + Send + Sync>(
        &self,
        key: &[u8],
        value: V,
    ) -> Result<()> {
        let full_key = self.make_meta_key(key);
        self.storage.put(CF_METADATA, full_key, value).await
    }

    /// メタデータを取得
    pub async fn get_metadata<V: for<'de> Deserialize<'de> + Send + Sync>(
        &self,
        key: &[u8],
    ) -> Result<Option<V>> {
        let full_key = self.make_meta_key(key);
        self.storage.get(CF_METADATA, full_key).await
    }

    /// トランザクションキーを生成
    fn make_tx_key(&self, tx_id: &TxId) -> Vec<u8> {
        let mut key = Vec::with_capacity(PREFIX_TX.len() + tx_id.0.len());
        key.extend_from_slice(PREFIX_TX);
        key.extend_from_slice(tx_id.0.as_bytes());
        key
    }

    /// メタデータキーを生成
    fn make_meta_key(&self, key: &[u8]) -> Vec<u8> {
        let mut full_key = Vec::with_capacity(PREFIX_META.len() + key.len());
        full_key.extend_from_slice(PREFIX_META);
        full_key.extend_from_slice(key);
        full_key
    }
}

/// ステート同期マネージャー
pub struct StateSyncManager {
    storage: Arc<dyn StorageEngine>,
}

impl StateSyncManager {
    pub fn new(storage: Arc<dyn StorageEngine>) -> Self {
        Self { storage }
    }

    /// 特定の期間のトランザクションを取得
    pub async fn get_transactions_in_range(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Vec<Transaction>> {
        let prefix = PREFIX_TX;
        let mut txs = Vec::new();

        let results: Vec<(Vec<u8>, Transaction)> = self.storage
            .scan_prefix(CF_TRANSACTIONS, prefix)
            .await?;

        for (_, tx) in results {
            if tx.timestamp >= start_time && tx.timestamp <= end_time {
                txs.push(tx);
            }
        }

        Ok(txs)
    }

    /// ステートの差分を計算
    pub async fn calculate_state_diff(
        &self,
        from_height: u64,
        to_height: u64,
    ) -> Result<StateDiff> {
        // TODO: ステート差分の計算ロジックを実装
        Ok(StateDiff {
            from_height,
            to_height,
            transactions: Vec::new(),
            metadata_changes: Vec::new(),
        })
    }

    /// ステートの差分を適用
    pub async fn apply_state_diff(&self, diff: StateDiff) -> Result<()> {
        // トランザクションを適用
        self.storage
            .batch_write(CF_TRANSACTIONS, diff.transactions)
            .await?;

        // メタデータの変更を適用
        self.storage
            .batch_write(CF_METADATA, diff.metadata_changes)
            .await?;

        Ok(())
    }
}

/// ステート差分
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateDiff {
    pub from_height: u64,
    pub to_height: u64,
    pub transactions: Vec<(Vec<u8>, Transaction)>,
    pub metadata_changes: Vec<(Vec<u8>, Vec<u8>)>,
}