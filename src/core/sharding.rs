use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use super::dag::{Transaction, TxId};

/// シャードID
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShardId(u64);

/// シャード状態
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardState {
    pub id: ShardId,
    pub transactions: HashSet<TxId>,
    pub state_root: Vec<u8>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// クロスシャードトランザクション
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossShardTx {
    pub tx: Transaction,
    pub source_shard: ShardId,
    pub target_shards: Vec<ShardId>,
    pub status: CrossShardTxStatus,
}

/// クロスシャードトランザクションのステータス
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrossShardTxStatus {
    Pending,
    PartiallyCommitted,
    Committed,
    Failed,
}

/// シャードマネージャ
pub struct ShardManager {
    shards: Arc<RwLock<HashMap<ShardId, ShardState>>>,
    cross_shard_txs: Arc<RwLock<HashMap<TxId, CrossShardTx>>>,
    network: Arc<RwLock<crate::network::P2PNetwork>>,
    shard_comm: Box<dyn ShardCommunication>,
    state_manager: Box<dyn ShardStateManager>,
}

impl ShardManager {
    pub fn new(
        network: Arc<RwLock<crate::network::P2PNetwork>>,
        shard_comm: Box<dyn ShardCommunication>,
        state_manager: Box<dyn ShardStateManager>,
    ) -> Self {
        Self {
            shards: Arc::new(RwLock::new(HashMap::new())),
            cross_shard_txs: Arc::new(RwLock::new(HashMap::new())),
            network,
            shard_comm,
            state_manager,
        }
    }

    /// 新しいシャードを作成
    pub async fn create_shard(&self, id: ShardId) -> anyhow::Result<()> {
        let mut shards = self.shards.write().await;
        if shards.contains_key(&id) {
            anyhow::bail!("Shard already exists: {:?}", id);
        }

        let state = ShardState {
            id: id.clone(),
            transactions: HashSet::new(),
            state_root: Vec::new(),
            last_updated: chrono::Utc::now(),
        };
        shards.insert(id, state);
        Ok(())
    }

    /// トランザクションをシャードに割り当て
    pub async fn assign_transaction(&self, tx: Transaction) -> anyhow::Result<ShardId> {
        // TODO: シャード割り当てロジックを実装
        // 現在は単純なハッシュベースの割り当て
        let shard_id = ShardId(tx.id.0.as_bytes()[0] as u64 % 4);
        
        let mut shards = self.shards.write().await;
        if let Some(shard) = shards.get_mut(&shard_id) {
            shard.transactions.insert(tx.id.clone());
            shard.last_updated = chrono::Utc::now();
        } else {
            anyhow::bail!("Shard not found: {:?}", shard_id);
        }

        Ok(shard_id)
    }

    /// クロスシャードトランザクションを処理
    pub async fn process_cross_shard_tx(&self, tx: CrossShardTx) -> anyhow::Result<()> {
        let mut cross_shard_txs = self.cross_shard_txs.write().await;
        cross_shard_txs.insert(tx.tx.id.clone(), tx.clone());

        // 2フェーズコミットプロトコルを開始
        self.start_two_phase_commit(&tx).await?;

        Ok(())
    }

    /// 2フェーズコミットを開始
    async fn start_two_phase_commit(&self, tx: &CrossShardTx) -> anyhow::Result<()> {
        use crate::network::NetworkMessage;

        // Phase 1: Prepare
        let mut all_prepared = true;
        for shard_id in &tx.target_shards {
            let prepared = self.shard_comm.send_prepare(shard_id, tx).await?;
            if !prepared {
                all_prepared = false;
                break;
            }
        }

        // Phase 2: Commit/Abort
        if all_prepared {
            // すべてのシャードがPrepareに成功した場合はCommit
            for shard_id in &tx.target_shards {
                if !self.shard_comm.send_commit(shard_id, tx).await? {
                    // Commitに失敗した場合はエラーをログに記録
                    warn!("Commit failed for shard: {:?}", shard_id);
                }
            }

            // トランザクションステータスを更新
            let mut txs = self.cross_shard_txs.write().await;
            if let Some(mut stored_tx) = txs.get_mut(&tx.tx.id) {
                stored_tx.status = CrossShardTxStatus::Committed;
            }

            // 他のノードに通知
            let network = self.network.read().await;
            let msg = NetworkMessage::CrossShardTransaction(tx.clone());
            let _ = network.message_sender().send(msg);
        } else {
            // 1つでもPrepareに失敗した場合はAbort
            for shard_id in &tx.target_shards {
                let _ = self.shard_comm.send_abort(shard_id, tx).await;
            }

            // トランザクションステータスを更新
            let mut txs = self.cross_shard_txs.write().await;
            if let Some(mut stored_tx) = txs.get_mut(&tx.tx.id) {
                stored_tx.status = CrossShardTxStatus::Failed;
            }
        }

        Ok(())
    }

    /// シャード状態を同期
    pub async fn sync_shard_state(&self, shard_id: &ShardId) -> anyhow::Result<()> {
        use crate::network::NetworkMessage;

        // 現在のシャード状態を取得
        let current_state = self.state_manager.get_state(shard_id).await?;

        // 他のノードにシャード状態を要求
        let network = self.network.read().await;
        let msg = NetworkMessage::ShardState {
            shard_id: shard_id.clone(),
            state: current_state.clone(),
        };
        network.message_sender().send(msg)?;

        // 応答を待機して状態をマージ
        let mut states = vec![current_state];
        let mut timeout = tokio::time::interval(Duration::from_secs(1));
        let mut attempts = 0;

        while attempts < 5 {
            timeout.tick().await;
            if let Ok(state) = self.state_manager.get_state(shard_id).await {
                states.push(state);
            }
            attempts += 1;
        }

        // 収集した状態をマージ
        let merged_state = self.state_manager.merge_states(states).await?;
        self.state_manager.update_state(merged_state).await?;

        Ok(())
    }

    /// シャードの再バランス
    pub async fn rebalance_shards(&self) -> anyhow::Result<()> {
        let shards = self.shards.read().await;
        let shard_count = shards.len();
        if shard_count < 2 {
            return Ok(());
        }

        // シャードのロードを計算
        let mut shard_loads: Vec<(ShardId, usize)> = shards
            .iter()
            .map(|(id, state)| (id.clone(), state.transactions.len()))
            .collect();

        // ロードでソート
        shard_loads.sort_by_key(|(_id, load)| *load);

        // 最も負荷の高いシャードから最も負荷の低いシャードにトランザクションを移動
        while let (Some(min), Some(max)) = (shard_loads.first(), shard_loads.last()) {
            if max.1 - min.1 <= 1 {
                break;
            }

            // トランザクションの移動を実行
            self.migrate_transactions(&max.0, &min.0).await?;

            // ロード情報を更新
            if let (Some(min_state), Some(max_state)) = (
                shards.get(&min.0),
                shards.get(&max.0),
            ) {
                shard_loads[0].1 = min_state.transactions.len();
                shard_loads[shard_loads.len() - 1].1 = max_state.transactions.len();
            }
        }

        Ok(())
    }

    /// トランザクションを別のシャードに移動
    async fn migrate_transactions(&self, from: &ShardId, to: &ShardId) -> anyhow::Result<()> {
        let mut shards = self.shards.write().await;
        
        if let (Some(from_state), Some(to_state)) = (
            shards.get_mut(from),
            shards.get_mut(to),
        ) {
            // 移動するトランザクションを選択
            let tx_count = from_state.transactions.len();
            let move_count = tx_count / 4; // 25%のトランザクションを移動
            
            let txs_to_move: Vec<_> = from_state.transactions
                .iter()
                .take(move_count)
                .cloned()
                .collect();

            // トランザクションを移動
            for tx_id in txs_to_move {
                from_state.transactions.remove(&tx_id);
                to_state.transactions.insert(tx_id);
            }

            // 状態を更新
            from_state.last_updated = chrono::Utc::now();
            to_state.last_updated = chrono::Utc::now();
        }

        Ok(())
    }
}

/// シャード間通信インターフェース
#[async_trait]
pub trait ShardCommunication: Send + Sync {
    async fn send_prepare(&self, shard_id: &ShardId, tx: &CrossShardTx) -> anyhow::Result<bool>;
    async fn send_commit(&self, shard_id: &ShardId, tx: &CrossShardTx) -> anyhow::Result<bool>;
    async fn send_abort(&self, shard_id: &ShardId, tx: &CrossShardTx) -> anyhow::Result<bool>;
}

/// シャードステート管理
#[async_trait]
pub trait ShardStateManager: Send + Sync {
    async fn get_state(&self, shard_id: &ShardId) -> anyhow::Result<ShardState>;
    async fn update_state(&self, state: ShardState) -> anyhow::Result<()>;
    async fn merge_states(&self, states: Vec<ShardState>) -> anyhow::Result<ShardState>;
}