use crate::api::models::{NetworkStatus, NodeStats};
use colored::*;
use prettytable::{format, Table};
use std::io::{self, Write};
use terminal_size::{terminal_size, Width, Height};

/// Clear the terminal screen
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

/// Logo display
pub struct Logo;

impl Logo {
    /// Display the Rustorium logo
    pub fn display() {
        let logo = r#"
╭───────────────────────────────────────────────────────────────╮
│                                                               │
│   ██████╗ ██╗   ██╗███████╗████████╗ ██████╗ ██████╗ ██╗██╗   │
│   ██╔══██╗██║   ██║██╔════╝╚══██╔══╝██╔═══██╗██╔══██╗██║██║   │
│   ██████╔╝██║   ██║███████╗   ██║   ██║   ██║██████╔╝██║██║   │
│   ██╔══██╗██║   ██║╚════██║   ██║   ██║   ██║██╔══██╗██║██║   │
│   ██║  ██║╚██████╔╝███████║   ██║   ╚██████╔╝██║  ██║██║███████│
│   ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝╚═╝╚══════╝
│                                                v1.0.0         │
╰───────────────────────────────────────────────────────────────╯
"#;
        println!("{}", logo.bright_cyan());
        
        println!("{}", "[INFO] Rustorium node starting...".green());
        println!("{}", "[INFO] Loading configuration from config.toml".green());
        println!("{}", "[INFO] Database initialized".green());
        println!("{}", "[INFO] P2P network initialized".green());
        println!("{}", "[INFO] API server listening on 127.0.0.1:50128".green());
        println!("{}", "[INFO] WebSocket server listening on 127.0.0.1:50129".green());
        println!();
    }
}

/// Print network status
pub fn print_network_status(status: &NetworkStatus) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    
    println!("╭─ {} ───────────────────────────────────────────────╮", "NETWORK STATUS".cyan().bold());
    
    table.add_row(row![
        "Chain ID:".cyan(),
        status.chain_id.to_string().white()
    ]);
    
    table.add_row(row![
        "Current Block:".cyan(),
        format!("#{}", status.current_block).white()
    ]);
    
    table.add_row(row![
        "Sync Status:".cyan(),
        format!("{}% ({})", status.sync_percentage, status.sync_status).white()
    ]);
    
    table.add_row(row![
        "Peers:".cyan(),
        format!("{} connected", status.peers).white()
    ]);
    
    table.add_row(row![
        "TPS:".cyan(),
        format!("{:.1}", status.tps).white()
    ]);
    
    table.add_row(row![
        "Gas Price:".cyan(),
        format!("{:.1} Gwei", status.gas_price).white()
    ]);
    
    table.printstd();
    println!("╰───────────────────────────────────────────────────────────────╯");
    println!();
}

/// Print node stats
pub fn print_node_stats(stats: &NodeStats) {
    println!("╭─ {} ─────────────────────────────────────────────────╮", "NODE STATS".cyan().bold());
    
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    
    table.add_row(row![
        format!("CPU: {}%", stats.cpu_usage).cyan(),
        format!("Memory: {}/{}", stats.memory_used, stats.memory_total).cyan(),
        format!("Disk: {} used", stats.disk_used).cyan()
    ]);
    
    table.add_row(row![
        format!("Uptime: {}", stats.uptime).cyan(),
        format!("Last Block: {} ago", stats.last_block_time).cyan(),
        ""
    ]);
    
    table.printstd();
    println!("╰───────────────────────────────────────────────────────────────╯");
    println!();
}

/// Print a table
pub fn print_table<T: AsRef<str>>(headers: &[T], rows: &[Vec<String>]) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    
    // Add headers
    let header_cells = headers.iter().map(|h| h.as_ref().cyan().bold()).collect::<Vec<_>>();
    table.add_row(prettytable::Row::new(header_cells.into_iter().map(prettytable::Cell::new).collect()));
    
    // Add rows
    for row in rows {
        table.add_row(row.iter().map(|cell| prettytable::Cell::new(cell)).collect());
    }
    
    table.printstd();
}

/// Print a progress bar
pub fn print_progress_bar(progress: f64, width: usize) {
    let progress = progress.max(0.0).min(1.0);
    let filled_width = (progress * width as f64) as usize;
    let empty_width = width - filled_width;
    
    print!("[");
    for _ in 0..filled_width {
        print!("█");
    }
    for _ in 0..empty_width {
        print!(" ");
    }
    println!("] {:.1}%", progress * 100.0);
}

/// Get terminal size
pub fn get_terminal_size() -> (usize, usize) {
    if let Some((Width(w), Height(h))) = terminal_size() {
        (w as usize, h as usize)
    } else {
        (80, 24) // Default size
    }
}

/// Print a box with text
pub fn print_box(title: &str, content: &str) {
    let (width, _) = get_terminal_size();
    let box_width = width.min(80);
    
    // Print top border with title
    print!("╭─ {} ", title.cyan().bold());
    for _ in 0..(box_width - title.len() - 5) {
        print!("─");
    }
    println!("╮");
    
    // Print content
    for line in content.lines() {
        println!("│ {:<width$} │", line, width = box_width - 4);
    }
    
    // Print bottom border
    print!("╰");
    for _ in 0..(box_width - 2) {
        print!("─");
    }
    println!("╯");
}