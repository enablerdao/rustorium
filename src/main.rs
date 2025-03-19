use std::process::{Command, Stdio};
use anyhow::Result;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use std::env;
use std::time::Duration;
use dialoguer::{theme::ColorfulTheme, Select};
use console::{Term, style};
use indicatif::{ProgressBar, ProgressStyle};
use structopt::StructOpt;

mod dev;
use dev::TestNodeManager;

// コマンドラインオプションの定義
#[derive(Debug, Clone)]
struct BlockchainCommand {
    command_type: String,
    description: String,
    args: Vec<String>,
}

#[derive(Debug, Clone)]
struct LocaleConfig {
    language: String,
    commands: Vec<BlockchainCommand>,
    messages: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone)]
struct AppState {
    api_port: u16,
    frontend_port: u16,
    locale: LocaleConfig,
    api_url: String,
    frontend_url: String,
}

impl LocaleConfig {
    fn new(language: &str) -> Self {
        let messages = match language {
            "ja" => {
                let mut m = std::collections::HashMap::new();
                m.insert("welcome".to_string(), "Rustoriumへようこそ！".to_string());
                m.insert("select_action".to_string(), "実行したいアクションを選択してください：".to_string());
                m.insert("account".to_string(), "アカウント管理".to_string());
                m.insert("transaction".to_string(), "トランザクション".to_string());
                m.insert("smart_contract".to_string(), "スマートコントラクト".to_string());
                m.insert("blockchain".to_string(), "ブロックチェーン情報".to_string());
                m.insert("settings".to_string(), "設定".to_string());
                m.insert("exit".to_string(), "終了".to_string());
                m
            },
            "en" => {
                let mut m = std::collections::HashMap::new();
                m.insert("welcome".to_string(), "Welcome to Rustorium!".to_string());
                m.insert("select_action".to_string(), "Select an action to perform:".to_string());
                m.insert("account".to_string(), "Account Management".to_string());
                m.insert("transaction".to_string(), "Transactions".to_string());
                m.insert("smart_contract".to_string(), "Smart Contracts".to_string());
                m.insert("blockchain".to_string(), "Blockchain Info".to_string());
                m.insert("settings".to_string(), "Settings".to_string());
                m.insert("exit".to_string(), "Exit".to_string());
                m
            },
            "zh" => {
                let mut m = std::collections::HashMap::new();
                m.insert("welcome".to_string(), "欢迎使用 Rustorium！".to_string());
                m.insert("select_action".to_string(), "请选择要执行的操作：".to_string());
                m.insert("account".to_string(), "账户管理".to_string());
                m.insert("transaction".to_string(), "交易".to_string());
                m.insert("smart_contract".to_string(), "智能合约".to_string());
                m.insert("blockchain".to_string(), "区块链信息".to_string());
                m.insert("settings".to_string(), "设置".to_string());
                m.insert("exit".to_string(), "退出".to_string());
                m
            },
            "ko" => {
                let mut m = std::collections::HashMap::new();
                m.insert("welcome".to_string(), "Rustorium에 오신 것을 환영합니다!".to_string());
                m.insert("select_action".to_string(), "실행할 작업을 선택하세요:".to_string());
                m.insert("account".to_string(), "계정 관리".to_string());
                m.insert("transaction".to_string(), "트랜잭션".to_string());
                m.insert("smart_contract".to_string(), "스마트 컨트랙트".to_string());
                m.insert("blockchain".to_string(), "블록체인 정보".to_string());
                m.insert("settings".to_string(), "설정".to_string());
                m.insert("exit".to_string(), "종료".to_string());
                m
            },
            _ => std::collections::HashMap::new(),
        };

        Self {
            language: language.to_string(),
            commands: Vec::new(),
            messages,
        }
    }

    fn get_message<'a>(&'a self, key: &'a str) -> &'a str {
        self.messages.get(key).map(|s| s.as_str()).unwrap_or(key)
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "rustorium")]
struct AppOptions {
    /// API server port
    #[structopt(long, env = "API_PORT")]
    api_port: Option<u16>,

