use std::sync::Arc;
use anyhow::Result;
use futures::StreamExt;
use libp2p::{
    core::upgrade,
    identity::Keypair,
    noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp::Config as TcpConfig,
    yamux,
    PeerId,
    Transport,
    gossipsub::{self, Gossipsub, GossipsubEvent, MessageAuthenticity, ValidationMode},
    kad::{store::MemoryStore, Kademlia, KademliaEvent},
    mdns::{Mdns, MdnsEvent},
    ping::{Ping, PingEvent},
    Multiaddr,
    Swarm,
};
use tokio::sync::mpsc;
use super::types::NetworkMessage;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "NetworkEvent")]
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

pub struct P2PNetwork {
    swarm: Swarm<RustoriumBehaviour>,
    message_sender: mpsc::UnboundedSender<NetworkMessage>,
    message_receiver: mpsc::UnboundedReceiver<NetworkMessage>,
}

impl P2PNetwork {
    pub async fn new(keypair: Keypair) -> Result<Self> {
        let peer_id = PeerId::from(keypair.public());
        let (message_sender, message_receiver) = mpsc::unbounded_channel();

        // トランスポートの設定
        let transport = TcpConfig::default()
            .nodelay(true)
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::NoiseAuthenticated::xx(&keypair)?)
            .multiplex(yamux::YamuxConfig::default())
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
        )?;

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
                    event = self.swarm.next() => {
                        match event {
                            Some(SwarmEvent::Behaviour(NetworkEvent::Gossipsub(event))) => {
                                // Gossipsubイベントの処理
                                if let GossipsubEvent::Message { message, .. } = event {
                                    // メッセージの処理
                                }
                            }
                            Some(SwarmEvent::Behaviour(NetworkEvent::Kademlia(event))) => {
                                // Kademliaイベントの処理
                            }
                            Some(SwarmEvent::Behaviour(NetworkEvent::Mdns(event))) => {
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
                            Some(SwarmEvent::Behaviour(NetworkEvent::Ping(_))) => {
                                // Pingイベントの処理
                            }
                            _ => {}
                        }
                    }
                    msg = self.message_receiver.recv() => {
                        match msg {
                            Some(msg) => {
                                // メッセージの送信
                                let topic = gossipsub::Topic::new("rustorium");
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