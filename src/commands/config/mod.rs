use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug, Clone)]
pub struct ConfigCommands {
    #[clap(subcommand)]
    command: ConfigSubCommands,
}

#[derive(Parser, Debug, Clone)]
pub enum ConfigSubCommands {
    #[clap(name = "list", about = "List configs")]
    List,
    #[clap(name = "show", about = "Show config details")]
    Show,
}

impl ConfigCommands {
    pub async fn run(&self) -> Result<()> {
        match &self.command {
            ConfigSubCommands::List => {
                println!("Listing configs...");
                Ok(())
            }
            ConfigSubCommands::Show => {
                println!("Showing config details...");
                Ok(())
            }
        }
    }
}
