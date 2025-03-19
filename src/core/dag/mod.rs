use std::collections::{HashMap, HashSet};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use std::sync::Arc;

/// トランザクションID
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct TxId(Vec<u8>);

/// トランザクションの状態
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TxStatus {
    /// 保留中
    Pending,
    /// 検証済み
    Verified,
    /// 確定済み
    Confirmed,
    /// 競合あり
    Conflicting,
    /// 拒否
    Rejected,
}

/// トランザクションデータ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxData {
    /// 送信者
    pub from: Address,
    /// 受信者
    pub to: Address,
    /// 金額
    pub amount: u64,
    /// 手数料
    pub fee: u64,
    /// 追加データ
    pub extra: Option<Vec<u8>>,
}

/// トランザクション
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// トランザクションID
    pub id: TxId,
    /// 依存するトランザクション
    pub dependencies: Vec<TxId>,
    /// トランザクションデータ
    pub data: TxData,
    /// 署名
    pub signature: Vec<u8>,
}

/// DAGマネージャー
pub struct DAGManager {
    /// トランザクションストア
    transactions: Arc<RwLock<HashMap<TxId, Transaction>>>,
    /// 保留中のトランザクション
    pending: Arc<RwLock<Vec<Transaction>>>,
    /// 確定済みのトランザクション
    confirmed: Arc<RwLock<HashSet<TxId>>>,
    /// ストレージエンジン
    storage: Arc<dyn StorageEngine>,
}

impl DAGManager {
    /// 新しいDAGマネージャーを作成
    pub fn new(storage: Arc<dyn StorageEngine>) -> Self {
        Self {
            transactions: Arc::new(RwLock::new(HashMap::new())),
            pending: Arc::new(RwLock::new(Vec::new())),
            confirmed: Arc::new(RwLock::new(HashSet::new())),
            storage,
        }
    }

    /// トランザクションを追加
    pub async fn add_transaction(&self, tx: Transaction) -> Result<()> {
        // 依存関係の検証
        self.validate_dependencies(&tx).await?;

        // トランザクションを保存
        let mut txs = self.transactions.write().await;
        txs.insert(tx.id.clone(), tx.clone());

        // 保留中のトランザクションに追加
        let mut pending = self.pending.write().await;
        pending.push(tx);

        Ok(())
    }

    /// 依存関係を検証
    async fn validate_dependencies(&self, tx: &Transaction) -> Result<()> {
        let txs = self.transactions.read().await;
        
        for dep_id in &tx.dependencies {
            if !txs.contains_key(dep_id) {
                return Err(anyhow::anyhow!("Missing dependency: {:?}", dep_id));
            }
        }

        Ok(())
    }

    /// トランザクションを確定
    pub async fn confirm_transaction(&self, tx_id: &TxId) -> Result<()> {
        // トランザクションを確定済みに移動
        let mut confirmed = self.confirmed.write().await;
        confirmed.insert(tx_id.clone());

        // ストレージに保存
        let txs = self.transactions.read().await;
        if let Some(tx) = txs.get(tx_id) {
            self.storage.put_transaction(tx).await?;
        }

        Ok(())
    }

    /// 並列実行可能なトランザクションを取得
    pub async fn get_executable_transactions(&self) -> Result<Vec<Transaction>> {
        let txs = self.transactions.read().await;
        let confirmed = self.confirmed.read().await;
        let pending = self.pending.read().await;

        let mut executable = Vec::new();
        for tx in pending.iter() {
            // 全ての依存関係が確定済みかチェック
            let all_deps_confirmed = tx.dependencies.iter()
                .all(|dep_id| confirmed.contains(dep_id));

            if all_deps_confirmed {
                executable.push(tx.clone());
            }
        }

        Ok(executable)
    }

    /// 競合を検出
    pub async fn detect_conflicts(&self, tx: &Transaction) -> Result<Vec<Transaction>> {
        let txs = self.transactions.read().await;
        let mut conflicts = Vec::new();

        // 同じ送信者からの未確定のトランザクションをチェック
        for other_tx in txs.values() {
            if other_tx.data.from == tx.data.from 
                && other_tx.id != tx.id 
                && !self.confirmed.read().await.contains(&other_tx.id)
            {
                conflicts.push(other_tx.clone());
            }
        }

        Ok(conflicts)
    }

    /// トランザクションの状態を取得
    pub async fn get_transaction_status(&self, tx_id: &TxId) -> Result<TxStatus> {
        let confirmed = self.confirmed.read().await;
        if confirmed.contains(tx_id) {
            return Ok(TxStatus::Confirmed);
        }

        let txs = self.transactions.read().await;
        if let Some(tx) = txs.get(tx_id) {
            let conflicts = self.detect_conflicts(tx).await?;
            if !conflicts.is_empty() {
                Ok(TxStatus::Conflicting)
            } else {
                Ok(TxStatus::Pending)
            }
        } else {
            Ok(TxStatus::Rejected)
        }
    }
}