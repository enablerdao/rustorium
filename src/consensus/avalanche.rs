use crate::common::errors::{ConsensusError, LedgerError};
use crate::common::types::{Block, NodeId, Transaction, TransactionId};
use crate::common::utils;
use crate::gossip::message::{GossipMessage, QueryType, ResponseType};
use crate::gossip::peer::PeerManager;
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, oneshot, RwLock};
use tokio::time;
use tracing::{debug, error, info, warn};

/// Avalancheコンセンサスパラメータ
pub struct AvalancheParams {
    /// クエリするピア数
    pub k: usize,
    /// 信頼度閾値
    pub alpha: usize,
    /// ラウンド数
    pub beta: usize,
    /// クエリタイムアウト（ミリ秒）
    pub query_timeout_ms: u64,
}

impl Default for AvalancheParams {
    fn default() -> Self {
        Self {
            k: 10,
            alpha: 7, // 70% of k
            beta: 5,
            query_timeout_ms: 1000,
        }
    }
}

/// Avalancheコンセンサス状態
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AvalancheState {
    /// 未確定
    Pending,
    /// 確定
    Accepted,
    /// 拒否
    Rejected,
}

/// Avalancheアイテム
struct AvalancheItem<T> {
    /// アイテムデータ
    data: T,
    /// 状態
    state: AvalancheState,
    /// 信頼度カウンター
    confidence: usize,
    /// ラウンド数
    rounds: usize,
    /// 最終更新時間
    last_updated: Instant,
}

/// Avalancheコンセンサスエンジン
pub struct AvalancheConsensus<T> {
    /// ノードID
    node_id: NodeId,
    /// ピアマネージャー
    peer_manager: Arc<PeerManager>,
    /// コンセンサスパラメータ
    params: AvalancheParams,
    /// アイテム状態
    items: Arc<DashMap<TransactionId, AvalancheItem<T>>>,
    /// メッセージ送信チャネル
    message_tx: mpsc::Sender<(NodeId, GossipMessage)>,
    /// 実行中フラグ
    running: bool,
    /// 検証コールバック
    validate_callback: Option<Arc<dyn Fn(&T) -> bool + Send + Sync>>,
}

