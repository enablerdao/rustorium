use crate::common::errors::LedgerError;
use crate::common::types::{Account, Address, Block, Transaction, TransactionId};
use dashmap::DashMap;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::hash::Hash;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tracing::{debug, info, warn};

/// Cache entry with expiration
struct CacheEntry<T> {
    value: T,
    expires_at: Instant,
}

/// Generic LRU cache with expiration
pub struct Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Cache storage
    cache: DashMap<K, CacheEntry<V>>,
    /// Priority queue for eviction
    eviction_queue: Mutex<PriorityQueue<K, Reverse<Instant>>>,
    /// Maximum cache size
    max_size: usize,
    /// Default TTL for entries
    ttl: Duration,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Create a new cache
    pub fn new(max_size: usize, ttl_seconds: u64) -> Self {
        Self {
            cache: DashMap::new(),
            eviction_queue: Mutex::new(PriorityQueue::new()),
            max_size,
            ttl: Duration::from_secs(ttl_seconds),
        }
    }
    
    /// Get a value from the cache
    pub async fn get(&self, key: &K) -> Option<V> {
        // Remove expired entries
        self.cleanup_expired().await;
        
        // Get from cache
        self.cache.get(key).map(|entry| {
            if entry.expires_at <= Instant::now() {
                None
            } else {
                Some(entry.value.clone())
            }
        }).flatten()
    }
    
    /// Put a value in the cache
    pub async fn put(&self, key: K, value: V) {
        // Remove expired entries
        self.cleanup_expired().await;
        
        // Check if we need to evict
        if self.cache.len() >= self.max_size {
            self.evict_oldest().await;
        }
        
        // Add to cache
        let expires_at = Instant::now() + self.ttl;
        self.cache.insert(key.clone(), CacheEntry { value, expires_at });
        
        // Add to eviction queue
        let mut queue = self.eviction_queue.lock().await;
        queue.push(key, Reverse(expires_at));
    }
    
    /// Remove a value from the cache
    pub async fn remove(&self, key: &K) {
        self.cache.remove(key);
        
        // Also remove from eviction queue on next cleanup
    }
    
    /// Clear the cache
    pub async fn clear(&self) {
        self.cache.clear();
        let mut queue = self.eviction_queue.lock().await;
        queue.clear();
    }
    
    /// Remove expired entries
    async fn cleanup_expired(&self) {
        let now = Instant::now();
        let mut queue = self.eviction_queue.lock().await;
        
        while let Some((key, Reverse(expires_at))) = queue.peek() {
            if *expires_at <= now {
                queue.pop();
                self.cache.remove(key);
            } else {
                break;
            }
        }
    }
    
    /// Evict the oldest entry
    async fn evict_oldest(&self) {
        let mut queue = self.eviction_queue.lock().await;
        
        if let Some((key, _)) = queue.pop() {
            self.cache.remove(&key);
            debug!("Evicted oldest cache entry");
        }
    }
}

/// Storage cache for frequently accessed data
pub struct StorageCache {
    /// Block cache
    blocks: Cache<u64, Block>,
    /// Transaction cache
    transactions: Cache<TransactionId, Transaction>,
    /// Account cache
    accounts: Cache<Address, Account>,
}

impl StorageCache {
    /// Create a new storage cache
    pub fn new(config: &crate::common::config::StorageConfig) -> Self {
        // Calculate cache sizes based on total cache size
        let total_cache_mb = config.cache_size_mb;
        let block_cache_mb = total_cache_mb / 4;
        let tx_cache_mb = total_cache_mb / 4;
        let account_cache_mb = total_cache_mb / 2;
        
        // Estimate entry sizes and calculate max entries
        const BLOCK_SIZE_BYTES: usize = 10 * 1024; // 10KB per block
        const TX_SIZE_BYTES: usize = 1 * 1024; // 1KB per transaction
        const ACCOUNT_SIZE_BYTES: usize = 2 * 1024; // 2KB per account
        
        let max_blocks = (block_cache_mb * 1024 * 1024) / BLOCK_SIZE_BYTES;
        let max_txs = (tx_cache_mb * 1024 * 1024) / TX_SIZE_BYTES;
        let max_accounts = (account_cache_mb * 1024 * 1024) / ACCOUNT_SIZE_BYTES;
        
        info!(
            "Initializing storage cache with {} blocks, {} transactions, {} accounts",
            max_blocks, max_txs, max_accounts
        );
        
        Self {
            blocks: Cache::new(max_blocks, 3600), // 1 hour TTL for blocks
            transactions: Cache::new(max_txs, 1800), // 30 minutes TTL for transactions
            accounts: Cache::new(max_accounts, 300), // 5 minutes TTL for accounts
        }
    }
    
    /// Get a block from the cache
    pub async fn get_block(&self, height: u64) -> Option<Block> {
        self.blocks.get(&height).await
    }
    
    /// Put a block in the cache
    pub async fn put_block(&self, block: Block) {
        self.blocks.put(block.header.height, block).await;
    }
    
    /// Get a transaction from the cache
    pub async fn get_transaction(&self, tx_id: &TransactionId) -> Option<Transaction> {
        self.transactions.get(tx_id).await
    }
    
    /// Put a transaction in the cache
    pub async fn put_transaction(&self, tx: Transaction) {
        self.transactions.put(tx.id, tx).await;
    }
    
    /// Get an account from the cache
    pub async fn get_account(&self, address: &Address) -> Option<Account> {
        self.accounts.get(address).await
    }
    
    /// Put an account in the cache
    pub async fn put_account(&self, account: Account) {
        self.accounts.put(account.address, account).await;
    }
    
    /// Clear all caches
    pub async fn clear_all(&self) {
        self.blocks.clear().await;
        self.transactions.clear().await;
        self.accounts.clear().await;
        
        info!("All caches cleared");
    }
}