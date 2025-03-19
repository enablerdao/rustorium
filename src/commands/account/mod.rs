use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug, Clone)]
pub struct AccountCommands {
    #[clap(subcommand)]
    command: AccountSubCommands,
}

#[derive(Parser, Debug, Clone)]
pub enum AccountSubCommands {
    #[clap(name = "create", about = "Create a new account")]
    Create,
    #[clap(name = "list", about = "List all accounts")]
    List,
    #[clap(name = "show", about = "Show account details")]
    Show,
}

impl AccountCommands {
    pub async fn run(&self) -> Result<()> {
        match &self.command {
            AccountSubCommands::Create => {
                println!("Creating new account...");
                Ok(())
            }
            AccountSubCommands::List => {
                println!("Listing accounts...");
                Ok(())
            }
            AccountSubCommands::Show => {
                println!("Showing account details...");
                Ok(())
            }
        }
    }
}