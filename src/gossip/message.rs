use crate::common::types::{Block, NodeId, Transaction};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Message types for the gossip protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GossipMessage {
    /// Transaction message
    Transaction(Transaction),
    /// Block message
    Block(Block),
    /// Query message
    Query(QueryMessage),
    /// Response message
    Response(ResponseMessage),
    /// Ping message
    Ping(PingMessage),
    /// Pong message
    Pong(PongMessage),
}

/// Query message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryMessage {
    /// Query ID
    pub id: u64,
    /// Query type
    pub query_type: QueryType,
    /// Sender node ID
    pub sender: NodeId,
    /// Timestamp
    pub timestamp: u64,
}

/// Query types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryType {
    /// Query for a transaction
    GetTransaction(String),
    /// Query for a block
    GetBlock(u64),
    /// Query for the latest block
    GetLatestBlock,
    /// Query for peers
    GetPeers,
}

/// Response message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMessage {
    /// Query ID
    pub id: u64,
    /// Response type
    pub response_type: ResponseType,
    /// Sender node ID
    pub sender: NodeId,
    /// Timestamp
    pub timestamp: u64,
}

/// Response types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseType {
    /// Transaction response
    Transaction(Option<Transaction>),
    /// Block response
    Block(Option<Block>),
    /// Latest block response
    LatestBlock(Option<Block>),
    /// Peers response
    Peers(Vec<String>),
    /// Error response
    Error(String),
}

/// Ping message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingMessage {
    /// Ping ID
    pub id: u64,
    /// Sender node ID
    pub sender: NodeId,
    /// Timestamp
    pub timestamp: u64,
}

/// Pong message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PongMessage {
    /// Ping ID
    pub id: u64,
    /// Sender node ID
    pub sender: NodeId,
    /// Original timestamp
    pub original_timestamp: u64,
    /// Response timestamp
    pub timestamp: u64,
}

impl GossipMessage {
    /// Create a new transaction message
    pub fn new_transaction(tx: Transaction) -> Self {
        Self::Transaction(tx)
    }
    
    /// Create a new block message
    pub fn new_block(block: Block) -> Self {
        Self::Block(block)
    }
    
    /// Create a new query message
    pub fn new_query(query_type: QueryType, sender: NodeId) -> Self {
        let id = rand::random::<u64>();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;
        
        Self::Query(QueryMessage {
            id,
            query_type,
            sender,
            timestamp,
        })
    }
    
    /// Create a new response message
    pub fn new_response(id: u64, response_type: ResponseType, sender: NodeId) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;
        
        Self::Response(ResponseMessage {
            id,
            response_type,
            sender,
            timestamp,
        })
    }
    
    /// Create a new ping message
    pub fn new_ping(sender: NodeId) -> Self {
        let id = rand::random::<u64>();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;
        
        Self::Ping(PingMessage {
            id,
            sender,
            timestamp,
        })
    }
    
    /// Create a new pong message
    pub fn new_pong(ping: &PingMessage, sender: NodeId) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;
        
        Self::Pong(PongMessage {
            id: ping.id,
            sender,
            original_timestamp: ping.timestamp,
            timestamp,
        })
    }
}