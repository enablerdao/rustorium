// バリデーター実装
// ブロック生成と検証を行うノード

use serde::{Deserialize, Serialize};

/// バリデーター
/// ブロック生成と検証を行うノード
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Validator {
    /// バリデーターのアドレス
    pub address: String,
    
    /// ステーク量
    pub stake: f64,
    
    /// 公開鍵
    pub public_key: Vec<u8>,
    
    /// 最後にアクティブだった時間
    pub last_active: u64,
    
    /// パフォーマンス（0.0 - 1.0）
    pub performance: f64,
}

impl Validator {
    /// 新しいバリデーターを作成
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
    
    /// ステーク量を更新
    pub fn update_stake(&mut self, stake: f64) {
        self.stake = stake;
    }
    
    /// アクティブ時間を更新
    pub fn update_active_time(&mut self) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        self.last_active = now;
    }
    
    /// パフォーマンスを更新
    pub fn update_performance(&mut self, success: bool) {
        // 指数移動平均でパフォーマンスを更新
        let alpha = 0.1; // 平滑化係数
        let new_value = if success { 1.0 } else { 0.0 };
        self.performance = alpha * new_value + (1.0 - alpha) * self.performance;
    }
    
    /// バリデーターがアクティブかどうかを確認
    pub fn is_active(&self, timeout_seconds: u64) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        now - self.last_active < timeout_seconds
    }
    
    /// 有効なステーク量を計算（パフォーマンスを考慮）
    pub fn effective_stake(&self) -> f64 {
        self.stake * self.performance
    }
}

/// バリデーター統計
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidatorStats {
    /// バリデーターのアドレス
    pub address: String,
    
    /// ステーク量
    pub stake: f64,
    
    /// 有効なステーク量（パフォーマンスを考慮）
    pub effective_stake: f64,
    
    /// パフォーマンス（0.0 - 1.0）
    pub performance: f64,
    
    /// 生成したブロック数
    pub blocks_produced: u64,
    
    /// 検証したブロック数
    pub blocks_validated: u64,
    
    /// 獲得した報酬の合計
    pub total_rewards: f64,
    
    /// アクティブ状態
    pub is_active: bool,
    
    /// 最後にアクティブだった時間
    pub last_active: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validator_creation() {
        let address = "validator1".to_string();
        let stake = 100.0;
        let public_key = vec![1, 2, 3, 4];
        
        let validator = Validator::new(address.clone(), stake, public_key.clone());
        
        assert_eq!(validator.address, address);
        assert_eq!(validator.stake, stake);
        assert_eq!(validator.public_key, public_key);
        assert_eq!(validator.performance, 1.0);
        assert!(validator.is_active(3600)); // 1時間のタイムアウト
    }
    
    #[test]
    fn test_validator_update_stake() {
        let mut validator = Validator::new("validator1".to_string(), 100.0, vec![1, 2, 3, 4]);
        
        validator.update_stake(200.0);
        assert_eq!(validator.stake, 200.0);
        assert_eq!(validator.effective_stake(), 200.0); // パフォーマンスは1.0なので同じ
    }
    
    #[test]
    fn test_validator_update_performance() {
        let mut validator = Validator::new("validator1".to_string(), 100.0, vec![1, 2, 3, 4]);
        
        // 失敗した場合のパフォーマンス更新
        validator.update_performance(false);
        assert!(validator.performance < 1.0);
        
        // 有効なステーク量の確認
        assert!(validator.effective_stake() < 100.0);
        
        // 成功した場合のパフォーマンス更新
        for _ in 0..10 {
            validator.update_performance(true);
        }
        
        // パフォーマンスが回復していることを確認
        assert!(validator.performance > 0.9);
    }
    
    #[test]
    fn test_validator_activity() {
        let mut validator = Validator::new("validator1".to_string(), 100.0, vec![1, 2, 3, 4]);
        
        // 現在はアクティブ
        assert!(validator.is_active(3600));
        
        // last_activeを過去の時間に設定
        validator.last_active = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - 7200; // 2時間前
        
        // 1時間のタイムアウトではアクティブでない
        assert!(!validator.is_active(3600));
        
        // 3時間のタイムアウトではアクティブ
        assert!(validator.is_active(10800));
        
        // アクティブ時間を更新
        validator.update_active_time();
        
        // 再びアクティブになる
        assert!(validator.is_active(3600));
    }
}