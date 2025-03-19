// 適応型設定
// ネットワーク状況に応じて自動的に設定を最適化

use crate::consensus::resource_monitor::ResourceUsage;
use crate::scaling::metrics::CurrentMetrics;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// 適応型設定
/// ネットワーク状況に応じて自動的に設定を最適化
pub struct AdaptiveConfig {
    /// 設定履歴
    config_history: Arc<Mutex<Vec<ConfigHistoryEntry>>>,
    
    /// 現在の設定
    current_config: Arc<Mutex<OptimizedConfig>>,
    
    /// 最後の更新時間
    last_update: Arc<Mutex<Instant>>,
}

/// 設定履歴エントリ
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigHistoryEntry {
    /// タイムスタンプ
    pub timestamp: u64,
    
    /// 設定
    pub config: OptimizedConfig,
    
    /// リソース使用状況
    pub resource_usage: ResourceUsage,
    
    /// メトリクス
    pub metrics: CurrentMetrics,
}

/// 最適化された設定
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptimizedConfig {
    /// ブロック生成間隔（秒）
    pub block_time: u64,
    
    /// トランザクションバッチサイズ
    pub tx_batch_size: usize,
    
    /// 最大シャード数
    pub max_shards: usize,
    
    /// ノードあたりの最適トランザクション数
    pub optimal_tx_per_node: usize,
    
    /// スケールアップのしきい値（CPU使用率）
    pub scale_up_threshold: f64,
    
    /// スケールダウンのしきい値（CPU使用率）
    pub scale_down_threshold: f64,
}

impl Default for OptimizedConfig {
    fn default() -> Self {
        Self {
            block_time: 5,
            tx_batch_size: 1000,
            max_shards: 16,
            optimal_tx_per_node: 1000,
            scale_up_threshold: 0.8,
            scale_down_threshold: 0.3,
        }
    }
}

impl AdaptiveConfig {
    /// 新しい適応型設定を作成
    pub fn new() -> Self {
        Self {
            config_history: Arc::new(Mutex::new(Vec::new())),
            current_config: Arc::new(Mutex::new(OptimizedConfig::default())),
            last_update: Arc::new(Mutex::new(Instant::now())),
        }
    }
    
    /// 設定を更新
    pub fn update(&self, resource_usage: &ResourceUsage, metrics: &CurrentMetrics) {
        // 現在の設定を取得
        let mut current_config = self.current_config.lock().unwrap();
        
        // リソース使用状況とメトリクスに基づいて設定を最適化
        self.optimize_block_time(&mut current_config, resource_usage, metrics);
        self.optimize_tx_batch_size(&mut current_config, resource_usage, metrics);
        self.optimize_scaling_thresholds(&mut current_config, resource_usage, metrics);
        
        // 設定履歴に追加
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let entry = ConfigHistoryEntry {
            timestamp: now,
            config: current_config.clone(),
            resource_usage: resource_usage.clone(),
            metrics: metrics.clone(),
        };
        
        let mut config_history = self.config_history.lock().unwrap();
        config_history.push(entry);
        
        // 履歴が長すぎる場合は古いエントリを削除
        if config_history.len() > 100 {
            config_history.remove(0);
        }
        
        // 最後の更新時間を更新
        let mut last_update = self.last_update.lock().unwrap();
        *last_update = Instant::now();
    }
    
    /// ブロック生成間隔を最適化
    fn optimize_block_time(&self, config: &mut OptimizedConfig, resource_usage: &ResourceUsage, metrics: &CurrentMetrics) {
        // CPU使用率に基づいてブロック生成間隔を調整
        if resource_usage.cpu > 0.9 {
            // CPU使用率が高い場合はブロック生成間隔を長くする
            config.block_time = (config.block_time + 1).min(10);
        } else if resource_usage.cpu < 0.5 && metrics.tps > 100.0 {
            // CPU使用率が低く、TPSが高い場合はブロック生成間隔を短くする
            config.block_time = (config.block_time - 1).max(1);
        }
    }
    
    /// トランザクションバッチサイズを最適化
    fn optimize_tx_batch_size(&self, config: &mut OptimizedConfig, resource_usage: &ResourceUsage, metrics: &CurrentMetrics) {
        // メモリ使用率とTPSに基づいてバッチサイズを調整
        if resource_usage.memory > 0.8 {
            // メモリ使用率が高い場合はバッチサイズを小さくする
            config.tx_batch_size = (config.tx_batch_size * 9 / 10).max(100);
        } else if resource_usage.memory < 0.5 && metrics.tps > config.tx_batch_size as f64 / 5.0 {
            // メモリ使用率が低く、TPSが高い場合はバッチサイズを大きくする
            config.tx_batch_size = (config.tx_batch_size * 11 / 10).min(10000);
        }
    }
    
