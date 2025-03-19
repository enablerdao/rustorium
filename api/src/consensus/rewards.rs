// 動的報酬システム
// ノード数に応じて報酬を調整する仕組み

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// 動的報酬システム
/// ノード数に応じて報酬を調整する仕組み
#[derive(Clone)]
pub struct DynamicRewardSystem {
    /// 基本報酬
    base_reward: f64,
    
    /// ノード数に応じた報酬減少係数
    node_scaling_factor: f64,
    
    /// 最適ノード数
    optimal_node_count: usize,
    
    /// 報酬履歴
    reward_history: Arc<Mutex<Vec<RewardHistoryEntry>>>,
}

/// 報酬履歴エントリ
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RewardHistoryEntry {
    /// タイムスタンプ
    pub timestamp: u64,
    
    /// ノード数
    pub node_count: usize,
    
    /// 報酬レート
    pub reward_rate: f64,
}

impl DynamicRewardSystem {
    /// 新しい動的報酬システムを作成
    pub fn new(base_reward: f64, node_scaling_factor: f64, optimal_node_count: usize) -> Self {
        Self {
            base_reward,
            node_scaling_factor,
            optimal_node_count,
            reward_history: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    /// 報酬レートを計算
    /// ノード数が増えるほど報酬レートが減少する
    pub fn calculate_reward_rate(&self, node_count: usize) -> f64 {
        if node_count <= self.optimal_node_count {
            // 最適ノード数以下の場合は基本報酬
            return 1.0;
        }
        
        // 最適ノード数を超える場合は、ノード数に応じて報酬が減少
        // 減少曲線は指数関数的: rate = node_scaling_factor^(node_count - optimal_node_count)
        let excess_nodes = node_count - self.optimal_node_count;
        let rate = self.node_scaling_factor.powi(excess_nodes as i32);
        
        // 報酬履歴に記録
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let entry = RewardHistoryEntry {
            timestamp: now,
            node_count,
            reward_rate: rate,
        };
        
        let mut history = self.reward_history.lock().unwrap();
        history.push(entry);
        
        // 履歴が長すぎる場合は古いエントリを削除
        if history.len() > 1000 {
            history.remove(0);
        }
        
        rate
    }
    
    /// 報酬を計算
    pub fn calculate_reward(&self, base_amount: f64, node_count: usize) -> f64 {
        let rate = self.calculate_reward_rate(node_count);
        base_amount * rate
    }
    
    /// 報酬履歴を取得
    pub fn get_reward_history(&self) -> Vec<RewardHistoryEntry> {
        let history = self.reward_history.lock().unwrap();
        history.clone()
    }
    
    /// 基本報酬を設定
    pub fn set_base_reward(&mut self, base_reward: f64) {
        self.base_reward = base_reward;
    }
    
    /// ノード数に応じた報酬減少係数を設定
    pub fn set_node_scaling_factor(&mut self, node_scaling_factor: f64) {
        self.node_scaling_factor = node_scaling_factor.max(0.0).min(1.0);
    }
    
    /// 最適ノード数を設定
    pub fn set_optimal_node_count(&mut self, optimal_node_count: usize) {
        self.optimal_node_count = optimal_node_count;
    }
    
    /// 基本報酬を取得
    pub fn get_base_reward(&self) -> f64 {
        self.base_reward
    }
    
    /// ノード数に応じた報酬減少係数を取得
    pub fn get_node_scaling_factor(&self) -> f64 {
        self.node_scaling_factor
    }
    
    /// 最適ノード数を取得
    pub fn get_optimal_node_count(&self) -> usize {
        self.optimal_node_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_reward_calculation() {
        let base_reward = 10.0;
        let node_scaling_factor = 0.95;
        let optimal_node_count = 100;
        
        let reward_system = DynamicRewardSystem::new(
            base_reward,
            node_scaling_factor,
            optimal_node_count,
        );
        
        // 最適ノード数以下の場合
        let rate_50 = reward_system.calculate_reward_rate(50);
        assert_eq!(rate_50, 1.0);
        
        let reward_50 = reward_system.calculate_reward(base_reward, 50);
        assert_eq!(reward_50, base_reward);
        
        // 最適ノード数の場合
        let rate_100 = reward_system.calculate_reward_rate(100);
        assert_eq!(rate_100, 1.0);
        
        let reward_100 = reward_system.calculate_reward(base_reward, 100);
        assert_eq!(reward_100, base_reward);
        
        // 最適ノード数を超える場合
        let rate_110 = reward_system.calculate_reward_rate(110);
        let expected_rate_110 = node_scaling_factor.powi(10);
        assert!((rate_110 - expected_rate_110).abs() < 1e-10);
        
        let reward_110 = reward_system.calculate_reward(base_reward, 110);
        let expected_reward_110 = base_reward * expected_rate_110;
        assert!((reward_110 - expected_reward_110).abs() < 1e-10);
        
        // さらに多くのノード数の場合
        let rate_200 = reward_system.calculate_reward_rate(200);
        let expected_rate_200 = node_scaling_factor.powi(100);
        assert!((rate_200 - expected_rate_200).abs() < 1e-10);
        
        let reward_200 = reward_system.calculate_reward(base_reward, 200);
        let expected_reward_200 = base_reward * expected_rate_200;
        assert!((reward_200 - expected_reward_200).abs() < 1e-10);
    }
    
    #[test]
    fn test_reward_history() {
        let reward_system = DynamicRewardSystem::new(10.0, 0.95, 100);
        
        // 複数のノード数で報酬レートを計算
        reward_system.calculate_reward_rate(50);
        reward_system.calculate_reward_rate(100);
        reward_system.calculate_reward_rate(150);
        reward_system.calculate_reward_rate(200);
        
        // 履歴を取得
        let history = reward_system.get_reward_history();
        
        // 履歴のエントリ数を確認
        assert_eq!(history.len(), 4);
        
        // 履歴の内容を確認
        assert_eq!(history[0].node_count, 50);
        assert_eq!(history[0].reward_rate, 1.0);
        
        assert_eq!(history[1].node_count, 100);
        assert_eq!(history[1].reward_rate, 1.0);
        
        assert_eq!(history[2].node_count, 150);
        let expected_rate_150 = 0.95f64.powi(50);
        assert!((history[2].reward_rate - expected_rate_150).abs() < 1e-10);
        
        assert_eq!(history[3].node_count, 200);
        let expected_rate_200 = 0.95f64.powi(100);
        assert!((history[3].reward_rate - expected_rate_200).abs() < 1e-10);
    }
    
    #[test]
    fn test_parameter_setters() {
        let mut reward_system = DynamicRewardSystem::new(10.0, 0.95, 100);
        
        // パラメータを変更
        reward_system.set_base_reward(20.0);
        reward_system.set_node_scaling_factor(0.9);
        reward_system.set_optimal_node_count(200);
        
        // 変更後のパラメータを確認
        assert_eq!(reward_system.get_base_reward(), 20.0);
        assert_eq!(reward_system.get_node_scaling_factor(), 0.9);
        assert_eq!(reward_system.get_optimal_node_count(), 200);
        
        // 変更後のパラメータで報酬を計算
        let rate = reward_system.calculate_reward_rate(250);
        let expected_rate = 0.9f64.powi(50);
        assert!((rate - expected_rate).abs() < 1e-10);
        
        let reward = reward_system.calculate_reward(20.0, 250);
        let expected_reward = 20.0 * expected_rate;
        assert!((reward - expected_reward).abs() < 1e-10);
    }
}