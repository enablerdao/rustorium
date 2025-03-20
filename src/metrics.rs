use std::sync::Arc;
use tokio::sync::broadcast;
use serde::{Serialize, Deserialize};
use crate::web::ws::{MetricsData, BlockData, PeerData};

const CHANNEL_SIZE: usize = 100;

/// メトリクス管理
#[derive(Debug)]
pub struct MetricsState {
    metrics_tx: broadcast::Sender<MetricsData>,
    blocks_tx: broadcast::Sender<BlockData>,
    peers_tx: broadcast::Sender<PeerData>,
    current: Arc<tokio::sync::RwLock<CurrentMetrics>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentMetrics {
    pub metrics: MetricsData,
    pub block: BlockData,
    pub peers: PeerData,
}

impl Default for CurrentMetrics {
    fn default() -> Self {
        Self {
            metrics: MetricsData {
                cpu_usage: 0.0,
                memory_usage: 0.0,
                network_in: 0,
                network_out: 0,
            },
            block: BlockData {
                height: 0,
                hash: String::new(),
                transactions: 0,
                timestamp: 0,
            },
            peers: PeerData {
                connected: 0,
                total_known: 0,
                bandwidth: 0.0,
            },
        }
    }
}

impl MetricsState {
    /// 新しいメトリクス管理を作成
    pub fn new() -> Self {
        let (metrics_tx, _) = broadcast::channel(CHANNEL_SIZE);
        let (blocks_tx, _) = broadcast::channel(CHANNEL_SIZE);
        let (peers_tx, _) = broadcast::channel(CHANNEL_SIZE);

        Self {
            metrics_tx,
            blocks_tx,
            peers_tx,
            current: Arc::new(tokio::sync::RwLock::new(CurrentMetrics::default())),
        }
    }

    /// メトリクス更新を購読
    pub fn subscribe(&self) -> broadcast::Receiver<MetricsData> {
        self.metrics_tx.subscribe()
    }

    /// ブロック更新を購読
    pub fn subscribe_blocks(&self) -> broadcast::Receiver<BlockData> {
        self.blocks_tx.subscribe()
    }

    /// ピア更新を購読
    pub fn subscribe_peers(&self) -> broadcast::Receiver<PeerData> {
        self.peers_tx.subscribe()
    }

    /// 現在のメトリクスを取得
    pub fn get_current(&self) -> CurrentMetrics {
        self.current.try_read()
            .map(|m| m.clone())
            .unwrap_or_default()
    }

    /// メトリクスを更新
    pub async fn update_metrics(&self, metrics: MetricsData) {
        let _ = self.metrics_tx.send(metrics.clone());
        if let Ok(mut current) = self.current.write() {
            current.metrics = metrics;
        }
    }

    /// ブロック情報を更新
    pub async fn update_block(&self, block: BlockData) {
        let _ = self.blocks_tx.send(block.clone());
        if let Ok(mut current) = self.current.write() {
            current.block = block;
        }
    }

    /// ピア情報を更新
    pub async fn update_peers(&self, peers: PeerData) {
        let _ = self.peers_tx.send(peers.clone());
        if let Ok(mut current) = self.current.write() {
            current.peers = peers;
        }
    }
}