use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, RwLock};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use super::dag::{TxId, Transaction, TxStatus};

/// 投票結果
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Vote {
    Accept,
    Reject,
}

/// コンフィデンス値
#[derive(Debug, Clone)]
pub struct Confidence {
    pub accept_count: u32,
    pub reject_count: u32,
}

impl Confidence {
    pub fn new() -> Self {
        Self {
            accept_count: 0,
            reject_count: 0,
        }
    }

    pub fn add_vote(&mut self, vote: Vote) {
        match vote {
            Vote::Accept => self.accept_count += 1,
            Vote::Reject => self.reject_count += 1,
        }
    }

    pub fn get_confidence(&self) -> f64 {
        let total = self.accept_count + self.reject_count;
        if total == 0 {
            return 0.0;
        }
        self.accept_count as f64 / total as f64
    }
}

/// Avalancheパラメータ
#[derive(Debug, Clone)]
pub struct AvalancheParams {
    pub sample_size: usize,
    pub threshold: f64,
    pub max_rounds: u32,
}

impl Default for AvalancheParams {
    fn default() -> Self {
        Self {
            sample_size: 20,
            threshold: 0.8,
            max_rounds: 10,
        }
    }
}

/// Avalancheコンセンサスエンジン
pub struct AvalancheEngine {
    params: AvalancheParams,
    confidence: Arc<RwLock<HashMap<TxId, Confidence>>>,
    peers: Arc<RwLock<Vec<String>>>,
    network: Arc<RwLock<crate::network::P2PNetwork>>,
    vote_sender: mpsc::UnboundedSender<Vote>,
    vote_receiver: mpsc::UnboundedReceiver<Vote>,
}

impl AvalancheEngine {
    pub fn new(params: AvalancheParams, network: Arc<RwLock<crate::network::P2PNetwork>>) -> Self {
        let (vote_sender, vote_receiver) = mpsc::unbounded_channel();
        Self {
            params,
            confidence: Arc::new(RwLock::new(HashMap::new())),
            peers: Arc::new(RwLock::new(Vec::new())),
            network,
            vote_sender,
            vote_receiver,
        }
    }

    /// ピアリストを更新
    pub async fn update_peers(&self, peers: Vec<String>) {
        let mut peer_list = self.peers.write().await;
        *peer_list = peers;
    }

    /// サンプリングベースの投票を実行
    pub async fn run_consensus(&mut self, tx: &Transaction) -> anyhow::Result<TxStatus> {
        let mut current_confidence = Confidence::new();
        let mut rng = rand::thread_rng();

        for _ in 0..self.params.max_rounds {
            // ランダムなピアをサンプリング
            let peers = self.peers.read().await;
            let sample: Vec<_> = peers
                .choose_multiple(&mut rng, self.params.sample_size)
                .cloned()
                .collect();
            drop(peers); // 早めにロックを解放

            // 各ピアから投票を収集
            for peer in sample {
                let vote = self.query_peer(&peer, tx).await?;
                current_confidence.add_vote(vote);
            }

            // コンフィデンスを確認
            let conf = current_confidence.get_confidence();
            if conf >= self.params.threshold {
                return Ok(TxStatus::Accepted);
            } else if (1.0 - conf) >= self.params.threshold {
                return Ok(TxStatus::Rejected);
            }
        }

        // 最大ラウンド数に達しても決定できない場合
        Ok(TxStatus::Conflicting)
    }

    /// ピアに投票をリクエスト
    async fn query_peer(&mut self, peer_id: &str, tx: &Transaction) -> anyhow::Result<Vote> {
        use crate::network::NetworkMessage;
        
        // ネットワークメッセージを作成
        let query = NetworkMessage::QueryTransaction {
            tx_id: tx.id.clone(),
        };
        
        // メッセージを送信
        let network = self.network.read().await;
        let sender = network.message_sender();
        drop(network); // 早めにロックを解放
        sender.send(query)?;
        
        // 応答を待機
        let response = self.wait_for_vote(tx.id.clone()).await?;
        Ok(response)
    }
    
    /// 投票応答を待機
    async fn wait_for_vote(&mut self, tx_id: TxId) -> anyhow::Result<Vote> {
        use tokio::time::timeout;
        
        // タイムアウト付きで応答を待機
        match timeout(Duration::from_secs(5), self.vote_receiver.recv()).await {
            Ok(Some(vote)) => Ok(vote),
            Ok(None) => anyhow::bail!("Vote channel closed"),
            Err(_) => anyhow::bail!("Vote request timed out"),
        }
    }

    /// トランザクションの検証
    pub async fn validate_transaction(&self, tx: &Transaction) -> anyhow::Result<bool> {
        // TODO: トランザクションの検証ロジックを実装
        Ok(true)
    }

    /// メタスタビリティの検出と解決
    pub async fn resolve_metastability(&self, tx: &Transaction) -> anyhow::Result<TxStatus> {
        // TODO: メタスタビリティ解決ロジックを実装
        Ok(TxStatus::Accepted)
    }
}