    /// スケーリングしきい値を最適化
    fn optimize_scaling_thresholds(&self, config: &mut OptimizedConfig, resource_usage: &ResourceUsage, metrics: &CurrentMetrics) {
        // ノード数とTPSに基づいてスケーリングしきい値を調整
        if metrics.node_count > 10 && metrics.tps_per_node < config.optimal_tx_per_node as f64 / 2.0 {
            // ノード数が多く、ノードあたりのTPSが低い場合はスケールダウンしきい値を上げる
            config.scale_down_threshold = (config.scale_down_threshold + 0.05).min(0.7);
        } else if metrics.node_count < 5 && metrics.tps_per_node > config.optimal_tx_per_node as f64 * 1.5 {
            // ノード数が少なく、ノードあたりのTPSが高い場合はスケールアップしきい値を下げる
            config.scale_up_threshold = (config.scale_up_threshold - 0.05).max(0.5);
        }
    }
    
    /// 現在の設定を取得
    pub fn get_current_config(&self) -> OptimizedConfig {
        self.current_config.lock().unwrap().clone()
    }
    
    /// 設定履歴を取得
    pub fn get_config_history(&self) -> Vec<ConfigHistoryEntry> {
        self.config_history.lock().unwrap().clone()
    }
    
    /// 最後の更新からの経過時間を取得
    pub fn time_since_last_update(&self) -> Duration {
        self.last_update.lock().unwrap().elapsed()
    }
    
    /// 設定を手動で更新
    pub fn set_config(&self, config: OptimizedConfig) {
        let mut current_config = self.current_config.lock().unwrap();
        *current_config = config;
    }
}

impl Default for AdaptiveConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    
    fn create_test_resource_usage(cpu: f64, memory: f64) -> ResourceUsage {
        ResourceUsage {
            cpu,
            memory,
            disk: 0.5,
            network: 0.5,
            efficiency: 0.8,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    fn create_test_metrics(tps: f64, node_count: usize) -> CurrentMetrics {
        let tps_per_node = if node_count == 0 {
            0.0
        } else {
            tps / node_count as f64
        };
        
        CurrentMetrics {
            tps,
            node_count,
            tps_per_node,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    #[test]
    fn test_adaptive_config_initialization() {
        let config = AdaptiveConfig::new();
        
        // 初期状態では履歴は空
        assert!(config.get_config_history().is_empty());
        
        // 初期設定はデフォルト値
        let current = config.get_current_config();
        assert_eq!(current.block_time, 5);
        assert_eq!(current.tx_batch_size, 1000);
        assert_eq!(current.max_shards, 16);
        assert_eq!(current.optimal_tx_per_node, 1000);
        assert_eq!(current.scale_up_threshold, 0.8);
        assert_eq!(current.scale_down_threshold, 0.3);
    }
    
    #[test]
    fn test_adaptive_config_update() {
        let config = AdaptiveConfig::new();
        
        // 高負荷状態
        let resource_usage = create_test_resource_usage(0.95, 0.9);
        let metrics = create_test_metrics(5000.0, 5);
        
        // 設定を更新
        config.update(&resource_usage, &metrics);
        
        // 更新後の設定を確認
        let current = config.get_current_config();
        
        // 高負荷状態ではブロック生成間隔が長くなり、バッチサイズが小さくなる
        assert!(current.block_time > 5);
        assert!(current.tx_batch_size < 1000);
        
        // 履歴にエントリが追加されている
        assert_eq!(config.get_config_history().len(), 1);
    }
    
    #[test]
    fn test_adaptive_config_multiple_updates() {
        let config = AdaptiveConfig::new();
        
        // 複数回更新
        // 1. 高負荷状態
        let resource_usage1 = create_test_resource_usage(0.95, 0.9);
        let metrics1 = create_test_metrics(5000.0, 5);
        config.update(&resource_usage1, &metrics1);
        
        thread::sleep(Duration::from_millis(10));
        
        // 2. 中負荷状態
        let resource_usage2 = create_test_resource_usage(0.7, 0.6);
        let metrics2 = create_test_metrics(3000.0, 10);
        config.update(&resource_usage2, &metrics2);
        
        thread::sleep(Duration::from_millis(10));
        
        // 3. 低負荷状態
        let resource_usage3 = create_test_resource_usage(0.3, 0.4);
        let metrics3 = create_test_metrics(1000.0, 15);
        config.update(&resource_usage3, &metrics3);
        
        // 履歴のエントリ数を確認
        assert_eq!(config.get_config_history().len(), 3);
        
        // 最新の設定を確認
        let current = config.get_current_config();
        
        // 低負荷状態ではスケールダウンしきい値が上がっている可能性がある
        assert!(current.scale_down_threshold >= 0.3);
    }
    
    #[test]
    fn test_manual_config_update() {
        let config = AdaptiveConfig::new();
        
        // 手動で設定を更新
        let mut new_config = OptimizedConfig::default();
        new_config.block_time = 10;
        new_config.tx_batch_size = 2000;
        new_config.max_shards = 32;
        
        config.set_config(new_config.clone());
        
        // 更新後の設定を確認
        let current = config.get_current_config();
        assert_eq!(current.block_time, 10);
        assert_eq!(current.tx_batch_size, 2000);
        assert_eq!(current.max_shards, 32);
    }
}