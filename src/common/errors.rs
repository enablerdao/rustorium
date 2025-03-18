use thiserror::Error;

#[derive(Error, Debug)]
pub enum LedgerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] bincode::Error),
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Transaction error: {0}")]
    Transaction(#[from] TransactionError),
    
    #[error("Sharding error: {0}")]
    Sharding(#[from] ShardingError),
    
    #[error("Consensus error: {0}")]
    Consensus(#[from] ConsensusError),
    
    #[error("VM execution error: {0}")]
    VmExecution(#[from] VmError),
    
    #[error("Network error: {0}")]
    Network(#[from] NetworkError),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Already exists: {0}")]
    AlreadyExists(String),
    
    #[error("Invalid state: {0}")]
    InvalidState(String),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    #[error("Rate limited: {0}")]
    RateLimited(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Error, Debug)]
pub enum TransactionError {
    #[error("Invalid transaction format: {0}")]
    InvalidFormat(String),
    
    #[error("Invalid signature")]
    InvalidSignature,
    
    #[error("Insufficient funds")]
    InsufficientFunds,
    
    #[error("Invalid nonce")]
    InvalidNonce,
    
    #[error("Transaction expired")]
    Expired,
    
    #[error("Transaction already processed")]
    AlreadyProcessed,
    
    #[error("Transaction execution failed: {0}")]
    ExecutionFailed(String),
}

#[derive(Error, Debug)]
pub enum ShardingError {
    #[error("Invalid shard ID: {0}")]
    InvalidShardId(String),
    
    #[error("Shard not found: {0}")]
    ShardNotFound(String),
    
    #[error("Cross-shard transaction failed: {0}")]
    CrossShardFailed(String),
    
    #[error("Shard rebalancing failed: {0}")]
    RebalancingFailed(String),
}

#[derive(Error, Debug)]
pub enum ConsensusError {
    #[error("Invalid block: {0}")]
    InvalidBlock(String),
    
    #[error("Block verification failed: {0}")]
    VerificationFailed(String),
    
    #[error("Consensus timeout")]
    Timeout,
    
    #[error("Fork detected")]
    ForkDetected,
    
    #[error("Invalid validator: {0}")]
    InvalidValidator(String),
}

#[derive(Error, Debug)]
pub enum VmError {
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Out of gas")]
    OutOfGas,
    
    #[error("Invalid opcode: {0}")]
    InvalidOpcode(String),
    
    #[error("Stack overflow")]
    StackOverflow,
    
    #[error("Stack underflow")]
    StackUnderflow,
    
    #[error("Memory access violation")]
    MemoryAccessViolation,
    
    #[error("Invalid jump destination")]
    InvalidJumpDestination,
    
    #[error("Invalid VM type: {0}")]
    InvalidVmType(String),
}

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    
    #[error("Peer not found: {0}")]
    PeerNotFound(String),
    
    #[error("Message too large")]
    MessageTooLarge,
    
    #[error("Invalid message format")]
    InvalidMessageFormat,
    
    #[error("Timeout")]
    Timeout,
    
    #[error("Too many connections")]
    TooManyConnections,
}