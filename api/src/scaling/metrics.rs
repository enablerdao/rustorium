// スケーリングメトリクス
// パフォーマンスと負荷の測定

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// スケーリングメトリクス
/// パフォーマンスと負荷の測定を担当
pub struct ScalingMetrics {
    /// トランザクション処理速度の履歴
    tps_history: Arc<Mutex<VecDeque<TPSSample>>>,
    
    /// ノード数の履歴
    node_count_history: Arc<Mutex<VecDeque<NodeCountSample>>>,
    
    /// 最後の更新時間
    last_update: Arc<Mutex<Instant>>,
    
    /// 現在のTPS
    current_tps: Arc<Mutex<f64>>,
    
    /// 現在のノード数
    current_node_count: Arc<Mutex<usize>>,
}

/// TPS（1秒あたりのトランザクション数）サンプル
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TPSSample {
    /// タイムスタンプ
    pub timestamp: u64,
    
    /// TPS値
    pub tps: f64,
}

/// ノード数サンプル
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeCountSample {
    /// タイムスタンプ
    pub timestamp: u64,
    
    /// ノード数
    pub node_count: usize,
}

/// 現在のメトリクス
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CurrentMetrics {
    /// 現在のTPS
    pub tps: f64,
    
    /// 現在のノード数
    pub node_count: usize,
    
    /// ノードあたりのTPS
    pub tps_per_node: f64,
    
    /// タイムスタンプ
    pub timestamp: u64,
}

impl ScalingMetrics {
    /// 新しいスケーリングメトリクスを作成
    pub fn new() -> Self {
        Self {
            tps_history: Arc::new(Mutex::new(VecDeque::with_capacity(100))),
            node_count_history: Arc::new(Mutex::new(VecDeque::with_capacity(100))),
            last_update: Arc::new(Mutex::new(Instant::now())),
            current_tps: Arc::new(Mutex::new(0.0)),
            current_node_count: Arc::new(Mutex::new(0)),
        }
    }
    
    /// メトリクスを更新
    pub fn update(&self, tps: f64, node_count: usize) {
        // 現在の時間を取得
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // TPSサンプルを作成
        let tps_sample = TPSSample {
            timestamp: now,
            tps,
        };
        
        // ノード数サンプルを作成
        let node_count_sample = NodeCountSample {
            timestamp: now,
            node_count,
        };
        
        // サンプルを追加
        {
            let mut tps_history = self.tps_history.lock().unwrap();
            tps_history.push_back(tps_sample);
            
            // 履歴が長すぎる場合は古いサンプルを削除
            if tps_history.len() > 100 {
                tps_history.pop_front();
            }
        }
        
        {
            let mut node_count_history = self.node_count_history.lock().unwrap();
            node_count_history.push_back(node_count_sample);
            
            if node_count_history.len() > 100 {
                node_count_history.pop_front();
            }
        }
        
        // 現在の値を更新
        {
            let mut current_tps = self.current_tps.lock().unwrap();
            *current_tps = tps;
        }
        
        {
            let mut current_node_count = self.current_node_count.lock().unwrap();
            *current_node_count = node_count;
        }
        
        // 最後の更新時間を更新
        let mut last_update = self.last_update.lock().unwrap();
        *last_update = Instant::now();
    }
    
    /// 現在のメトリクスを取得
    pub fn get_current_metrics(&self) -> CurrentMetrics {
        let tps = *self.current_tps.lock().unwrap();
        let node_count = *self.current_node_count.lock().unwrap();
        
        let tps_per_node = if node_count == 0 {
            0.0
        } else {
            tps / node_count as f64
        };
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        CurrentMetrics {
            tps,
            node_count,
            tps_per_node,
            timestamp: now,
        }
    }
    
    /// TPS履歴を取得
    pub fn get_tps_history(&self) -> Vec<TPSSample> {
        let tps_history = self.tps_history.lock().unwrap();
        tps_history.iter().cloned().collect()
    }
    
    /// ノード数履歴を取得
    pub fn get_node_count_history(&self) -> Vec<NodeCountSample> {
        let node_count_history = self.node_count_history.lock().unwrap();
        node_count_history.iter().cloned().collect()
    }
    
    /// 平均TPSを計算
    pub fn get_average_tps(&self, duration_secs: u64) -> f64 {
        let tps_history = self.tps_history.lock().unwrap();
        
        if tps_history.is_empty() {
            return 0.0;
        }
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let cutoff = now.saturating_sub(duration_secs);
        
        let recent_samples: Vec<&TPSSample> = tps_history.iter()
            .filter(|sample| sample.timestamp >= cutoff)
            .collect();
        
        if recent_samples.is_empty() {
            return 0.0;
        }
        
        let sum: f64 = recent_samples.iter().map(|sample| sample.tps).sum();
        sum / recent_samples.len() as f64
    }
    
    /// 平均ノード数を計算
    pub fn get_average_node_count(&self, duration_secs: u64) -> f64 {
        let node_count_history = self.node_count_history.lock().unwrap();
        
        if node_count_history.is_empty() {
            return 0.0;
        }
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let cutoff = now.saturating_sub(duration_secs);
        
        let recent_samples: Vec<&NodeCountSample> = node_count_history.iter()
            .filter(|sample| sample.timestamp >= cutoff)
            .collect();
        
        if recent_samples.is_empty() {
            return 0.0;
        }
        
        let sum: usize = recent_samples.iter().map(|sample| sample.node_count).sum();
        sum as f64 / recent_samples.len() as f64
    }
    
    /// 最後の更新からの経過時間を取得
    pub fn time_since_last_update(&self) -> Duration {
        let last_update = self.last_update.lock().unwrap();
        last_update.elapsed()
    }
    
