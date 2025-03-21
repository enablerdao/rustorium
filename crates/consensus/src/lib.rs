//! Rustoriumコンセンサスモジュール

pub mod hotstuff;
pub mod avalanche;
pub mod tendermint;
pub mod raft;

pub use hotstuff::HotStuffModule;
pub use avalanche::AvalancheModule;
pub use tendermint::TendermintModule;
pub use raft::RaftModule;

use rustorium_core::{
    Module, ModuleConfig, ModuleStatus, ModuleMetrics,
    consensus::{ConsensusModule, ConsensusModuleFactory},
};

/// コンセンサスモジュールのファクトリ実装
pub struct ConsensusModuleFactoryImpl;

impl ConsensusModuleFactory for ConsensusModuleFactoryImpl {
    fn create(config: ModuleConfig) -> Box<dyn ConsensusModule> {
        match config.name.as_str() {
            "hotstuff" => Box::new(HotStuffModule::new(config)),
            "avalanche" => Box::new(AvalancheModule::new(config)),
            "tendermint" => Box::new(TendermintModule::new(config)),
            "raft" => Box::new(RaftModule::new(config)),
            _ => panic!("Unknown consensus module: {}", config.name),
        }
    }
}
