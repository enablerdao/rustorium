use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod blockchain;
mod integrated_server;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets the log level
    #[arg(short, long, default_value = "info")]
    log_level: String,

    /// Port to listen on
    #[arg(short, long, default_value_t = 57620)]
    port: u16,
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
    
    // 統合サーバーを起動
    info!("Starting Rustorium integrated server on port {}", cli.port);
    integrated_server::start_integrated_server(cli.port).await?;
    
    Ok(())
}
