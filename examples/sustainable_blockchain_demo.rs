// 持続可能なブロックチェーン機能のデモ
// 持続可能なコンセンサスメカニズム、動的報酬システム、リソース使用効率モニタリング、適応型スケーリングの基盤設計

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// 既存のブロックチェーン構造体をインポート
mod blockchain_adapter {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    
    // 簡略化したブロック構造体
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Block {
        pub index: u64,
        pub timestamp: DateTime<Utc>,
        pub transactions: Vec<Transaction>,
        pub previous_hash: String,
        pub hash: String,
        pub validator: String,
    }
    
    // 簡略化したトランザクション構造体
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Transaction {
        pub id: String,
        pub sender: String,
        pub recipient: String,
        pub amount: f64,
    }
    
    impl Block {
        pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String, validator: String) -> Self {
            Block {
                index,
                timestamp: Utc::now(),
                transactions,
                previous_hash,
                hash: "dummy_hash".to_string(),
                validator,
            }
        }
    }
    
    impl Transaction {
        pub fn new(id: String, sender: String, recipient: String, amount: f64) -> Self {
            Transaction {
                id,
                sender,
                recipient,
                amount,
            }
        }
    }
}

// 持続可能なコンセンサスメカニズム
mod consensus {
    use super::blockchain_adapter::{Block, Transaction};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};
    use rand::SeedableRng;
    
    // バリデーター
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Validator {
        pub address: String,
        pub stake: f64,
        pub public_key: Vec<u8>,
        pub last_active: u64,
        pub performance: f64,
    }
    
    impl Validator {
        pub fn new(address: String, stake: f64, public_key: Vec<u8>) -> Self {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            Self {
                address,
                stake,
                public_key,
                last_active: now,
                performance: 1.0,
            }
        }
    }
    
    // コンセンサスタイプ
    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
    pub enum ConsensusType {
        ProofOfStake,
        DelegatedProofOfStake,
        ProofOfAuthority,
    }
    
    // コンセンサス設定
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct ConsensusConfig {
        pub consensus_type: ConsensusType,
        pub block_time: u64,
        pub min_stake: f64,
        pub max_validators: usize,
        pub base_reward: f64,
        pub resource_efficiency_factor: f64,
        pub node_scaling_factor: f64,
    }
    
    impl Default for ConsensusConfig {
        fn default() -> Self {
            Self {
                consensus_type: ConsensusType::ProofOfStake,
                block_time: 5,
                min_stake: 100.0,
                max_validators: 100,
                base_reward: 5.0,
                resource_efficiency_factor: 0.8,
                node_scaling_factor: 0.95,
            }
        }
    }
    
    // コンセンサスステータス
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct ConsensusStatus {
        pub validator_count: usize,
        pub total_stake: f64,
        pub current_proposer: Option<String>,
        pub next_block_time: Option<String>,
        pub last_block_time_ms: u64,
        pub participation_rate: f64,
        pub resource_efficiency: f64,
        pub current_reward_rate: f64,
    }
    
    // 動的報酬システム
    #[derive(Clone)]
    pub struct DynamicRewardSystem {
        base_reward: f64,
        node_scaling_factor: f64,
        optimal_node_count: usize,
    }
    
    impl DynamicRewardSystem {
        pub fn new(base_reward: f64, node_scaling_factor: f64, optimal_node_count: usize) -> Self {
            Self {
                base_reward,
                node_scaling_factor,
                optimal_node_count,
            }
        }
        
        pub fn calculate_reward_rate(&self, node_count: usize) -> f64 {
            if node_count <= self.optimal_node_count {
                return 1.0;
            }
            
            let excess_nodes = node_count - self.optimal_node_count;
            self.node_scaling_factor.powi(excess_nodes as i32)
        }
    }
    
    // リソースモニター
    pub struct ResourceMonitor {
        efficiency_score: Arc<Mutex<f64>>,
    }
    
    impl ResourceMonitor {
        pub fn new() -> Self {
            Self {
                efficiency_score: Arc::new(Mutex::new(1.0)),
            }
        }
        
        pub fn update(&self) {
            // 実際の実装ではシステムリソースを監視
            // ここではランダムな効率性スコアを生成
            let efficiency = 0.7 + (rand::random::<f64>() * 0.3);
            let mut score = self.efficiency_score.lock().unwrap();
            *score = efficiency;
        }
        
        pub fn get_efficiency(&self) -> f64 {
            let score = self.efficiency_score.lock().unwrap();
            *score
        }
    }
    
    // Proof of Stakeコンセンサスアルゴリズム
    pub struct ProofOfStake {
        config: ConsensusConfig,
        validators: Arc<Mutex<HashMap<String, Validator>>>,
    }
    
    impl ProofOfStake {
        pub fn new(config: &ConsensusConfig) -> Self {
            Self {
                config: config.clone(),
                validators: Arc::new(Mutex::new(HashMap::new())),
            }
        }
        
        pub fn add_validator(&self, validator: Validator) {
            let mut validators = self.validators.lock().unwrap();
            validators.insert(validator.address.clone(), validator);
        }
        
        pub fn select_validator(&self) -> Option<Validator> {
            let validators = self.validators.lock().unwrap();
            
            if validators.is_empty() {
                return None;
            }
            
            // 総ステーク量を計算
            let total_stake: f64 = validators.values().map(|v| v.stake).sum();
            
            if total_stake <= 0.0 {
                return None;
            }
            
            // 現在の時間をシードとして使用
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis();
            let seed = now as u64;
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
            
            // ランダムな値を生成（0.0 <= r < total_stake）
            let r = rand::random::<f64>() * total_stake;
            
            // ステーク量に比例した確率で選出
            let mut cumulative = 0.0;
            for validator in validators.values() {
                cumulative += validator.stake;
                if r < cumulative {
                    return Some(validator.clone());
                }
            }
            
            // 念のため、最後のバリデーターを返す
            validators.values().last().cloned()
        }
        
        pub fn create_block(&self, transactions: Vec<Transaction>) -> Block {
            // バリデーターを選出
            let validator = self.select_validator().unwrap_or(Validator {
                address: "system".to_string(),
                stake: 0.0,
                public_key: vec![],
                last_active: 0,
                performance: 1.0,
            });
            
            // ブロックの作成
            Block::new(
                0, // 実際の実装ではブロック高を設定
                transactions,
                "previous_hash".to_string(), // 実際の実装では前のブロックのハッシュを設定
                validator.address,
            )
        }
    }
    
    // コンセンサスマネージャー
    pub struct ConsensusManager {
        algorithm: Box<ProofOfStake>,
        config: ConsensusConfig,
        validators: Arc<Mutex<HashMap<String, Validator>>>,
        reward_system: DynamicRewardSystem,
        resource_monitor: ResourceMonitor,
        status: Arc<Mutex<ConsensusStatus>>,
    }
    
    impl ConsensusManager {
        pub fn new(config: ConsensusConfig) -> Self {
            let algorithm = Box::new(ProofOfStake::new(&config));
            
            let validators = Arc::new(Mutex::new(HashMap::new()));
            let reward_system = DynamicRewardSystem::new(
                config.base_reward,
                config.node_scaling_factor,
                config.max_validators,
            );
            let resource_monitor = ResourceMonitor::new();
            
            let status = Arc::new(Mutex::new(ConsensusStatus {
                validator_count: 0,
                total_stake: 0.0,
                current_proposer: None,
                next_block_time: None,
                last_block_time_ms: 0,
                participation_rate: 0.0,
                resource_efficiency: 0.0,
                current_reward_rate: config.base_reward,
            }));
            
            Self {
                algorithm,
                config,
                validators,
                reward_system,
                resource_monitor,
                status,
            }
        }
        
        pub fn register_validator(&self, validator: Validator) -> Result<(), String> {
            if validator.stake < self.config.min_stake {
                return Err(format!("Stake amount {} is less than minimum required {}", 
                    validator.stake, self.config.min_stake));
            }
            
            let mut validators = self.validators.lock().unwrap();
            
            // 最大バリデーター数のチェック
            if validators.len() >= self.config.max_validators && !validators.contains_key(&validator.address) {
                return Err(format!("Maximum validator count {} reached", self.config.max_validators));
            }
            
            validators.insert(validator.address.clone(), validator.clone());
            
            // ステータスの更新
            let mut status = self.status.lock().unwrap();
            status.validator_count = validators.len();
            status.total_stake = validators.values().map(|v| v.stake).sum();
            
            // 報酬レートの更新
            status.current_reward_rate = self.reward_system.calculate_reward_rate(validators.len());
            
            Ok(())
        }
        
        pub fn get_status(&self) -> ConsensusStatus {
            self.status.lock().unwrap().clone()
        }
        
        pub fn get_validators(&self) -> Vec<Validator> {
            let validators = self.validators.lock().unwrap();
            validators.values().cloned().collect()
        }
        
        pub fn update_resource_efficiency(&self) {
            self.resource_monitor.update();
            let efficiency = self.resource_monitor.get_efficiency();
            
            let mut status = self.status.lock().unwrap();
            status.resource_efficiency = efficiency;
        }
        
        pub fn create_block(&self, transactions: Vec<Transaction>) -> Block {
            self.algorithm.create_block(transactions)
        }
    }
}

