use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub node: NodeConfig,
    pub network: NetworkConfig,
    pub storage: StorageConfig,
    pub sharding: ShardingConfig,
    pub consensus: ConsensusConfig,
    pub api: ApiConfig,
    pub vm: VmConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            node: NodeConfig::default(),
            network: NetworkConfig::default(),
            storage: StorageConfig::default(),
            sharding: ShardingConfig::default(),
            consensus: ConsensusConfig::default(),
            api: ApiConfig::default(),
            vm: VmConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub id: String,
    pub data_dir: PathBuf,
    pub log_level: String,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            id: "node-1".to_string(),
            data_dir: PathBuf::from("./data"),
            log_level: "info".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub listen_addr: IpAddr,
    pub listen_port: u16,
    pub bootstrap_nodes: Vec<String>,
    pub max_peers: usize,
    pub gossip_interval_ms: u64,
    pub connection_timeout_ms: u64,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            listen_addr: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            listen_port: 30333,
            bootstrap_nodes: vec![],
            max_peers: 50,
            gossip_interval_ms: 1000,
            connection_timeout_ms: 5000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub db_path: PathBuf,
    pub cache_size_mb: usize,
    pub max_open_files: i32,
    pub compaction_style: String,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            db_path: PathBuf::from("./data/db"),
            cache_size_mb: 512,
            max_open_files: 1000,
            compaction_style: "level".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardingConfig {
    pub shard_count: u32,
    pub rebalance_interval_sec: u64,
    pub min_shard_size: usize,
    pub max_shard_size: usize,
}

impl Default for ShardingConfig {
    fn default() -> Self {
        Self {
            shard_count: 4,
            rebalance_interval_sec: 3600,
            min_shard_size: 1000,
            max_shard_size: 1000000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusConfig {
    pub algorithm: String,
    pub block_time_ms: u64,
    pub validators: Vec<String>,
    pub min_validators: usize,
    pub threshold_percentage: u8,
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            algorithm: "avalanche".to_string(),
            block_time_ms: 2000,
            validators: vec![],
            min_validators: 4,
            threshold_percentage: 67,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub enabled: bool,
    pub listen_addr: IpAddr,
    pub listen_port: u16,
    pub cors_allowed_origins: Vec<String>,
    pub rate_limit_requests: u32,
    pub rate_limit_period_sec: u64,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            listen_addr: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            listen_port: 8080,
            cors_allowed_origins: vec!["*".to_string()],
            rate_limit_requests: 100,
            rate_limit_period_sec: 60,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmConfig {
    pub evm_enabled: bool,
    pub move_vm_enabled: bool,
    pub solana_vm_enabled: bool,
    pub wasm_enabled: bool,
    pub gas_limit: u64,
}

impl Default for VmConfig {
    fn default() -> Self {
        Self {
            evm_enabled: true,
            move_vm_enabled: false,
            solana_vm_enabled: false,
            wasm_enabled: true,
            gas_limit: 10_000_000,
        }
    }
}

pub fn load_config(path: &str) -> Result<Config, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::with_name(path))
        .add_source(config::Environment::with_prefix("RUSTLEDGER"))
        .build()?;
    
    settings.try_deserialize()
}

pub fn save_config(config: &Config, path: &str) -> Result<(), std::io::Error> {
    let toml = toml::to_string(config).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    std::fs::write(path, toml)
}