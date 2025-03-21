//! Rustoriumストレージモジュール

pub mod tikv;
pub mod rocksdb;
pub mod custom;

pub use tikv::TiKVModule;
pub use rocksdb::RocksDBModule;
pub use custom::CustomStorageModule;

use rustorium_core::{
    Module, ModuleConfig, ModuleStatus, ModuleMetrics,
    storage::{StorageModule, StorageModuleFactory, StorageOperation},
};

/// ストレージモジュールのファクトリ実装
pub struct StorageModuleFactoryImpl;

impl StorageModuleFactory for StorageModuleFactoryImpl {
    fn create(config: ModuleConfig) -> Box<dyn StorageModule> {
        match config.name.as_str() {
            "tikv" => Box::new(TiKVModule::new(config)),
            "rocksdb" => Box::new(RocksDBModule::new(config)),
            "custom" => Box::new(CustomStorageModule::new(config)),
            _ => panic!("Unknown storage module: {}", config.name),
        }
    }
}