// 適応型スケーリングの基盤設計
mod scaling {
    use super::blockchain_adapter::Block;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    
    // シャード
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Shard {
        pub id: usize,
        pub name: String,
        pub node_count: usize,
        pub active_transactions: usize,
        pub total_transactions: usize,
        pub active: bool,
    }
    
    // スケーリングモード
    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
    pub enum ScalingMode {
        Automatic,
        Manual,
        Hybrid,
    }
    
    // スケーリング設定
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct ScalingConfig {
        pub mode: ScalingMode,
        pub min_shards: usize,
        pub max_shards: usize,
        pub optimal_tx_per_node: usize,
        pub scale_up_threshold: f64,
        pub scale_down_threshold: f64,
        pub scaling_interval: u64,
        pub enable_adaptive_config: bool,
    }
    
    impl Default for ScalingConfig {
        fn default() -> Self {
            Self {
                mode: ScalingMode::Automatic,
                min_shards: 1,
                max_shards: 16,
                optimal_tx_per_node: 1000,
                scale_up_threshold: 0.8,
                scale_down_threshold: 0.3,
                scaling_interval: 300, // 5分
                enable_adaptive_config: true,
            }
        }
    }
    
    // スケーリングステータス
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct ScalingStatus {
        pub current_shards: usize,
        pub current_nodes: usize,
        pub cpu_usage: f64,
        pub memory_usage: f64,
        pub tps: f64,
        pub mode: ScalingMode,
        pub last_scaling: String,
        pub next_scaling: String,
        pub scaling_recommendation: String,
    }
    
