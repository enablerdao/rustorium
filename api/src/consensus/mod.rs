// 持続可能なコンセンサスメカニズムモジュール
// リソース効率の良いProof of Stakeベースのコンセンサスアルゴリズム

mod pos;
mod rewards;
mod validator;
mod resource_monitor;

pub use pos::ProofOfStake;
pub use rewards::DynamicRewardSystem;
pub use validator::Validator;
pub use resource_monitor::ResourceMonitor;

use crate::blockchain::{Block, Transaction};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// コンセンサスアルゴリズムのトレイト
pub trait ConsensusAlgorithm: Send + Sync {
    /// 新しいブロックを生成する
    fn create_block(&self, transactions: Vec<Transaction>) -> Block;
    
    /// ブロックを検証する
    fn validate_block(&self, block: &Block) -> bool;
    
    /// バリデーターを選出する
    fn select_validator(&self) -> Option<Validator>;
    
    /// 報酬を計算する
    fn calculate_reward(&self, block: &Block) -> f64;
}

/// コンセンサス設定
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsensusConfig {
    /// コンセンサスタイプ
    pub consensus_type: ConsensusType,
    
    /// ブロック生成間隔（秒）
    pub block_time: u64,
    
    /// 最小ステーク量
    pub min_stake: f64,
    
    /// 最大バリデーター数
    pub max_validators: usize,
    
    /// 報酬基本量
    pub base_reward: f64,
    
    /// リソース効率係数
    pub resource_efficiency_factor: f64,
    
    /// ノード数に応じた報酬減少係数
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

/// コンセンサスタイプ
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ConsensusType {
    /// Proof of Stake
    ProofOfStake,
    /// Delegated Proof of Stake
    DelegatedProofOfStake,
    /// Proof of Authority
    ProofOfAuthority,
}

/// コンセンサスステータス
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsensusStatus {
    /// 現在のバリデーター数
    pub validator_count: usize,
    
    /// 総ステーク量
    pub total_stake: f64,
    
    /// 現在のブロック生成者
    pub current_proposer: Option<String>,
    
    /// 次のブロック生成予定時間
    pub next_block_time: Option<String>,
    
    /// 直近のブロック生成時間（ミリ秒）
    pub last_block_time_ms: u64,
    
    /// コンセンサス参加率
    pub participation_rate: f64,
    
    /// リソース使用効率
    pub resource_efficiency: f64,
    
    /// 現在の報酬レート
    pub current_reward_rate: f64,
}

/// コンセンサスマネージャー
/// コンセンサスアルゴリズムの管理と実行を担当
pub struct ConsensusManager {
    /// コンセンサスアルゴリズム
    algorithm: Box<dyn ConsensusAlgorithm>,
    
    /// コンセンサス設定
    config: ConsensusConfig,
    
    /// バリデーターリスト
    validators: Arc<Mutex<HashMap<String, Validator>>>,
    
    /// 動的報酬システム
    reward_system: DynamicRewardSystem,
    
    /// リソースモニター
    resource_monitor: ResourceMonitor,
    
    /// 最後のブロック生成時間
    last_block_time: Arc<Mutex<Instant>>,
    
    /// コンセンサスステータス
    status: Arc<Mutex<ConsensusStatus>>,
}

impl ConsensusManager {
    /// 新しいコンセンサスマネージャーを作成
    pub fn new(config: ConsensusConfig) -> Self {
        let algorithm: Box<dyn ConsensusAlgorithm> = match config.consensus_type {
            ConsensusType::ProofOfStake => Box::new(ProofOfStake::new(&config)),
            ConsensusType::DelegatedProofOfStake => Box::new(ProofOfStake::new(&config)), // 将来的に実装
            ConsensusType::ProofOfAuthority => Box::new(ProofOfStake::new(&config)),      // 将来的に実装
        };
        
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
            last_block_time: Arc::new(Mutex::new(Instant::now())),
            status,
        }
    }
    
    /// バリデーターを登録
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
    
    /// バリデーターを削除
    pub fn unregister_validator(&self, address: &str) -> Result<(), String> {
        let mut validators = self.validators.lock().unwrap();
        
        if !validators.contains_key(address) {
            return Err(format!("Validator {} not found", address));
        }
        
        validators.remove(address);
        
        // ステータスの更新
        let mut status = self.status.lock().unwrap();
        status.validator_count = validators.len();
        status.total_stake = validators.values().map(|v| v.stake).sum();
        
        // 報酬レートの更新
        status.current_reward_rate = self.reward_system.calculate_reward_rate(validators.len());
        
        Ok(())
    }
    
    /// ブロックを生成
    pub fn create_block(&self, transactions: Vec<Transaction>) -> Block {
        // バリデーターの選出
        let validator = self.algorithm.select_validator();
        
        // 選出されたバリデーターの記録
        if let Some(ref v) = validator {
            let mut status = self.status.lock().unwrap();
            status.current_proposer = Some(v.address.clone());
        }
        
        // ブロックの生成
        let block = self.algorithm.create_block(transactions);
        
        // 最後のブロック生成時間を更新
        let now = Instant::now();
        let mut last_time = self.last_block_time.lock().unwrap();
        let elapsed = now.duration_since(*last_time).as_millis() as u64;
        *last_time = now;
        
        // ステータスの更新
        let mut status = self.status.lock().unwrap();
        status.last_block_time_ms = elapsed;
        status.next_block_time = Some(chrono::Utc::now()
            .checked_add_signed(chrono::Duration::seconds(self.config.block_time as i64))
            .unwrap()
            .to_rfc3339());
        
        // リソース効率の更新
        status.resource_efficiency = self.resource_monitor.get_efficiency();
        
        block
    }
    
    /// ブロックを検証
    pub fn validate_block(&self, block: &Block) -> bool {
        self.algorithm.validate_block(block)
    }
    
    /// 報酬を分配
    pub fn distribute_rewards(&self, block: &Block) -> HashMap<String, f64> {
        let base_reward = self.algorithm.calculate_reward(block);
        let validators = self.validators.lock().unwrap();
        let validator_count = validators.len();
        
        // 報酬レートの計算
        let reward_rate = self.reward_system.calculate_reward_rate(validator_count);
        
        // リソース効率の取得
        let resource_efficiency = self.resource_monitor.get_efficiency();
        
        // 報酬の調整
        let adjusted_reward = base_reward * reward_rate * self.config.resource_efficiency_factor * resource_efficiency;
        
        // 報酬の分配
        let mut rewards = HashMap::new();
        
        if let Some(proposer) = validators.get(&block.miner) {
            // ブロック提案者への報酬
            let proposer_reward = adjusted_reward * 0.8; // 80%をプロポーザーに
            rewards.insert(proposer.address.clone(), proposer_reward);
            
            // 残りの報酬をステーク比率で分配
            let remaining_reward = adjusted_reward * 0.2; // 残り20%
            let total_stake: f64 = validators.values().map(|v| v.stake).sum();
            
            for (address, validator) in validators.iter() {
                if address != &block.miner {
                    let stake_ratio = validator.stake / total_stake;
                    let validator_reward = remaining_reward * stake_ratio;
                    rewards.insert(address.clone(), validator_reward);
                }
            }
        }
        
        // ステータスの更新
        let mut status = self.status.lock().unwrap();
        status.current_reward_rate = reward_rate;
        
        rewards
    }
    
    /// コンセンサスステータスを取得
    pub fn get_status(&self) -> ConsensusStatus {
        self.status.lock().unwrap().clone()
    }
    
    /// バリデーターリストを取得
    pub fn get_validators(&self) -> Vec<Validator> {
        let validators = self.validators.lock().unwrap();
        validators.values().cloned().collect()
    }
    
    /// リソース使用効率を更新
    pub fn update_resource_efficiency(&self) {
        self.resource_monitor.update();
        let efficiency = self.resource_monitor.get_efficiency();
        
        let mut status = self.status.lock().unwrap();
        status.resource_efficiency = efficiency;
    }
}