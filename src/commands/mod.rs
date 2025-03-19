pub mod account;
pub mod tx;
pub mod contract;
pub mod token;
pub mod network;
pub mod dev;
pub mod analytics;
pub mod security;
pub mod system;
pub mod config;

use async_trait::async_trait;
use anyhow::Result;
use clap::Parser;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::core::storage::rocksdb::RocksDBStorage;
use crate::network::P2PNetwork;

#[async_trait]
pub trait Command {
    async fn run(&self) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct CommandContext {
    pub storage: Arc<RocksDBStorage>,
    pub network: Arc<RwLock<P2PNetwork>>,
}

#[derive(Parser, Debug)]
#[clap(
    name = "rustorium",
    about = "A comprehensive blockchain platform CLI",
    version,
    author
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
    #[clap(name = "account", about = "Account management commands")]
    Account(account::AccountCommands),

    #[clap(name = "tx", about = "Transaction commands")]
    Transaction(tx::TransactionCommands),

    #[clap(name = "contract", about = "Smart contract commands")]
    Contract(contract::ContractCommands),

    #[clap(name = "token", about = "Token management commands")]
    Token(token::TokenCommands),

    #[clap(name = "network", about = "Network management commands")]
    Network(network::NetworkCommands),

    #[clap(name = "dev", about = "Developer tools")]
    Dev(dev::DevCommands),

    #[clap(name = "analytics", about = "Analytics and reporting")]
    Analytics(analytics::AnalyticsCommands),

    #[clap(name = "security", about = "Security management")]
    Security(security::SecurityCommands),

    #[clap(name = "system", about = "System management")]
    System(system::SystemCommands),

    #[clap(name = "config", about = "Configuration commands")]
    Config(config::ConfigCommands),
}

impl Commands {
    pub async fn execute(self, ctx: CommandContext) -> Result<()> {
        match self {
            Commands::Account(cmd) => cmd.run().await,
            Commands::Transaction(cmd) => cmd.run().await,
            Commands::Contract(cmd) => cmd.run().await,
            Commands::Token(cmd) => cmd.run().await,
            Commands::Network(cmd) => cmd.run().await,
            Commands::Dev(cmd) => cmd.run().await,
            Commands::Analytics(cmd) => cmd.run().await,
            Commands::Security(cmd) => cmd.run().await,
            Commands::System(cmd) => cmd.run().await,
            Commands::Config(cmd) => cmd.run().await,
        }
    }
}