    /// TPSの傾向を計算（上昇中、下降中、安定）
    pub fn get_tps_trend(&self, duration_secs: u64) -> TrendDirection {
        let tps_history = self.tps_history.lock().unwrap();
        
        if tps_history.len() < 2 {
            return TrendDirection::Stable;
        }
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let cutoff = now.saturating_sub(duration_secs);
        
        let recent_samples: Vec<&TPSSample> = tps_history.iter()
            .filter(|sample| sample.timestamp >= cutoff)
            .collect();
        
        if recent_samples.len() < 2 {
            return TrendDirection::Stable;
        }
        
        // 線形回帰で傾きを計算
        let n = recent_samples.len() as f64;
        let sum_x: f64 = recent_samples.iter().map(|s| s.timestamp as f64).sum();
        let sum_y: f64 = recent_samples.iter().map(|s| s.tps).sum();
        let sum_xy: f64 = recent_samples.iter().map(|s| s.timestamp as f64 * s.tps).sum();
        let sum_xx: f64 = recent_samples.iter().map(|s| (s.timestamp as f64).powi(2)).sum();
        
        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_xx - sum_x.powi(2));
        
        // 傾きの閾値
        const THRESHOLD: f64 = 0.01;
        
        if slope > THRESHOLD {
            TrendDirection::Increasing
        } else if slope < -THRESHOLD {
            TrendDirection::Decreasing
        } else {
            TrendDirection::Stable
        }
    }
}

/// 傾向の方向
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TrendDirection {
    /// 上昇中
    Increasing,
    /// 下降中
    Decreasing,
    /// 安定
    Stable,
}

impl Default for ScalingMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    
    #[test]
    fn test_metrics_initialization() {
        let metrics = ScalingMetrics::new();
        
        // 初期状態では履歴は空
        assert!(metrics.get_tps_history().is_empty());
        assert!(metrics.get_node_count_history().is_empty());
        
        // 初期値は0
        let current = metrics.get_current_metrics();
        assert_eq!(current.tps, 0.0);
        assert_eq!(current.node_count, 0);
        assert_eq!(current.tps_per_node, 0.0);
    }
    
    #[test]
    fn test_metrics_update() {
        let metrics = ScalingMetrics::new();
        
        // メトリクスを更新
        metrics.update(100.0, 10);
        
        // 更新後の値を確認
        let current = metrics.get_current_metrics();
        assert_eq!(current.tps, 100.0);
        assert_eq!(current.node_count, 10);
        assert_eq!(current.tps_per_node, 10.0);
        
        // 履歴にサンプルが追加されている
        assert_eq!(metrics.get_tps_history().len(), 1);
        assert_eq!(metrics.get_node_count_history().len(), 1);
    }
    
    #[test]
    fn test_metrics_multiple_updates() {
        let metrics = ScalingMetrics::new();
        
        // 複数回更新
        metrics.update(100.0, 10);
        thread::sleep(Duration::from_millis(10));
        metrics.update(200.0, 20);
        thread::sleep(Duration::from_millis(10));
        metrics.update(300.0, 30);
        
        // 最新の値を確認
        let current = metrics.get_current_metrics();
        assert_eq!(current.tps, 300.0);
        assert_eq!(current.node_count, 30);
        assert_eq!(current.tps_per_node, 10.0);
        
        // 履歴のサンプル数を確認
        assert_eq!(metrics.get_tps_history().len(), 3);
        assert_eq!(metrics.get_node_count_history().len(), 3);
    }
    
    #[test]
    fn test_average_calculations() {
        let metrics = ScalingMetrics::new();
        
        // 複数回更新
        metrics.update(100.0, 10);
        thread::sleep(Duration::from_millis(10));
        metrics.update(200.0, 20);
        thread::sleep(Duration::from_millis(10));
        metrics.update(300.0, 30);
        
        // 平均値を計算
        let avg_tps = metrics.get_average_tps(60); // 過去60秒
        let avg_node_count = metrics.get_average_node_count(60);
        
        // 平均値を確認
        assert_eq!(avg_tps, (100.0 + 200.0 + 300.0) / 3.0);
        assert_eq!(avg_node_count, (10.0 + 20.0 + 30.0) / 3.0);
    }
    
    #[test]
    fn test_tps_trend() {
        let metrics = ScalingMetrics::new();
        
        // 上昇傾向のデータ
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // 上昇傾向のサンプルを追加
        {
            let mut tps_history = metrics.tps_history.lock().unwrap();
            for i in 0..5 {
                tps_history.push_back(TPSSample {
                    timestamp: now - 10 + i * 2,
                    tps: 100.0 + i as f64 * 20.0,
                });
            }
        }
        
        // 傾向を確認
        let trend = metrics.get_tps_trend(60);
        assert_eq!(trend, TrendDirection::Increasing);
        
        // 下降傾向のデータに変更
        {
            let mut tps_history = metrics.tps_history.lock().unwrap();
            tps_history.clear();
            for i in 0..5 {
                tps_history.push_back(TPSSample {
                    timestamp: now - 10 + i * 2,
                    tps: 200.0 - i as f64 * 20.0,
                });
            }
        }
        
        // 傾向を確認
        let trend = metrics.get_tps_trend(60);
        assert_eq!(trend, TrendDirection::Decreasing);
        
        // 安定傾向のデータに変更
        {
            let mut tps_history = metrics.tps_history.lock().unwrap();
            tps_history.clear();
            for i in 0..5 {
                tps_history.push_back(TPSSample {
                    timestamp: now - 10 + i * 2,
                    tps: 100.0 + (i as f64 * 0.1), // ほぼ一定
                });
            }
        }
        
        // 傾向を確認
        let trend = metrics.get_tps_trend(60);
        assert_eq!(trend, TrendDirection::Stable);
    }
}