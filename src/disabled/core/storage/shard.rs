use super::{StorageEngine, CF_SHARD_STATE, PREFIX_SHARD};
use crate::core::sharding::{ShardId, ShardState};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

/// シャードステートマネージャー
pub struct ShardStateManager {
    storage: Arc<dyn StorageEngine>,
}

impl ShardStateManager {
    pub fn new(storage: Arc<dyn StorageEngine>) -> Self {
        Self { storage }
    }

    /// シャードIDからストレージキーを生成
    fn make_key(shard_id: &ShardId) -> Vec<u8> {
        let mut key = Vec::with_capacity(PREFIX_SHARD.len() + 8);
        key.extend_from_slice(PREFIX_SHARD);
        key.extend_from_slice(&shard_id.0.to_be_bytes());
        key
    }
}

#[async_trait]
impl crate::core::sharding::ShardStateManager for ShardStateManager {
    async fn get_state(&self, shard_id: &ShardId) -> Result<ShardState> {
        let key = Self::make_key(shard_id);
        if let Some(state) = self.storage.get(CF_SHARD_STATE, key).await? {
            Ok(state)
        } else {
            // 新しいシャード状態を作成
            Ok(ShardState {
                id: shard_id.clone(),
                transactions: Default::default(),
                state_root: Vec::new(),
                last_updated: chrono::Utc::now(),
            })
        }
    }

    async fn update_state(&self, state: ShardState) -> Result<()> {
        let key = Self::make_key(&state.id);
        self.storage.put(CF_SHARD_STATE, key, state).await
    }

    async fn merge_states(&self, states: Vec<ShardState>) -> Result<ShardState> {
        if states.is_empty() {
            anyhow::bail!("No states to merge");
        }

        let mut merged = states[0].clone();
        for state in states.into_iter().skip(1) {
            // トランザクションをマージ
            for tx_id in state.transactions {
                merged.transactions.insert(tx_id);
            }

            // 最新の更新時刻を使用
            if state.last_updated > merged.last_updated {
                merged.last_updated = state.last_updated;
                merged.state_root = state.state_root;
            }
        }

        Ok(merged)
    }
}

/// シャードステートのバックアップマネージャー
pub struct ShardStateBackup {
    storage: Arc<dyn StorageEngine>,
}

impl ShardStateBackup {
    pub fn new(storage: Arc<dyn StorageEngine>) -> Self {
        Self { storage }
    }

    /// すべてのシャードステートをバックアップ
    pub async fn backup_all_states(&self, path: &std::path::Path) -> Result<()> {
        self.storage.create_snapshot(path).await
    }

    /// バックアップからシャードステートを復元
    pub async fn restore_states(&self, path: &std::path::Path) -> Result<()> {
        self.storage.restore_from_snapshot(path).await
    }

    /// 特定のシャードのステートをエクスポート
    pub async fn export_shard_state(&self, shard_id: &ShardId, path: &std::path::Path) -> Result<()> {
        let key = ShardStateManager::make_key(shard_id);
        if let Some(state) = self.storage.get::<_, ShardState>(CF_SHARD_STATE, key).await? {
            let file = std::fs::File::create(path)?;
            serde_json::to_writer_pretty(file, &state)?;
        }
        Ok(())
    }

    /// エクスポートされたシャードステートをインポート
    pub async fn import_shard_state(&self, path: &std::path::Path) -> Result<()> {
        let file = std::fs::File::open(path)?;
        let state: ShardState = serde_json::from_reader(file)?;
        let key = ShardStateManager::make_key(&state.id);
        self.storage.put(CF_SHARD_STATE, key, state).await
    }
}