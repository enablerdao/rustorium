//! P2Pネットワークの実装
//! 
//! このモジュールは、ノード間の通信を管理します。
//! 主な機能：
//! - ピアツーピア通信
//! - メッセージングプロトコル
//! - ネットワークイベント処理

use std::{
    collections::HashSet,
    sync::Arc,
    time::Duration,
};
use tokio::sync::Mutex;
use anyhow::Result;
use futures::{StreamExt, task::Poll};
use libp2p::{
    core::upgrade::Version,
    floodsub::{Floodsub, FloodsubEvent, Topic},
    identity,
    mdns::{self, tokio::Behaviour as MdnsBehaviour},
    noise,
    swarm::{NetworkBehaviour, SwarmEvent, Config as SwarmConfig},
    tcp::Config as TcpConfig,
    yamux,
    Multiaddr,
    PeerId,
    Swarm,
    Transport,
};
use tokio::sync::mpsc;
use tracing::{info, warn};

/// P2Pネットワーク設定
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    /// リッスンアドレス
    pub listen_addresses: Vec<Multiaddr>,
    /// 外部アドレス
    pub external_addresses: Vec<Multiaddr>,
    /// プロトコルプレフィックス
    pub protocol_prefix: String,
    /// 接続タイムアウト
    pub timeout: Duration,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            listen_addresses: vec![
                "/ip4/0.0.0.0/tcp/4001".parse().unwrap(),
                "/ip6/::/tcp/4001".parse().unwrap(),
            ],
            external_addresses: vec![],
            protocol_prefix: "/rustorium/1.0.0".to_string(),
            timeout: Duration::from_secs(20),
        }
    }
}

/// P2Pネットワークマネージャー
pub struct P2PNetwork {
    swarm: Arc<Mutex<Swarm<RustoriumBehaviour>>>,
    peers: HashSet<PeerId>,
    tx: mpsc::Sender<NetworkEvent>,
    rx: mpsc::Receiver<NetworkEvent>,
    config: NetworkConfig,
    local_peer_id: PeerId,
}

impl Clone for P2PNetwork {
    fn clone(&self) -> Self {
        let (tx, rx) = mpsc::channel(32);
        Self {
            swarm: self.swarm.clone(),
            peers: self.peers.clone(),
            tx,
            rx,
            config: self.config.clone(),
            local_peer_id: self.local_peer_id,
        }
    }
}

impl std::fmt::Debug for P2PNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("P2PNetwork")
            .field("peers", &self.peers)
            .field("config", &self.config)
            .field("local_peer_id", &self.local_peer_id)
            .finish()
    }
}

impl P2PNetwork {
    /// 新しいP2Pネットワークマネージャーを作成
    pub async fn new(keypair: identity::Keypair) -> Result<Self> {
        let config = NetworkConfig::default();
        let local_peer_id = PeerId::from(keypair.public());
        info!("Local peer id: {}", local_peer_id);

        let (tx, rx) = mpsc::channel(32);

        // ノイズプロトコルの設定
        let noise_config = noise::Config::new(&keypair)?;

        // トランスポートの設定
        let transport = libp2p::tcp::tokio::Transport::new(TcpConfig::default().nodelay(true))
            .upgrade(Version::V1)
            .authenticate(noise_config)
            .multiplex(yamux::Config::default())
            .timeout(config.timeout)
            .boxed();

        // ビヘイビアの初期化
        let behaviour = RustoriumBehaviour::new(local_peer_id).await?;

        // スワームの設定
        let mut swarm = Swarm::new(
            transport,
            behaviour,
            local_peer_id,
            SwarmConfig::with_tokio_executor(),
        );

        // リッスンアドレスを設定
        for addr in &config.listen_addresses {
            swarm.listen_on(addr.clone())?;
        }

        // 外部アドレスを設定
        for addr in &config.external_addresses {
            swarm.add_external_address(addr.clone());
        }

        Ok(Self {
            swarm: Arc::new(Mutex::new(swarm)),
            peers: HashSet::new(),
            tx,
            rx,
            config,
            local_peer_id,
        })
    }

    /// ネットワークイベントの受信チャネルを取得
    pub fn event_channel(&mut self) -> mpsc::Receiver<NetworkEvent> {
        std::mem::replace(&mut self.rx, mpsc::channel(32).1)
    }

    /// メッセージをブロードキャスト
    pub async fn broadcast(&self, topic: &str, data: Vec<u8>) -> Result<()> {
        let topic = Topic::new(topic);
        let mut swarm = self.swarm.lock().await;
        swarm.behaviour_mut().floodsub.publish(topic, data);
        Ok(())
    }

