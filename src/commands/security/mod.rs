use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug, Clone)]
pub struct SecurityCommands {
    #[clap(subcommand)]
    command: SecuritySubCommands,
}

#[derive(Parser, Debug, Clone)]
pub enum SecuritySubCommands {
    #[clap(name = "list", about = "List securitys")]
    List,
    #[clap(name = "show", about = "Show security details")]
    Show,
}

impl SecurityCommands {
    pub async fn run(&self) -> Result<()> {
        match &self.command {
            SecuritySubCommands::List => {
                println!("Listing securitys...");
                Ok(())
            }
            SecuritySubCommands::Show => {
                println!("Showing security details...");
                Ok(())
            }
        }
    }
}
