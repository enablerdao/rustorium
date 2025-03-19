use std::process::{Command, Stdio};
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use tracing::info;

pub struct ServerManager {
    pub api_port: u16,
    pub frontend_port: u16,
    pub api_only: bool,
    pub frontend_only: bool,
    pub fast: bool,
    pub release: bool,
}

impl ServerManager {
    pub fn new(
        api_port: u16,
        frontend_port: u16,
        api_only: bool,
        frontend_only: bool,
        fast: bool,
        release: bool,
    ) -> Self {
        Self {
            api_port,
            frontend_port,
            api_only,
            frontend_only,
            fast,
            release,
        }
    }

    pub fn get_cargo_command(&self) -> &'static str {
        if self.release {
            "cargo run --release"
        } else if self.fast {
            "cargo run --profile fast-dev"
        } else {
            "cargo run"
        }
    }

    pub async fn start_servers(&self) -> Result<()> {
        // プログレスバーのスタイルを設定
        let spinner_style = ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏");

        // サービスの起動
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(spinner_style.clone());
        spinner.set_message("Starting services...".to_string());
        spinner.enable_steady_tick(Duration::from_millis(100));

        let cargo_command = self.get_cargo_command();

        // APIサーバーを起動（フロントエンドのみモードでない場合）
        if !self.frontend_only {
            info!("Starting API server...");
            
            let api_args = cargo_command.split_whitespace().collect::<Vec<&str>>();
            let _api_process = Command::new(api_args[0])
                .current_dir("api")
                .args(&api_args[1..])
                .args(["--bin", "api"])
                .env("PORT", self.api_port.to_string())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()?;
        }

        // フロントエンドサーバーを起動（APIのみモードでない場合）
        if !self.api_only {
            info!("Starting frontend server...");
            
            let frontend_args = cargo_command.split_whitespace().collect::<Vec<&str>>();
            let _frontend_process = Command::new(frontend_args[0])
                .current_dir("frontend")
                .args(&frontend_args[1..])
                .args(["--bin", "frontend"])
                .env("PORT", self.frontend_port.to_string())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()?;
        }

        // サービスの起動を待機
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        spinner.finish_with_message("✨ All services started successfully!");
        println!();

        Ok(())
    }

    pub fn stop_servers(&self) -> Result<()> {
        // APIサーバーを停止
        if !self.frontend_only {
            let _ = Command::new("pkill")
                .args(["-f", "target/debug/api"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }

        // フロントエンドサーバーを停止
        if !self.api_only {
            let _ = Command::new("pkill")
                .args(["-f", "target/debug/frontend"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }

        Ok(())
    }
}