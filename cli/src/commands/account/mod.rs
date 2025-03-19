use crate::app::App;
use clap::Subcommand;
use colored::*;
use prettytable::{format, Table};

#[derive(Subcommand)]
pub enum AccountCommands {
    /// Get account by address
    Get {
        /// Account address
        address: String,
    },
    
    /// Create a new account
    Create,
    
    /// List accounts
    List {
        /// Number of accounts to show
        #[arg(short, long, default_value = "10")]
        limit: usize,
        
        /// Offset for pagination
        #[arg(short, long, default_value = "0")]
        offset: usize,
    },
    
    /// Set current account
    Use {
        /// Account address
        address: String,
    },
}

/// Handle account commands
pub async fn handle_command(app: &mut App, command: AccountCommands) -> anyhow::Result<()> {
    match command {
        AccountCommands::Get { address } => {
            let account = app.api_client.get_account(&address).await?;
            print_account_details(&account);
        }
        AccountCommands::Create => {
            let account = app.api_client.create_account().await?;
            println!("Account created successfully:");
            print_account_details(&account);
        }
        AccountCommands::List { limit, offset } => {
            let accounts = app.api_client.get_accounts(limit, offset).await?;
            print_account_list(&accounts);
        }
        AccountCommands::Use { address } => {
            // Verify account exists
            let account = app.api_client.get_account(&address).await?;
            app.current_account = Some(account.address.clone());
            println!("Current account set to: {}", account.address.green());
        }
    }
    
    Ok(())
}

/// Handle account shell commands
pub async fn handle_shell_command(app: &mut App, args: &[&str]) -> anyhow::Result<()> {
    if args.is_empty() {
        display_help();
        return Ok(());
    }
    
    match args[0] {
        "get" => {
            if args.len() < 2 {
                println!("Usage: account get <address>");
                return Ok(());
            }
            
            let address = args[1];
            let account = app.api_client.get_account(address).await?;
            print_account_details(&account);
        }
        "create" => {
            let account = app.api_client.create_account().await?;
            println!("Account created successfully:");
            print_account_details(&account);
        }
        "list" => {
            let limit = args.get(1).and_then(|s| s.parse::<usize>().ok()).unwrap_or(10);
            let offset = args.get(2).and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
            
            let accounts = app.api_client.get_accounts(limit, offset).await?;
            print_account_list(&accounts);
        }
        "use" => {
            if args.len() < 2 {
                println!("Usage: account use <address>");
                return Ok(());
            }
            
            let address = args[1];
            // Verify account exists
            let account = app.api_client.get_account(address).await?;
            app.current_account = Some(account.address.clone());
            println!("Current account set to: {}", account.address.green());
        }
        "help" => {
            display_help();
        }
        _ => {
            println!("Unknown account command: {}", args[0]);
            display_help();
        }
    }
    
    Ok(())
}

/// Display help for account commands
pub fn display_help() {
    println!("Account commands:");
    println!("  {} <address>  - Get account by address", "get".cyan());
    println!("  {}        - Create a new account", "create".cyan());
    println!("  {} [limit] [offset] - List accounts", "list".cyan());
    println!("  {} <address>  - Set current account", "use".cyan());
    println!("  {}         - Display this help", "help".cyan());
}

/// Print account details
fn print_account_details(account: &crate::api::models::Account) {
    println!("Address: {}", account.address.green());
    println!("Balance: {} ETH", account.balance.to_string().yellow());
    println!("Nonce: {}", account.nonce);
    println!("Type: {}", account.account_type);
    println!("Created At: {}", account.created_at);
    
    if let Some(last_activity) = &account.last_activity {
        println!("Last Activity: {}", last_activity);
    }
}

/// Print account list
fn print_account_list(accounts: &[crate::api::models::Account]) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    
    table.set_titles(row![
        "Address".cyan().bold(),
        "Balance (ETH)".cyan().bold(),
        "Nonce".cyan().bold(),
        "Type".cyan().bold(),
        "Created At".cyan().bold()
    ]);
    
    for account in accounts {
        table.add_row(row![
            account.address,
            account.balance,
            account.nonce,
            account.account_type,
            account.created_at
        ]);
    }
    
    table.printstd();
}