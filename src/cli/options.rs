use std::{fs, path::PathBuf};
use anyhow::Result;
use libp2p::identity::Keypair;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rustorium", about = "Next-generation blockchain infrastructure")]
pub struct AppOptions {
    /// データディレクトリ
    #[structopt(long, default_value = "/tmp/rustorium")]
    pub data_dir: PathBuf,

    /// ベースポート (default: 8000)
    /// P2P: base_port
    /// Web UI: base_port + 1
    /// API: base_port + 2
    /// WebSocket: base_port + 3
    #[structopt(long, default_value = "8000")]
    pub base_port: u16,

    /// 外部公開アドレス
    #[structopt(long)]
    pub external_addr: Option<String>,

    /// ブートストラップノード（複数指定可）
    #[structopt(long)]
    pub bootstrap: Vec<String>,

    /// ノードの鍵ペアファイル
    #[structopt(long)]
    pub keypair: Option<String>,

    /// ログレベル (debug, info, warn, error)
    #[structopt(long, default_value = "info")]
    pub log_level: String,

    /// テストモード: 複数のノードを起動（開発用）
    #[structopt(long)]
    pub test: bool,

    /// テストモードのノード数 (default: 1)
    #[structopt(long, default_value = "1")]
    pub nodes: u8,
}

impl AppOptions {
    pub fn new() -> Self {
        Self::from_args()
    }

    /// 鍵ペアを読み込む
    pub fn load_keypair(&self) -> Result<Keypair> {
        if let Some(path) = &self.keypair {
            // ファイルから鍵ペアを読み込む
            let bytes = fs::read(path)?;
            Ok(Keypair::from_protobuf_encoding(&bytes)?)
        } else {
            // 新しい鍵ペアを生成
            Ok(Keypair::generate_ed25519())
        }
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