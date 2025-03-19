// リソース使用効率モニタリングシステム
// ネットワーク全体のリソース使用状況を監視し、効率性を測定・最適化

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// リソースモニター
/// ネットワーク全体のリソース使用状況を監視
#[derive(Clone)]
pub struct ResourceMonitor {
    /// CPU使用率の履歴
    cpu_usage: Arc<Mutex<Vec<ResourceSample>>>,
    
    /// メモリ使用率の履歴
    memory_usage: Arc<Mutex<Vec<ResourceSample>>>,
    
    /// ディスク使用率の履歴
    disk_usage: Arc<Mutex<Vec<ResourceSample>>>,
    
    /// ネットワーク使用率の履歴
    network_usage: Arc<Mutex<Vec<ResourceSample>>>,
    
    /// 最後の更新時間
    last_update: Arc<Mutex<Instant>>,
    
    /// 効率性スコア（0.0 - 1.0）
    efficiency_score: Arc<Mutex<f64>>,
}

/// リソースサンプル
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceSample {
    /// タイムスタンプ
    pub timestamp: u64,
    
    /// 使用率（0.0 - 1.0）
    pub usage: f64,
}

/// リソース使用状況
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceUsage {
    /// CPU使用率（0.0 - 1.0）
    pub cpu: f64,
    
    /// メモリ使用率（0.0 - 1.0）
    pub memory: f64,
    
    /// ディスク使用率（0.0 - 1.0）
    pub disk: f64,
    
    /// ネットワーク使用率（0.0 - 1.0）
    pub network: f64,
    
    /// 効率性スコア（0.0 - 1.0）
    pub efficiency: f64,
    
    /// タイムスタンプ
    pub timestamp: u64,
}

impl ResourceMonitor {
    /// 新しいリソースモニターを作成
    pub fn new() -> Self {
        Self {
            cpu_usage: Arc::new(Mutex::new(Vec::new())),
            memory_usage: Arc::new(Mutex::new(Vec::new())),
            disk_usage: Arc::new(Mutex::new(Vec::new())),
            network_usage: Arc::new(Mutex::new(Vec::new())),
            last_update: Arc::new(Mutex::new(Instant::now())),
            efficiency_score: Arc::new(Mutex::new(1.0)), // 初期値は最大効率
        }
    }
    
    /// リソース使用状況を更新
    pub fn update(&self) {
        // 実際の実装では、システムのリソース使用状況を取得
        // ここではダミーデータを使用
        let cpu = self.get_cpu_usage();
        let memory = self.get_memory_usage();
        let disk = self.get_disk_usage();
        let network = self.get_network_usage();
        
        // 現在の時間を取得
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // サンプルを作成
        let cpu_sample = ResourceSample {
            timestamp: now,
            usage: cpu,
        };
        
        let memory_sample = ResourceSample {
            timestamp: now,
            usage: memory,
        };
        
        let disk_sample = ResourceSample {
            timestamp: now,
            usage: disk,
        };
        
        let network_sample = ResourceSample {
            timestamp: now,
            usage: network,
        };
        
        // サンプルを追加
        {
            let mut cpu_usage = self.cpu_usage.lock().unwrap();
            cpu_usage.push(cpu_sample);
            
            // 履歴が長すぎる場合は古いサンプルを削除
            if cpu_usage.len() > 100 {
                cpu_usage.remove(0);
            }
        }
        
        {
            let mut memory_usage = self.memory_usage.lock().unwrap();
            memory_usage.push(memory_sample);
            
            if memory_usage.len() > 100 {
                memory_usage.remove(0);
            }
        }
        
        {
            let mut disk_usage = self.disk_usage.lock().unwrap();
            disk_usage.push(disk_sample);
            
            if disk_usage.len() > 100 {
                disk_usage.remove(0);
            }
        }
        
        {
            let mut network_usage = self.network_usage.lock().unwrap();
            network_usage.push(network_sample);
            
            if network_usage.len() > 100 {
                network_usage.remove(0);
            }
        }
        
        // 効率性スコアを計算
        self.calculate_efficiency();
        
        // 最後の更新時間を更新
        let mut last_update = self.last_update.lock().unwrap();
        *last_update = Instant::now();
    }
    