impl<T: Clone + Send + Sync + 'static> AvalancheConsensus<T> {
    /// 新しいAvalancheコンセンサスエンジンを作成
    pub fn new(
        node_id: NodeId,
        peer_manager: Arc<PeerManager>,
        params: AvalancheParams,
        message_tx: mpsc::Sender<(NodeId, GossipMessage)>,
    ) -> Self {
        Self {
            node_id,
            peer_manager,
            params,
            items: Arc::new(DashMap::new()),
            message_tx,
            running: false,
            validate_callback: None,
        }
    }
    
    /// 検証コールバックを設定
    pub fn set_validate_callback<F>(&mut self, callback: F)
    where
        F: Fn(&T) -> bool + Send + Sync + 'static,
    {
        self.validate_callback = Some(Arc::new(callback));
    }
    
    /// コンセンサスエンジンを開始
    pub async fn start(&mut self) -> Result<(), LedgerError> {
        if self.running {
            return Err(LedgerError::InvalidState(
                "Avalanche consensus is already running".to_string(),
            ));
        }
        
        self.running = true;
        info!("Avalanche consensus started");
        
        // クリーンアップタスクを開始
        self.start_cleanup_task().await;
        
        // コンセンサスループを開始
        self.consensus_loop().await?;
        
        self.running = false;
        info!("Avalanche consensus stopped");
        
        Ok(())
    }
    
    /// コンセンサスループ
    async fn consensus_loop(&self) -> Result<(), LedgerError> {
        let mut interval = time::interval(Duration::from_millis(100));
        
        while self.running {
            interval.tick().await;
            
            // 未確定アイテムを処理
            let pending_items: Vec<_> = self
                .items
                .iter()
                .filter(|item| item.state == AvalancheState::Pending)
                .map(|item| (item.key().clone(), item.data.clone()))
                .collect();
            
            for (tx_id, data) in pending_items {
                self.process_item(tx_id, data).await?;
            }
        }
        
        Ok(())
    }
    
    /// アイテムを処理
    async fn process_item(&self, tx_id: TransactionId, data: T) -> Result<(), LedgerError> {
        // ランダムなピアを選択
        let peers = self.peer_manager.get_random_peers(self.params.k);
        
        if peers.is_empty() {
            return Ok(());
        }
        
        let mut positive_responses = 0;
        
        // 各ピアにクエリを送信
        for peer in &peers {
            let query_type = QueryType::GetTransaction(format!("0x{}", hex::encode(tx_id.as_bytes())));
            let query = GossipMessage::new_query(query_type, self.node_id.clone());
            
            if let GossipMessage::Query(q) = &query {
                let (tx, rx) = oneshot::channel();
                
                // クエリを送信
                if let Err(e) = self.message_tx.send((peer.id.clone(), query)).await {
                    error!("Failed to send query: {}", e);
                    continue;
                }
                
                // レスポンスを待機
                match tokio::time::timeout(
                    Duration::from_millis(self.params.query_timeout_ms),
                    rx,
                )
                .await
                {
                    Ok(Ok(response)) => match response {
                        ResponseType::Transaction(Some(_)) => {
                            positive_responses += 1;
                        }
                        _ => {
                            // 否定的または無応答
                        }
                    },
                    Ok(Err(e)) => {
                        error!("Failed to receive response: {:?}", e);
                    }
                    Err(_) => {
                        // タイムアウト
                        debug!("Query timeout for transaction {}", tx_id);
                    }
                }
            }
        }
        
        // 信頼度を更新
        if let Some(mut item) = self.items.get_mut(&tx_id) {
            if positive_responses >= self.params.alpha {
                item.confidence += 1;
            } else {
                item.confidence = 0; // 否定的なラウンドでリセット
            }
            
            item.rounds += 1;
            item.last_updated = Instant::now();
            
            // 確定判定
            if item.confidence >= self.params.beta {
                item.state = AvalancheState::Accepted;
                info!("Transaction {} accepted with confidence {}", tx_id, item.confidence);
            } else if item.rounds >= self.params.beta * 2 && item.confidence < self.params.beta / 2 {
                item.state = AvalancheState::Rejected;
                info!("Transaction {} rejected after {} rounds", tx_id, item.rounds);
            }
        }
        
        Ok(())
    }
    
    /// アイテムを追加
    pub async fn add_item(&self, tx_id: TransactionId, data: T) -> Result<(), LedgerError> {
        // すでに存在するか確認
        if self.items.contains_key(&tx_id) {
            return Ok(());
        }
        
        // 検証
        if let Some(callback) = &self.validate_callback {
            if !callback(&data) {
                return Err(LedgerError::Consensus(ConsensusError::VerificationFailed(
                    "Item validation failed".to_string(),
                )));
            }
        }
        
        // アイテムを追加
        let item = AvalancheItem {
            data,
            state: AvalancheState::Pending,
            confidence: 0,
            rounds: 0,
            last_updated: Instant::now(),
        };
        
        self.items.insert(tx_id, item);
        debug!("Added transaction {} to Avalanche consensus", tx_id);
        
        Ok(())
    }
    
    /// アイテムの状態を取得
    pub fn get_item_state(&self, tx_id: &TransactionId) -> Option<AvalancheState> {
        self.items.get(tx_id).map(|item| item.state.clone())
    }
    
    /// クリーンアップタスクを開始
    async fn start_cleanup_task(&self) {
        let items = self.items.clone();
        
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(60));
            
            loop {
                interval.tick().await;
                
                let now = Instant::now();
                let timeout = Duration::from_secs(300); // 5分
                
                // 古いアイテムを削除
                let stale_items: Vec<_> = items
                    .iter()
                    .filter(|item| now.duration_since(item.last_updated) > timeout)
                    .map(|item| *item.key())
                    .collect();
                
                for tx_id in stale_items {
                    items.remove(&tx_id);
                    debug!("Removed stale transaction {} from Avalanche consensus", tx_id);
                }
            }
        });
    }
    
    /// コンセンサスエンジンを停止
    pub fn stop(&mut self) {
        self.running = false;
    }
}