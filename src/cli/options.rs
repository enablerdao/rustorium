use std::fs;
use anyhow::Result;
use libp2p::identity::Keypair;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rustorium")]
pub struct AppOptions {
    /// API server host
    #[structopt(long, env = "API_HOST")]
    pub api_host: Option<String>,

    /// API server port
    #[structopt(long, env = "API_PORT")]
    pub api_port: Option<u16>,

    /// WebSocket server port
    #[structopt(long, env = "WS_PORT")]
    pub ws_port: Option<u16>,

    /// GraphQL server port
    #[structopt(long, env = "GRAPHQL_PORT")]
    pub graphql_port: Option<u16>,

    /// Log level (debug, info, warn, error)
    #[structopt(long, env = "LOG_LEVEL")]
    pub log_level: Option<String>,

    /// CORS origin
    #[structopt(long, env = "CORS_ORIGIN")]
    pub cors_origin: Option<String>,

    /// Rate limit (requests per minute)
    #[structopt(long, env = "RATE_LIMIT")]
    pub rate_limit: Option<u32>,

    /// Development mode: Start multiple test nodes
    #[structopt(long)]
    pub dev: bool,

    /// Number of test nodes to start (in dev mode)
    #[structopt(long, default_value = "10")]
    pub nodes: u8,

    /// Base port for test nodes (in dev mode)
    #[structopt(long, default_value = "40000")]
    pub base_port: u16,

    /// Data directory
    #[structopt(long, default_value = "/tmp/rustorium")]
    pub data_dir: String,

    /// Node keypair file path
    #[structopt(long, env = "KEYPAIR_PATH")]
    pub keypair_path: Option<String>,

    /// P2P network address
    #[structopt(long, env = "P2P_ADDR", default_value = "/ip4/0.0.0.0/tcp/0")]
    pub p2p_addr: String,

    /// Bootstrap nodes
    #[structopt(long, env = "BOOTSTRAP_NODES")]
    pub bootstrap_nodes: Vec<String>,

    /// Initial shard count
    #[structopt(long, default_value = "1")]
    pub shard_count: u32,

    /// Maximum shard count
    #[structopt(long, default_value = "16")]
    pub max_shards: u32,

    /// Minimum nodes per shard
    #[structopt(long, default_value = "4")]
    pub min_nodes_per_shard: u32,
}

impl AppOptions {
    pub fn new() -> Self {
        Self::from_args()
    }

    /// 鍵ペアを読み込む
    pub fn load_keypair(&self) -> Result<Keypair> {
        if let Some(path) = &self.keypair_path {
            // ファイルから鍵ペアを読み込む
            let bytes = fs::read(path)?;
            Ok(Keypair::from_protobuf_encoding(&bytes)?)
        } else {
            // 新しい鍵ペアを生成
            Ok(Keypair::generate_ed25519())
        }
    }
}
}