    // シャードマネージャー
    pub struct ShardManager {
        shards: Arc<Mutex<Vec<Shard>>>,
    }
    
    impl ShardManager {
        pub fn new(initial_shards: usize) -> Self {
            let mut shards = Vec::with_capacity(initial_shards);
            
            // 初期シャードを作成
            for i in 0..initial_shards {
                shards.push(Shard {
                    id: i,
                    name: format!("shard-{}", i),
                    node_count: 0,
                    active_transactions: 0,
                    total_transactions: 0,
                    active: true,
                });
            }
            
            Self {
                shards: Arc::new(Mutex::new(shards)),
            }
        }
        
        pub fn set_shard_count(&self, count: usize) -> Result<(), String> {
            let mut shards = self.shards.lock().unwrap();
            let current_count = shards.len();
            
            if count == current_count {
                return Ok(());
            }
            
            if count > current_count {
                // シャードを追加
                for i in current_count..count {
                    shards.push(Shard {
                        id: i,
                        name: format!("shard-{}", i),
                        node_count: 0,
                        active_transactions: 0,
                        total_transactions: 0,
                        active: true,
                    });
                }
            } else {
                // シャードを削除
                shards.truncate(count);
            }
            
            Ok(())
        }
        
        pub fn get_shards(&self) -> Vec<Shard> {
            let shards = self.shards.lock().unwrap();
            shards.clone()
        }
        
        pub fn get_shard_count(&self) -> usize {
            let shards = self.shards.lock().unwrap();
            shards.len()
        }
    }
    
    // スケーリングメトリクス
    pub struct ScalingMetrics {
        current_tps: Arc<Mutex<f64>>,
        current_node_count: Arc<Mutex<usize>>,
    }
    
    impl ScalingMetrics {
        pub fn new() -> Self {
            Self {
                current_tps: Arc::new(Mutex::new(0.0)),
                current_node_count: Arc::new(Mutex::new(0)),
            }
        }
        
