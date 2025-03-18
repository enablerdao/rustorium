use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

/// Unique identifier for transactions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TransactionId([u8; 32]);

impl TransactionId {
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
    
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Display for TransactionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

/// Transaction status in the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Rejected,
    Failed,
}

/// Transaction data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Unique transaction ID
    pub id: TransactionId,
    /// Transaction sender address
    pub sender: Address,
    /// Transaction recipient address
    pub recipient: Address,
    /// Transaction amount
    pub amount: u64,
    /// Transaction fee
    pub fee: u64,
    /// Transaction nonce
    pub nonce: u64,
    /// Transaction timestamp
    pub timestamp: u64,
    /// Transaction data payload
    pub data: Vec<u8>,
    /// Transaction signature
    pub signature: Option<Signature>,
    /// Virtual machine type for execution
    pub vm_type: VmType,
}

impl Transaction {
    pub fn new(
        sender: Address,
        recipient: Address,
        amount: u64,
        fee: u64,
        nonce: u64,
        data: Vec<u8>,
        vm_type: VmType,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        
        // Generate transaction ID by hashing the transaction data
        let mut hasher = blake3::Hasher::new();
        hasher.update(&sender.0);
        hasher.update(&recipient.0);
        hasher.update(&amount.to_be_bytes());
        hasher.update(&fee.to_be_bytes());
        hasher.update(&nonce.to_be_bytes());
        hasher.update(&timestamp.to_be_bytes());
        hasher.update(&data);
        hasher.update(&[vm_type as u8]);
        
        let id = TransactionId(hasher.finalize().into());
        
        Self {
            id,
            sender,
            recipient,
            amount,
            fee,
            nonce,
            timestamp,
            data,
            signature: None,
            vm_type,
        }
    }
    
    pub fn sign(&mut self, private_key: &PrivateKey) -> Result<(), SignatureError> {
        // TODO: Implement actual signature logic
        self.signature = Some(Signature([0; 64]));
        Ok(())
    }
    
    pub fn verify_signature(&self) -> Result<bool, SignatureError> {
        // TODO: Implement actual signature verification
        Ok(true)
    }
}

/// Address type for accounts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address(pub [u8; 20]);

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

/// Signature type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signature(pub [u8; 64]);

/// Private key type
#[derive(Debug, Clone)]
pub struct PrivateKey(pub [u8; 32]);

/// Error type for signature operations
#[derive(Debug, thiserror::Error)]
pub enum SignatureError {
    #[error("Invalid signature format")]
    InvalidFormat,
    #[error("Signature verification failed")]
    VerificationFailed,
}

/// Virtual machine types supported by the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VmType {
    Evm,
    MoveVm,
    SolanaVm,
    Wasm,
}

/// Block structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Block header
    pub header: BlockHeader,
    /// Transactions included in this block
    pub transactions: Vec<Transaction>,
}

/// Block header structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    /// Block height
    pub height: u64,
    /// Previous block hash
    pub prev_hash: [u8; 32],
    /// Merkle root of transactions
    pub merkle_root: [u8; 32],
    /// Block timestamp
    pub timestamp: u64,
    /// Block validator/producer
    pub validator: Address,
    /// Block signature
    pub signature: Option<Signature>,
}

/// Shard identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ShardId(pub u32);

impl fmt::Display for ShardId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Shard-{}", self.0)
    }
}

/// Node identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub String);

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Account state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Account address
    pub address: Address,
    /// Account balance
    pub balance: u64,
    /// Account nonce
    pub nonce: u64,
    /// Account code (for smart contracts)
    pub code: Vec<u8>,
    /// Account storage (for smart contracts)
    pub storage: Vec<(Vec<u8>, Vec<u8>)>,
}

/// Peer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    /// Peer ID
    pub id: NodeId,
    /// Peer address
    pub address: String,
    /// Peer port
    pub port: u16,
    /// Peer last seen timestamp
    pub last_seen: u64,
}