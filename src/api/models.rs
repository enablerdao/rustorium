use crate::common::types::{Address, Transaction, TransactionId, VmType};
use serde::{Deserialize, Serialize};

/// API response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }
    
    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

/// Transaction creation request
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransactionRequest {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub fee: u64,
    pub nonce: Option<u64>,
    pub data: Option<String>,
    pub vm_type: Option<String>,
}

/// Transaction response
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub id: String,
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub fee: u64,
    pub nonce: u64,
    pub timestamp: u64,
    pub data: String,
    pub vm_type: String,
    pub status: String,
}

impl From<Transaction> for TransactionResponse {
    fn from(tx: Transaction) -> Self {
        Self {
            id: tx.id.to_string(),
            sender: tx.sender.to_string(),
            recipient: tx.recipient.to_string(),
            amount: tx.amount,
            fee: tx.fee,
            nonce: tx.nonce,
            timestamp: tx.timestamp,
            data: hex::encode(&tx.data),
            vm_type: format!("{:?}", tx.vm_type),
            status: "Pending".to_string(), // Default status
        }
    }
}

/// Block response
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockResponse {
    pub height: u64,
    pub hash: String,
    pub prev_hash: String,
    pub timestamp: u64,
    pub validator: String,
    pub transactions: Vec<String>,
    pub merkle_root: String,
}

/// Account response
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountResponse {
    pub address: String,
    pub balance: u64,
    pub nonce: u64,
    pub is_contract: bool,
}

/// Node status response
#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStatusResponse {
    pub node_id: String,
    pub version: String,
    pub network: String,
    pub latest_block_height: u64,
    pub connected_peers: usize,
    pub uptime_seconds: u64,
    pub pending_transactions: usize,
}