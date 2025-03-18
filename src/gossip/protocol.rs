use crate::common::errors::NetworkError;
use crate::common::types::{Block, NodeId, Transaction, TransactionId};
use crate::gossip::message::{GossipMessage, QueryType, ResponseType};
use crate::gossip::peer::PeerManager;
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, oneshot, RwLock};
use tokio::time;
use tracing::{debug, error, info, warn};

/// Avalanche protocol parameters
pub struct AvalancheParams {
    /// Number of peers to query
    pub k: usize,
    /// Confidence threshold
    pub alpha: usize,
    /// Number of rounds
    pub beta: usize,
}

impl Default for AvalancheParams {
    fn default() -> Self {
        Self {
            k: 10,
            alpha: 7, // 70% of k
            beta: 5,
        }
    }
}

/// Avalanche protocol state for a transaction
struct AvalancheState {
    /// Transaction
    transaction: Transaction,
    /// Confidence counter
    confidence: usize,
    /// Number of rounds
    rounds: usize,
    /// Last updated time
    last_updated: Instant,
    /// Is finalized
    finalized: bool,
}

/// Avalanche protocol
pub struct AvalancheProtocol {
    /// Node ID
    node_id: NodeId,
    /// Peer manager
    peer_manager: Arc<PeerManager>,
    /// Protocol parameters
    params: AvalancheParams,
    /// Transaction states
    tx_states: Arc<DashMap<TransactionId, AvalancheState>>,
    /// Pending queries
    pending_queries: Arc<DashMap<u64, oneshot::Sender<ResponseType>>>,
    /// Message sender
    message_tx: mpsc::Sender<(NodeId, GossipMessage)>,
    /// Message receiver
    message_rx: mpsc::Receiver<(NodeId, GossipMessage)>,
    /// Transaction callback
    tx_callback: Option<Arc<dyn Fn(Transaction) -> bool + Send + Sync>>,
    /// Block callback
    block_callback: Option<Arc<dyn Fn(Block) -> bool + Send + Sync>>,
}

impl AvalancheProtocol {
    /// Create a new Avalanche protocol instance
    pub fn new(
        node_id: NodeId,
        peer_manager: Arc<PeerManager>,
        params: AvalancheParams,
    ) -> Self {
        let (message_tx, message_rx) = mpsc::channel(1000);
        
        Self {
            node_id,
            peer_manager,
            params,
            tx_states: Arc::new(DashMap::new()),
            pending_queries: Arc::new(DashMap::new()),
            message_tx,
            message_rx,
            tx_callback: None,
            block_callback: None,
        }
    }
    
    /// Set transaction callback
    pub fn set_tx_callback<F>(&mut self, callback: F)
    where
        F: Fn(Transaction) -> bool + Send + Sync + 'static,
    {
        self.tx_callback = Some(Arc::new(callback));
    }
    
    /// Set block callback
    pub fn set_block_callback<F>(&mut self, callback: F)
    where
        F: Fn(Block) -> bool + Send + Sync + 'static,
    {
        self.block_callback = Some(Arc::new(callback));
    }
    
    /// Get message sender
    pub fn get_message_sender(&self) -> mpsc::Sender<(NodeId, GossipMessage)> {
        self.message_tx.clone()
    }
    
    /// Start the protocol
    pub async fn start(&mut self) {
        info!("Starting Avalanche protocol");
        
        // Start cleanup task
        self.start_cleanup_task().await;
        
        // Process messages
        while let Some((peer_id, message)) = self.message_rx.recv().await {
            self.handle_message(peer_id, message).await;
        }
    }
    
    /// Handle a message
    async fn handle_message(&self, peer_id: NodeId, message: GossipMessage) {
        match message {
            GossipMessage::Transaction(tx) => {
                self.handle_transaction(tx).await;
            }
            GossipMessage::Block(block) => {
                self.handle_block(block).await;
            }
            GossipMessage::Query(query) => {
                self.handle_query(peer_id, query).await;
            }
            GossipMessage::Response(response) => {
                self.handle_response(response).await;
            }
            GossipMessage::Ping(ping) => {
                self.handle_ping(peer_id, ping).await;
            }
            GossipMessage::Pong(pong) => {
                self.handle_pong(pong).await;
            }
        }
    }
    
