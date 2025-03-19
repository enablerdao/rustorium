use crate::app::App;
use clap::Subcommand;
use colored::*;
use prettytable::{format, Table};

#[derive(Subcommand)]
pub enum BlockCommands {
    /// Get block by number or hash
    Get {
        /// Block number or hash
        id: String,
    },
    
    /// Get latest block
    Latest,
    
    /// List blocks
    List {
        /// Number of blocks to show
        #[arg(short, long, default_value = "10")]
        limit: usize,
        
        /// Offset for pagination
        #[arg(short, long, default_value = "0")]
        offset: usize,
    },
}

/// Handle block commands
pub async fn handle_command(app: &mut App, command: BlockCommands) -> anyhow::Result<()> {
    match command {
        BlockCommands::Get { id } => {
            let block = app.api_client.get_block(&id).await?;
            print_block_details(&block);
        }
        BlockCommands::Latest => {
            let block = app.api_client.get_latest_block().await?;
            print_block_details(&block);
        }
        BlockCommands::List { limit, offset } => {
            let blocks = app.api_client.get_blocks(limit, offset).await?;
            print_block_list(&blocks);
        }
    }
    
    Ok(())
}

/// Handle block shell commands
pub async fn handle_shell_command(app: &mut App, args: &[&str]) -> anyhow::Result<()> {
    if args.is_empty() {
        display_help();
        return Ok(());
    }
    
    match args[0] {
        "get" => {
            if args.len() < 2 {
                println!("Usage: block get <id>");
                return Ok(());
            }
            
            let id = args[1];
            let block = app.api_client.get_block(id).await?;
            print_block_details(&block);
        }
        "latest" => {
            let block = app.api_client.get_latest_block().await?;
            print_block_details(&block);
        }
        "list" => {
            let limit = args.get(1).and_then(|s| s.parse::<usize>().ok()).unwrap_or(10);
            let offset = args.get(2).and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
            
            let blocks = app.api_client.get_blocks(limit, offset).await?;
            print_block_list(&blocks);
        }
        "help" => {
            display_help();
        }
        _ => {
            println!("Unknown block command: {}", args[0]);
            display_help();
        }
    }
    
    Ok(())
}

/// Display help for block commands
pub fn display_help() {
    println!("Block commands:");
    println!("  {} <id>       - Get block by number or hash", "get".cyan());
    println!("  {}         - Get latest block", "latest".cyan());
    println!("  {} [limit] [offset] - List blocks", "list".cyan());
    println!("  {}         - Display this help", "help".cyan());
}

/// Print block details
fn print_block_details(block: &crate::api::models::Block) {
    println!("Block #{}", block.number.to_string().yellow());
    println!("Hash: {}", block.hash.cyan());
    println!("Parent Hash: {}", block.parent_hash);
    println!("Timestamp: {}", block.timestamp);
    println!("Miner: {}", block.miner);
    println!("Size: {} bytes", block.size);
    println!("Gas Used: {}", block.gas_used);
    println!("Gas Limit: {}", block.gas_limit);
    println!("Transactions: {}", block.transactions.len());
    
    if !block.transactions.is_empty() {
        println!("\nTransactions:");
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        
        for (i, tx) in block.transactions.iter().enumerate().take(5) {
            table.add_row(row![i + 1, tx]);
        }
        
        if block.transactions.len() > 5 {
            table.add_row(row!["...", "..."]);
        }
        
        table.printstd();
    }
}

/// Print block list
fn print_block_list(blocks: &[crate::api::models::Block]) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    
    table.set_titles(row![
        "Number".cyan().bold(),
        "Hash".cyan().bold(),
        "Time".cyan().bold(),
        "Txs".cyan().bold(),
        "Size".cyan().bold(),
        "Gas Used".cyan().bold()
    ]);
    
    for block in blocks {
        table.add_row(row![
            block.number,
            &block.hash[0..10],
            &block.timestamp,
            block.transactions.len(),
            format!("{} bytes", block.size),
            block.gas_used
        ]);
    }
    
    table.printstd();
}