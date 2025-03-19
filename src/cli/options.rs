use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rustorium")]
pub struct AppOptions {
    /// API server port
    #[structopt(long, env = "API_PORT")]
    pub api_port: Option<u16>,

    /// Frontend server port
    #[structopt(long, env = "FRONTEND_PORT")]
    pub frontend_port: Option<u16>,

    /// Log level (debug, info, warn, error)
    #[structopt(long, env = "LOG_LEVEL")]
    pub log_level: Option<String>,

    /// CORS origin
    #[structopt(long, env = "CORS_ORIGIN")]
    pub cors_origin: Option<String>,

    /// Development mode: Start multiple test nodes
    #[structopt(long)]
    pub dev: bool,

    /// Number of test nodes to start (in dev mode)
    #[structopt(long, default_value = "10")]
    pub nodes: u8,

    /// Base port for test nodes (in dev mode)
    #[structopt(long, default_value = "40000")]
    pub base_port: u16,

    /// Data directory for test nodes (in dev mode)
    #[structopt(long, default_value = "/tmp/rustorium_test")]
    pub data_dir: String,

    /// Start API server only
    #[structopt(long)]
    pub api_only: bool,

    /// Start frontend server only
    #[structopt(long)]
    pub frontend_only: bool,

    /// Fast development mode (lower optimization)
    #[structopt(long)]
    pub fast: bool,

    /// Release mode (higher optimization)
    #[structopt(long)]
    pub release: bool,

    /// Run sustainable blockchain demo
    #[structopt(long)]
    pub sustainable_demo: bool,
}

impl AppOptions {
    pub fn new() -> Self {
        Self::from_args()
    }
}