    /// CPU使用率を取得（ダミー実装）
    fn get_cpu_usage(&self) -> f64 {
        // 実際の実装では、システムのCPU使用率を取得
        // ここではランダムな値を返す
        rand::random::<f64>() * 0.5 // 0.0 - 0.5 の範囲（50%以下を想定）
    }
    
    /// メモリ使用率を取得（ダミー実装）
    fn get_memory_usage(&self) -> f64 {
        // 実際の実装では、システムのメモリ使用率を取得
        rand::random::<f64>() * 0.7 // 0.0 - 0.7 の範囲（70%以下を想定）
    }
    
    /// ディスク使用率を取得（ダミー実装）
    fn get_disk_usage(&self) -> f64 {
        // 実際の実装では、システムのディスク使用率を取得
        rand::random::<f64>() * 0.6 // 0.0 - 0.6 の範囲（60%以下を想定）
    }
    
    /// ネットワーク使用率を取得（ダミー実装）
    fn get_network_usage(&self) -> f64 {
        // 実際の実装では、システムのネットワーク使用率を取得
        rand::random::<f64>() * 0.4 // 0.0 - 0.4 の範囲（40%以下を想定）
    }
    
    /// 効率性スコアを計算
    fn calculate_efficiency(&self) {
        // CPU、メモリ、ディスク、ネットワークの平均使用率を計算
        let cpu_avg = self.get_average_usage(&self.cpu_usage);
        let memory_avg = self.get_average_usage(&self.memory_usage);
        let disk_avg = self.get_average_usage(&self.disk_usage);
        let network_avg = self.get_average_usage(&self.network_usage);
        
        // 重み付け係数
        let cpu_weight = 0.4;
        let memory_weight = 0.3;
        let disk_weight = 0.2;
        let network_weight = 0.1;
        
        // 効率性スコアの計算
        // 各リソースの使用率が低いほど効率が良い
        let cpu_efficiency = 1.0 - cpu_avg;
        let memory_efficiency = 1.0 - memory_avg;
        let disk_efficiency = 1.0 - disk_avg;
        let network_efficiency = 1.0 - network_avg;
        
        // 重み付け平均
        let efficiency = cpu_weight * cpu_efficiency +
                         memory_weight * memory_efficiency +
                         disk_weight * disk_efficiency +
                         network_weight * network_efficiency;
        
        // 効率性スコアを更新
        let mut efficiency_score = self.efficiency_score.lock().unwrap();
        *efficiency_score = efficiency;
    }
    
    /// 平均使用率を計算
    fn get_average_usage(&self, samples: &Arc<Mutex<Vec<ResourceSample>>>) -> f64 {
        let samples = samples.lock().unwrap();
        
        if samples.is_empty() {
            return 0.0;
        }
        
        let sum: f64 = samples.iter().map(|s| s.usage).sum();
        sum / samples.len() as f64
    }
    
    /// 効率性スコアを取得
    pub fn get_efficiency(&self) -> f64 {
        let efficiency_score = self.efficiency_score.lock().unwrap();
        *efficiency_score
    }
    
    /// 現在のリソース使用状況を取得
    pub fn get_current_usage(&self) -> ResourceUsage {
        let cpu = self.get_last_usage(&self.cpu_usage);
        let memory = self.get_last_usage(&self.memory_usage);
        let disk = self.get_last_usage(&self.disk_usage);
        let network = self.get_last_usage(&self.network_usage);
        let efficiency = self.get_efficiency();
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        ResourceUsage {
            cpu,
            memory,
            disk,
            network,
            efficiency,
            timestamp: now,
        }
    }
    
    /// 最新の使用率を取得
    fn get_last_usage(&self, samples: &Arc<Mutex<Vec<ResourceSample>>>) -> f64 {
        let samples = samples.lock().unwrap();
        
        if samples.is_empty() {
            return 0.0;
        }
        
        samples.last().unwrap().usage
    }
    
