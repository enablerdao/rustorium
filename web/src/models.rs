use serde::{Deserialize, Serialize};

/// API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

/// Node status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStatus {
    pub node_id: String,
    pub version: String,
    pub network: String,
    pub latest_block_height: u64,
    pub connected_peers: usize,
    pub uptime_seconds: u64,
    pub pending_transactions: usize,
}

/// Block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub height: u64,
    pub hash: String,
    pub prev_hash: String,
    pub timestamp: u64,
    pub validator: String,
    pub transactions: Vec<String>,
    pub merkle_root: String,
}

/// Transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub fee: u64,
    pub nonce: u64,
    pub timestamp: u64,
    pub data: String,
    pub status: String,
}

/// Account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub address: String,
    pub balance: u64,
    pub nonce: u64,
    pub is_contract: bool,
}

/// Transaction creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTransactionRequest {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub fee: u64,
    pub nonce: Option<u64>,
    pub data: Option<String>,
}

/// Application state
#[derive(Debug, Clone, PartialEq)]
pub enum AppPage {
    Dashboard,
    Blocks,
    Transactions,
    Accounts,
    SendTransaction,
    Settings,
}