use crate::common::errors::LedgerError;
use crate::common::types::{Account, Address, Block, BlockHeader, Transaction};
use crate::common::utils;
use crate::storage::db::Database;
use std::path::Path;
use tracing::info;

/// Initialize storage at the given path
pub fn initialize_storage(path: &str) -> Result<(), LedgerError> {
    let path = Path::new(path);
    
    // Create directory if it doesn't exist
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    
    // Create database directory
    let db_path = path.join("db");
    if !db_path.exists() {
        std::fs::create_dir_all(&db_path)?;
    }
    
    // Open database
    let db = Database::open(&db_path)?;
    
    // Create genesis block if it doesn't exist
    if db.get_latest_block_height()? == 0 {
        info!("Creating genesis block");
        create_genesis_block(&db)?;
    }
    
    info!("Storage initialized at {}", path.display());
    
    Ok(())
}

/// Create the genesis block
fn create_genesis_block(db: &Database) -> Result<(), LedgerError> {
    // Create genesis block
    let genesis_header = BlockHeader {
        height: 0,
        prev_hash: [0; 32],
        merkle_root: [0; 32],
        timestamp: utils::current_time_sec(),
        validator: Address([0; 20]),
        signature: None,
    };
    
    let genesis_block = Block {
        header: genesis_header,
        transactions: vec![],
    };
    
    // Store genesis block
    db.put_block(&genesis_block)?;
    db.set_latest_block_height(0)?;
    
    // Create system account
    let system_address = Address([0; 20]);
    let system_account = Account {
        address: system_address,
        balance: 1_000_000_000_000, // Initial supply
        nonce: 0,
        code: vec![],
        storage: vec![],
    };
    
    // Store system account
    db.put_account(&system_account)?;
    
    // Store metadata
    db.put_metadata("genesis_timestamp", &genesis_block.header.timestamp.to_be_bytes())?;
    db.put_metadata("network_id", b"rustorium-mainnet")?;
    
    info!("Genesis block created at height 0");
    
    Ok(())
}