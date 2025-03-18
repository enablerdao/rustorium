use crate::common::types::{ShardId, TransactionId};
use consistent_hash_ring::{HashRing, Node};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

/// Wrapper for ShardId to implement Node trait
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShardNode {
    pub id: ShardId,
    pub weight: u32,
}

impl Hash for ShardNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.0.hash(state);
    }
}

impl Node for ShardNode {
    fn name(&self) -> String {
        format!("shard-{}", self.id.0)
    }

    fn weight(&self) -> u32 {
        self.weight
    }
}

/// Consistent hash ring for shard assignment
pub struct ShardRing {
    ring: HashRing<ShardNode>,
}

impl ShardRing {
    pub fn new(shard_count: u32, weight_per_shard: u32) -> Self {
        let mut ring = HashRing::new();
        
        for i in 0..shard_count {
            let node = ShardNode {
                id: ShardId(i),
                weight: weight_per_shard,
            };
            ring.add(node);
        }
        
        Self { ring }
    }
    
    /// Get the shard ID for a transaction
    pub fn get_shard_for_transaction(&self, tx_id: &TransactionId) -> ShardId {
        let key = hex::encode(tx_id.as_bytes());
        let node = self.ring.get(&key).expect("Ring should not be empty");
        node.id
    }
    
    /// Add a new shard to the ring
    pub fn add_shard(&mut self, shard_id: ShardId, weight: u32) {
        let node = ShardNode {
            id: shard_id,
            weight,
        };
        self.ring.add(node);
    }
    
    /// Remove a shard from the ring
    pub fn remove_shard(&mut self, shard_id: ShardId) {
        self.ring.remove_by(|node| node.id == shard_id);
    }
    
    /// Update the weight of a shard
    pub fn update_shard_weight(&mut self, shard_id: ShardId, new_weight: u32) {
        self.ring.remove_by(|node| node.id == shard_id);
        let node = ShardNode {
            id: shard_id,
            weight: new_weight,
        };
        self.ring.add(node);
    }
    
    /// Get all shards in the ring
    pub fn get_all_shards(&self) -> Vec<ShardId> {
        self.ring.nodes().iter().map(|node| node.id).collect()
    }
    
    /// Get the number of shards in the ring
    pub fn shard_count(&self) -> usize {
        self.ring.nodes().len()
    }
}

/// Thread-safe wrapper for ShardRing
pub type SharedShardRing = Arc<parking_lot::RwLock<ShardRing>>;

/// Create a new shared shard ring
pub fn new_shared_ring(shard_count: u32, weight_per_shard: u32) -> SharedShardRing {
    Arc::new(parking_lot::RwLock::new(ShardRing::new(shard_count, weight_per_shard)))
}