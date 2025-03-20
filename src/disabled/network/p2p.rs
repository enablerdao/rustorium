use std::sync::Arc;
use anyhow::Result;
use futures::StreamExt;
use libp2p::{
    core::{
        transport::Transport,
        upgrade::Version,
        muxing::StreamMuxerBox,
    },
    identity::Keypair,
    swarm::{NetworkBehaviour, SwarmEvent, Swarm},
    tcp::TokioTcpConfig,
    noise::{NoiseConfig, X25519Spec, NoiseAuthenticated},
    yamux::YamuxConfig,
    PeerId,
    Multiaddr,
    gossipsub::{self, Gossipsub, GossipsubEvent, MessageAuthenticity, ValidationMode},
    kad::{store::MemoryStore, Kademlia, KademliaEvent},
    mdns::{Mdns, MdnsEvent},
    ping::{Ping, PingEvent},
};
use tokio::sync::mpsc;
use tracing;
use super::types::NetworkMessage;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "NetworkEvent", event_process = false)]
struct RustoriumBehaviour {
    gossipsub: Gossipsub,
    kademlia: Kademlia<MemoryStore>,
    mdns: Mdns,
    ping: Ping,
}

#[derive(Debug)]
enum NetworkEvent {
    Gossipsub(GossipsubEvent),
    Kademlia(KademliaEvent),
    Mdns(MdnsEvent),
    Ping(PingEvent),
}

impl From<GossipsubEvent> for NetworkEvent {
    fn from(event: GossipsubEvent) -> Self {
        NetworkEvent::Gossipsub(event)
    }
}

impl From<KademliaEvent> for NetworkEvent {
    fn from(event: KademliaEvent) -> Self {
        NetworkEvent::Kademlia(event)
    }
}

impl From<MdnsEvent> for NetworkEvent {
    fn from(event: MdnsEvent) -> Self {
        NetworkEvent::Mdns(event)
    }
}

impl From<PingEvent> for NetworkEvent {
    fn from(event: PingEvent) -> Self {
        NetworkEvent::Ping(event)
    }
}

pub struct P2PNetwork {
    swarm: Arc<tokio::sync::Mutex<Swarm<RustoriumBehaviour>>>,
    message_sender: mpsc::UnboundedSender<NetworkMessage>,
    message_receiver: Arc<tokio::sync::Mutex<mpsc::UnboundedReceiver<NetworkMessage>>>,
}

impl P2PNetwork {
    pub async fn new(keypair: Keypair) -> Result<Self> {
        let peer_id = PeerId::from(keypair.public());
        let (message_sender, message_receiver) = mpsc::unbounded_channel();

        // トランスポートの設定
        let noise = NoiseConfig::xx_spec()
            .into_authenticated(keypair.clone());
        let transport = TokioTcpConfig::new()
            .nodelay(true)
            .upgrade(Version::V1)
            .authenticate(noise)
            .multiplex(YamuxConfig::default())
            .boxed();

        // Gossipsubの設定
        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(std::time::Duration::from_secs(1))
            .validation_mode(ValidationMode::Strict)
            .build()
            .expect("Valid config");

        let gossipsub = Gossipsub::new(
            MessageAuthenticity::Signed(keypair.clone()),
            gossipsub_config,
        ).expect("Valid config");

        // Kademliaの設定
        let store = MemoryStore::new(peer_id);
        let kademlia = Kademlia::new(peer_id, store);

        // mDNSの設定
        let mdns = Mdns::new(Default::default()).await?;

        // Pingの設定
        let ping = Ping::default();

        // ネットワーク動作の設定
        let behaviour = RustoriumBehaviour {
            gossipsub,
            kademlia,
            mdns,
            ping,
        };

        // Swarmの構築
        let swarm = Swarm::new(transport, behaviour, peer_id);

        Ok(Self {
            swarm: Arc::new(tokio::sync::Mutex::new(swarm)),
            message_sender,
            message_receiver: Arc::new(tokio::sync::Mutex::new(message_receiver)),
        })
    }

    pub async fn start(&mut self, addr: Multiaddr) -> Result<()> {
        // アドレスをリッスン
        {
            let mut swarm = self.swarm.lock().await;
            swarm.listen_on(addr)?;

            // デフォルトのトピックを購読
            let topic = gossipsub::IdentTopic::new("rustorium");
            swarm.behaviour_mut().gossipsub.subscribe(&topic)?;
        }

        // イベントループを開始
        let swarm = self.swarm.clone();
        let receiver = self.message_receiver.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    event = {
                        let mut swarm = swarm.lock().await;
                        swarm.select_next_some()
                    } => {
                        let mut swarm = swarm.lock().await;
                        match event {
                            SwarmEvent::Behaviour(NetworkEvent::Gossipsub(event)) => {
                                if let GossipsubEvent::Message { message, .. } = event {
                                    if let Ok(msg) = bincode::deserialize::<NetworkMessage>(&message.data) {
                                        // メッセージの処理
                                        tracing::debug!("Received message: {:?}", msg);
                                    }
                                }
                            }
                            SwarmEvent::Behaviour(NetworkEvent::Kademlia(event)) => {
                                tracing::debug!("Kademlia event: {:?}", event);
                            }
                            SwarmEvent::Behaviour(NetworkEvent::Mdns(event)) => {
                                match event {
                                    MdnsEvent::Discovered(list) => {
                                        for (peer_id, multiaddr) in list {
                                            swarm.behaviour_mut().kademlia.add_address(&peer_id, multiaddr);
                                            tracing::debug!("Discovered peer: {}", peer_id);
                                        }
                                    }
                                    MdnsEvent::Expired(list) => {
                                        for (peer_id, multiaddr) in list {
                                            swarm.behaviour_mut().kademlia.remove_address(&peer_id, &multiaddr);
                                            tracing::debug!("Expired peer: {}", peer_id);
                                        }
                                    }
                                }
                            }
                            SwarmEvent::Behaviour(NetworkEvent::Ping(event)) => {
                                tracing::debug!("Ping event: {:?}", event);
                            }
                            SwarmEvent::NewListenAddr { address, .. } => {
                                tracing::info!("Listening on {}", address);
                            }
                            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                                tracing::info!("Connected to {}", peer_id);
                            }
                            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                                tracing::info!("Disconnected from {}", peer_id);
                            }
                            _ => {}
                        }
                    }
                    Some(msg) = {
                        let mut receiver = receiver.lock().await;
                        receiver.recv()
                    } => {
                        let mut swarm = swarm.lock().await;
                        let topic = gossipsub::IdentTopic::new("rustorium");
                        match bincode::serialize(&msg) {
                            Ok(data) => {
                                if let Err(e) = swarm.behaviour_mut().gossipsub.publish(topic, data) {
                                    tracing::error!("Failed to publish message: {}", e);
                                }
                            }
                            Err(e) => {
                                tracing::error!("Failed to serialize message: {}", e);
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn connect_to_peer(&mut self, addr: &Multiaddr) -> Result<()> {
        let mut swarm = self.swarm.lock().await;
        swarm.dial(addr.clone())?;
        Ok(())
    }

    pub fn message_sender(&self) -> mpsc::UnboundedSender<NetworkMessage> {
        self.message_sender.clone()
    }
}