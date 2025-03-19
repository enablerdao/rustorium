// Proof of Stakeコンセンサスアルゴリズム実装
// リソース効率の良いPoSベースのコンセンサス

use super::{ConsensusAlgorithm, ConsensusConfig, Validator};
use crate::blockchain::{Block, Transaction};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// Proof of Stakeコンセンサスアルゴリズム
pub struct ProofOfStake {
    /// コンセンサス設定
    config: ConsensusConfig,
    
    /// バリデーターリスト
    validators: Arc<Mutex<HashMap<String, Validator>>>,
    
    /// 最後に選出されたバリデーター
    last_selected: Arc<Mutex<Option<String>>>,
}

impl ProofOfStake {
    /// 新しいProof of Stakeインスタンスを作成
    pub fn new(config: &ConsensusConfig) -> Self {
        Self {
            config: config.clone(),
            validators: Arc::new(Mutex::new(HashMap::new())),
            last_selected: Arc::new(Mutex::new(None)),
        }
    }
    
    /// バリデーターを追加
    pub fn add_validator(&self, validator: Validator) {
        let mut validators = self.validators.lock().unwrap();
        validators.insert(validator.address.clone(), validator);
    }
    
    /// バリデーターを削除
    pub fn remove_validator(&self, address: &str) {
        let mut validators = self.validators.lock().unwrap();
        validators.remove(address);
    }
    
    /// バリデーターリストを取得
    pub fn get_validators(&self) -> Vec<Validator> {
        let validators = self.validators.lock().unwrap();
        validators.values().cloned().collect()
    }
    
