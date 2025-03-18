use crate::common::errors::LedgerError;
use crate::common::types::{Transaction, TransactionId};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::Direction;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// DAGノードの状態
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeState {
    /// 未実行
    Pending,
    /// 実行中
    Executing,
    /// 実行完了
    Executed,
    /// 実行失敗
    Failed(String),
}

/// DAGノード
#[derive(Debug, Clone)]
pub struct DagNode {
    /// トランザクションID
    pub tx_id: TransactionId,
    /// トランザクション
    pub transaction: Transaction,
    /// ノードの状態
    pub state: NodeState,
    /// 実行結果（あれば）
    pub result: Option<Vec<u8>>,
}

/// トランザクションDAG
pub struct TransactionDag {
    /// グラフ
    graph: DiGraph<DagNode, ()>,
    /// トランザクションIDからノードインデックスへのマッピング
    tx_to_node: HashMap<TransactionId, NodeIndex>,
    /// 実行可能なノード
    executable: HashSet<NodeIndex>,
    /// 実行済みノード
    executed: HashSet<NodeIndex>,
}

impl TransactionDag {
    /// 新しいDAGを作成
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            tx_to_node: HashMap::new(),
            executable: HashSet::new(),
            executed: HashSet::new(),
        }
    }
    
    /// トランザクションをDAGに追加
    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), LedgerError> {
        let tx_id = transaction.id;
        
        // すでに存在するか確認
        if self.tx_to_node.contains_key(&tx_id) {
            return Err(LedgerError::AlreadyExists(format!(
                "Transaction {} already exists in DAG",
                tx_id
            )));
        }
        
        // ノードを作成
        let node = DagNode {
            tx_id,
            transaction: transaction.clone(),
            state: NodeState::Pending,
            result: None,
        };
        
        // グラフに追加
        let node_idx = self.graph.add_node(node);
        self.tx_to_node.insert(tx_id, node_idx);
        
        // 依存関係を追加
        self.add_dependencies(tx_id, &transaction)?;
        
        // 実行可能かチェック
        if self.is_executable(node_idx) {
            self.executable.insert(node_idx);
        }
        
        debug!("Added transaction {} to DAG", tx_id);
        
        Ok(())
    }
    
    /// トランザクションの依存関係を追加
    fn add_dependencies(&mut self, tx_id: TransactionId, transaction: &Transaction) -> Result<(), LedgerError> {
        // 送信者の前のトランザクションに依存
        let sender = transaction.sender;
        let nonce = transaction.nonce;
        
        // 同じ送信者の前のノンスを持つトランザクションを探す
        for (other_tx_id, node_idx) in &self.tx_to_node {
            let other_node = &self.graph[*node_idx];
            let other_tx = &other_node.transaction;
            
            if other_tx.sender == sender {
                if other_tx.nonce < nonce {
                    // 前のノンスに依存
                    let tx_node_idx = self.tx_to_node.get(&tx_id).unwrap();
                    self.graph.add_edge(*node_idx, *tx_node_idx, ());
                    debug!("Added dependency: {} -> {}", other_tx_id, tx_id);
                } else if other_tx.nonce == nonce {
                    // 同じノンスは競合
                    return Err(LedgerError::Transaction(
                        crate::common::errors::TransactionError::InvalidNonce,
                    ));
                }
            }
        }
        
        Ok(())
    }
    
    /// ノードが実行可能かチェック
    fn is_executable(&self, node_idx: NodeIndex) -> bool {
        // すべての依存関係が実行済みかチェック
        let dependencies = self.graph.neighbors_directed(node_idx, Direction::Incoming);
        
        for dep_idx in dependencies {
            if !self.executed.contains(&dep_idx) {
                return false;
            }
        }
        
        true
    }
    
    /// 実行可能なトランザクションを取得
    pub fn get_executable_transactions(&self) -> Vec<Transaction> {
        self.executable
            .iter()
            .map(|node_idx| self.graph[*node_idx].transaction.clone())
            .collect()
    }
    
    /// トランザクションの実行を開始
    pub fn start_execution(&mut self, tx_id: &TransactionId) -> Result<(), LedgerError> {
        let node_idx = self.tx_to_node.get(tx_id).ok_or_else(|| {
            LedgerError::NotFound(format!("Transaction {} not found in DAG", tx_id))
        })?;
        
        if !self.executable.contains(node_idx) {
            return Err(LedgerError::InvalidState(format!(
                "Transaction {} is not executable",
                tx_id
            )));
        }
        
        // 状態を更新
        let node = &mut self.graph[*node_idx];
        node.state = NodeState::Executing;
        
        // 実行可能リストから削除
        self.executable.remove(node_idx);
        
        debug!("Started execution of transaction {}", tx_id);
        
        Ok(())
    }
    
    /// トランザクションの実行を完了
    pub fn complete_execution(
        &mut self,
        tx_id: &TransactionId,
        result: Option<Vec<u8>>,
        success: bool,
    ) -> Result<(), LedgerError> {
        let node_idx = self.tx_to_node.get(tx_id).ok_or_else(|| {
            LedgerError::NotFound(format!("Transaction {} not found in DAG", tx_id))
        })?;
        
        // 状態を更新
        let node = &mut self.graph[*node_idx];
        
        if node.state != NodeState::Executing {
            return Err(LedgerError::InvalidState(format!(
                "Transaction {} is not in executing state",
                tx_id
            )));
        }
        
        if success {
            node.state = NodeState::Executed;
            node.result = result;
            
            // 実行済みリストに追加
            self.executed.insert(*node_idx);
            
            // 依存するトランザクションを実行可能にする
            let dependents = self.graph.neighbors_directed(*node_idx, Direction::Outgoing);
            
            for dep_idx in dependents {
                if self.is_executable(dep_idx) {
                    self.executable.insert(dep_idx);
                    debug!(
                        "Transaction {} is now executable",
                        self.graph[dep_idx].tx_id
                    );
                }
            }
        } else {
            let error_msg = String::from_utf8_lossy(&result.unwrap_or_default()).to_string();
            node.state = NodeState::Failed(error_msg.clone());
            
            warn!("Transaction {} failed: {}", tx_id, error_msg);
        }
        
        debug!("Completed execution of transaction {}", tx_id);
        
        Ok(())
    }
    
    /// トランザクションの状態を取得
    pub fn get_transaction_state(&self, tx_id: &TransactionId) -> Result<NodeState, LedgerError> {
        let node_idx = self.tx_to_node.get(tx_id).ok_or_else(|| {
            LedgerError::NotFound(format!("Transaction {} not found in DAG", tx_id))
        })?;
        
        Ok(self.graph[*node_idx].state.clone())
    }
    
    /// トランザクションの結果を取得
    pub fn get_transaction_result(&self, tx_id: &TransactionId) -> Result<Option<Vec<u8>>, LedgerError> {
        let node_idx = self.tx_to_node.get(tx_id).ok_or_else(|| {
            LedgerError::NotFound(format!("Transaction {} not found in DAG", tx_id))
        })?;
        
        Ok(self.graph[*node_idx].result.clone())
    }
    
    /// DAGの統計情報を取得
    pub fn get_stats(&self) -> (usize, usize, usize, usize) {
        (
            self.graph.node_count(),
            self.executable.len(),
            self.executed.len(),
            self.graph.node_count() - self.executable.len() - self.executed.len(),
        )
    }
}

/// スレッドセーフなDAG
pub type SharedDag = Arc<RwLock<TransactionDag>>;

/// 新しい共有DAGを作成
pub fn new_shared_dag() -> SharedDag {
    Arc::new(RwLock::new(TransactionDag::new()))
}