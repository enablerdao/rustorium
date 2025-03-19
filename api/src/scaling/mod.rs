// 適応型スケーリングの基盤設計
// ノード数に応じて自動的にスケールする仕組み

mod sharding;
mod load_balancer;
mod metrics;
mod adaptive_config;

pub use sharding::ShardManager;
pub use load_balancer::LoadBalancer;
pub use metrics::ScalingMetrics;
pub use adaptive_config::AdaptiveConfig;

use crate::blockchain::Block;
use crate::consensus::ResourceMonitor;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// スケーリングモード
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ScalingMode {
    /// 自動スケーリング
    Automatic,
    /// 手動スケーリング
    Manual,
    /// ハイブリッドスケーリング
    Hybrid,
}

/// スケーリング設定
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScalingConfig {
    /// スケーリングモード
    pub mode: ScalingMode,
    
    /// 最小シャード数
    pub min_shards: usize,
    
    /// 最大シャード数
    pub max_shards: usize,
    
    /// ノードあたりの最適トランザクション数
    pub optimal_tx_per_node: usize,
    
    /// スケールアップのしきい値（CPU使用率）
    pub scale_up_threshold: f64,
    
    /// スケールダウンのしきい値（CPU使用率）
    pub scale_down_threshold: f64,
    
    /// スケーリング間隔（秒）
    pub scaling_interval: u64,
    
    /// 適応型設定の有効化
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

/// スケーリングステータス
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScalingStatus {
    /// 現在のシャード数
    pub current_shards: usize,
    
    /// 現在のノード数
    pub current_nodes: usize,
    
    /// 現在のCPU使用率
    pub cpu_usage: f64,
    
    /// 現在のメモリ使用率
    pub memory_usage: f64,
    
    /// 1秒あたりのトランザクション数
    pub tps: f64,
    
    /// スケーリングモード
    pub mode: ScalingMode,
    
    /// 最後のスケーリング時間
    pub last_scaling: String,
    
    /// 次のスケーリング予定時間
    pub next_scaling: String,
    
    /// スケーリング推奨
    pub scaling_recommendation: String,
}

/// スケーリングマネージャー
/// ノード数に応じて自動的にスケールする仕組み
pub struct ScalingManager {
    /// スケーリング設定
    config: ScalingConfig,
    
    /// シャードマネージャー
    shard_manager: ShardManager,
    
    /// ロードバランサー
    load_balancer: LoadBalancer,
    
    /// スケーリングメトリクス
    metrics: ScalingMetrics,
    
    /// リソースモニター
    resource_monitor: ResourceMonitor,
    
    /// 適応型設定
    adaptive_config: AdaptiveConfig,
    
    /// 最後のスケーリング時間
    last_scaling: Arc<Mutex<Instant>>,
    
    /// スケーリングステータス
    status: Arc<Mutex<ScalingStatus>>,
}

impl ScalingManager {
    /// 新しいスケーリングマネージャーを作成
    pub fn new(config: ScalingConfig) -> Self {
        let shard_manager = ShardManager::new(config.min_shards);
        let load_balancer = LoadBalancer::new();
        let metrics = ScalingMetrics::new();
        let resource_monitor = ResourceMonitor::new();
        let adaptive_config = AdaptiveConfig::new();
        
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
            load_balancer,
            metrics,
            resource_monitor,
            adaptive_config,
            last_scaling: Arc::new(Mutex::new(Instant::now())),
            status: Arc::new(Mutex::new(status)),
        }
    }
    
    /// スケーリングを実行
    pub fn scale(&self) -> Result<(), String> {
        // 自動スケーリングが無効の場合は何もしない
        if self.config.mode == ScalingMode::Manual {
            return Ok(());
        }
        
        // 最後のスケーリングからの経過時間を確認
        let last_scaling = self.last_scaling.lock().unwrap();
        let elapsed = last_scaling.elapsed().as_secs();
        
        if elapsed < self.config.scaling_interval {
            return Ok(());
        }
        
        // リソース使用状況を取得
        self.resource_monitor.update();
        let usage = self.resource_monitor.get_current_usage();
        
        // 現在のメトリクスを取得
        let metrics = self.metrics.get_current_metrics();
        
        // スケーリング判断
        let current_shards = self.shard_manager.get_shard_count();
        let mut new_shards = current_shards;
        
        let mut recommendation = "No action needed".to_string();
        
        if usage.cpu > self.config.scale_up_threshold && current_shards < self.config.max_shards {
            // スケールアップ
            new_shards = current_shards + 1;
            recommendation = format!("Scale up from {} to {} shards due to high CPU usage ({})",
                current_shards, new_shards, usage.cpu);
        } else if usage.cpu < self.config.scale_down_threshold && current_shards > self.config.min_shards {
            // スケールダウン
            new_shards = current_shards - 1;
            recommendation = format!("Scale down from {} to {} shards due to low CPU usage ({})",
                current_shards, new_shards, usage.cpu);
        }
        
        // シャード数を更新
        if new_shards != current_shards {
            self.shard_manager.set_shard_count(new_shards)?;
            
            // ロードバランサーを更新
            self.load_balancer.update_shards(self.shard_manager.get_shards());
        }
        
        // 適応型設定の更新
        if self.config.enable_adaptive_config {
            self.adaptive_config.update(&usage, &metrics);
        }
        
        // ステータスの更新
        let now = chrono::Utc::now();
        let next_scaling = now + chrono::Duration::seconds(self.config.scaling_interval as i64);
        
        let mut status = self.status.lock().unwrap();
        status.current_shards = new_shards;
        status.current_nodes = metrics.node_count;
        status.cpu_usage = usage.cpu;
        status.memory_usage = usage.memory;
        status.tps = metrics.tps;
        status.last_scaling = now.to_rfc3339();
        status.next_scaling = next_scaling.to_rfc3339();
        status.scaling_recommendation = recommendation;
        
        // 最後のスケーリング時間を更新
        drop(last_scaling);
        let mut last_scaling = self.last_scaling.lock().unwrap();
        *last_scaling = Instant::now();
        
        Ok(())
    }
    
    /// ブロックをシャードに割り当て
    pub fn assign_block(&self, block: &Block) -> usize {
        self.load_balancer.assign_block(block)
    }
    
    /// スケーリングステータスを取得
    pub fn get_status(&self) -> ScalingStatus {
        self.status.lock().unwrap().clone()
    }
    
    /// スケーリング設定を更新
    pub fn update_config(&mut self, config: ScalingConfig) {
        self.config = config;
        
        // ステータスの更新
        let mut status = self.status.lock().unwrap();
        status.mode = self.config.mode.clone();
    }
    
    /// 手動でシャード数を設定
    pub fn set_shard_count(&self, count: usize) -> Result<(), String> {
        if count < self.config.min_shards || count > self.config.max_shards {
            return Err(format!("Shard count must be between {} and {}",
                self.config.min_shards, self.config.max_shards));
        }
        
        self.shard_manager.set_shard_count(count)?;
        
        // ロードバランサーを更新
        self.load_balancer.update_shards(self.shard_manager.get_shards());
        
        // ステータスの更新
        let mut status = self.status.lock().unwrap();
        status.current_shards = count;
        
        Ok(())
    }
    
    /// メトリクスを更新
    pub fn update_metrics(&self, tps: f64, node_count: usize) {
        self.metrics.update(tps, node_count);
    }
}