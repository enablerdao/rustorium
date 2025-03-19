use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug, Clone)]
pub struct TxCommands {
    #[clap(subcommand)]
    command: TxSubCommands,
}

#[derive(Parser, Debug, Clone)]
pub enum TxSubCommands {
    #[clap(name = "list", about = "List txs")]
    List,
    #[clap(name = "show", about = "Show tx details")]
    Show,
}

impl TxCommands {
    pub async fn run(&self) -> Result<()> {
        match &self.command {
            TxSubCommands::List => {
                println!("Listing txs...");
                Ok(())
            }
            TxSubCommands::Show => {
                println!("Showing tx details...");
                Ok(())
            }
        }
    }
}
