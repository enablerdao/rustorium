mod validation;
mod voting;

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, RwLock};
use rand::seq::SliceRandom;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use super::dag::{TxId, Transaction, TxStatus};
use crate::network::{NetworkMessage, P2PNetwork};

pub use voting::VoteStats;

/// 投票結果
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Vote {
    /// 承認
    Accept,
    /// 拒否
    Reject,
}

/// コンフィデンス値
#[derive(Debug, Clone)]
pub struct Confidence {
    /// 承認票数
    pub accept_count: u32,
    /// 拒否票数
    pub reject_count: u32,
}

impl Confidence {
    /// 新しいコンフィデンス値を作成
    pub fn new() -> Self {
        Self {
            accept_count: 0,
            reject_count: 0,
        }
    }

    /// 投票を追加
    pub fn add_vote(&mut self, vote: Vote) {
        match vote {
            Vote::Accept => self.accept_count += 1,
            Vote::Reject => self.reject_count += 1,
        }
    }

    /// コンフィデンス値を計算
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
    /// サンプリングサイズ
    pub sample_size: usize,
    /// 閾値
    pub threshold: f64,
    /// 最大ラウンド数
    pub max_rounds: u32,
    /// 投票タイムアウト
    pub vote_timeout: Duration,
}

impl Default for AvalancheParams {
    fn default() -> Self {
        Self {
            sample_size: 20,
            threshold: 0.8,
            max_rounds: 10,
            vote_timeout: Duration::from_secs(5),
        }
    }
}

/// Avalancheコンセンサスエンジン
#[derive(Clone)]
pub struct AvalancheEngine {
    /// パラメータ
    params: AvalancheParams,
    /// コンフィデンス値
    confidence: Arc<RwLock<HashMap<TxId, Confidence>>>,
    /// ピアリスト
    peers: Arc<RwLock<Vec<String>>>,
    /// P2Pネットワーク
    network: Arc<P2PNetwork>,
    /// 投票送信チャネル
    vote_sender: mpsc::UnboundedSender<Vote>,
    /// 投票受信チャネル
    vote_receiver: Arc<tokio::sync::Mutex<mpsc::UnboundedReceiver<Vote>>>,
}

impl AvalancheEngine {
    /// 新しいAvalancheエンジンを作成
    pub fn new(params: AvalancheParams, network: Arc<P2PNetwork>) -> Self {
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

    /// コンセンサスを実行
    pub async fn run_consensus(&mut self, tx: &Transaction) -> Result<TxStatus> {
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
                return Ok(TxStatus::Confirmed);
            } else if (1.0 - conf) >= self.params.threshold {
                return Ok(TxStatus::Rejected);
            }
        }

        // 最大ラウンド数に達しても決定できない場合
        Ok(TxStatus::Conflicting)
    }

    /// ピアに投票をリクエスト
    async fn query_peer(&mut self, peer_id: &str, tx: &Transaction) -> Result<Vote> {
        // ネットワークメッセージを作成
        let query = NetworkMessage::QueryTransaction {
            tx_id: tx.id.clone(),
        };
        
        // メッセージを送信
        let network = &*self.network;
        let sender = network.message_sender();
        drop(network); // 早めにロックを解放
        sender.send(query)?;
        
        // 応答を待機
        let response = self.wait_for_vote(tx.id.clone()).await?;
        Ok(response)
    }

    /// 投票応答を待機
    async fn wait_for_vote(&mut self, _tx_id: TxId) -> Result<Vote> {
        use tokio::time::timeout;
        
        // タイムアウト付きで応答を待機
        match timeout(self.params.vote_timeout, self.vote_receiver.recv()).await {
            Ok(Some(vote)) => Ok(vote),
            Ok(None) => anyhow::bail!("Vote channel closed"),
            Err(_) => anyhow::bail!("Vote request timed out"),
        }
    }

    /// トランザクションを検証
    pub async fn validate_transaction(&self, tx: &Transaction) -> Result<bool> {
        // 検証ロジックを実装
        let validator = validation::TransactionValidator::new(self.clone());
        validator.validate_transaction(tx).await
    }

    /// メタスタビリティの検出と解決
    pub async fn resolve_metastability(&self, tx: &Transaction) -> Result<TxStatus> {
        // メタスタビリティ解決ロジックを実装
        let validator = validation::TransactionValidator::new(self.clone());
        validator.resolve_metastability(tx).await
    }
}