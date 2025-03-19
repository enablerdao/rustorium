mod app;
mod commands;
mod config;
mod display;
mod api;
mod utils;

use app::App;
use clap::{Parser, Subcommand};
use colored::*;
use std::process;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// API endpoint URL
    #[arg(short, long, default_value = "http://localhost:50128")]
    api_url: String,

    /// Enable debug mode
    #[arg(short, long)]
    debug: bool,

    /// Command to execute
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start interactive shell
    Shell,
    
    /// Manage accounts and wallets
    Account {
        #[command(subcommand)]
        action: commands::account::AccountCommands,
    },
    
    /// View block information
    Block {
        #[command(subcommand)]
        action: commands::block::BlockCommands,
    },
    
    /// Deploy and interact with smart contracts
    Contract {
        #[command(subcommand)]
        action: commands::contract::ContractCommands,
    },
    
    /// View and configure network settings
    Network {
        #[command(subcommand)]
        action: commands::network::NetworkCommands,
    },
    
    /// Manage tokens (ERC-20/ERC-721)
    Token {
        #[command(subcommand)]
        action: commands::token::TokenCommands,
    },
    
    /// Create and manage transactions
    Tx {
        #[command(subcommand)]
        action: commands::tx::TxCommands,
    },
    
    /// System and node management
    System {
        #[command(subcommand)]
        action: commands::system::SystemCommands,
    },
    
    /// Configure node settings
    Config {
        #[command(subcommand)]
        action: commands::config::ConfigCommands,
    },
    
    /// Debugging tools
    Debug {
        #[command(subcommand)]
        action: commands::debug::DebugCommands,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logger
    env_logger::init();
    
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Set up API client
    let api_client = api::ApiClient::new(&cli.api_url);
    
    // Check if API is reachable
    match api_client.check_connection().await {
        Ok(_) => {
            if cli.debug {
                println!("{}", "API connection successful".green());
            }
        }
        Err(e) => {
            eprintln!("{}: {}", "Error connecting to API".red(), e);
            process::exit(1);
        }
    }
    
    // Create app instance
    let mut app = App::new(api_client, cli.debug);
    
    // Process command or start interactive shell
    match cli.command {
        Some(Commands::Shell) | None => {
            // Start interactive shell
            app.run_interactive_shell().await?;
        }
        Some(Commands::Account { action }) => {
            commands::account::handle_command(&mut app, action).await?;
        }
        Some(Commands::Block { action }) => {
            commands::block::handle_command(&mut app, action).await?;
        }
        Some(Commands::Contract { action }) => {
            commands::contract::handle_command(&mut app, action).await?;
        }
        Some(Commands::Network { action }) => {
            commands::network::handle_command(&mut app, action).await?;
        }
        Some(Commands::Token { action }) => {
            commands::token::handle_command(&mut app, action).await?;
        }
        Some(Commands::Tx { action }) => {
            commands::tx::handle_command(&mut app, action).await?;
        }
        Some(Commands::System { action }) => {
            commands::system::handle_command(&mut app, action).await?;
        }
        Some(Commands::Config { action }) => {
            commands::config::handle_command(&mut app, action).await?;
        }
        Some(Commands::Debug { action }) => {
            commands::debug::handle_command(&mut app, action).await?;
        }
    }
    
    Ok(())
}