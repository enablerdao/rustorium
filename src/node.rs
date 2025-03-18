use crate::ai::detector::AnomalyDetector;
use crate::ai::predictor::Predictor;
use crate::api::server as api_server;
use crate::common::config::Config;
use crate::common::errors::LedgerError;
use crate::common::types::{Block, NodeId, Transaction, TransactionId};
use crate::consensus::avalanche::{AvalancheConsensus, AvalancheParams};
use crate::consensus::validator::Validator;
use crate::dag::executor::DagExecutor;
use crate::dag::graph::{new_shared_dag, SharedDag};
use crate::gossip::network::NetworkService;
use crate::gossip::peer::PeerManager;
use crate::sharding::manager::{start_shard_manager, ShardManagerMessage};
use crate::sharding::rebalancer::start_rebalancer;
use crate::storage::cache::StorageCache;
use crate::storage::db::Database;
use crate::storage::state::StateManager;
use crate::vm::executor::VmExecutor;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, error, info, warn};

/// ノード
pub struct Node {
    /// ノードID
    node_id: NodeId,
    /// 設定
    config: Config,
    /// データベース
    db: Option<Arc<Database>>,
    /// キャッシュ
    cache: Option<Arc<StorageCache>>,
    /// 状態マネージャー
    state_manager: Option<Arc<StateManager>>,
    /// シャードマネージャー
    shard_manager: Option<mpsc::Sender<ShardManagerMessage>>,
    /// ピアマネージャー
    peer_manager: Option<Arc<PeerManager>>,
    /// ネットワークサービス
    network: Option<Arc<RwLock<NetworkService>>>,
    /// VM実行エンジン
    vm_executor: Option<Arc<VmExecutor>>,
    /// DAG
    dag: Option<SharedDag>,
    /// DAG実行エンジン
    dag_executor: Option<Arc<RwLock<DagExecutor>>>,
    /// Avalancheコンセンサス
    avalanche: Option<Arc<AvalancheConsensus<Transaction>>>,
    /// バリデータ
    validator: Option<Arc<RwLock<Validator>>>,
    /// 異常検出器
    anomaly_detector: Option<Arc<AnomalyDetector>>,
    /// 予測器
    predictor: Option<Arc<Predictor>>,
    /// 実行中フラグ
    running: bool,
}

impl Node {
    /// 新しいノードを作成
    pub fn new(config: Config) -> Self {
        let node_id = NodeId(config.node.id.clone());
        
        Self {
            node_id,
            config,
            db: None,
            cache: None,
            state_manager: None,
            shard_manager: None,
            peer_manager: None,
            network: None,
            vm_executor: None,
            dag: None,
            dag_executor: None,
            avalanche: None,
            validator: None,
            anomaly_detector: None,
            predictor: None,
            running: false,
        }
    }
    
    /// ノードを初期化
    pub async fn init(&mut self) -> Result<(), LedgerError> {
        info!("Initializing node {}", self.node_id);
        
        // データベースを初期化
        let db_path = Path::new(&self.config.storage.db_path);
        let db = Database::open(db_path)?;
        self.db = Some(Arc::new(db));
        
        // キャッシュを初期化
        let cache = StorageCache::new(&self.config.storage);
        self.cache = Some(Arc::new(cache));
        
        // 状態マネージャーを初期化
        let state_manager = StateManager::new(
            self.db.as_ref().unwrap().clone(),
            self.cache.as_ref().unwrap().clone(),
        )?;
        self.state_manager = Some(Arc::new(state_manager));
        
        // シャードマネージャーを初期化
        let shard_manager = start_shard_manager(self.config.sharding.shard_count).await;
        self.shard_manager = Some(shard_manager.clone());
        
        // シャードリバランサーを初期化
        start_rebalancer(shard_manager, &self.config.sharding).await?;
        
        // ピアマネージャーを初期化
        let peer_manager = PeerManager::new(
            self.node_id.clone(),
            self.config.network.max_peers,
        );
        self.peer_manager = Some(Arc::new(peer_manager));
        
        // VM実行エンジンを初期化
        let vm_executor = VmExecutor::new(self.state_manager.as_ref().unwrap().clone());
        self.vm_executor = Some(Arc::new(vm_executor));
        
        // DAGを初期化
        let dag = new_shared_dag();
        self.dag = Some(dag.clone());
        
        // DAG実行エンジンを初期化
        let dag_executor = DagExecutor::new(
            dag,
            self.vm_executor.as_ref().unwrap().clone(),
            4, // 並列実行数
        );
        self.dag_executor = Some(Arc::new(RwLock::new(dag_executor)));
        
        // 異常検出器を初期化
        let anomaly_detector = AnomalyDetector::new(0.8);
        self.anomaly_detector = Some(Arc::new(anomaly_detector));
        
        // 予測器を初期化
        let predictor = Predictor::new(100);
        self.predictor = Some(Arc::new(predictor));
        
        // バリデータを初期化
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        
        let validator = Validator::new(
            self.node_id.0.as_bytes()[0..20].try_into().unwrap(),
            signing_key,
            self.state_manager.as_ref().unwrap().clone(),
            self.config.consensus.block_time_ms,
        );
        self.validator = Some(Arc::new(RwLock::new(validator)));
        
        info!("Node initialization completed");
        
        Ok(())
    }
    
