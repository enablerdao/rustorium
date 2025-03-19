use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug, Clone)]
pub struct NetworkCommands {
    #[clap(subcommand)]
    command: NetworkSubCommands,
}

#[derive(Parser, Debug, Clone)]
pub enum NetworkSubCommands {
    #[clap(name = "list", about = "List networks")]
    List,
    #[clap(name = "show", about = "Show network details")]
    Show,
}

impl NetworkCommands {
    pub async fn run(&self) -> Result<()> {
        match &self.command {
            NetworkSubCommands::List => {
                println!("Listing networks...");
                Ok(())
            }
            NetworkSubCommands::Show => {
                println!("Showing network details...");
                Ok(())
            }
        }
    }
}
