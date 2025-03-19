mod state;
mod sync;
mod load_balancer;

use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use super::storage::StorageEngine;
use crate::network::P2PNetwork;

pub use state::{ShardState, ShardTransaction};
pub use load_balancer::LoadBalancer;

/// シャードID
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShardId(Vec<u8>);

impl ShardId {
    /// 新しいシャードIDを作成
    pub fn new(id: Vec<u8>) -> Self {
        Self(id)
    }

    /// シャードIDをバイト列として取得
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// シャードマネージャー
pub struct ShardManager {
    /// シャードの状態
    shards: Arc<RwLock<HashMap<ShardId, ShardState>>>,
    /// ストレージエンジン
    storage: Arc<dyn StorageEngine>,
    /// P2Pネットワーク
    network: Arc<P2PNetwork>,
    /// 負荷分散マネージャー
    load_balancer: Arc<LoadBalancer>,
    /// 同期マネージャー
    sync_manager: Arc<sync::SyncManager>,
}

impl ShardManager {
    /// 新しいシャードマネージャーを作成
    pub fn new(storage: Arc<dyn StorageEngine>, network: Arc<P2PNetwork>) -> Self {
        let load_balancer = Arc::new(LoadBalancer::new());
        let sync_manager = Arc::new(sync::SyncManager::new(storage.clone(), network.clone()));
        
        Self {
            shards: Arc::new(RwLock::new(HashMap::new())),
            storage,
            network,
            load_balancer,
            sync_manager,
        }
    }

    /// シャードマネージャーを起動
    pub async fn start(&self) -> Result<()> {
        // 1. 保存されているシャードを読み込む
        self.load_shards().await?;

        // 2. 負荷分散マネージャーを起動
        self.load_balancer.start().await?;

        // 3. 同期マネージャーを起動
        self.sync_manager.start().await?;

        Ok(())
    }

    /// シャードマネージャーを停止
    pub async fn stop(&self) -> Result<()> {
        // 1. 同期マネージャーを停止
        self.sync_manager.stop().await?;

        // 2. 負荷分散マネージャーを停止
        self.load_balancer.stop().await?;

        // 3. シャードの状態を保存
        self.save_shards().await?;

        Ok(())
    }

    /// シャードを作成
    pub async fn create_shard(&self, id: ShardId) -> Result<()> {
        let mut shards = self.shards.write().await;
        let state = ShardState::new(id.clone());
        shards.insert(id.clone(), state);
        self.storage.put_shard_state(&id, &state).await?;
        Ok(())
    }

    /// シャードを削除
    pub async fn delete_shard(&self, id: &ShardId) -> Result<()> {
        let mut shards = self.shards.write().await;
        shards.remove(id);
        self.storage.delete_shard_state(id).await?;
        Ok(())
    }

    /// シャードの状態を取得
    pub async fn get_shard_state(&self, id: &ShardId) -> Result<Option<ShardState>> {
        let shards = self.shards.read().await;
        Ok(shards.get(id).cloned())
    }

    /// トランザクションを処理
    pub async fn process_transaction(&self, tx: ShardTransaction) -> Result<()> {
        // 1. シャードを特定
        let shard_id = self.load_balancer.get_shard_for_transaction(&tx).await?;

        // 2. シャードの状態を取得
        let mut shards = self.shards.write().await;
        let state = shards.get_mut(&shard_id).ok_or_else(|| {
            anyhow::anyhow!("Shard not found: {:?}", shard_id)
        })?;

        // 3. トランザクションを処理
        state.process_transaction(tx).await?;

        // 4. 状態を保存
        self.storage.put_shard_state(&shard_id, state).await?;

        Ok(())
    }

    /// クロスシャードトランザクションを処理
    pub async fn process_cross_shard_transaction(
        &self,
        tx: ShardTransaction,
        source_shard: &ShardId,
        target_shard: &ShardId,
    ) -> Result<()> {
        // 1. ソースシャードでトランザクションを準備
        let mut shards = self.shards.write().await;
        let source_state = shards.get_mut(source_shard).ok_or_else(|| {
            anyhow::anyhow!("Source shard not found: {:?}", source_shard)
        })?;
        source_state.prepare_transaction(&tx).await?;

        // 2. ターゲットシャードでトランザクションを準備
        let target_state = shards.get_mut(target_shard).ok_or_else(|| {
            anyhow::anyhow!("Target shard not found: {:?}", target_shard)
        })?;
        target_state.prepare_transaction(&tx).await?;

        // 3. 両方のシャードでトランザクションをコミット
        source_state.commit_transaction(&tx).await?;
        target_state.commit_transaction(&tx).await?;

        // 4. 状態を保存
        self.storage.put_shard_state(source_shard, source_state).await?;
        self.storage.put_shard_state(target_shard, target_state).await?;

        Ok(())
    }

    /// シャードを再分割
    pub async fn reshard(&self) -> Result<()> {
        // 1. 負荷情報を収集
        let load_info = self.load_balancer.collect_load_info().await?;

        // 2. 再分割が必要か判断
        if !self.load_balancer.should_reshard(&load_info).await? {
            return Ok(());
        }

        // 3. 新しいシャード構成を計算
        let new_shards = self.load_balancer.calculate_new_shards(&load_info).await?;

        // 4. シャードを再構成
        self.apply_new_shards(new_shards).await?;

        Ok(())
    }

    /// 保存されているシャードを読み込む
    async fn load_shards(&self) -> Result<()> {
        let mut shards = self.shards.write().await;
        let stored_shards = self.storage.get_all_shard_states().await?;
        *shards = stored_shards;
        Ok(())
    }

    /// シャードの状態を保存
    async fn save_shards(&self) -> Result<()> {
        let shards = self.shards.read().await;
        for (id, state) in shards.iter() {
            self.storage.put_shard_state(id, state).await?;
        }
        Ok(())
    }

    /// 新しいシャード構成を適用
    async fn apply_new_shards(&self, new_shards: Vec<ShardState>) -> Result<()> {
        // 1. 現在のシャードをロック
        let mut shards = self.shards.write().await;

        // 2. 新しいシャードを作成
        for state in new_shards {
            shards.insert(state.id.clone(), state.clone());
            self.storage.put_shard_state(&state.id, &state).await?;
        }

        // 3. 古いシャードを削除
        let old_shards: Vec<_> = shards.keys().cloned().collect();
        for id in old_shards {
            if !shards.contains_key(&id) {
                self.storage.delete_shard_state(&id).await?;
            }
        }

        Ok(())
    }
}