use anyhow::Result;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Select, Input};
use rustyline::DefaultEditor;
use crate::{
    config::NodeConfig,
    services::ServiceManager,
};

const LOGO: &str = r#"
╭──────────────────────────────────────────────────────────────────────────╮
│                                                                          │
│  ██████╗ ██╗   ██╗███████╗████████╗ ██████╗ ██████╗ ██╗██╗   ██╗███╗   ███╗│
│  ██╔══██╗██║   ██║██╔════╝╚══██╔══╝██╔═══██╗██╔══██╗██║██║   ██║████╗ ████║│
│  ██████╔╝██║   ██║███████╗   ██║   ██║   ██║██████╔╝██║██║   ██║██╔████╔██║│
│  ██╔══██╗██║   ██║╚════██║   ██║   ██║   ██║██╔══██╗██║██║   ██║██║╚██╔╝██║│
│  ██║  ██║╚██████╔╝███████║   ██║   ╚██████╔╝██║  ██║██║╚██████╔╝██║ ╚═╝ ██║│
│  ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝╚═╝ ╚═════╝ ╚═╝     ╚═╝│
│                                                                          │
│  ▪ Next-Generation Blockchain Infrastructure                             │
│  ▪ Powered by Rust                                                      │
│  ▪ Visit: https://rustorium.org                                         │
│                                                                          │
╰──────────────────────────────────────────────────────────────────────────╯
"#;

#[allow(dead_code)]
pub struct InteractiveConsole {
    config: NodeConfig,
    term: Term,
    service_manager: ServiceManager,
}

impl InteractiveConsole {
    pub fn new(config: NodeConfig, service_manager: ServiceManager) -> Self {
        Self {
            config,
            term: Term::stdout(),
            service_manager,
        }
    }

    pub async fn run(service_manager: &ServiceManager) -> Result<()> {
        let term = Term::stdout();
        term.clear_screen()?;
        
        // ロゴを表示
        println!("{}", style(LOGO).cyan());
        println!("{}", style("Rustorium Node v0.1.0").bold());
        println!();

        // URLの表示
        println!("{}",
            style("Access URLs").bold().underlined()
        );
        println!("  • Web UI:     {}", 
            style(format!("http://localhost:{}", 53037)).green()
        );
        println!("  • API:        {}", 
            style(format!("http://localhost:{}", 53038)).green()
        );
        println!("  • WebSocket:  {}", 
            style(format!("ws://localhost:{}", 53039)).green()
        );
        println!("  • Website:    {}", 
            style("https://rustorium.org").cyan()
        );
        println!();

        // メインメニューを表示
        let menu_items = vec![
            "Node Status",
            "Network Info",
            "Blockchain Info",
            "Peers",
            "Settings",
            "Exit",
        ];

        let _rl = DefaultEditor::new()?;
        loop {
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt(style("Select an option").cyan().bold().to_string())
                .items(&menu_items)
                .default(0)
                .interact()?;

            match selection {
                0 => Self::show_node_status(service_manager).await?,
                1 => Self::show_network_info(service_manager).await?,
                2 => Self::show_blockchain_info(service_manager).await?,
                3 => Self::show_peers(service_manager).await?,
                4 => Self::show_settings(service_manager).await?,
                5 => {
                    println!("\n{}", style("Exiting...").dim());
                    break;
                }
                _ => unreachable!(),
            }
        }

        Ok(())
    }

    async fn show_node_status(_service_manager: &ServiceManager) -> Result<()> {
        println!("\n{}", style("Node Status").bold().underlined());
        
        // システム情報を表示
        let cpu_cores = sys_info::cpu_num().unwrap_or(1);
        let memory_gb = sys_info::mem_info()
            .map(|m| m.total / 1024 / 1024)
            .unwrap_or(0);

        println!("  • CPU Cores:  {}", style(cpu_cores).cyan());
        println!("  • Memory:     {} GB", style(memory_gb).cyan());
        println!("  • Uptime:     {} minutes", style("10").cyan());
        println!();

        // 任意のキーで戻る
        Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt(style("Press Enter to return").dim().to_string())
            .allow_empty(true)
            .interact_text()?;

        Ok(())
    }

    async fn show_network_info(_service_manager: &ServiceManager) -> Result<()> {
        println!("\n{}", style("Network Information").bold().underlined());
        
        // ネットワーク情報を表示
        println!("  • Connected Peers: {}", style("5").green());
        println!("  • Bandwidth In:    {} KB/s", style("1.2").green());
        println!("  • Bandwidth Out:   {} KB/s", style("0.8").green());
        println!();

        Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt(style("Press Enter to return").dim().to_string())
            .allow_empty(true)
            .interact_text()?;

        Ok(())
    }

    async fn show_blockchain_info(_service_manager: &ServiceManager) -> Result<()> {
        println!("\n{}", style("Blockchain Information").bold().underlined());
        
        // ブロックチェーン情報を表示
        println!("  • Height:        {}", style("1,234").yellow());
        println!("  • Transactions:  {}", style("5,678").yellow());
        println!("  • Block Time:    {} ms", style("1000").yellow());
        println!();

        Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt(style("Press Enter to return").dim().to_string())
            .allow_empty(true)
            .interact_text()?;

        Ok(())
    }

    async fn show_peers(_service_manager: &ServiceManager) -> Result<()> {
        println!("\n{}", style("Connected Peers").bold().underlined());
        
        // ピア情報を表示
        println!("  • Peer 1:  {}", style("12D3...abc").magenta());
        println!("  • Peer 2:  {}", style("12D3...def").magenta());
        println!("  • Peer 3:  {}", style("12D3...ghi").magenta());
        println!();

        Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt(style("Press Enter to return").dim().to_string())
            .allow_empty(true)
            .interact_text()?;

        Ok(())
    }

    async fn show_settings(_service_manager: &ServiceManager) -> Result<()> {
        println!("\n{}", style("Settings").bold().underlined());
        
        // 設定情報を表示
        println!("  • Log Level:     {}", style("info").cyan());
        println!("  • Auto Mining:   {}", style("enabled").cyan());
        println!("  • Max Peers:     {}", style("50").cyan());
        println!();

        Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt(style("Press Enter to return").dim().to_string())
            .allow_empty(true)
            .interact_text()?;

        Ok(())
    }
}