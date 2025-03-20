use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug, Clone)]
pub struct SystemCommands {
    #[clap(subcommand)]
    command: SystemSubCommands,
}

#[derive(Parser, Debug, Clone)]
pub enum SystemSubCommands {
    #[clap(name = "list", about = "List systems")]
    List,
    #[clap(name = "show", about = "Show system details")]
    Show,
}

impl SystemCommands {
    pub async fn run(&self) -> Result<()> {
        match &self.command {
            SystemSubCommands::List => {
                println!("Listing systems...");
                Ok(())
            }
            SystemSubCommands::Show => {
                println!("Showing system details...");
                Ok(())
            }
        }
    }
}
