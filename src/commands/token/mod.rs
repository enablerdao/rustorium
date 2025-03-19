use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug, Clone)]
pub struct TokenCommands {
    #[clap(subcommand)]
    command: TokenSubCommands,
}

#[derive(Parser, Debug, Clone)]
pub enum TokenSubCommands {
    #[clap(name = "list", about = "List tokens")]
    List,
    #[clap(name = "show", about = "Show token details")]
    Show,
}

impl TokenCommands {
    pub async fn run(&self) -> Result<()> {
        match &self.command {
            TokenSubCommands::List => {
                println!("Listing tokens...");
                Ok(())
            }
            TokenSubCommands::Show => {
                println!("Showing token details...");
                Ok(())
            }
        }
    }
}
