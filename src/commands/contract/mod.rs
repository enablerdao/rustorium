use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug, Clone)]
pub struct ContractCommands {
    #[clap(subcommand)]
    command: ContractSubCommands,
}

#[derive(Parser, Debug, Clone)]
pub enum ContractSubCommands {
    #[clap(name = "list", about = "List contracts")]
    List,
    #[clap(name = "show", about = "Show contract details")]
    Show,
}

impl ContractCommands {
    pub async fn run(&self) -> Result<()> {
        match &self.command {
            ContractSubCommands::List => {
                println!("Listing contracts...");
                Ok(())
            }
            ContractSubCommands::Show => {
                println!("Showing contract details...");
                Ok(())
            }
        }
    }
}
