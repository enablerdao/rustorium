//! WebAssemblyランタイムモジュール

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use wasmer::{Store, Module, Instance, Memory, Value, Function};
use rustorium_core::{
    Module as CoreModule, ModuleConfig, ModuleStatus, ModuleMetrics,
    runtime::RuntimeModule,
};
use async_trait::async_trait;
use tracing::info;

/// WebAssemblyランタイムモジュール
pub struct WasmModule {
    /// 設定
    config: ModuleConfig,
    /// ステータス
    status: ModuleStatus,
    /// Wasmerストア
    store: Option<Store>,
    /// デプロイ済みモジュール
    modules: Arc<RwLock<HashMap<Vec<u8>, Module>>>,
    /// モジュールインスタンス
    instances: Arc<RwLock<HashMap<Vec<u8>, Instance>>>,
}

impl WasmModule {
    /// 新しいWebAssemblyランタイムモジュールを作成
    pub fn new(config: ModuleConfig) -> Self {
        Self {
            config,
            status: ModuleStatus::Uninitialized,
            store: None,
            modules: Arc::new(RwLock::new(HashMap::new())),
            instances: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// モジュールのインスタンス化
    async fn instantiate_module(&self, code: &[u8]) -> anyhow::Result<Instance> {
        if let Some(store) = &self.store {
            let module = Module::new(&store, code)?;
            let instance = Instance::new(&module, &[])?;
            Ok(instance)
        } else {
            anyhow::bail!("Store not initialized")
        }
    }
}

#[async_trait]
impl CoreModule for WasmModule {
    async fn init(&mut self) -> anyhow::Result<()> {
        info!("Initializing WebAssembly runtime module...");
        self.store = Some(Store::default());
        self.status = ModuleStatus::Initialized;
        info!("WebAssembly runtime module initialized");
        Ok(())
    }

    async fn start(&mut self) -> anyhow::Result<()> {
        info!("Starting WebAssembly runtime module...");
        self.status = ModuleStatus::Running;
        info!("WebAssembly runtime module started");
        Ok(())
    }

    async fn stop(&mut self) -> anyhow::Result<()> {
        info!("Stopping WebAssembly runtime module...");
        self.store = None;
        self.modules.write().await.clear();
        self.instances.write().await.clear();
        self.status = ModuleStatus::Stopped;
        info!("WebAssembly runtime module stopped");
        Ok(())
    }

    async fn status(&self) -> anyhow::Result<ModuleStatus> {
        Ok(self.status.clone())
    }

    async fn metrics(&self) -> anyhow::Result<ModuleMetrics> {
        let mut metrics = HashMap::new();
        metrics.insert("deployed_modules".to_string(), self.modules.read().await.len() as f64);
        metrics.insert("active_instances".to_string(), self.instances.read().await.len() as f64);
        
        Ok(ModuleMetrics {
            timestamp: std::time::SystemTime::now(),
            metrics,
        })
    }
}

#[async_trait]
impl RuntimeModule for WasmModule {
    async fn deploy(&mut self, code: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let instance = self.instantiate_module(&code).await?;
        let module_id = code.clone();
        
        self.modules.write().await.insert(module_id.clone(), Module::new(self.store.as_ref().unwrap(), &code)?);
        self.instances.write().await.insert(module_id.clone(), instance);
        
        info!("Deployed module: {}", hex::encode(&module_id));
        Ok(module_id)
    }

    async fn execute(&mut self, contract: Vec<u8>, input: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let instances = self.instances.read().await;
        if let Some(instance) = instances.get(&contract) {
            let memory = instance.exports.get_memory("memory")?;
            let execute = instance.exports.get_function("execute")?;
            
            // メモリにinputを書き込む
            let input_ptr = memory.data_size() as u32;
            memory.grow(((input.len() + 0xffff) & !0xffff) as u32 / 0x10000)?;
            memory.view()[input_ptr as usize..][..input.len()].copy_from_slice(&input);
            
            // 関数を実行
            let result = execute.call(&[Value::I32(input_ptr as i32), Value::I32(input.len() as i32)])?;
            
            // 結果を読み取る
            let result_ptr = result[0].unwrap_i32() as u32;
            let result_len = result[1].unwrap_i32() as u32;
            let result_data = memory.view()[result_ptr as usize..][..result_len as usize].to_vec();
            
            info!("Executed module: {}", hex::encode(&contract));
            Ok(result_data)
        } else {
            anyhow::bail!("Contract not found")
        }
    }

    async fn get_state(&self, contract: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let instances = self.instances.read().await;
        if let Some(instance) = instances.get(&contract) {
            let memory = instance.exports.get_memory("memory")?;
            let get_state = instance.exports.get_function("get_state")?;
            
            // 関数を実行
            let result = get_state.call(&[])?;
            
            // 結果を読み取る
            let result_ptr = result[0].unwrap_i32() as u32;
            let result_len = result[1].unwrap_i32() as u32;
            let result_data = memory.view()[result_ptr as usize..][..result_len as usize].to_vec();
            
            Ok(result_data)
        } else {
            anyhow::bail!("Contract not found")
        }
    }

    async fn delete(&mut self, contract: Vec<u8>) -> anyhow::Result<()> {
        self.modules.write().await.remove(&contract);
        self.instances.write().await.remove(&contract);
        info!("Deleted module: {}", hex::encode(&contract));
        Ok(())
    }
}