        pub fn update(&self, tps: f64, node_count: usize) {
            let mut current_tps = self.current_tps.lock().unwrap();
            *current_tps = tps;
            
            let mut current_node_count = self.current_node_count.lock().unwrap();
            *current_node_count = node_count;
        }
        
        pub fn get_current_metrics(&self) -> (f64, usize) {
            let tps = *self.current_tps.lock().unwrap();
            let node_count = *self.current_node_count.lock().unwrap();
            (tps, node_count)
        }
    }
    
    // スケーリングマネージャー
    pub struct ScalingManager {
        config: ScalingConfig,
        shard_manager: ShardManager,
        metrics: ScalingMetrics,
        status: Arc<Mutex<ScalingStatus>>,
    }
    
    impl ScalingManager {
        pub fn new(config: ScalingConfig) -> Self {
            let shard_manager = ShardManager::new(config.min_shards);
            let metrics = ScalingMetrics::new();
            
            let now = chrono::Utc::now();
            let next_scaling = now + chrono::Duration::seconds(config.scaling_interval as i64);
            
            let status = ScalingStatus {
                current_shards: config.min_shards,
                current_nodes: 0,
                cpu_usage: 0.0,
                memory_usage: 0.0,
                tps: 0.0,
                mode: config.mode.clone(),
                last_scaling: now.to_rfc3339(),
                next_scaling: next_scaling.to_rfc3339(),
                scaling_recommendation: "No action needed".to_string(),
            };
            
            Self {
                config,
                shard_manager,
                metrics,
                status: Arc::new(Mutex::new(status)),
            }
        }
        
        pub fn set_shard_count(&self, count: usize) -> Result<(), String> {
            if count < self.config.min_shards || count > self.config.max_shards {
                return Err(format!("Shard count must be between {} and {}",
                    self.config.min_shards, self.config.max_shards));
            }
            
            self.shard_manager.set_shard_count(count)?;
            
            // ステータスの更新
            let mut status = self.status.lock().unwrap();
            status.current_shards = count;
            
            Ok(())
        }
        
        pub fn update_metrics(&self, tps: f64, node_count: usize) {
            self.metrics.update(tps, node_count);
            
            // ステータスの更新
            let (current_tps, current_nodes) = self.metrics.get_current_metrics();
            let mut status = self.status.lock().unwrap();
            status.tps = current_tps;
            status.current_nodes = current_nodes;
            
            // CPU使用率とメモリ使用率をシミュレート
            status.cpu_usage = 0.3 + (current_tps / 10000.0).min(0.6);
            status.memory_usage = 0.2 + (current_nodes as f64 / 100.0).min(0.7);
        }
        
        pub fn scale(&self) -> Result<(), String> {
            // 自動スケーリングが無効の場合は何もしない
            if self.config.mode == ScalingMode::Manual {
                return Ok(());
            }
            
            // 現在のステータスを取得
            let mut status = self.status.lock().unwrap();
            
            // スケーリング判断
            let current_shards = self.shard_manager.get_shard_count();
            let mut new_shards = current_shards;
            
            let mut recommendation = "No action needed".to_string();
            
            if status.cpu_usage > self.config.scale_up_threshold && current_shards < self.config.max_shards {
                // スケールアップ
                new_shards = current_shards + 1;
                recommendation = format!("Scale up from {} to {} shards due to high CPU usage ({})",
                    current_shards, new_shards, status.cpu_usage);
            } else if status.cpu_usage < self.config.scale_down_threshold && current_shards > self.config.min_shards {
                // スケールダウン
                new_shards = current_shards - 1;
                recommendation = format!("Scale down from {} to {} shards due to low CPU usage ({})",
                    current_shards, new_shards, status.cpu_usage);
            }
            
            // シャード数を更新
            if new_shards != current_shards {
                self.shard_manager.set_shard_count(new_shards)?;
                status.current_shards = new_shards;
            }
            
            // ステータスの更新
            let now = chrono::Utc::now();
            let next_scaling = now + chrono::Duration::seconds(self.config.scaling_interval as i64);
            
            status.last_scaling = now.to_rfc3339();
            status.next_scaling = next_scaling.to_rfc3339();
            status.scaling_recommendation = recommendation;
            
            Ok(())
        }
        
        pub fn get_status(&self) -> ScalingStatus {
            self.status.lock().unwrap().clone()
        }
    }
}

