use crate::common::errors::ShardingError;
use crate::common::types::{ShardId, Transaction, TransactionId};
use crate::sharding::ring::{SharedShardRing, ShardRing};
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info};

/// Message types for shard manager
#[derive(Debug)]
pub enum ShardManagerMessage {
    /// Add a transaction to a shard
    AddTransaction(Transaction),
    /// Get a transaction from a shard
    GetTransaction(TransactionId, mpsc::Sender<Option<Transaction>>),
    /// Add a new shard
    AddShard(ShardId),
    /// Remove a shard
    RemoveShard(ShardId),
    /// Rebalance shards
    Rebalance,
    /// Shutdown the shard manager
    Shutdown,
}

/// Shard manager handles transaction distribution across shards
pub struct ShardManager {
    /// Consistent hash ring for shard assignment
    ring: SharedShardRing,
    /// Transaction storage per shard
    shards: Arc<DashMap<ShardId, DashMap<TransactionId, Transaction>>>,
    /// Command channel
    command_rx: mpsc::Receiver<ShardManagerMessage>,
    /// Command sender for external use
    command_tx: mpsc::Sender<ShardManagerMessage>,
}

impl ShardManager {
    /// Create a new shard manager
    pub fn new(shard_count: u32) -> Self {
        let ring = Arc::new(parking_lot::RwLock::new(ShardRing::new(shard_count, 100)));
        let shards = Arc::new(DashMap::new());
        
        // Initialize shards
        for i in 0..shard_count {
            let shard_id = ShardId(i);
            shards.insert(shard_id, DashMap::new());
        }
        
        let (command_tx, command_rx) = mpsc::channel(1000);
        
        Self {
            ring,
            shards,
            command_rx,
            command_tx,
        }
    }
    
    /// Get the command sender
    pub fn get_sender(&self) -> mpsc::Sender<ShardManagerMessage> {
        self.command_tx.clone()
    }
    
    /// Start the shard manager
    pub async fn run(&mut self) {
        info!("Shard manager started with {} shards", self.shards.len());
        
        while let Some(msg) = self.command_rx.recv().await {
            match msg {
                ShardManagerMessage::AddTransaction(tx) => {
                    self.add_transaction(tx).await;
                }
                ShardManagerMessage::GetTransaction(tx_id, response_tx) => {
                    let tx = self.get_transaction(&tx_id).await;
                    if let Err(e) = response_tx.send(tx).await {
                        error!("Failed to send transaction response: {}", e);
                    }
                }
                ShardManagerMessage::AddShard(shard_id) => {
                    self.add_shard(shard_id).await;
                }
                ShardManagerMessage::RemoveShard(shard_id) => {
                    self.remove_shard(shard_id).await;
                }
                ShardManagerMessage::Rebalance => {
                    self.rebalance().await;
                }
                ShardManagerMessage::Shutdown => {
                    info!("Shard manager shutting down");
                    break;
                }
            }
        }
        
        info!("Shard manager stopped");
    }
    
    /// Add a transaction to the appropriate shard
    async fn add_transaction(&self, tx: Transaction) {
        let shard_id = {
            let ring = self.ring.read();
            ring.get_shard_for_transaction(&tx.id)
        };
        
        debug!("Adding transaction {} to shard {}", tx.id, shard_id);
        
        if let Some(shard) = self.shards.get(&shard_id) {
            shard.insert(tx.id, tx);
        } else {
            error!("Shard {} not found", shard_id);
        }
    }
    
    /// Get a transaction from its shard
    async fn get_transaction(&self, tx_id: &TransactionId) -> Option<Transaction> {
        let shard_id = {
            let ring = self.ring.read();
            ring.get_shard_for_transaction(tx_id)
        };
        
        debug!("Looking for transaction {} in shard {}", tx_id, shard_id);
        
        if let Some(shard) = self.shards.get(&shard_id) {
            shard.get(tx_id).map(|tx| tx.clone())
        } else {
            error!("Shard {} not found", shard_id);
            None
        }
    }
    
    /// Add a new shard
    async fn add_shard(&self, shard_id: ShardId) {
        info!("Adding new shard {}", shard_id);
        
        {
            let mut ring = self.ring.write();
            ring.add_shard(shard_id, 100);
        }
        
        if !self.shards.contains_key(&shard_id) {
            self.shards.insert(shard_id, DashMap::new());
        }
    }
    
    /// Remove a shard
    async fn remove_shard(&self, shard_id: ShardId) {
        info!("Removing shard {}", shard_id);
        
        // First, get all transactions from the shard
        let transactions = if let Some(shard) = self.shards.get(&shard_id) {
            shard.iter().map(|entry| entry.value().clone()).collect::<Vec<_>>()
        } else {
            vec![]
        };
        
        // Remove the shard from the ring
        {
            let mut ring = self.ring.write();
            ring.remove_shard(shard_id);
        }
        
        // Remove the shard from storage
        self.shards.remove(&shard_id);
        
        // Redistribute transactions
        for tx in transactions {
            self.add_transaction(tx).await;
        }
    }
    
    /// Rebalance shards
    async fn rebalance(&self) {
        info!("Rebalancing shards");
        
        // Collect shard sizes
        let mut shard_sizes = Vec::new();
        for entry in self.shards.iter() {
            shard_sizes.push((entry.key().clone(), entry.value().len()));
        }
        
        // Sort by size (descending)
        shard_sizes.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Update weights in the ring based on shard sizes
        {
            let mut ring = self.ring.write();
            for (shard_id, size) in &shard_sizes {
                // Calculate weight based on size (inverse relationship)
                // Smaller shards get higher weights to attract more transactions
                let weight = if *size > 0 {
                    10000 / size
                } else {
                    1000 // Default weight for empty shards
                };
                
                ring.update_shard_weight(*shard_id, weight as u32);
            }
        }
        
        info!("Shard rebalancing completed");
    }
}

/// Create a new shard manager and return the handle
pub async fn start_shard_manager(shard_count: u32) -> mpsc::Sender<ShardManagerMessage> {
    let mut manager = ShardManager::new(shard_count);
    let sender = manager.get_sender();
    
    tokio::spawn(async move {
        manager.run().await;
    });
    
    sender
}