    /// Handle a transaction message
    async fn handle_transaction(&self, tx: Transaction) {
        let tx_id = tx.id;
        
        // Check if we already have this transaction
        if self.tx_states.contains_key(&tx_id) {
            return;
        }
        
        // Validate transaction
        let valid = match &self.tx_callback {
            Some(callback) => callback(tx.clone()),
            None => true, // No callback, assume valid
        };
        
        if !valid {
            warn!("Received invalid transaction {}", tx_id);
            return;
        }
        
        // Add to state
        let state = AvalancheState {
            transaction: tx.clone(),
            confidence: 0,
            rounds: 0,
            last_updated: Instant::now(),
            finalized: false,
        };
        
        self.tx_states.insert(tx_id, state);
        
        // Start Avalanche process for this transaction
        self.start_avalanche_process(tx_id).await;
        
        // Propagate to peers
        self.propagate_transaction(tx).await;
    }
    
    /// Handle a block message
    async fn handle_block(&self, block: Block) {
        // Validate block
        let valid = match &self.block_callback {
            Some(callback) => callback(block.clone()),
            None => true, // No callback, assume valid
        };
        
        if !valid {
            warn!("Received invalid block at height {}", block.header.height);
            return;
        }
        
        // Propagate to peers
        self.propagate_block(block).await;
    }
    
    /// Handle a query message
    async fn handle_query(&self, peer_id: NodeId, query: crate::gossip::message::QueryMessage) {
        let response_type = match query.query_type {
            QueryType::GetTransaction(tx_id_hex) => {
                // Parse transaction ID
                let tx_id_bytes = match hex::decode(tx_id_hex.trim_start_matches("0x")) {
                    Ok(bytes) => {
                        if bytes.len() != 32 {
                            ResponseType::Error("Invalid transaction ID format".to_string())
                        } else {
                            let mut array = [0u8; 32];
                            array.copy_from_slice(&bytes);
                            let tx_id = TransactionId::new(array);
                            
                            // Check if we have this transaction
                            if let Some(state) = self.tx_states.get(&tx_id) {
                                ResponseType::Transaction(Some(state.transaction.clone()))
                            } else {
                                ResponseType::Transaction(None)
                            }
                        }
                    }
                    Err(_) => ResponseType::Error("Invalid transaction ID format".to_string()),
                };
                
                response_type
            }
            QueryType::GetBlock(_) => {
                // TODO: Implement block query
                ResponseType::Error("Not implemented".to_string())
            }
            QueryType::GetLatestBlock => {
                // TODO: Implement latest block query
                ResponseType::Error("Not implemented".to_string())
            }
            QueryType::GetPeers => {
                let peers = self
                    .peer_manager
                    .get_connected_peers()
                    .iter()
                    .map(|peer| format!("{}:{}", peer.address, peer.port))
                    .collect();
                
                ResponseType::Peers(peers)
            }
        };
        
        // Send response
        let response = GossipMessage::new_response(query.id, response_type, self.node_id.clone());
        
        if let Err(e) = self.message_tx.send((peer_id, response)).await {
            error!("Failed to send response: {}", e);
        }
    }
    
    /// Handle a response message
    async fn handle_response(&self, response: crate::gossip::message::ResponseMessage) {
        // Check if we have a pending query for this response
        if let Some((_, sender)) = self.pending_queries.remove(&response.id) {
            if let Err(e) = sender.send(response.response_type) {
                error!("Failed to send response to pending query: {:?}", e);
            }
        }
    }
    
    /// Handle a ping message
    async fn handle_ping(&self, peer_id: NodeId, ping: crate::gossip::message::PingMessage) {
        // Update peer last seen
        self.peer_manager.update_peer_last_seen(&peer_id);
        
        // Send pong
        let pong = GossipMessage::new_pong(&ping, self.node_id.clone());
        
        if let Err(e) = self.message_tx.send((peer_id, pong)).await {
            error!("Failed to send pong: {}", e);
        }
    }
    
    /// Handle a pong message
    async fn handle_pong(&self, pong: crate::gossip::message::PongMessage) {
        // Update peer last seen
        self.peer_manager.update_peer_last_seen(&pong.sender);
        
        // Calculate round-trip time
        let rtt = Instant::now().duration_since(
            Instant::now() - Duration::from_millis(pong.timestamp - pong.original_timestamp),
        );
        
        debug!("Received pong from {} with RTT {:?}", pong.sender, rtt);
    }
    
