use crate::common::types::{NodeId, PeerInfo};
use crate::common::utils;
use dashmap::DashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time;
use tracing::{debug, info, warn};

/// Peer manager
pub struct PeerManager {
    /// Known peers
    peers: Arc<DashMap<NodeId, PeerInfo>>,
    /// Connected peers
    connected_peers: Arc<DashMap<NodeId, PeerInfo>>,
    /// Maximum number of peers
    max_peers: usize,
    /// Node ID
    node_id: NodeId,
}

impl PeerManager {
    /// Create a new peer manager
    pub fn new(node_id: NodeId, max_peers: usize) -> Self {
        Self {
            peers: Arc::new(DashMap::new()),
            connected_peers: Arc::new(DashMap::new()),
            max_peers,
            node_id,
        }
    }
    
    /// Add a peer
    pub fn add_peer(&self, peer: PeerInfo) {
        if peer.id == self.node_id {
            return; // Don't add ourselves
        }
        
        self.peers.insert(peer.id.clone(), peer);
    }
    
    /// Remove a peer
    pub fn remove_peer(&self, peer_id: &NodeId) {
        self.peers.remove(peer_id);
        self.connected_peers.remove(peer_id);
    }
    
    /// Connect to a peer
    pub fn connect_peer(&self, peer_id: &NodeId) -> bool {
        if self.connected_peers.len() >= self.max_peers {
            warn!("Cannot connect to peer {}: maximum number of peers reached", peer_id);
            return false;
        }
        
        if let Some(peer) = self.peers.get(peer_id) {
            let peer_info = peer.value().clone();
            self.connected_peers.insert(peer_id.clone(), peer_info);
            true
        } else {
            warn!("Cannot connect to peer {}: peer not found", peer_id);
            false
        }
    }
    
    /// Disconnect from a peer
    pub fn disconnect_peer(&self, peer_id: &NodeId) {
        self.connected_peers.remove(peer_id);
    }
    
    /// Get a peer
    pub fn get_peer(&self, peer_id: &NodeId) -> Option<PeerInfo> {
        self.peers.get(peer_id).map(|peer| peer.value().clone())
    }
    
    /// Get all peers
    pub fn get_all_peers(&self) -> Vec<PeerInfo> {
        self.peers.iter().map(|peer| peer.value().clone()).collect()
    }
    
    /// Get connected peers
    pub fn get_connected_peers(&self) -> Vec<PeerInfo> {
        self.connected_peers.iter().map(|peer| peer.value().clone()).collect()
    }
    
    /// Get random peers
    pub fn get_random_peers(&self, count: usize) -> Vec<PeerInfo> {
        let mut peers = self.get_connected_peers();
        
        if peers.len() <= count {
            return peers;
        }
        
        // Shuffle peers
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        peers.shuffle(&mut rng);
        
        peers.truncate(count);
        peers
    }
    
    /// Update peer last seen
    pub fn update_peer_last_seen(&self, peer_id: &NodeId) {
        if let Some(mut peer) = self.peers.get_mut(peer_id) {
            peer.last_seen = utils::current_time_sec();
        }
        
        if let Some(mut peer) = self.connected_peers.get_mut(peer_id) {
            peer.last_seen = utils::current_time_sec();
        }
    }
    
    /// Start peer cleanup task
    pub async fn start_cleanup_task(&self, interval_sec: u64) {
        let peers = self.peers.clone();
        let connected_peers = self.connected_peers.clone();
        
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(interval_sec));
            
            loop {
                interval.tick().await;
                
                let now = utils::current_time_sec();
                let timeout = 300; // 5 minutes
                
                // Remove stale peers
                let stale_peers: Vec<NodeId> = peers
                    .iter()
                    .filter(|peer| now - peer.last_seen > timeout)
                    .map(|peer| peer.id.clone())
                    .collect();
                
                for peer_id in &stale_peers {
                    peers.remove(peer_id);
                    connected_peers.remove(peer_id);
                    debug!("Removed stale peer {}", peer_id);
                }
                
                if !stale_peers.is_empty() {
                    info!("Removed {} stale peers", stale_peers.len());
                }
            }
        });
    }
}