use blockchain_adapter::{Block, Transaction};
use consensus::{ConsensusConfig, ConsensusManager, Validator};
use scaling::{ScalingConfig, ScalingManager};

fn main() {
    println!("持続可能なブロックチェーン機能のデモを開始します");
    
    // コンセンサスマネージャーの初期化
    let consensus_config = ConsensusConfig::default();
    let consensus_manager = ConsensusManager::new(consensus_config);
    
    // バリデーターの登録
    println!("\n=== バリデーターの登録 ===");
    for i in 1..=5 {
        let stake = 100.0 * i as f64;
        let validator = Validator::new(
            format!("validator{}", i),
            stake,
            vec![i as u8, (i+1) as u8, (i+2) as u8, (i+3) as u8],
        );
        
        match consensus_manager.register_validator(validator) {
            Ok(_) => println!("バリデーター {} を登録しました（ステーク量: {}）", i, stake),
            Err(e) => println!("バリデーター {} の登録に失敗しました: {}", i, e),
        }
    }
    
    // コンセンサスステータスの表示
    let status = consensus_manager.get_status();
    println!("\n=== コンセンサスステータス ===");
    println!("バリデーター数: {}", status.validator_count);
    println!("総ステーク量: {}", status.total_stake);
    println!("現在の報酬レート: {}", status.current_reward_rate);
    
    // リソース効率の更新
    consensus_manager.update_resource_efficiency();
    let updated_status = consensus_manager.get_status();
    println!("リソース効率: {:.2}", updated_status.resource_efficiency);
    
    // ブロック生成のシミュレーション
    println!("\n=== ブロック生成 ===");
    let transactions = vec![
        Transaction::new("tx1".to_string(), "user1".to_string(), "user2".to_string(), 10.0),
        Transaction::new("tx2".to_string(), "user3".to_string(), "user4".to_string(), 20.0),
        Transaction::new("tx3".to_string(), "user5".to_string(), "user6".to_string(), 30.0),
    ];
    
    let block = consensus_manager.create_block(transactions);
    println!("ブロックを生成しました: {:?}", block);
    println!("ブロック生成者: {}", block.validator);
    
    // スケーリングマネージャーの初期化
    let scaling_config = ScalingConfig::default();
    let scaling_manager = ScalingManager::new(scaling_config);
    
    // スケーリングステータスの表示
    let scaling_status = scaling_manager.get_status();
    println!("\n=== スケーリングステータス（初期） ===");
    println!("現在のシャード数: {}", scaling_status.current_shards);
    println!("現在のノード数: {}", scaling_status.current_nodes);
    println!("CPU使用率: {:.2}", scaling_status.cpu_usage);
    println!("メモリ使用率: {:.2}", scaling_status.memory_usage);
    println!("TPS: {:.2}", scaling_status.tps);
    
    // 負荷のシミュレーション
    println!("\n=== 負荷シミュレーション ===");
    for i in 1..=5 {
        let tps = 1000.0 * i as f64;
        let node_count = 10 * i;
        
        println!("ステップ {}: TPS = {:.2}, ノード数 = {}", i, tps, node_count);
        scaling_manager.update_metrics(tps, node_count);
        
        // スケーリングの実行
        match scaling_manager.scale() {
            Ok(_) => {
                let status = scaling_manager.get_status();
                println!("  シャード数: {} -> {}", scaling_status.current_shards, status.current_shards);
                println!("  CPU使用率: {:.2}", status.cpu_usage);
                println!("  推奨: {}", status.scaling_recommendation);
            },
            Err(e) => println!("  スケーリングエラー: {}", e),
        }
        
        // 少し待機
        thread::sleep(Duration::from_millis(500));
    }
    
    // 最終的なスケーリングステータスの表示
    let final_status = scaling_manager.get_status();
    println!("\n=== スケーリングステータス（最終） ===");
    println!("現在のシャード数: {}", final_status.current_shards);
    println!("現在のノード数: {}", final_status.current_nodes);
    println!("CPU使用率: {:.2}", final_status.cpu_usage);
    println!("メモリ使用率: {:.2}", final_status.memory_usage);
    println!("TPS: {:.2}", final_status.tps);
    println!("最後のスケーリング: {}", final_status.last_scaling);
    println!("次のスケーリング予定: {}", final_status.next_scaling);
    
    println!("\n持続可能なブロックチェーン機能のデモを終了します");
}