//! EVMランタイムモジュール

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use rustorium_core::{
    Module, ModuleConfig, ModuleStatus, ModuleMetrics,
    runtime::RuntimeModule,
};
use async_trait::async_trait;
use tracing::info;

/// EVMランタイムモジュール
pub struct EvmModule {
    /// 設定
    config: ModuleConfig,
    /// ステータス
    status: ModuleStatus,
    /// コントラクト
    contracts: Arc<RwLock<HashMap<Vec<u8>, Vec<u8>>>>,
}

impl EvmModule {
    /// 新しいEVMランタイムモジュールを作成
    pub fn new(config: ModuleConfig) -> Self {
        Self {
            config,
            status: ModuleStatus::Uninitialized,
            contracts: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl Module for EvmModule {
    async fn init(&mut self) -> anyhow::Result<()> {
        info!("Initializing EVM runtime module...");
        self.status = ModuleStatus::Initialized;
        info!("EVM runtime module initialized");
        Ok(())
    }

    async fn start(&mut self) -> anyhow::Result<()> {
        info!("Starting EVM runtime module...");
        self.status = ModuleStatus::Running;
        info!("EVM runtime module started");
        Ok(())
    }

    async fn stop(&mut self) -> anyhow::Result<()> {
        info!("Stopping EVM runtime module...");
        self.status = ModuleStatus::Stopped;
        info!("EVM runtime module stopped");
        Ok(())
    }

    async fn status(&self) -> anyhow::Result<ModuleStatus> {
        Ok(self.status.clone())
    }

    async fn metrics(&self) -> anyhow::Result<ModuleMetrics> {
        let mut metrics = HashMap::new();
        metrics.insert("deployed_contracts".to_string(), self.contracts.read().await.len() as f64);
        
        Ok(ModuleMetrics {
            timestamp: std::time::SystemTime::now(),
            metrics,
        })
    }
}

#[async_trait]
impl RuntimeModule for EvmModule {
    async fn deploy(&mut self, code: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        // TODO: 実装
        unimplemented!()
    }

    async fn execute(&mut self, contract: Vec<u8>, input: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        // TODO: 実装
        unimplemented!()
    }

    async fn get_state(&self, contract: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        // TODO: 実装
        unimplemented!()
    }

    async fn delete(&mut self, contract: Vec<u8>) -> anyhow::Result<()> {
        // TODO: 実装
        unimplemented!()
    }
}
