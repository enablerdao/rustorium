//! Rustoriumネットワークモジュール

pub mod quic;
pub mod libp2p;
pub mod custom;

pub use quic::QuicNetworkModule;
pub use libp2p::Libp2pNetworkModule;
pub use custom::CustomNetworkModule;

use rustorium_core::{
    Module, ModuleConfig, ModuleStatus, ModuleMetrics,
    network::{NetworkModule, NetworkModuleFactory},
};

/// ネットワークモジュールのファクトリ実装
pub struct NetworkModuleFactoryImpl;

impl NetworkModuleFactory for NetworkModuleFactoryImpl {
    fn create(config: ModuleConfig) -> Box<dyn NetworkModule> {
        match config.name.as_str() {
            "quic" => Box::new(QuicNetworkModule::new(config)),
            "libp2p" => Box::new(Libp2pNetworkModule::new(config)),
            "custom" => Box::new(CustomNetworkModule::new(config)),
            _ => panic!("Unknown network module: {}", config.name),
        }
    }
}
