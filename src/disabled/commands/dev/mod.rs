use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug, Clone)]
pub struct DevCommands {
    #[clap(subcommand)]
    command: DevSubCommands,
}

#[derive(Parser, Debug, Clone)]
pub enum DevSubCommands {
    #[clap(name = "list", about = "List devs")]
    List,
    #[clap(name = "show", about = "Show dev details")]
    Show,
}

impl DevCommands {
    pub async fn run(&self) -> Result<()> {
        match &self.command {
            DevSubCommands::List => {
                println!("Listing devs...");
                Ok(())
            }
            DevSubCommands::Show => {
                println!("Showing dev details...");
                Ok(())
            }
        }
    }
}
