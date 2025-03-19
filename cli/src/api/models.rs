use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Network status
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkStatus {
    /// Chain ID
    pub chain_id: u64,
    /// Current block number
    pub current_block: u64,
    /// Sync status
    pub sync_status: String,
    /// Sync percentage
    pub sync_percentage: f64,
    /// Number of connected peers
    pub peers: u32,
    /// Transactions per second
    pub tps: f64,
    /// Gas price in Gwei
    pub gas_price: f64,
}

/// Node stats
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeStats {
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory used
    pub memory_used: String,
    /// Total memory
    pub memory_total: String,
    /// Disk space used
    pub disk_used: String,
    /// Node uptime
    pub uptime: String,
    /// Time since last block
    pub last_block_time: String,
}

/// Block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Block hash
    pub hash: String,
    /// Block number
    pub number: u64,
    /// Parent hash
    pub parent_hash: String,
    /// Timestamp
    pub timestamp: String,
    /// Transactions
    pub transactions: Vec<String>,
    /// Miner
    pub miner: String,
    /// Gas used
    pub gas_used: u64,
    /// Gas limit
    pub gas_limit: u64,
    /// Size in bytes
    pub size: u64,
}

/// Transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Transaction ID
    pub id: String,
    /// From address
    pub from: String,
    /// To address
    pub to: String,
    /// Value
    pub value: f64,
    /// Gas price
    pub gas_price: u64,
    /// Gas limit
    pub gas_limit: u64,
    /// Gas used
    pub gas_used: u64,
    /// Nonce
    pub nonce: u64,
    /// Timestamp
    pub timestamp: String,
    /// Status
    pub status: String,
    /// Block ID
    pub block_id: Option<String>,
    /// Data
    pub data: Option<String>,
}

/// Account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Account address
    pub address: String,
    /// Balance
    pub balance: f64,
    /// Nonce
    pub nonce: u64,
    /// Account type
    pub account_type: String,
    /// Creation timestamp
    pub created_at: String,
    /// Last activity timestamp
    pub last_activity: Option<String>,
}

/// Contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    /// Contract address
    pub address: String,
    /// Creator address
    pub creator: String,
    /// Bytecode
    pub bytecode: String,
    /// ABI
    pub abi: Option<String>,
    /// Creation transaction
    pub creation_transaction: String,
    /// Creation block
    pub creation_block: Option<u64>,
    /// Contract state
    pub state: HashMap<String, String>,
    /// Creation timestamp
    pub created_at: String,
    /// Last activity timestamp
    pub last_activity: String,
}

/// Token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    /// Token address
    pub address: String,
    /// Token name
    pub name: String,
    /// Token symbol
    pub symbol: String,
    /// Token type
    pub token_type: String,
    /// Decimals (for fungible tokens)
    pub decimals: Option<u8>,
    /// Total supply (for fungible tokens)
    pub total_supply: Option<u64>,
    /// Creator address
    pub creator: String,
    /// Creation timestamp
    pub created_at: String,
}