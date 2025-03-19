use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug, Clone)]
pub struct AnalyticsCommands {
    #[clap(subcommand)]
    command: AnalyticsSubCommands,
}

#[derive(Parser, Debug, Clone)]
pub enum AnalyticsSubCommands {
    #[clap(name = "list", about = "List analyticss")]
    List,
    #[clap(name = "show", about = "Show analytics details")]
    Show,
}

impl AnalyticsCommands {
    pub async fn run(&self) -> Result<()> {
        match &self.command {
            AnalyticsSubCommands::List => {
                println!("Listing analyticss...");
                Ok(())
            }
            AnalyticsSubCommands::Show => {
                println!("Showing analytics details...");
                Ok(())
            }
        }
    }
}
