use crate::api::ApiClient;
use crate::commands;
use crate::display::{self, Logo};
use crate::utils;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Main application state
pub struct App {
    /// API client
    pub api_client: ApiClient,
    /// Debug mode flag
    pub debug: bool,
    /// Command history
    pub history: Vec<String>,
    /// Last command execution time
    pub last_command_time: Option<Duration>,
    /// Current working directory
    pub current_dir: String,
    /// Environment variables
    pub env_vars: HashMap<String, String>,
    /// Current account address
    pub current_account: Option<String>,
}

impl App {
    /// Create a new App instance
    pub fn new(api_client: ApiClient, debug: bool) -> Self {
        Self {
            api_client,
            debug,
            history: Vec::new(),
            last_command_time: None,
            current_dir: String::from("/"),
            env_vars: HashMap::new(),
            current_account: None,
        }
    }

    /// Run the interactive shell
    pub async fn run_interactive_shell(&mut self) -> Result<()> {
        // Display logo and welcome message
        display::clear_screen();
        Logo::display();
        
        // Get network status
        let network_status = match self.api_client.get_network_status().await {
            Ok(status) => status,
            Err(_) => {
                println!("{}", "Warning: Could not fetch network status".yellow());
                Default::default()
            }
        };
        
        // Display network status
        display::print_network_status(&network_status);
        
        // Display node stats
        let node_stats = match self.api_client.get_node_stats().await {
            Ok(stats) => stats,
            Err(_) => {
                println!("{}", "Warning: Could not fetch node stats".yellow());
                Default::default()
            }
        };
        
        display::print_node_stats(&node_stats);
        
        println!("\nRustorium CLI ready. Type '{}' for available commands.", "help".cyan());
        
        // Create readline editor
        let mut rl = DefaultEditor::new()?;
        
        // Set up command completion
        // TODO: Implement command completion
        
        // Main loop
        loop {
            // Display prompt
            let prompt = if let Some(account) = &self.current_account {
                format!("{}> ", account.green())
            } else {
                "> ".to_string()
            };
            
            // Read line
            let readline = rl.readline(&prompt);
            
            match readline {
                Ok(line) => {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }
                    
                    // Add to history
                    rl.add_history_entry(line)?;
                    self.history.push(line.to_string());
                    
                    // Process command
                    let start_time = Instant::now();
                    let result = self.process_command(line).await;
                    let elapsed = start_time.elapsed();
                    self.last_command_time = Some(elapsed);
                    
                    // Handle result
                    if let Err(e) = result {
                        eprintln!("{}: {}", "Error".red(), e);
                    }
                    
                    // Display command execution time in debug mode
                    if self.debug {
                        println!("{}: {:?}", "Command execution time".dimmed(), elapsed);
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("Ctrl-C");
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    println!("Exiting...");
                    break;
                }
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    /// Process a command
    async fn process_command(&mut self, command: &str) -> anyhow::Result<()> {
        // Split command and arguments
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }
        
        // Process command
        match parts[0] {
            "help" => {
                self.display_help(parts.get(1).copied());
            }
            "exit" | "quit" => {
                println!("Exiting...");
                std::process::exit(0);
            }
            "clear" | "cls" => {
                display::clear_screen();
            }
            "account" => {
                let args = &parts[1..];
                commands::account::handle_shell_command(self, args).await?;
            }
            "block" => {
                let args = &parts[1..];
                commands::block::handle_shell_command(self, args).await?;
            }
            "contract" => {
                let args = &parts[1..];
                commands::contract::handle_shell_command(self, args).await?;
            }
            "network" => {
                let args = &parts[1..];
                commands::network::handle_shell_command(self, args).await?;
            }
            "token" => {
                let args = &parts[1..];
                commands::token::handle_shell_command(self, args).await?;
            }
            "tx" => {
                let args = &parts[1..];
                commands::tx::handle_shell_command(self, args).await?;
            }
            "system" => {
                let args = &parts[1..];
                commands::system::handle_shell_command(self, args).await?;
            }
            "config" => {
                let args = &parts[1..];
                commands::config::handle_shell_command(self, args).await?;
            }
            "debug" => {
                let args = &parts[1..];
                commands::debug::handle_shell_command(self, args).await?;
            }
            "history" => {
                self.display_history();
            }
            "env" => {
                self.display_env_vars();
            }
            "set" => {
                if parts.len() >= 3 {
                    self.env_vars.insert(parts[1].to_string(), parts[2].to_string());
                    println!("Set {} = {}", parts[1], parts[2]);
                } else {
                    println!("Usage: set <variable> <value>");
                }
            }
            _ => {
                println!("{}: Unknown command '{}'", "Error".red(), parts[0]);
                println!("Type '{}' for available commands.", "help".cyan());
            }
        }
        
        Ok(())
    }
    
    /// Display help
    fn display_help(&self, command: Option<&str>) {
        match command {
            None => {
                println!("Available commands:");
                println!("  {} - Manage accounts and wallets", "account".cyan());
                println!("  {} - View block information", "block".cyan());
                println!("  {} - Deploy and interact with smart contracts", "contract".cyan());
                println!("  {} - View and configure network settings", "network".cyan());
                println!("  {} - Manage tokens (ERC-20/ERC-721)", "token".cyan());
                println!("  {} - Create and manage transactions", "tx".cyan());
                println!("  {} - System and node management", "system".cyan());
                println!("  {} - Configure node settings", "config".cyan());
                println!("  {} - Debugging tools", "debug".cyan());
                println!("  {} - Display command history", "history".cyan());
                println!("  {} - Display environment variables", "env".cyan());
                println!("  {} - Set environment variable", "set".cyan());
                println!("  {} - Clear the screen", "clear".cyan());
                println!("  {} - Exit the CLI", "exit".cyan());
                println!("\nType '{} <command>' for more information on a specific command.", "help".cyan());
            }
            Some("account") => commands::account::display_help(),
            Some("block") => commands::block::display_help(),
            Some("contract") => commands::contract::display_help(),
            Some("network") => commands::network::display_help(),
            Some("token") => commands::token::display_help(),
            Some("tx") => commands::tx::display_help(),
            Some("system") => commands::system::display_help(),
            Some("config") => commands::config::display_help(),
            Some("debug") => commands::debug::display_help(),
            Some(cmd) => {
                println!("No help available for '{}'", cmd);
            }
        }
    }
    
    /// Display command history
    fn display_history(&self) {
        println!("Command history:");
        for (i, cmd) in self.history.iter().enumerate() {
            println!("  {}: {}", i + 1, cmd);
        }
    }
    
    /// Display environment variables
    fn display_env_vars(&self) {
        println!("Environment variables:");
        for (key, value) in &self.env_vars {
            println!("  {} = {}", key, value);
        }
    }
}