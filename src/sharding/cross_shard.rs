use crate::common::errors::ShardingError;
use crate::common::types::{ShardId, Transaction, TransactionId};
use crate::sharding::manager::ShardManagerMessage;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, Mutex};
use tracing::{debug, error, info, warn};

/// Status of a cross-shard transaction
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrossShardStatus {
    /// Transaction is pending
    Pending,
    /// Transaction is prepared (phase 1 of 2PC)
    Prepared,
    /// Transaction is committed (phase 2 of 2PC)
    Committed,
    /// Transaction is aborted
    Aborted,
}

/// Cross-shard transaction coordinator
pub struct CrossShardCoordinator {
    /// Shard manager sender
    manager_tx: mpsc::Sender<ShardManagerMessage>,
    /// Active cross-shard transactions
    transactions: Arc<Mutex<HashMap<TransactionId, CrossShardTransaction>>>,
}

/// Cross-shard transaction
struct CrossShardTransaction {
    /// Transaction data
    transaction: Transaction,
    /// Involved shards
    shards: Vec<ShardId>,
    /// Current status
    status: CrossShardStatus,
    /// Prepared shards
    prepared_shards: Vec<ShardId>,
    /// Committed shards
    committed_shards: Vec<ShardId>,
}

impl CrossShardCoordinator {
    /// Create a new cross-shard coordinator
    pub fn new(manager_tx: mpsc::Sender<ShardManagerMessage>) -> Self {
        Self {
            manager_tx,
            transactions: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Start a new cross-shard transaction
    pub async fn start_transaction(
        &self,
        transaction: Transaction,
        shards: Vec<ShardId>,
    ) -> Result<(), ShardingError> {
        if shards.is_empty() {
            return Err(ShardingError::CrossShardFailed(
                "No shards specified".to_string(),
            ));
        }
        
        let tx_id = transaction.id;
        
        let cross_tx = CrossShardTransaction {
            transaction: transaction.clone(),
            shards: shards.clone(),
            status: CrossShardStatus::Pending,
            prepared_shards: Vec::new(),
            committed_shards: Vec::new(),
        };
        
        {
            let mut txs = self.transactions.lock().await;
            txs.insert(tx_id, cross_tx);
        }
        
        info!("Started cross-shard transaction {} across {} shards", tx_id, shards.len());
        
        // Phase 1: Prepare
        for shard_id in &shards {
            if let Err(e) = self.prepare_shard(*shard_id, tx_id).await {
                warn!("Failed to prepare shard {}: {}", shard_id, e);
                self.abort_transaction(tx_id).await?;
                return Err(e);
            }
        }
        
        // Phase 2: Commit
        self.commit_transaction(tx_id).await
    }
    
    /// Prepare a shard for the transaction
    async fn prepare_shard(&self, shard_id: ShardId, tx_id: TransactionId) -> Result<(), ShardingError> {
        debug!("Preparing shard {} for transaction {}", shard_id, tx_id);
        
        // In a real implementation, we would send a prepare message to the shard
        // and wait for a response. For simplicity, we'll just mark it as prepared.
        
        {
            let mut txs = self.transactions.lock().await;
            if let Some(tx) = txs.get_mut(&tx_id) {
                tx.prepared_shards.push(shard_id);
                
                // If all shards are prepared, update status
                if tx.prepared_shards.len() == tx.shards.len() {
                    tx.status = CrossShardStatus::Prepared;
                    debug!("All shards prepared for transaction {}", tx_id);
                }
            } else {
                return Err(ShardingError::CrossShardFailed(
                    format!("Transaction {} not found", tx_id),
                ));
            }
        }
        
        Ok(())
    }
    
    /// Commit a transaction
    async fn commit_transaction(&self, tx_id: TransactionId) -> Result<(), ShardingError> {
        info!("Committing cross-shard transaction {}", tx_id);
        
        let transaction;
        let shards;
        
        {
            let mut txs = self.transactions.lock().await;
            if let Some(tx) = txs.get_mut(&tx_id) {
                if tx.status != CrossShardStatus::Prepared {
                    return Err(ShardingError::CrossShardFailed(
                        format!("Transaction {} is not prepared", tx_id),
                    ));
                }
                
                tx.status = CrossShardStatus::Committed;
                transaction = tx.transaction.clone();
                shards = tx.shards.clone();
            } else {
                return Err(ShardingError::CrossShardFailed(
                    format!("Transaction {} not found", tx_id),
                ));
            }
        }
        
        // In a real implementation, we would send commit messages to all shards
        // For simplicity, we'll just add the transaction to each shard
        
        for shard_id in shards {
            if let Err(e) = self.manager_tx.send(ShardManagerMessage::AddTransaction(transaction.clone())).await {
                error!("Failed to send transaction to shard {}: {}", shard_id, e);
                return Err(ShardingError::CrossShardFailed(
                    format!("Failed to commit to shard {}: {}", shard_id, e),
                ));
            }
            
            {
                let mut txs = self.transactions.lock().await;
                if let Some(tx) = txs.get_mut(&tx_id) {
                    tx.committed_shards.push(shard_id);
                }
            }
        }
        
        // Clean up the transaction
        {
            let mut txs = self.transactions.lock().await;
            txs.remove(&tx_id);
        }
        
        info!("Cross-shard transaction {} committed successfully", tx_id);
        
        Ok(())
    }
    
    /// Abort a transaction
    async fn abort_transaction(&self, tx_id: TransactionId) -> Result<(), ShardingError> {
        info!("Aborting cross-shard transaction {}", tx_id);
        
        {
            let mut txs = self.transactions.lock().await;
            if let Some(tx) = txs.get_mut(&tx_id) {
                tx.status = CrossShardStatus::Aborted;
            } else {
                return Err(ShardingError::CrossShardFailed(
                    format!("Transaction {} not found", tx_id),
                ));
            }
        }
        
        // In a real implementation, we would send abort messages to all prepared shards
        // For simplicity, we'll just remove the transaction
        
        {
            let mut txs = self.transactions.lock().await;
            txs.remove(&tx_id);
        }
        
        info!("Cross-shard transaction {} aborted", tx_id);
        
        Ok(())
    }
    
    /// Get the status of a transaction
    pub async fn get_transaction_status(&self, tx_id: TransactionId) -> Option<CrossShardStatus> {
        let txs = self.transactions.lock().await;
        txs.get(&tx_id).map(|tx| tx.status.clone())
    }
}