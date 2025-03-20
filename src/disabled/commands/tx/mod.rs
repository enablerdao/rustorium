use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug, Clone)]
pub struct TransactionCommands {
    #[clap(subcommand)]
    command: TransactionSubCommands,
}

#[derive(Parser, Debug, Clone)]
pub enum TransactionSubCommands {
    #[clap(name = "send", about = "Send a transaction")]
    Send {
        #[clap(long)]
        to: String,
        #[clap(long)]
        amount: u64,
    },
    #[clap(name = "list", about = "List transactions")]
    List,
    #[clap(name = "show", about = "Show transaction details")]
    Show {
        #[clap(long)]
        id: String,
    },
}

impl TransactionCommands {
    pub async fn run(&self) -> Result<()> {
        match &self.command {
            TransactionSubCommands::Send { to, amount } => {
                println!("Sending {} to {}", amount, to);
                Ok(())
            }
            TransactionSubCommands::List => {
                println!("Listing transactions...");
                Ok(())
            }
            TransactionSubCommands::Show { id } => {
                println!("Showing transaction details for {}", id);
                Ok(())
            }
        }
    }
}
