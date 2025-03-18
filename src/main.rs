use anyhow::Result;
use clap::{Parser, Subcommand};
use rustorium::{
    common::config::{load_config, Config},
    common::types::{Address, Transaction, VmType},
    common::utils,
    storage::init::initialize_storage,
    Node,
};
use std::path::Path;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets the log level
    #[arg(short, long, default_value = "info")]
    log_level: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a full node
    Node {
        /// Path to the configuration file
        #[arg(short, long, default_value = "config.toml")]
        config: String,
        /// Enable API server
        #[arg(long, default_value_t = true)]
        api: bool,
        /// Enable web interface
        #[arg(long, default_value_t = true)]
        web: bool,
        /// API port
        #[arg(long, default_value_t = 8080)]
        api_port: u16,
        /// Web port
        #[arg(long, default_value_t = 8081)]
        web_port: u16,
    },
    /// Start only the API server
    Api {
        /// Port to listen on
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
    /// Start only the web interface
    Web {
        /// Port to listen on
        #[arg(short, long, default_value_t = 8081)]
        port: u16,
        /// API server URL
        #[arg(long, default_value = "http://localhost:8080")]
        api_url: String,
    },
    /// Initialize a new ledger
    Init {
        /// Path to store the ledger data
        #[arg(short, long, default_value = "./data")]
        path: String,
    },
    /// Send a test transaction
    SendTx {
        /// Sender address
        #[arg(long)]
        from: String,
        /// Recipient address
        #[arg(long)]
        to: String,
        /// Amount to send
        #[arg(long)]
        amount: u64,
        /// Transaction fee
        #[arg(long, default_value = "1")]
        fee: u64,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Set up logging
    let log_level = match cli.log_level.to_lowercase().as_str() {
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        "trace" => Level::TRACE,
        _ => Level::INFO,
    };
    
    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
    
    match cli.command {
        Commands::Node { config, api, web, api_port, web_port } => {
            info!("Starting Rustorium node with config: {}", config);
            
            // Load configuration
            let mut config = match load_config(&config) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Failed to load configuration: {}", e);
                    // Use default configuration if loading fails
                    Config::default()
                }
            };
            
            // Override API configuration
            config.api.enabled = api;
            config.api.listen_port = api_port;
            
            // Create and start node
            let mut node = Node::new(config);
            node.init().await?;
            
            // Start web server if enabled
            if web {
                info!("Starting web interface on port {}", web_port);
                let web_config = rustorium::api::web::WebServerConfig {
                    port: web_port,
                    static_dir: std::path::PathBuf::from("./web/dist"),
                };
                
                tokio::spawn(async move {
                    if let Err(e) = rustorium::api::web::start_web_server(web_config).await {
                        error!("Web server error: {}", e);
                    }
                });
            }
            
            // Start node
            node.start().await?;
            
            // Keep the node running
            tokio::signal::ctrl_c().await?;
            info!("Received shutdown signal");
            
            // Stop the node
            node.stop().await?;
        }
        Commands::Api { port } => {
            info!("Starting API server on port {}", port);
            // Use the standalone API server that includes sample data
            rustorium::api::standalone::run_standalone_api(port).await?;
        }
        Commands::Web { port, api_url } => {
            info!("Starting web interface on port {} with API URL: {}", port, api_url);
            
            let web_config = rustorium::api::web::WebServerConfig {
                port,
                static_dir: std::path::PathBuf::from("./web/dist"),
            };
            
            rustorium::api::web::start_web_server(web_config).await?;
        }
        Commands::Init { path } => {
            info!("Initializing new ledger at {}", path);
            initialize_storage(&path)?;
        }
        Commands::SendTx { from, to, amount, fee } => {
            info!("Sending test transaction");
            
            // Parse addresses
            let from_bytes = match hex::decode(from.trim_start_matches("0x")) {
                Ok(bytes) => {
                    if bytes.len() != 20 {
                        eprintln!("Invalid sender address length");
                        return Ok(());
                    }
                    let mut array = [0u8; 20];
                    array.copy_from_slice(&bytes);
                    array
                }
                Err(e) => {
                    eprintln!("Invalid sender address format: {}", e);
                    return Ok(());
                }
            };
            
            let to_bytes = match hex::decode(to.trim_start_matches("0x")) {
                Ok(bytes) => {
                    if bytes.len() != 20 {
                        eprintln!("Invalid recipient address length");
                        return Ok(());
                    }
                    let mut array = [0u8; 20];
                    array.copy_from_slice(&bytes);
                    array
                }
                Err(e) => {
                    eprintln!("Invalid recipient address format: {}", e);
                    return Ok(());
                }
            };
            
            // Create transaction
            let tx = Transaction::new(
                Address(from_bytes),
                Address(to_bytes),
                amount,
                fee,
                0, // nonce
                vec![], // data
                VmType::Evm,
            );
            
            println!("Transaction created with ID: {}", tx.id);
            
            // TODO: Send transaction to node
            println!("Transaction sending not yet implemented");
        }
    }
    
    Ok(())
}
