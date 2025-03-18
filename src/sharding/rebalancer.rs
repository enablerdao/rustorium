use crate::common::config::ShardingConfig;
use crate::common::errors::ShardingError;
use crate::common::types::ShardId;
use crate::sharding::manager::ShardManagerMessage;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;
use tracing::{error, info};

/// Shard rebalancer periodically triggers rebalancing
pub struct ShardRebalancer {
    /// Shard manager sender
    manager_tx: mpsc::Sender<ShardManagerMessage>,
    /// Rebalance interval
    interval: Duration,
    /// Running flag
    running: bool,
}

impl ShardRebalancer {
    /// Create a new shard rebalancer
    pub fn new(
        manager_tx: mpsc::Sender<ShardManagerMessage>,
        config: &ShardingConfig,
    ) -> Self {
        let interval = Duration::from_secs(config.rebalance_interval_sec);
        
        Self {
            manager_tx,
            interval,
            running: false,
        }
    }
    
    /// Start the rebalancer
    pub async fn start(&mut self) -> Result<(), ShardingError> {
        if self.running {
            return Err(ShardingError::InvalidShardId(
                "Rebalancer already running".to_string(),
            ));
        }
        
        self.running = true;
        info!("Shard rebalancer started with interval of {} seconds", self.interval.as_secs());
        
        let manager_tx = self.manager_tx.clone();
        let interval = self.interval;
        
        tokio::spawn(async move {
            let mut timer = time::interval(interval);
            
            loop {
                timer.tick().await;
                
                info!("Triggering shard rebalance");
                if let Err(e) = manager_tx.send(ShardManagerMessage::Rebalance).await {
                    error!("Failed to send rebalance message: {}", e);
                    break;
                }
            }
            
            info!("Shard rebalancer stopped");
        });
        
        Ok(())
    }
    
    /// Stop the rebalancer
    pub fn stop(&mut self) {
        self.running = false;
    }
}

/// Start a new shard rebalancer
pub async fn start_rebalancer(
    manager_tx: mpsc::Sender<ShardManagerMessage>,
    config: &ShardingConfig,
) -> Result<ShardRebalancer, ShardingError> {
    let mut rebalancer = ShardRebalancer::new(manager_tx, config);
    rebalancer.start().await?;
    Ok(rebalancer)
}