    /// トピックをサブスクライブ
    pub async fn subscribe(&self, topic: &str) -> Result<()> {
        let topic = Topic::new(topic);
        let mut swarm = self.swarm.lock().await;
        swarm.behaviour_mut().floodsub.subscribe(topic);
        Ok(())
    }

    /// トピックのサブスクライブを解除
    pub async fn unsubscribe(&self, topic: &str) -> Result<()> {
        let topic = Topic::new(topic);
        let mut swarm = self.swarm.lock().await;
        swarm.behaviour_mut().floodsub.unsubscribe(topic);
        Ok(())
    }

    /// ネットワークイベントの処理を開始
    pub async fn run(&self) -> Result<()> {
        let swarm = self.swarm.clone();
        let tx = self.tx.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(100));
            loop {
                interval.tick().await;
                let mut swarm_guard = swarm.lock().await;
                
                // イベントを処理
                while let Poll::Ready(event) = futures::poll!(swarm_guard.next()) {
                    if let Some(event) = event {
                        match event {
                            SwarmEvent::Behaviour(RustoriumBehaviourEvent::Floodsub(
                                FloodsubEvent::Message(message),
                            )) => {
                                let topic = format!("{:?}", message.topics[0]);
                                let data = message.data.to_vec();
                                let _ = tx.send(NetworkEvent::Message {
                                    topic,
                                    data,
                                    source: Some(message.source),
                                }).await;
                            }
                            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                                let _ = tx.send(NetworkEvent::PeerConnected(peer_id)).await;
                            }
                            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                                let _ = tx.send(NetworkEvent::PeerDisconnected(peer_id)).await;
                            }
                            event => {
                                warn!("Unhandled swarm event: {:?}", event);
                            }
                        }
                    }
                }
                drop(swarm_guard);
            }
        });

        Ok(())
    }

    /// 接続中のピアを取得
    pub fn connected_peers(&self) -> HashSet<PeerId> {
        self.peers.clone()
    }

    /// ローカルのピアIDを取得
    pub fn local_peer_id(&self) -> PeerId {
        self.local_peer_id
    }
}

/// カスタムネットワーク動作
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "RustoriumBehaviourEvent")]
struct RustoriumBehaviour {
    floodsub: Floodsub,
    mdns: MdnsBehaviour,
}

/// カスタムネットワークイベント
#[derive(Debug)]
enum RustoriumBehaviourEvent {
    Floodsub(FloodsubEvent),
    Mdns(()),
}

impl From<FloodsubEvent> for RustoriumBehaviourEvent {
    fn from(event: FloodsubEvent) -> Self {
        RustoriumBehaviourEvent::Floodsub(event)
    }
}

impl From<mdns::Event> for RustoriumBehaviourEvent {
    fn from(_event: mdns::Event) -> Self {
        RustoriumBehaviourEvent::Mdns(())
    }
}

impl RustoriumBehaviour {
    async fn new(peer_id: PeerId) -> Result<Self> {
        Ok(Self {
            floodsub: Floodsub::new(peer_id),
            mdns: MdnsBehaviour::new(mdns::Config::default(), peer_id)?,
        })
    }
}

/// ネットワークイベント
#[derive(Debug)]
pub enum NetworkEvent {
    /// メッセージ受信
    Message {
        topic: String,
        data: Vec<u8>,
        source: Option<PeerId>,
    },
    /// ピア接続
    PeerConnected(PeerId),
    /// ピア切断
    PeerDisconnected(PeerId),
    /// エラー
    Error(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_network_creation() {
        let keypair = identity::Keypair::generate_ed25519();
        let network = P2PNetwork::new(keypair).await.unwrap();

        assert!(!network.local_peer_id().to_string().is_empty());
        assert_eq!(network.connected_peers().len(), 0);
    }

    #[tokio::test]
    async fn test_pubsub() {
        let keypair = identity::Keypair::generate_ed25519();
        let network = P2PNetwork::new(keypair).await.unwrap();

        // トピックをサブスクライブ
        let topic = "test";
        network.subscribe(topic).await.unwrap();

        // メッセージをブロードキャスト
        let data = b"Hello, world!".to_vec();
        network.broadcast(topic, data.clone()).await.unwrap();

        // イベントチャネルを取得
        let mut rx = network.event_channel();

        // ネットワークイベント処理を開始
        network.run().await.unwrap();

        // タイムアウト付きでイベントを待機
        tokio::select! {
            Some(event) = rx.recv() => {
                match event {
                    NetworkEvent::Message { topic: t, data: d, .. } => {
                        assert_eq!(t, topic);
                        assert_eq!(d, data);
                    }
                    _ => panic!("Unexpected event"),
                }
            }
            _ = tokio::time::sleep(Duration::from_secs(1)) => {
                panic!("Timeout waiting for message");
            }
        }
    }
}