    /// Start Avalanche process for a transaction
    async fn start_avalanche_process(&self, tx_id: TransactionId) {
        let tx_states = self.tx_states.clone();
        let peer_manager = self.peer_manager.clone();
        let message_tx = self.message_tx.clone();
        let node_id = self.node_id.clone();
        let params = self.params.clone();
        let pending_queries = self.pending_queries.clone();
        
        tokio::spawn(async move {
            let mut rounds = 0;
            let mut confidence = 0;
            
            while rounds < params.beta {
                // Get random peers
                let peers = peer_manager.get_random_peers(params.k);
                
                if peers.is_empty() {
                    warn!("No peers available for Avalanche process");
                    break;
                }
                
                let mut positive_responses = 0;
                
                // Query peers
                for peer in &peers {
                    let query_type = QueryType::GetTransaction(format!("0x{}", hex::encode(tx_id.as_bytes())));
                    let query = GossipMessage::new_query(query_type, node_id.clone());
                    
                    if let GossipMessage::Query(q) = &query {
                        let (tx, rx) = oneshot::channel();
                        pending_queries.insert(q.id, tx);
                        
                        if let Err(e) = message_tx.send((peer.id.clone(), query)).await {
                            error!("Failed to send query: {}", e);
                            continue;
                        }
                        
                        // Wait for response with timeout
                        match tokio::time::timeout(Duration::from_secs(5), rx).await {
                            Ok(Ok(response)) => {
                                match response {
                                    ResponseType::Transaction(Some(_)) => {
                                        positive_responses += 1;
                                    }
                                    _ => {
                                        // Negative or no response
                                    }
                                }
                            }
                            Ok(Err(e)) => {
                                error!("Failed to receive response: {:?}", e);
                            }
                            Err(_) => {
                                // Timeout
                                debug!("Query timeout for transaction {}", tx_id);
                            }
                        }
                        
                        // Remove from pending queries if still there
                        pending_queries.remove(&q.id);
                    }
                }
                
                // Update confidence
                if positive_responses >= params.alpha {
                    confidence += 1;
                } else {
                    confidence = 0; // Reset confidence on negative round
                }
                
                rounds += 1;
                
                // Update state
                if let Some(mut state) = tx_states.get_mut(&tx_id) {
                    state.confidence = confidence;
                    state.rounds = rounds;
                    state.last_updated = Instant::now();
                    
                    // Check if finalized
                    if confidence >= params.beta {
                        state.finalized = true;
                        info!("Transaction {} finalized with confidence {}", tx_id, confidence);
                        break;
                    }
                } else {
                    // Transaction was removed
                    break;
                }
                
                // Sleep before next round
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        });
    }
    
    /// Propagate a transaction to peers
    async fn propagate_transaction(&self, tx: Transaction) {
        let peers = self.peer_manager.get_connected_peers();
        let message = GossipMessage::new_transaction(tx);
        
        for peer in peers {
            if let Err(e) = self.message_tx.send((peer.id.clone(), message.clone())).await {
                error!("Failed to propagate transaction to {}: {}", peer.id, e);
            }
        }
    }
    
    /// Propagate a block to peers
    async fn propagate_block(&self, block: Block) {
        let peers = self.peer_manager.get_connected_peers();
        let message = GossipMessage::new_block(block);
        
        for peer in peers {
            if let Err(e) = self.message_tx.send((peer.id.clone(), message.clone())).await {
                error!("Failed to propagate block to {}: {}", peer.id, e);
            }
        }
    }
    
    /// Start cleanup task
    async fn start_cleanup_task(&self) {
        let tx_states = self.tx_states.clone();
        
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(60));
            
            loop {
                interval.tick().await;
                
                let now = Instant::now();
                let timeout = Duration::from_secs(300); // 5 minutes
                
                // Remove stale transactions
                let stale_txs: Vec<TransactionId> = tx_states
                    .iter()
                    .filter(|entry| now.duration_since(entry.last_updated) > timeout)
                    .map(|entry| entry.transaction.id)
                    .collect();
                
                for tx_id in &stale_txs {
                    tx_states.remove(tx_id);
                    debug!("Removed stale transaction {}", tx_id);
                }
                
                if !stale_txs.is_empty() {
                    info!("Removed {} stale transactions", stale_txs.len());
                }
            }
        });
    }
}