    /// Frontend server port
    #[structopt(long, env = "FRONTEND_PORT")]
    frontend_port: Option<u16>,

    /// Log level (debug, info, warn, error)
    #[structopt(long, env = "LOG_LEVEL")]
    log_level: Option<String>,

    /// CORS origin
    #[structopt(long, env = "CORS_ORIGIN")]
    cors_origin: Option<String>,

    /// Development mode: Start multiple test nodes
    #[structopt(long)]
    dev: bool,

    /// Number of test nodes to start (in dev mode)
    #[structopt(long, default_value = "10")]
    nodes: u8,

    /// Base port for test nodes (in dev mode)
    #[structopt(long, default_value = "40000")]
    base_port: u16,

    /// Data directory for test nodes (in dev mode)
    #[structopt(long, default_value = "/tmp/rustorium_test")]
    data_dir: String,
}

impl AppOptions {
    fn new() -> Self {
        Self::from_args()
    }

    fn display_logo() {
        println!("{}", style(r#"
    ╭──────────────────────────────────────╮
    │                                      │
    │   ╭─────────╮  RUSTORIUM            │
    │   │ ₿ │ ⟠ │ │                       │
    │   │ ◎ │ ₳ │ │  Blockchain Platform  │
    │   ╰─────────╯                       │
    │                                      │
    ╰──────────────────────────────────────╯
    "#).cyan());
    }

    async fn interactive_console(app_state: &AppState) -> Result<()> {
        let term = Term::stdout();
        term.clear_screen()?;
        
        Self::display_logo();

        // 言語選択
        let languages = vec!["English", "日本語", "中文", "한국어"];
        let language_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(style("Select your language / 言語を選択 / 选择语言 / 언어 선택").cyan().bold().to_string())
            .items(&languages)
            .default(0)
            .interact()?;

        let language_code = match language_selection {
            0 => "en",
            1 => "ja",
            2 => "zh",
            3 => "ko",
            _ => "en",
        };

        let locale = LocaleConfig::new(language_code);
        println!("\n{}", style(locale.get_message("welcome")).bold());
        println!("\n{}", style("🌐 Services:").cyan().bold());
        println!("  API: {}", style(&app_state.api_url).underlined());
        println!("  Frontend: {}", style(&app_state.frontend_url).underlined());
        println!();

        loop {
            let actions = vec![
                locale.get_message("account"),
                locale.get_message("transaction"),
                locale.get_message("smart_contract"),
                locale.get_message("blockchain"),
                locale.get_message("settings"),
                locale.get_message("exit"),
            ];

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt(style(locale.get_message("select_action")).cyan().bold().to_string())
                .items(&actions)
                .default(0)
                .interact()?;

            match selection {
                0 => Self::handle_account_management(&locale).await?,
                1 => Self::handle_transactions(&locale).await?,
                2 => Self::handle_smart_contracts(&locale).await?,
                3 => Self::handle_blockchain_info(&locale).await?,
                4 => Self::handle_settings(&locale).await?,
                5 => break,
                _ => {}
            }

            println!();
        }

        Ok(())
    }

    async fn handle_account_management(_locale: &LocaleConfig) -> Result<()> {
        // アカウント管理の実装
        Ok(())
    }

    async fn handle_transactions(_locale: &LocaleConfig) -> Result<()> {
        // トランザクション管理の実装
        Ok(())
    }

    async fn handle_smart_contracts(_locale: &LocaleConfig) -> Result<()> {
        // スマートコントラクト管理の実装
        Ok(())
    }

    async fn handle_blockchain_info(_locale: &LocaleConfig) -> Result<()> {
        // ブロックチェーン情報の実装
        Ok(())
    }

    async fn handle_settings(_locale: &LocaleConfig) -> Result<()> {
        // 設定の実装
        Ok(())
    }
}

fn print_help() {
    println!("{}", style("Rustorium - Blockchain Platform").bold());
    println!();
    println!("{}", style("USAGE:").yellow());
    println!("  cargo run [OPTIONS]");
    println!();
    println!("{}", style("PORT OPTIONS:").yellow());
    println!("  --api-port <PORT>      API server port (default: auto)");
    println!("  --frontend-port <PORT> Frontend server port (default: auto)");
    println!();
    println!("{}", style("ADDITIONAL OPTIONS:").yellow());
    println!("  --log-level <LEVEL>    Set logging level (default: info)");
    println!("  --cors-origin <ORIGIN> Set CORS origin (default: *)");
    println!("  -h, --help            Show this help message");
    println!("  -v, --version         Show version information");
}

#[tokio::main]
async fn main() -> Result<()> {
    // コマンドラインオプションの解析
    let options = AppOptions::new();
    
    // ロギングレベルの設定
    let log_level = match options.log_level.as_deref() {
        Some("debug") => Level::DEBUG,
        Some("warn") => Level::WARN,
        Some("error") => Level::ERROR,
        Some("trace") => Level::TRACE,
        _ => Level::INFO,
    };

    // ロギングの設定
    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    // 開発モードの場合はテストノードを起動
    if options.dev {
        println!("{}", style("\n🧪 Development Mode: Starting Test Nodes").yellow().bold());
        println!("{}", style("⚠️  This mode is for development and testing only!").red());
        println!();

        let mut node_manager = TestNodeManager::new(options.base_port, options.data_dir.into());
        
        // テストノードを追加
        for i in 1..=options.nodes {
            node_manager.add_node(i)?;
        }

        // テストノードを起動
        node_manager.start_nodes(options.nodes).await?;

        // Ctrl+Cを待機
        let (tx, rx) = tokio::sync::oneshot::channel();
        ctrlc::set_handler(move || {
            let _ = tx.send(());
        })?;

        println!("\n{}", style("Press Ctrl+C to stop all nodes").cyan());
        let _ = rx.await;
        
        println!("\n{}", style("Stopping all test nodes...").yellow());
        node_manager.stop_nodes().await?;
        println!("{}", style("✨ All test nodes stopped successfully!").green());
        return Ok(());
    }

    // 通常モード: ポート設定
    let api_port = options.api_port.unwrap_or(53036);
    let frontend_port = options.frontend_port.unwrap_or(55938);

    // アプリケーション状態の初期化
    let app_state = AppState {
        api_port,
        frontend_port,
        locale: LocaleConfig::new("en"), // デフォルトは英語
        api_url: format!("http://localhost:{}", api_port),
        frontend_url: format!("http://localhost:{}", frontend_port),
    };

    // プログレスバーのスタイルを設定
    let spinner_style = ProgressStyle::with_template("{spinner:.green} {msg}")
        .unwrap()
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏");

    // サービスの起動
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(spinner_style.clone());
    spinner.set_message("Starting services...".to_string());
    spinner.enable_steady_tick(Duration::from_millis(100));

    // APIサーバーの起動
    let api_args = "cargo run".split_whitespace().collect::<Vec<&str>>();
    let _api_process = Command::new(api_args[0])
        .current_dir("api")
        .args(&api_args[1..])
        .args(["--bin", "api"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    // フロントエンドサーバーの起動
    let frontend_args = "cargo run".split_whitespace().collect::<Vec<&str>>();
    let _frontend_process = Command::new(frontend_args[0])
        .current_dir("frontend")
        .args(&frontend_args[1..])
        .args(["--bin", "frontend"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    // サービスの起動を待機
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    spinner.finish_with_message("✨ All services started successfully!");
    println!();

    // インタラクティブコンソールの起動
    AppOptions::interactive_console(&app_state).await?;

    // 終了処理
    println!("\n{}", style("Shutting down services...").yellow());
    let _ = Command::new("pkill")
        .args(["-f", "target/debug/api"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    let _ = Command::new("pkill")
        .args(["-f", "target/debug/frontend"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    println!("{}", style("✨ Thank you for using Rustorium!").green().bold());
    Ok(())
}