use crate::common::config::NetworkConfig;
use crate::common::errors::NetworkError;
use crate::common::types::{NodeId, PeerInfo};
use crate::gossip::message::GossipMessage;
use crate::gossip::peer::PeerManager;
use crate::gossip::protocol::AvalancheProtocol;
use libp2p::{
    gossipsub::{
        Gossipsub, GossipsubConfig, GossipsubConfigBuilder, GossipsubEvent, MessageAuthenticity,
        MessageId, Topic, ValidationMode,
    },
    identity, mdns, noise, swarm::SwarmEvent, tcp, yamux, Multiaddr, PeerId, Swarm,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

/// Network service
pub struct NetworkService {
    /// Node ID
    node_id: NodeId,
    /// Peer manager
    peer_manager: Arc<PeerManager>,
    /// Avalanche protocol
    protocol: Arc<tokio::sync::Mutex<AvalancheProtocol>>,
    /// Libp2p swarm
    swarm: Swarm<Gossipsub>,
    /// Message sender
    message_tx: mpsc::Sender<(NodeId, GossipMessage)>,
    /// Message receiver
    message_rx: mpsc::Receiver<(NodeId, GossipMessage)>,
    /// Network config
    config: NetworkConfig,
}

impl NetworkService {
    /// Create a new network service
    pub async fn new(
        node_id: NodeId,
        config: NetworkConfig,
        peer_manager: Arc<PeerManager>,
        protocol: Arc<tokio::sync::Mutex<AvalancheProtocol>>,
    ) -> Result<Self, NetworkError> {
        // Create identity
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        
        info!("Local peer ID: {}", local_peer_id);
        
        // Create transport
        let transport = tcp::tokio::Transport::default()
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(noise::NoiseAuthenticated::xx(&local_key).unwrap())
            .multiplex(yamux::YamuxConfig::default())
            .boxed();
        
        // Create gossipsub
        let gossipsub_config = GossipsubConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10))
            .validation_mode(ValidationMode::Strict)
            .message_id_fn(|message| {
                let mut hasher = DefaultHasher::new();
                message.data.hash(&mut hasher);
                MessageId::from(hasher.finish().to_string())
            })
            .build()
            .map_err(|e| NetworkError::ConnectionFailed(e.to_string()))?;
        
        let mut gossipsub = Gossipsub::new(MessageAuthenticity::Signed(local_key), gossipsub_config)
            .map_err(|e| NetworkError::ConnectionFailed(e.to_string()))?;
        
        // Subscribe to topics
        let tx_topic = Topic::new("transactions");
        let block_topic = Topic::new("blocks");
        
        gossipsub
            .subscribe(&tx_topic)
            .map_err(|e| NetworkError::ConnectionFailed(e.to_string()))?;
        
        gossipsub
            .subscribe(&block_topic)
            .map_err(|e| NetworkError::ConnectionFailed(e.to_string()))?;
        
        // Create swarm
        let mut swarm = Swarm::with_tokio_executor(transport, gossipsub, local_peer_id);
        
        // Listen on address
        let listen_addr = format!("/ip4/{}/tcp/{}", config.listen_addr, config.listen_port);
        swarm
            .listen_on(listen_addr.parse().map_err(|e| NetworkError::ConnectionFailed(e.to_string()))?)
            .map_err(|e| NetworkError::ConnectionFailed(e.to_string()))?;
        
        // Create message channel
        let (message_tx, message_rx) = mpsc::channel(1000);
        
        Ok(Self {
            node_id,
            peer_manager,
            protocol,
            swarm,
            message_tx,
            message_rx,
            config,
        })
    }
    
    /// Get message sender
    pub fn get_message_sender(&self) -> mpsc::Sender<(NodeId, GossipMessage)> {
        self.message_tx.clone()
    }
    
    /// Start the network service
    pub async fn start(&mut self) -> Result<(), NetworkError> {
        info!("Starting network service");
        
        // Connect to bootstrap nodes
        for addr in &self.config.bootstrap_nodes {
            match addr.parse::<Multiaddr>() {
                Ok(multiaddr) => {
                    info!("Connecting to bootstrap node: {}", multiaddr);
                    if let Err(e) = self.swarm.dial(multiaddr) {
                        warn!("Failed to dial bootstrap node: {}", e);
                    }
                }
                Err(e) => {
                    warn!("Invalid bootstrap node address {}: {}", addr, e);
                }
            }
        }
        
        // Start event loop
        loop {
            tokio::select! {
                event = self.swarm.select_next_some() => {
                    self.handle_swarm_event(event).await?;
                }
                Some((peer_id, message)) = self.message_rx.recv() => {
                    self.handle_outgoing_message(peer_id, message).await?;
                }
            }
        }
    }
    
    /// Handle swarm event
    async fn handle_swarm_event(&mut self, event: SwarmEvent<GossipsubEvent, void::Void>) -> Result<(), NetworkError> {
        match event {
            SwarmEvent::Behaviour(GossipsubEvent::Message {
                propagation_source,
                message_id,
                message,
            }) => {
                debug!(
                    "Received message: {} from {}",
                    message_id, propagation_source
                );
                
                // Deserialize message
                match bincode::deserialize::<GossipMessage>(&message.data) {
                    Ok(gossip_message) => {
                        // Convert PeerId to NodeId
                        let peer_id = NodeId(propagation_source.to_string());
                        
                        // Forward to protocol
                        let mut protocol = self.protocol.lock().await;
                        if let Err(e) = protocol.get_message_sender().send((peer_id, gossip_message)).await {
                            error!("Failed to forward message to protocol: {}", e);
                        }
                    }
                    Err(e) => {
                        warn!("Failed to deserialize message: {}", e);
                    }
                }
            }
            SwarmEvent::NewListenAddr { address, .. } => {
                info!("Listening on {}", address);
            }
            SwarmEvent::ConnectionEstablished {
                peer_id, endpoint, ..
            } => {
                info!("Connected to {}", peer_id);
                
                // Add peer to peer manager
                let addr = match endpoint.get_remote_address() {
                    Some(addr) => addr.to_string(),
                    None => "unknown".to_string(),
                };
                
                let peer_info = PeerInfo {
                    id: NodeId(peer_id.to_string()),
                    address: addr,
                    port: 0, // Unknown
                    last_seen: crate::common::utils::current_time_sec(),
                };
                
                self.peer_manager.add_peer(peer_info);
                self.peer_manager.connect_peer(&NodeId(peer_id.to_string()));
            }
            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                info!("Disconnected from {}", peer_id);
                self.peer_manager.disconnect_peer(&NodeId(peer_id.to_string()));
            }
            _ => {}
        }
        
        Ok(())
    }
    
    /// Handle outgoing message
    async fn handle_outgoing_message(
        &mut self,
        peer_id: NodeId,
        message: GossipMessage,
    ) -> Result<(), NetworkError> {
        // Serialize message
        let data = bincode::serialize(&message)
            .map_err(|e| NetworkError::InvalidMessageFormat)?;
        
        // Determine topic
        let topic = match &message {
            GossipMessage::Transaction(_) => Topic::new("transactions"),
            GossipMessage::Block(_) => Topic::new("blocks"),
            _ => {
                // Direct message to peer
                // Convert NodeId to PeerId
                let peer_id_str = peer_id.0;
                match peer_id_str.parse::<PeerId>() {
                    Ok(libp2p_peer_id) => {
                        // TODO: Implement direct messaging
                        // For now, just log
                        debug!("Direct message to {} not implemented", libp2p_peer_id);
                        return Ok(());
                    }
                    Err(e) => {
                        warn!("Invalid peer ID {}: {}", peer_id_str, e);
                        return Err(NetworkError::PeerNotFound(peer_id_str));
                    }
                }
            }
        };
        
        // Publish to topic
        if let Err(e) = self.swarm.behaviour_mut().publish(topic, data) {
            error!("Failed to publish message: {}", e);
            return Err(NetworkError::ConnectionFailed(e.to_string()));
        }
        
        Ok(())
    }
}