    /// ノードを開始
    pub async fn start(&mut self) -> Result<(), LedgerError> {
        if self.running {
            return Err(LedgerError::InvalidState(
                "Node is already running".to_string(),
            ));
        }
        
        info!("Starting node {}", self.node_id);
        
        // 初期化されていない場合は初期化
        if self.state_manager.is_none() {
            self.init().await?;
        }
        
        self.running = true;
        
        // APIサーバーを開始
        if self.config.api.enabled {
            let state_manager = self.state_manager.as_ref().unwrap().clone();
            tokio::spawn(async move {
                if let Err(e) = api_server::start_with_state(
                    self.config.api.listen_port,
                    state_manager,
                )
                .await
                {
                    error!("API server error: {}", e);
                }
            });
        }
        
        // TODO: ネットワークサービスを開始
        
        // TODO: Avalancheコンセンサスを開始
        
        // TODO: バリデータを開始
        
        info!("Node started successfully");
        
        Ok(())
    }
    
    /// トランザクションを送信
    pub async fn send_transaction(&self, transaction: Transaction) -> Result<(), LedgerError> {
        if !self.running {
            return Err(LedgerError::InvalidState(
                "Node is not running".to_string(),
            ));
        }
        
        // 異常検出
        if let Some(detector) = &self.anomaly_detector {
            match detector.detect_anomaly(&transaction).await? {
                crate::ai::detector::AnomalyResult::Anomalous(score, reason) => {
                    warn!(
                        "Transaction {} flagged as anomalous (score: {:.4}): {}",
                        transaction.id, score, reason
                    );
                    // 異常なトランザクションは拒否するか、特別な処理を行う
                }
                _ => {}
            }
        }
        
        // シャードマネージャーにトランザクションを追加
        if let Some(shard_manager) = &self.shard_manager {
            shard_manager
                .send(ShardManagerMessage::AddTransaction(transaction.clone()))
                .await
                .map_err(|e| {
                    LedgerError::Internal(format!("Failed to send transaction to shard manager: {}", e))
                })?;
        }
        
        // DAGに追加
        if let Some(dag) = &self.dag {
            let mut dag = dag.write().await;
            dag.add_transaction(transaction.clone())?;
        }
        
        // バリデータに追加
        if let Some(validator) = &self.validator {
            let validator = validator.read().await;
            validator.add_transaction(transaction.clone()).await?;
        }
        
        // ネットワークに伝播
        // TODO: ネットワーク伝播を実装
        
        info!("Transaction {} sent successfully", transaction.id);
        
        Ok(())
    }
    
    /// ブロックを受信
    pub async fn receive_block(&self, block: Block) -> Result<(), LedgerError> {
        if !self.running {
            return Err(LedgerError::InvalidState(
                "Node is not running".to_string(),
            ));
        }
        
        // ブロックを検証
        if let Some(validator) = &self.validator {
            let validator = validator.read().await;
            validator.verify_block(&block).await?;
        }
        
        // ブロックを適用
        if let Some(state_manager) = &self.state_manager {
            state_manager.apply_block(&block).await?;
        }
        
        // 予測器に追加
        if let Some(predictor) = &self.predictor {
            predictor.add_block(&block).await?;
        }
        
        // ネットワークに伝播
        // TODO: ネットワーク伝播を実装
        
        info!("Block at height {} received and applied", block.header.height);
        
        Ok(())
    }
    
    /// ノードを停止
    pub async fn stop(&mut self) -> Result<(), LedgerError> {
        if !self.running {
            return Ok(());
        }
        
        info!("Stopping node {}", self.node_id);
        
        // TODO: 各コンポーネントを停止
        
        self.running = false;
        
        info!("Node stopped");
        
        Ok(())
    }
}