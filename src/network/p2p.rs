use std::sync::Arc;
use anyhow::Result;
use libp2p::{
    identity::Keypair,
    PeerId,
    Swarm,
    swarm::SwarmBuilder,
    core::transport::Transport,
    tcp::TokioTcpConfig,
    noise,
    yamux,
    gossipsub::{self, Gossipsub, GossipsubEvent, MessageAuthenticity, ValidationMode},
    kad::{store::MemoryStore, Kademlia, KademliaEvent},
    mdns::{Mdns, MdnsEvent},
    ping::{Ping, PingEvent},
    Multiaddr,
};
use tokio::sync::mpsc;
use super::types::NetworkMessage;

#[derive(Debug)]
pub struct P2PNetwork {
    swarm: Swarm<NetworkBehaviour>,
    message_sender: mpsc::UnboundedSender<NetworkMessage>,
    message_receiver: mpsc::UnboundedReceiver<NetworkMessage>,
}

#[derive(libp2p::NetworkBehaviour)]
#[behaviour(out_event = "NetworkEvent")]
struct NetworkBehaviour {
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

impl P2PNetwork {
    pub async fn new(keypair: Keypair) -> Result<Self> {
        let peer_id = PeerId::from(keypair.public());
        let (message_sender, message_receiver) = mpsc::unbounded_channel();

        // トランスポートの設定
        let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
            .into_authentic(&keypair)?;

        let transport = TokioTcpConfig::new()
            .nodelay(true)
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(noise::NoiseConfig::xx(noise_keys).into_authenticated())
            .multiplex(yamux::YamuxConfig::default())
            .boxed();

        // Gossipsubの設定
        let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
            .heartbeat_interval(std::time::Duration::from_secs(1))
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

        // mDNSの設定
        let mdns = Mdns::new(Default::default()).await?;

        // Pingの設定
        let ping = Ping::new(Default::default());

        // ネットワーク動作の設定
        let behaviour = NetworkBehaviour {
            gossipsub,
            kademlia,
            mdns,
            ping,
        };

        // Swarmの構築
        let swarm = SwarmBuilder::new(transport, behaviour, peer_id)
            .executor(Box::new(|fut| {
                tokio::spawn(fut);
            }))
            .build();

        Ok(Self {
            swarm,
            message_sender,
            message_receiver,
        })
    }

    pub async fn start(&mut self, addr: Multiaddr) -> Result<()> {
        // アドレスをリッスン
        self.swarm.listen_on(addr)?;

        // イベントループを開始
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    event = self.swarm.select_next_some() => {
                        match event {
                            NetworkEvent::Gossipsub(event) => {
                                // Gossipsubイベントの処理
                                if let GossipsubEvent::Message { message, .. } = event {
                                    // メッセージの処理
                                }
                            }
                            NetworkEvent::Kademlia(event) => {
                                // Kademliaイベントの処理
                            }
                            NetworkEvent::Mdns(event) => {
                                // mDNSイベントの処理
                                match event {
                                    MdnsEvent::Discovered(list) => {
                                        for (peer_id, multiaddr) in list {
                                            self.swarm.behaviour_mut().kademlia.add_address(&peer_id, multiaddr);
                                        }
                                    }
                                    MdnsEvent::Expired(list) => {
                                        for (peer_id, multiaddr) in list {
                                            self.swarm.behaviour_mut().kademlia.remove_address(&peer_id, &multiaddr);
                                        }
                                    }
                                }
                            }
                            NetworkEvent::Ping(_) => {
                                // Pingイベントの処理
                            }
                        }
                    }
                    msg = self.message_receiver.recv() => {
                        match msg {
                            Some(msg) => {
                                // メッセージの送信
                                let topic = gossipsub::IdentTopic::new("rustorium");
                                let data = bincode::serialize(&msg)?;
                                self.swarm.behaviour_mut().gossipsub.publish(topic, data)?;
                            }
                            None => break,
                        }
                    }
                }
            }
            Ok::<(), anyhow::Error>(())
        });

        Ok(())
    }

    pub async fn connect_to_peer(&mut self, addr: &Multiaddr) -> Result<()> {
        self.swarm.dial(addr.clone())?;
        Ok(())
    }

    pub fn message_sender(&self) -> mpsc::UnboundedSender<NetworkMessage> {
        self.message_sender.clone()
    }
}