use anyhow::Result;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Select, Input};
use rustyline::DefaultEditor;
use crate::{
    config::NodeConfig,
    services::ServiceManager,
};

use std::time::{SystemTime, UNIX_EPOCH};

const STATUS_LOGO_TEMPLATE: &str = r#"
╭────────────────────────────────────────────────────────────────────╮
│                                                                    │
│  ██████╗ ██╗   ██╗███████╗████████╗ ██████╗ ██████╗ ██╗██╗   ██╗███╗╗╗█╗╗│
│  ██╔══██╗██║   ██║██╔════╝╚══██╔══╝██╔═══██╗██╔══██╗██║██║   ██║████╗ ██║│
│  ██████╔╝██║   ██║███████╗   ██║   ██║   ██║██████╔╝██║██║   ██║██╔████║│
│  ██╔══██╗██║   ██║╚════██║   ██║   ██║   ██║██╔══██╗██║██║   ██║██║╚██╔╝│
│  ██║  ██║╚██████╔╝███████║   ██║   ╚██████╔╝██║  ██║██║╚██████╔╝██║ ╚═╝ │
│  ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝╚═╝ ╚═════╝ ╚═╝     │
│                                                                    │
│                          R U S T O R I U M v0.1.0                  │
│                                                                    │
│  ➤ Next-Generation Blockchain Infrastructure                       │
│  ➤ Built with Rust                                                 │
│  ➤ Secure. Fast. Reliable. Scalable.                               │
│  ➤ Visit: https://rustorium.org                                    │
│                                                                    │
├─────────────────────────────── Status ─────────────────────────────┤
│                                                                    │
│  Node            : ✅ Running           Network   : 🟢 Connected     │
│  Version         : v0.1.0              Peers     : 14 connected    │
│  Sync Status     : ✔ Fully Synced      Latency   : 32ms avg        │
│  Uptime          : 7 days, 16 hrs      Blocks    : 1,029,481       │
│                                                                    │
├─────────────────────────────── Access ─────────────────────────────┤
│                                                                    │
│  Web Dashboard   : 🌐 http://localhost:53037                       │
│  REST API        : ⚙️  http://localhost:53038                       │
│  WebSocket       : 🔌 ws://localhost:53039                          │
│                                                                    │
╰────────────────────────────────────────────────────────────────────╯
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



    /// インタラクティブモードが利用可能かどうかを確認
    fn is_interactive() -> bool {
        // 以下の条件のいずれかに該当する場合は非インタラクティブ
        if !atty::is(atty::Stream::Stdin) || !atty::is(atty::Stream::Stdout) {
            return false;  // 標準入出力がTTYでない
        }

        // CI環境の確認
        if std::env::var("CI").is_ok() || std::env::var("GITHUB_ACTIONS").is_ok() {
            return false;  // CI環境
        }

        // TERM環境変数の確認
        if let Ok(term) = std::env::var("TERM") {
            if term == "dumb" || term.is_empty() {
                return false;  // 非インタラクティブターミナル
            }
        }

        true
    }

    pub async fn run(service_manager: &ServiceManager) -> Result<()> {
        let term = Term::stdout();
        term.clear_screen()?;

        // インタラクティブモードが利用できない場合は即座にバックグラウンドモードへ
        if !Self::is_interactive() {
            println!("{}", style("Non-interactive environment detected, running in background mode...").dim());
            return Ok(());
        }
        
        // 実際の値を取得
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let uptime = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() - start_time;
        let days = uptime / (24 * 60 * 60);
        let hours = (uptime % (24 * 60 * 60)) / (60 * 60);
        let uptime_str = format!("{} days, {} hrs", days, hours);

        // P2Pネットワークの情報を取得
        let network_status = if service_manager.config().network.enabled { "🟢 Connected" } else { "🔴 Disconnected" };
        let peer_count = service_manager.get_peer_count().await;
        let latency = service_manager.get_average_latency().await;
        let block_count = service_manager.get_block_count().await;

        // ポート情報を取得
        let base_port = service_manager.config().network.port;
        let web_port = base_port;
        let api_port = base_port + 1;
        let ws_port = base_port + 2;

        // ロゴを表示（動的な情報を含む）
        let logo = STATUS_LOGO_TEMPLATE.replace(
            "Node            : ✅ Running           Network   : 🟢 Connected     │\n\
             │  Version         : v0.1.0              Peers     : 14 connected    │\n\
             │  Sync Status     : ✔ Fully Synced      Latency   : 32ms avg        │\n\
             │  Uptime          : 7 days, 16 hrs      Blocks    : 1,029,481       │\n\
             │                                                                    │\n\
             ├─────────────────────────────── Access ─────────────────────────────┤\n\
             │                                                                    │\n\
             │  Web Dashboard   : 🌐 http://localhost:53037                       │\n\
             │  REST API        : ⚙️  http://localhost:53038                       │\n\
             │  WebSocket       : 🔌 ws://localhost:53039",
            &format!(
                "Node            : ✅ Running           Network   : {}     │\n\
                 │  Version         : v0.1.0              Peers     : {} connected    │\n\
                 │  Sync Status     : ✔ Starting          Latency   : {}ms avg        │\n\
                 │  Uptime          : {}      Blocks    : {}       │\n\
                 │                                                                    │\n\
                 ├─────────────────────────────── Access ─────────────────────────────┤\n\
                 │                                                                    │\n\
                 │  Web Dashboard   : 🌐 http://localhost:{}                       │\n\
                 │  REST API        : ⚙️  http://localhost:{}                       │\n\
                 │  WebSocket       : 🔌 ws://localhost:{}",
                network_status,
                peer_count,
                latency,
                uptime_str,
                block_count,
                web_port,
                api_port,
                ws_port,
            )
        );
        println!("{}", style(logo).cyan());

        // インタラクティブモードのタイムアウト処理
        use std::io::{self, Write};
        use crossterm::{
            event::{self, Event, MouseEventKind},
            terminal::{disable_raw_mode, enable_raw_mode},
            ExecutableCommand,
            cursor::{Hide, Show},
        };

        enable_raw_mode()?;
        io::stdout().execute(Hide)?;

        let mut countdown = 10;
        let start = std::time::Instant::now();
        let mut had_input = false;

        while countdown > 0 && !had_input {
            // カウントダウンを表示（前の行を消去してから）
            print!("\r\x1B[K{}", 
                style(format!("Press any key to enter interactive mode... ({})", countdown)).dim()
            );
            io::stdout().flush()?;

            // 1秒間キー入力やマウス移動を待つ
            while start.elapsed() < std::time::Duration::from_secs(10 - countdown as u64) {
                if event::poll(std::time::Duration::from_millis(100))? {
                    match event::read()? {
                        Event::Key(_) => {
                            had_input = true;
                            break;
                        }
                        Event::Mouse(event) => {
                            match event.kind {
                                MouseEventKind::Down(_) | MouseEventKind::Drag(_) => {
                                    had_input = true;
                                    break;
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
            countdown -= 1;
        }

        // 後処理
        disable_raw_mode()?;
        io::stdout().execute(Show)?;
        println!();  // 改行

        if !had_input {
            println!("{}", style("Continuing in background mode...").dim());
            return Ok(());
        }

        println!("{}", style("Entering interactive mode...").cyan());

        // メインメニューを表示
        let menu_items = vec![
            "📊 Node Status",
            "🌍 Network Information",
            "📦 Blockchain Information",
            "🔗 Peer Management",
            "⚙️  Settings",
            "❌ Exit",
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