//! カスタムランタイムモジュール

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use rustorium_core::{
    Module, ModuleConfig, ModuleStatus, ModuleMetrics,
    runtime::RuntimeModule,
};
use async_trait::async_trait;
use tracing::info;

/// カスタムランタイムモジュール
pub struct CustomRuntimeModule {
    /// 設定
    config: ModuleConfig,
    /// ステータス
    status: ModuleStatus,
    /// コントラクト
    contracts: Arc<RwLock<HashMap<Vec<u8>, Vec<u8>>>>,
}

impl CustomRuntimeModule {
    /// 新しいカスタムランタイムモジュールを作成
    pub fn new(config: ModuleConfig) -> Self {
        Self {
            config,
            status: ModuleStatus::Uninitialized,
            contracts: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl Module for CustomRuntimeModule {
    async fn init(&mut self) -> anyhow::Result<()> {
        info!("Initializing custom runtime module...");
        self.status = ModuleStatus::Initialized;
        info!("Custom runtime module initialized");
        Ok(())
    }

    async fn start(&mut self) -> anyhow::Result<()> {
        info!("Starting custom runtime module...");
        self.status = ModuleStatus::Running;
        info!("Custom runtime module started");
        Ok(())
    }

    async fn stop(&mut self) -> anyhow::Result<()> {
        info!("Stopping custom runtime module...");
        self.status = ModuleStatus::Stopped;
        info!("Custom runtime module stopped");
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
impl RuntimeModule for CustomRuntimeModule {
    async fn deploy(&mut self, code: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let contract_id = code.clone();
        self.contracts.write().await.insert(contract_id.clone(), code);
        info!("Deployed contract: {}", hex::encode(&contract_id));
        Ok(contract_id)
    }

    async fn execute(&mut self, contract: Vec<u8>, input: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let contracts = self.contracts.read().await;
        if let Some(_code) = contracts.get(&contract) {
            // TODO: 実際のコード実行を実装
            info!("Executed contract: {}", hex::encode(&contract));
            Ok(input)
        } else {
            anyhow::bail!("Contract not found")
        }
    }

    async fn get_state(&self, contract: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let contracts = self.contracts.read().await;
        if let Some(code) = contracts.get(&contract) {
            Ok(code.clone())
        } else {
            anyhow::bail!("Contract not found")
        }
    }

    async fn delete(&mut self, contract: Vec<u8>) -> anyhow::Result<()> {
        self.contracts.write().await.remove(&contract);
        info!("Deleted contract: {}", hex::encode(&contract));
        Ok(())
    }
}
