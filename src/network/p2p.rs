use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use anyhow::Result;
use libp2p::{
    core::upgrade,
    gossipsub::{
        self, Gossipsub, GossipsubConfigBuilder, GossipsubMessage, MessageAuthenticity, MessageId,
        ValidationMode,
    },
    identity::Keypair,
    kad::{store::MemoryStore, Kademlia, KademliaConfig},
    mdns::{self, Mdns, MdnsConfig},
    noise,
    request_response::{
        self, ProtocolSupport, RequestResponse, RequestResponseCodec, RequestResponseConfig,
        RequestResponseMessage,
    },
    swarm::{NetworkBehaviour, SwarmBuilder, SwarmEvent},
    tcp::Config as TcpConfig,
    yamux, Multiaddr, PeerId, Swarm, Transport,
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

// ネットワークメッセージ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    // DAG関連
    NewTransaction(crate::core::dag::Transaction),
    TransactionConfirmation(crate::core::dag::TxId),
    
    // Avalanche関連
    Vote {
        tx_id: crate::core::dag::TxId,
        vote: crate::core::avalanche::Vote,
    },
    QueryTransaction {
        tx_id: crate::core::dag::TxId,
    },
    
    // シャーディング関連
    CrossShardTransaction(crate::core::sharding::CrossShardTx),
    ShardState {
        shard_id: crate::core::sharding::ShardId,
        state: crate::core::sharding::ShardState,
    },
}

// カスタムコーデック
#[derive(Debug, Clone)]
pub struct MessageCodec;

impl RequestResponseCodec for MessageCodec {
    type Protocol = &'static str;
    type Request = NetworkMessage;
    type Response = NetworkMessage;

    fn encode_request(&mut self, req: &Self::Request) -> Vec<u8> {
        serde_json::to_vec(req).unwrap()
    }

    fn encode_response(&mut self, res: &Self::Response) -> Vec<u8> {
        serde_json::to_vec(res).unwrap()
    }

    fn decode_request(&mut self, bytes: &[u8]) -> Result<Self::Request, std::io::Error> {
        serde_json::from_slice(bytes).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    fn decode_response(&mut self, bytes: &[u8]) -> Result<Self::Response, std::io::Error> {
        serde_json::from_slice(bytes).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }
}

// ネットワーク動作の定義
#[derive(NetworkBehaviour)]
pub struct RustoriumBehaviour {
    gossipsub: Gossipsub,
    kademlia: Kademlia<MemoryStore>,
    mdns: Mdns,
    request_response: RequestResponse<MessageCodec>,
}

pub struct P2PNetwork {
    swarm: Swarm<RustoriumBehaviour>,
    message_sender: mpsc::UnboundedSender<NetworkMessage>,
    message_receiver: mpsc::UnboundedReceiver<NetworkMessage>,
}

impl P2PNetwork {
    pub async fn new(keypair: Keypair) -> Result<Self> {
        // PeerIDの生成
        let peer_id = PeerId::from(keypair.public());
        info!("Local peer id: {}", peer_id);

        // トランスポートの設定
        let transport = libp2p::development_transport(keypair.clone()).await?;

        // Gossipsubの設定
        let gossipsub_config = GossipsubConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(1))
            .validation_mode(ValidationMode::Strict)
            .build()
            .expect("Valid config");

        let gossipsub = Gossipsub::new(
            MessageAuthenticity::Signed(keypair.clone()),
            gossipsub_config,
        )?;

        // Kademliaの設定
        let store = MemoryStore::new(peer_id);
        let kademlia = Kademlia::new(peer_id, store);

        // MDNSの設定
        let mdns = Mdns::new(MdnsConfig::default()).await?;

        // Request-Responseの設定
        let request_response = RequestResponse::new(
            MessageCodec,
            vec![(NetworkProtocol::default(), ProtocolSupport::Full)],
            RequestResponseConfig::default(),
        );

        // ビヘイビアの組み立て
        let behaviour = RustoriumBehaviour {
            gossipsub,
            kademlia,
            mdns,
            request_response,
        };

        // Swarmの構築
        let swarm = SwarmBuilder::with_tokio_executor(transport, behaviour, peer_id).build();

        // メッセージチャネルの作成
        let (sender, receiver) = mpsc::unbounded_channel();

        Ok(Self {
            swarm,
            message_sender: sender,
            message_receiver: receiver,
        })
    }

    pub async fn start(&mut self, listen_addr: Multiaddr) -> Result<()> {
        // リスニングアドレスのバインド
        self.swarm.listen_on(listen_addr)?;

        // メインイベントループ
        loop {
            tokio::select! {
                event = self.swarm.select_next_some() => {
                    self.handle_swarm_event(event).await?;
                }
                msg = self.message_receiver.recv() => {
                    if let Some(msg) = msg {
                        self.handle_outbound_message(msg).await?;
                    }
                }
            }
        }
    }

    async fn handle_swarm_event(&mut self, event: SwarmEvent<RustoriumBehaviourEvent>) -> Result<()> {
        match event {
            SwarmEvent::NewListenAddr { address, .. } => {
                info!("Listening on {}", address);
            }
            SwarmEvent::Behaviour(RustoriumBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                for (peer_id, _multiaddr) in list {
                    info!("mDNS discovered a new peer: {}", peer_id);
                    self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                }
            }
            SwarmEvent::Behaviour(RustoriumBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                propagation_source: peer_id,
                message_id: id,
                message,
            })) => {
                debug!("Got message: {} with id: {} from peer: {}", message.data.len(), id, peer_id);
                if let Ok(msg) = serde_json::from_slice::<NetworkMessage>(&message.data) {
                    self.handle_inbound_message(msg, peer_id).await?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_inbound_message(&mut self, msg: NetworkMessage, source: PeerId) -> Result<()> {
        match msg {
            NetworkMessage::NewTransaction(tx) => {
                // TODO: トランザクションの処理
            }
            NetworkMessage::Vote { tx_id, vote } => {
                // TODO: 投票の処理
            }
            NetworkMessage::CrossShardTransaction(tx) => {
                // TODO: クロスシャードトランザクションの処理
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_outbound_message(&mut self, msg: NetworkMessage) -> Result<()> {
        let data = serde_json::to_vec(&msg)?;
        let topic = self.get_topic_for_message(&msg);
        
        if let Err(e) = self.swarm.behaviour_mut().gossipsub.publish(topic, data) {
            warn!("Failed to publish message: {}", e);
        }
        Ok(())
    }

    fn get_topic_for_message(&self, msg: &NetworkMessage) -> libp2p::gossipsub::Topic {
        let topic_str = match msg {
            NetworkMessage::NewTransaction(_) => "transactions",
            NetworkMessage::Vote { .. } => "votes",
            NetworkMessage::CrossShardTransaction(_) => "cross-shard",
            _ => "general",
        };
        libp2p::gossipsub::Topic::new(topic_str)
    }

    pub fn message_sender(&self) -> mpsc::UnboundedSender<NetworkMessage> {
        self.message_sender.clone()
    }
}

#[derive(Debug, Clone)]
pub struct NetworkProtocol(&'static str);

impl Default for NetworkProtocol {
    fn default() -> Self {
        NetworkProtocol("/rustorium/1.0.0")
    }
}