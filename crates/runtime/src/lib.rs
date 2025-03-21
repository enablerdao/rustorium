//! Rustoriumランタイムモジュール

pub mod wasm;
pub mod evm;
pub mod move_vm;
pub mod custom;

pub use wasm::WasmModule;
pub use evm::EvmModule;
pub use move_vm::MoveModule;
pub use custom::CustomRuntimeModule;

use rustorium_core::{
    Module, ModuleConfig, ModuleStatus, ModuleMetrics,
    runtime::{RuntimeModule, RuntimeModuleFactory},
};

/// ランタイムモジュールのファクトリ実装
pub struct RuntimeModuleFactoryImpl;

impl RuntimeModuleFactory for RuntimeModuleFactoryImpl {
    fn create(config: ModuleConfig) -> Box<dyn RuntimeModule> {
        match config.name.as_str() {
            "wasm" => Box::new(WasmModule::new(config)),
            "evm" => Box::new(EvmModule::new(config)),
            "move" => Box::new(MoveModule::new(config)),
            "custom" => Box::new(CustomRuntimeModule::new(config)),
            _ => panic!("Unknown runtime module: {}", config.name),
        }
    }
}