    /// 最後の更新からの経過時間を取得
    pub fn time_since_last_update(&self) -> Duration {
        let last_update = self.last_update.lock().unwrap();
        last_update.elapsed()
    }
    
    /// CPU使用率の履歴を取得
    pub fn get_cpu_history(&self) -> Vec<ResourceSample> {
        let cpu_usage = self.cpu_usage.lock().unwrap();
        cpu_usage.clone()
    }
    
    /// メモリ使用率の履歴を取得
    pub fn get_memory_history(&self) -> Vec<ResourceSample> {
        let memory_usage = self.memory_usage.lock().unwrap();
        memory_usage.clone()
    }
    
    /// ディスク使用率の履歴を取得
    pub fn get_disk_history(&self) -> Vec<ResourceSample> {
        let disk_usage = self.disk_usage.lock().unwrap();
        disk_usage.clone()
    }
    
    /// ネットワーク使用率の履歴を取得
    pub fn get_network_history(&self) -> Vec<ResourceSample> {
        let network_usage = self.network_usage.lock().unwrap();
        network_usage.clone()
    }
}

impl Default for ResourceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    
    #[test]
    fn test_resource_monitor_initialization() {
        let monitor = ResourceMonitor::new();
        
        // 初期状態では履歴は空
        assert!(monitor.get_cpu_history().is_empty());
        assert!(monitor.get_memory_history().is_empty());
        assert!(monitor.get_disk_history().is_empty());
        assert!(monitor.get_network_history().is_empty());
        
        // 初期効率性スコアは1.0
        assert_eq!(monitor.get_efficiency(), 1.0);
    }
    
    #[test]
    fn test_resource_monitor_update() {
        let monitor = ResourceMonitor::new();
        
        // リソース使用状況を更新
        monitor.update();
        
        // 更新後は履歴にサンプルが追加されている
        assert_eq!(monitor.get_cpu_history().len(), 1);
        assert_eq!(monitor.get_memory_history().len(), 1);
        assert_eq!(monitor.get_disk_history().len(), 1);
        assert_eq!(monitor.get_network_history().len(), 1);
        
        // 効率性スコアが計算されている
        assert!(monitor.get_efficiency() > 0.0);
        assert!(monitor.get_efficiency() <= 1.0);
    }
    
    #[test]
    fn test_resource_monitor_multiple_updates() {
        let monitor = ResourceMonitor::new();
        
        // 複数回更新
        for _ in 0..5 {
            monitor.update();
            thread::sleep(Duration::from_millis(10));
        }
        
        // 更新後は履歴にサンプルが追加されている
        assert_eq!(monitor.get_cpu_history().len(), 5);
        assert_eq!(monitor.get_memory_history().len(), 5);
        assert_eq!(monitor.get_disk_history().len(), 5);
        assert_eq!(monitor.get_network_history().len(), 5);
        
        // 現在の使用状況を取得
        let usage = monitor.get_current_usage();
        
        // 使用率は0.0 - 1.0の範囲
        assert!(usage.cpu >= 0.0 && usage.cpu <= 1.0);
        assert!(usage.memory >= 0.0 && usage.memory <= 1.0);
        assert!(usage.disk >= 0.0 && usage.disk <= 1.0);
        assert!(usage.network >= 0.0 && usage.network <= 1.0);
        assert!(usage.efficiency >= 0.0 && usage.efficiency <= 1.0);
    }
    
    #[test]
    fn test_time_since_last_update() {
        let monitor = ResourceMonitor::new();
        
        // 初期状態では最後の更新時間は現在
        assert!(monitor.time_since_last_update() < Duration::from_secs(1));
        
        // 少し待ってから経過時間を確認
        thread::sleep(Duration::from_millis(100));
        assert!(monitor.time_since_last_update() >= Duration::from_millis(100));
        
        // 更新すると経過時間がリセット
        monitor.update();
        assert!(monitor.time_since_last_update() < Duration::from_millis(100));
    }
}