    /// ステーク量に基づいてバリデーターを選出
    fn select_validator_by_stake(&self) -> Option<Validator> {
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
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let seed = now as u64;
        let mut rng = StdRng::seed_from_u64(seed);
        
        // ランダムな値を生成（0.0 <= r < total_stake）
        let r = rng.gen_range(0.0..total_stake);
        
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
}

impl ConsensusAlgorithm for ProofOfStake {
    fn create_block(&self, transactions: Vec<Transaction>) -> Block {
        // 現在の時間を取得
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        // バリデーターを選出
        let validator = self.select_validator().unwrap_or(Validator {
            address: "system".to_string(),
            stake: 0.0,
            public_key: vec![],
            last_active: now,
            performance: 1.0,
        });
        
        // ブロックの作成
        let mut block = Block {
            hash: String::new(),
            previous_hash: String::new(), // 実際の実装では前のブロックのハッシュを設定
            timestamp: now,
            nonce: 0,
            transactions,
            miner: validator.address,
            difficulty: 1,
            height: 0, // 実際の実装ではブロック高を設定
        };
        
        // ブロックハッシュの計算
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}{}", 
            block.previous_hash, 
            block.timestamp, 
            block.miner,
            block.transactions.len()
        ));
        
        for tx in &block.transactions {
            hasher.update(&tx.id);
        }
        
        block.hash = format!("{:x}", hasher.finalize());
        
        // 最後に選出されたバリデーターを更新
        let mut last_selected = self.last_selected.lock().unwrap();
        *last_selected = Some(block.miner.clone());
        
        block
    }
    
    fn validate_block(&self, block: &Block) -> bool {
        // ブロックハッシュの検証
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}{}", 
            block.previous_hash, 
            block.timestamp, 
            block.miner,
            block.transactions.len()
        ));
        
        for tx in &block.transactions {
            hasher.update(&tx.id);
        }
        
        let calculated_hash = format!("{:x}", hasher.finalize());
        
        if calculated_hash != block.hash {
            return false;
        }
        
        // マイナーがバリデーターリストに含まれているか確認
        let validators = self.validators.lock().unwrap();
        if !validators.contains_key(&block.miner) && block.miner != "system" {
            return false;
        }
        
        true
    }
    
    fn select_validator(&self) -> Option<Validator> {
        self.select_validator_by_stake()
    }
    
    fn calculate_reward(&self, block: &Block) -> f64 {
        // 基本報酬
        let base_reward = self.config.base_reward;
        
        // トランザクション数に応じた追加報酬
        let tx_count = block.transactions.len() as f64;
        let tx_reward = tx_count * 0.01; // トランザクションごとに0.01の追加報酬
        
        base_reward + tx_reward
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validator_selection() {
        let config = ConsensusConfig::default();
        let pos = ProofOfStake::new(&config);
        
        // バリデーターを追加
        pos.add_validator(Validator {
            address: "validator1".to_string(),
            stake: 100.0,
            public_key: vec![],
            last_active: 0,
            performance: 1.0,
        });
        
        pos.add_validator(Validator {
            address: "validator2".to_string(),
            stake: 200.0,
            public_key: vec![],
            last_active: 0,
            performance: 1.0,
        });
        
        pos.add_validator(Validator {
            address: "validator3".to_string(),
            stake: 300.0,
            public_key: vec![],
            last_active: 0,
            performance: 1.0,
        });
        
        // バリデーター選出のテスト
        let validator = pos.select_validator();
        assert!(validator.is_some());
        
        // 統計的なテスト（多数回実行して確率を検証）
        let mut counts = HashMap::new();
        let iterations = 1000;
        
        for _ in 0..iterations {
            let validator = pos.select_validator().unwrap();
            *counts.entry(validator.address).or_insert(0) += 1;
        }
        
        // ステーク比率に近い選出確率になっているか確認
        let total_stake = 100.0 + 200.0 + 300.0;
        let expected_ratio1 = 100.0 / total_stake;
        let expected_ratio2 = 200.0 / total_stake;
        let expected_ratio3 = 300.0 / total_stake;
        
        let actual_ratio1 = counts.get("validator1").unwrap_or(&0) as f64 / iterations as f64;
        let actual_ratio2 = counts.get("validator2").unwrap_or(&0) as f64 / iterations as f64;
        let actual_ratio3 = counts.get("validator3").unwrap_or(&0) as f64 / iterations as f64;
        
        // 統計的な揺らぎを考慮して、ある程度の誤差は許容
        assert!((actual_ratio1 - expected_ratio1).abs() < 0.1);
        assert!((actual_ratio2 - expected_ratio2).abs() < 0.1);
        assert!((actual_ratio3 - expected_ratio3).abs() < 0.1);
    }
    
    #[test]
    fn test_block_creation_and_validation() {
        let config = ConsensusConfig::default();
        let pos = ProofOfStake::new(&config);
        
        // バリデーターを追加
        pos.add_validator(Validator {
            address: "validator1".to_string(),
            stake: 100.0,
            public_key: vec![],
            last_active: 0,
            performance: 1.0,
        });
        
        // トランザクションの作成
        let transactions = vec![
            Transaction {
                id: "tx1".to_string(),
                from: "user1".to_string(),
                to: "user2".to_string(),
                value: 10.0,
                data: None,
                gas_price: 1,
                gas_limit: 21000,
                nonce: 0,
                timestamp: 0,
                signature: None,
                status: "pending".to_string(),
                gas_used: 0,
                block_id: None,
            }
        ];
        
        // ブロックの作成
        let block = pos.create_block(transactions);
        
        // ブロックの検証
        assert!(pos.validate_block(&block));
        
        // 不正なブロックの検証
        let mut invalid_block = block.clone();
        invalid_block.hash = "invalid_hash".to_string();
        assert!(!pos.validate_block(&invalid_block));
    }
    
    #[test]
    fn test_reward_calculation() {
        let config = ConsensusConfig::default();
        let pos = ProofOfStake::new(&config);
        
        // トランザクションの作成
        let transactions = vec![
            Transaction {
                id: "tx1".to_string(),
                from: "user1".to_string(),
                to: "user2".to_string(),
                value: 10.0,
                data: None,
                gas_price: 1,
                gas_limit: 21000,
                nonce: 0,
                timestamp: 0,
                signature: None,
                status: "pending".to_string(),
                gas_used: 0,
                block_id: None,
            },
            Transaction {
                id: "tx2".to_string(),
                from: "user3".to_string(),
                to: "user4".to_string(),
                value: 20.0,
                data: None,
                gas_price: 1,
                gas_limit: 21000,
                nonce: 0,
                timestamp: 0,
                signature: None,
                status: "pending".to_string(),
                gas_used: 0,
                block_id: None,
            }
        ];
        
        // ブロックの作成
        let block = pos.create_block(transactions);
        
        // 報酬の計算
        let reward = pos.calculate_reward(&block);
        
        // 基本報酬 + トランザクション報酬
        let expected_reward = config.base_reward + (2.0 * 0.01);
        assert_eq!(reward, expected_reward);
    }
}