use std::{fs, path::PathBuf};
use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "rustorium", about = "Next-generation blockchain infrastructure")]
pub struct AppOptions {
    /// データディレクトリ
    #[clap(long, default_value = "/tmp/rustorium")]
    pub data_dir: PathBuf,

    /// ベースポート (default: 9070)
    /// P2P: base_port
    /// Web UI: base_port + 1
    /// API: base_port + 2
    /// WebSocket: base_port + 3
    #[clap(long, default_value = "9070")]
    pub base_port: u16,

    /// 外部公開アドレス
    #[clap(long)]
    pub external_addr: Option<String>,

    /// ブートストラップノード（複数指定可）
    #[clap(long)]
    pub bootstrap: Vec<String>,

    /// ログレベル (debug, info, warn, error)
    #[clap(long, default_value = "info")]
    pub log_level: String,

    /// 開発モード
    #[clap(long)]
    pub dev: bool,

    /// テストモード: 複数のノードを起動（開発用）
    #[clap(long)]
    pub test: bool,

    /// テストモードのノード数 (default: 1)
    #[clap(long, default_value = "1")]
    pub nodes: u8,

    /// CUIを開かずにバックグラウンドで実行
    #[clap(long)]
    pub no_interactive: bool,

    /// メトリクスを有効化
    #[clap(long)]
    pub metrics: bool,

    /// デバッグモード
    #[clap(long)]
    pub debug: bool,
}

impl AppOptions {
    pub fn new() -> Self {
        Self::parse()
    }

    /// テストモードかどうか
    pub fn is_test(&self) -> bool {
        self.test
    }

    /// Web UIのURL
    pub fn web_ui_url(&self) -> String {
        format!("http://localhost:{}", self.base_port + 1)
    }

    /// APIのURL
    pub fn api_url(&self) -> String {
        format!("http://localhost:{}/api", self.base_port + 1)
    }

    /// WebSocketのURL
    pub fn ws_url(&self) -> String {
        format!("ws://localhost:{}/ws", self.base_port + 1)
    }
}
