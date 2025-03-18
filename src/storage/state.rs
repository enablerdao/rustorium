use crate::common::errors::LedgerError;
use crate::common::types::{Account, Address, Block, Transaction, TransactionId};
use crate::storage::cache::StorageCache;
use crate::storage::db::Database;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info};

/// State manager for the ledger
pub struct StateManager {
    /// Database
    db: Arc<Database>,
    /// Cache
    cache: Arc<StorageCache>,
    /// Latest block height
    latest_block_height: RwLock<u64>,
}

impl StateManager {
    /// Create a new state manager
    pub fn new(db: Arc<Database>, cache: Arc<StorageCache>) -> Result<Self, LedgerError> {
        let latest_block_height = db.get_latest_block_height()?;
        
        Ok(Self {
            db,
            cache,
            latest_block_height: RwLock::new(latest_block_height),
        })
    }
    
    /// Get the latest block height
    pub async fn get_latest_block_height(&self) -> u64 {
        *self.latest_block_height.read().await
    }
    
    /// Get a block by height
    pub async fn get_block(&self, height: u64) -> Result<Option<Block>, LedgerError> {
        // Try cache first
        if let Some(block) = self.cache.get_block(height).await {
            return Ok(Some(block));
        }
        
        // Try database
        match self.db.get_block(height)? {
            Some(block) => {
                // Cache the block
                self.cache.put_block(block.clone()).await;
                Ok(Some(block))
            }
            None => Ok(None),
        }
    }
    
    /// Get a transaction by ID
    pub async fn get_transaction(&self, tx_id: &TransactionId) -> Result<Option<Transaction>, LedgerError> {
        // Try cache first
        if let Some(tx) = self.cache.get_transaction(tx_id).await {
            return Ok(Some(tx));
        }
        
        // Try database
        match self.db.get_transaction(tx_id)? {
            Some(tx) => {
                // Cache the transaction
                self.cache.put_transaction(tx.clone()).await;
                Ok(Some(tx))
            }
            None => Ok(None),
        }
    }
    
    /// Get an account by address
    pub async fn get_account(&self, address: &Address) -> Result<Option<Account>, LedgerError> {
        // Try cache first
        if let Some(account) = self.cache.get_account(address).await {
            return Ok(Some(account));
        }
        
        // Try database
        match self.db.get_account(address)? {
            Some(account) => {
                // Cache the account
                self.cache.put_account(account.clone()).await;
                Ok(Some(account))
            }
            None => Ok(None),
        }
    }
    
    /// Apply a block to the state
    pub async fn apply_block(&self, block: &Block) -> Result<(), LedgerError> {
        let height = block.header.height;
        let current_height = self.get_latest_block_height().await;
        
        if height != current_height + 1 {
            return Err(LedgerError::InvalidState(format!(
                "Cannot apply block at height {}. Current height is {}",
                height, current_height
            )));
        }
        
        // Apply all transactions in the block
        for tx in &block.transactions {
            self.apply_transaction(tx).await?;
        }
        
        // Store the block
        self.db.put_block(block)?;
        
        // Update latest block height
        self.db.set_latest_block_height(height)?;
        *self.latest_block_height.write().await = height;
        
        // Cache the block
        self.cache.put_block(block.clone()).await;
        
        info!("Applied block at height {}", height);
        
        Ok(())
    }
    
    /// Apply a transaction to the state
    pub async fn apply_transaction(&self, tx: &Transaction) -> Result<(), LedgerError> {
        // Get sender account
        let mut sender = match self.get_account(&tx.sender).await? {
            Some(account) => account,
            None => {
                // Create new account if it doesn't exist
                Account {
                    address: tx.sender,
                    balance: 0,
                    nonce: 0,
                    code: vec![],
                    storage: vec![],
                }
            }
        };
        
        // Check nonce
        if tx.nonce != sender.nonce {
            return Err(LedgerError::Transaction(
                crate::common::errors::TransactionError::InvalidNonce,
            ));
        }
        
        // Check balance
        if sender.balance < tx.amount + tx.fee {
            return Err(LedgerError::Transaction(
                crate::common::errors::TransactionError::InsufficientFunds,
            ));
        }
        
        // Get recipient account
        let mut recipient = match self.get_account(&tx.recipient).await? {
            Some(account) => account,
            None => {
                // Create new account if it doesn't exist
                Account {
                    address: tx.recipient,
                    balance: 0,
                    nonce: 0,
                    code: vec![],
                    storage: vec![],
                }
            }
        };
        
        // Update balances
        sender.balance -= tx.amount + tx.fee;
        recipient.balance += tx.amount;
        
        // Update sender nonce
        sender.nonce += 1;
        
        // Store updated accounts
        self.db.put_account(&sender)?;
        self.db.put_account(&recipient)?;
        
        // Store transaction
        self.db.put_transaction(tx)?;
        
        // Update cache
        self.cache.put_account(sender).await;
        self.cache.put_account(recipient).await;
        self.cache.put_transaction(tx.clone()).await;
        
        debug!("Applied transaction {}", tx.id);
        
        Ok(())
    }
    
    /// Revert a block
    pub async fn revert_block(&self, height: u64) -> Result<(), LedgerError> {
        let current_height = self.get_latest_block_height().await;
        
        if height != current_height {
            return Err(LedgerError::InvalidState(format!(
                "Cannot revert block at height {}. Current height is {}",
                height, current_height
            )));
        }
        
        // Get the block to revert
        let block = match self.get_block(height).await? {
            Some(block) => block,
            None => {
                return Err(LedgerError::NotFound(format!("Block at height {} not found", height)));
            }
        };
        
        // Revert all transactions in reverse order
        for tx in block.transactions.iter().rev() {
            self.revert_transaction(tx).await?;
        }
        
        // Update latest block height
        let new_height = height - 1;
        self.db.set_latest_block_height(new_height)?;
        *self.latest_block_height.write().await = new_height;
        
        info!("Reverted block at height {}", height);
        
        Ok(())
    }
    
    /// Revert a transaction
    pub async fn revert_transaction(&self, tx: &Transaction) -> Result<(), LedgerError> {
        // Get sender account
        let mut sender = match self.get_account(&tx.sender).await? {
            Some(account) => account,
            None => {
                return Err(LedgerError::NotFound(format!("Sender account {} not found", tx.sender)));
            }
        };
        
        // Get recipient account
        let mut recipient = match self.get_account(&tx.recipient).await? {
            Some(account) => account,
            None => {
                return Err(LedgerError::NotFound(format!("Recipient account {} not found", tx.recipient)));
            }
        };
        
        // Update balances
        sender.balance += tx.amount + tx.fee;
        recipient.balance -= tx.amount;
        
        // Update sender nonce
        sender.nonce -= 1;
        
        // Store updated accounts
        self.db.put_account(&sender)?;
        self.db.put_account(&recipient)?;
        
        // Update cache
        self.cache.put_account(sender).await;
        self.cache.put_account(recipient).await;
        
        debug!("Reverted transaction {}", tx.id);
        
        Ok(())
    }
}