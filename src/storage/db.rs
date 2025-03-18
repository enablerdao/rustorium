use crate::common::errors::LedgerError;
use crate::common::types::{Account, Address, Block, Transaction, TransactionId};
use bincode::{deserialize, serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, RwLock};
use tracing::{debug, error, info};

// RocksDBの代わりにメモリ内ストレージを使用
struct MemoryDB {
    blocks: RwLock<HashMap<Vec<u8>, Vec<u8>>>,
    transactions: RwLock<HashMap<Vec<u8>, Vec<u8>>>,
    accounts: RwLock<HashMap<Vec<u8>, Vec<u8>>>,
    state: RwLock<HashMap<Vec<u8>, Vec<u8>>>,
    metadata: RwLock<HashMap<Vec<u8>, Vec<u8>>>,
}

/// Column family names
const CF_BLOCKS: &str = "blocks";
const CF_TRANSACTIONS: &str = "transactions";
const CF_ACCOUNTS: &str = "accounts";
const CF_STATE: &str = "state";
const CF_METADATA: &str = "metadata";

/// Database wrapper for in-memory storage
pub struct Database {
    db: Arc<MemoryDB>,
}

impl MemoryDB {
    fn new() -> Self {
        Self {
            blocks: RwLock::new(HashMap::new()),
            transactions: RwLock::new(HashMap::new()),
            accounts: RwLock::new(HashMap::new()),
            state: RwLock::new(HashMap::new()),
            metadata: RwLock::new(HashMap::new()),
        }
    }
}

impl Database {
    /// Open a database at the given path
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, LedgerError> {
        let path = path.as_ref();
        
        // Create directory if it doesn't exist
        if !path.exists() {
            std::fs::create_dir_all(path)?;
        }
        
        // Create in-memory database
        let db = MemoryDB::new();
        
        info!("In-memory database initialized (path: {} is ignored)", path.display());
        
        Ok(Self { db: Arc::new(db) })
    }
    
    /// Store a block
    pub fn put_block(&self, block: &Block) -> Result<(), LedgerError> {
        let key = block.header.height.to_be_bytes().to_vec();
        let value = serialize(block)?;
        
        let mut blocks = self.db.blocks.write().unwrap();
        blocks.insert(key, value);
        
        debug!("Stored block at height {}", block.header.height);
        
        Ok(())
    }
    
    /// Get a block by height
    pub fn get_block(&self, height: u64) -> Result<Option<Block>, LedgerError> {
        let key = height.to_be_bytes().to_vec();
        let blocks = self.db.blocks.read().unwrap();
        
        match blocks.get(&key) {
            Some(value) => {
                let block = deserialize(value)?;
                Ok(Some(block))
            }
            None => Ok(None),
        }
    }
    
    /// Store a transaction
    pub fn put_transaction(&self, tx: &Transaction) -> Result<(), LedgerError> {
        let key = tx.id.as_bytes().to_vec();
        let value = serialize(tx)?;
        
        let mut transactions = self.db.transactions.write().unwrap();
        transactions.insert(key, value);
        
        debug!("Stored transaction {}", tx.id);
        
        Ok(())
    }
    
    /// Get a transaction by ID
    pub fn get_transaction(&self, tx_id: &TransactionId) -> Result<Option<Transaction>, LedgerError> {
        let key = tx_id.as_bytes().to_vec();
        let transactions = self.db.transactions.read().unwrap();
        
        match transactions.get(&key) {
            Some(value) => {
                let tx = deserialize(value)?;
                Ok(Some(tx))
            }
            None => Ok(None),
        }
    }
    
    /// Store an account
    pub fn put_account(&self, account: &Account) -> Result<(), LedgerError> {
        let key = account.address.0.to_vec();
        let value = serialize(account)?;
        
        let mut accounts = self.db.accounts.write().unwrap();
        accounts.insert(key, value);
        
        debug!("Stored account {}", account.address);
        
        Ok(())
    }
    
    /// Get an account by address
    pub fn get_account(&self, address: &Address) -> Result<Option<Account>, LedgerError> {
        let key = address.0.to_vec();
        let accounts = self.db.accounts.read().unwrap();
        
        match accounts.get(&key) {
            Some(value) => {
                let account = deserialize(value)?;
                Ok(Some(account))
            }
            None => Ok(None),
        }
    }
    
    /// Store a key-value pair in the state
    pub fn put_state(&self, key: &[u8], value: &[u8]) -> Result<(), LedgerError> {
        let mut state = self.db.state.write().unwrap();
        state.insert(key.to_vec(), value.to_vec());
        
        Ok(())
    }
    
    /// Get a value from the state by key
    pub fn get_state(&self, key: &[u8]) -> Result<Option<Vec<u8>>, LedgerError> {
        let state = self.db.state.read().unwrap();
        Ok(state.get(key).cloned())
    }
    
    /// Store metadata
    pub fn put_metadata(&self, key: &str, value: &[u8]) -> Result<(), LedgerError> {
        let mut metadata = self.db.metadata.write().unwrap();
        metadata.insert(key.as_bytes().to_vec(), value.to_vec());
        
        Ok(())
    }
    
    /// Get metadata by key
    pub fn get_metadata(&self, key: &str) -> Result<Option<Vec<u8>>, LedgerError> {
        let metadata = self.db.metadata.read().unwrap();
        Ok(metadata.get(key.as_bytes()).cloned())
    }
    
    /// Get the latest block height
    pub fn get_latest_block_height(&self) -> Result<u64, LedgerError> {
        match self.get_metadata("latest_block_height")? {
            Some(value) => {
                if value.len() == 8 {
                    let mut bytes = [0u8; 8];
                    bytes.copy_from_slice(&value);
                    Ok(u64::from_be_bytes(bytes))
                } else {
                    Err(LedgerError::Database("Invalid latest block height format".to_string()))
                }
            }
            None => Ok(0), // No blocks yet
        }
    }
    
    /// Set the latest block height
    pub fn set_latest_block_height(&self, height: u64) -> Result<(), LedgerError> {
        self.put_metadata("latest_block_height", &height.to_be_bytes())
    }
    
    /// Batch write operations (simplified for in-memory DB)
    pub fn batch_write(&self, operations: Vec<(String, Vec<u8>, Vec<u8>)>) -> Result<(), LedgerError> {
        for (cf_name, key, value) in operations {
            match cf_name.as_str() {
                CF_BLOCKS => {
                    let mut blocks = self.db.blocks.write().unwrap();
                    blocks.insert(key, value);
                }
                CF_TRANSACTIONS => {
                    let mut transactions = self.db.transactions.write().unwrap();
                    transactions.insert(key, value);
                }
                CF_ACCOUNTS => {
                    let mut accounts = self.db.accounts.write().unwrap();
                    accounts.insert(key, value);
                }
                CF_STATE => {
                    let mut state = self.db.state.write().unwrap();
                    state.insert(key, value);
                }
                CF_METADATA => {
                    let mut metadata = self.db.metadata.write().unwrap();
                    metadata.insert(key, value);
                }
                _ => {
                    return Err(LedgerError::Database(format!("Column family {} not found", cf_name)));
                }
            }
        }
        
        Ok(())
    }
}