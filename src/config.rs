//! 設定ファイルの定義
//! 
//! このモジュールは、Rustoriumノードの設定を管理します。

use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use crate::cli::options::AppOptions;

/// ノードの設定
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NodeConfig {
    /// ノードの基本設定
    pub node: NodeSettings,
    /// ネットワーク設定
    pub network: NetworkSettings,
    /// API設定
    pub api: ApiSettings,
    /// Web UI設定
    pub web: WebSettings,
    /// WebSocket設定
    pub websocket: WebSocketSettings,
    /// バリデーター設定
    pub validator: ValidatorSettings,
    /// パフォーマンス設定
    pub performance: PerformanceSettings,
    /// ストレージ設定
    pub storage: StorageSettings,
    /// 開発モード設定
    pub dev: DevSettings,
}

/// ノードの基本設定
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NodeSettings {
    /// ノード名（空の場合はIDから自動生成）
    pub name: String,
    /// ノードの役割 (auto, validator, full, light)
    pub role: String,
    /// データディレクトリ
    pub data_dir: PathBuf,
    /// ログレベル
    pub log_level: String,
}

/// ネットワーク設定
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NetworkSettings {
    /// ネットワークの有効化
    pub enabled: bool,
    /// ホストアドレス
    pub host: String,
    /// 基本ポート（P2P用）
    pub port: u16,
    /// 外部公開アドレス
    pub external_addr: Option<String>,
    /// ブートストラップノード
    pub bootstrap_nodes: Vec<String>,
}

/// API設定
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApiSettings {
    /// APIの有効化
    pub enabled: bool,
    /// APIポートのオフセット
    pub port_offset: u16,
    /// レート制限（リクエスト/分）
    pub rate_limit: u32,
    /// CORS設定
    pub cors_origins: Vec<String>,
}

/// Web UI設定
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WebSettings {
    /// Web UIの有効化
    pub enabled: bool,
    /// Web UIポートのオフセット
    pub port_offset: u16,
    /// 起動時にブラウザを開く
    pub open_browser: bool,
}

/// WebSocket設定
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WebSocketSettings {
    /// WebSocketの有効化
    pub enabled: bool,
    /// WebSocketポートのオフセット
    pub port_offset: u16,
}

/// バリデーター設定
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ValidatorSettings {
    /// ステーク量
    pub stake: u64,
    /// 手数料率
    pub commission: f64,
    /// 最小ステーク量
    pub min_stake: u64,
}

/// パフォーマンス設定
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PerformanceSettings {
    /// 最大ピア数
    pub max_peers: u32,
    /// 最大保留トランザクション数
    pub max_pending_tx: u32,
    /// ブロック生成間隔（ミリ秒）
    pub block_time: u64,
}

/// ストレージ設定
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StorageSettings {
    /// ストレージエンジン
    pub engine: String,
    /// データベースパス
    pub path: PathBuf,
    /// 最大オープンファイル数
    pub max_open_files: u32,
    /// キャッシュサイズ（MB）
    pub cache_size: u32,
}

/// 開発モード設定
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DevSettings {
    /// ノード数
    pub nodes: u8,
    /// 開始ポート
    pub base_port: u16,
    /// 自動マイニング
    pub auto_mining: bool,
    /// ブロック生成間隔（ミリ秒）
    pub block_time: u64,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            node: NodeSettings {
                name: String::new(),
                role: "auto".to_string(),
                data_dir: PathBuf::from("data"),
                log_level: "info".to_string(),
            },
            network: NetworkSettings {
                enabled: true,
                host: "0.0.0.0".to_string(),
                port: 9070,  // ダッシュボードポート
                external_addr: None,
                bootstrap_nodes: vec![
                    // メインネットのブートストラップノード
                    "/ip4/mainnet.rustorium.org/tcp/4001/p2p/12D3KooWQP6ubbGrRFGSbDyiCuw2mi1LMNLFPmwgGsXfGJNRvn2v".to_string(),
                    "/ip4/mainnet2.rustorium.org/tcp/4001/p2p/12D3KooWBmT4c6YvhVYy3KmXMEGaxJXuTVqGtCwwS2GTncxSoje7".to_string(),
                ],
            },
            web: WebSettings {
                enabled: true,
                port_offset: 0,  // 9070 (ダッシュボード)
                open_browser: false,
            },
            api: ApiSettings {
                enabled: true,
                port_offset: 1,  // 9071 (API)
                rate_limit: 1000,
                cors_origins: vec!["*".to_string()],
            },
            websocket: WebSocketSettings {
                enabled: true,
                port_offset: 2,  // 9072 (WebSocket)
            },
            validator: ValidatorSettings {
                stake: 0,
                commission: 0.1,
                min_stake: 100000,
            },
            performance: PerformanceSettings {
                max_peers: 50,
                max_pending_tx: 10000,
                block_time: 2000,
            },
            storage: StorageSettings {
                engine: "rocksdb".to_string(),
                path: PathBuf::new(),  // 空のパスを設定
                max_open_files: 1000,
                cache_size: 512,
            },
            dev: DevSettings {
                nodes: 1,
                base_port: 8000,
                auto_mining: false,
                block_time: 2000,
            },
        }
    }
}

impl NodeConfig {
    /// 設定ファイルを読み込む
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let config_str = std::fs::read_to_string(path)?;
        let config: NodeConfig = toml::from_str(&config_str)?;
        Ok(config)
    }

    /// 設定ファイルを保存
    pub fn save(&self, path: &str) -> anyhow::Result<()> {
        let config_str = toml::to_string_pretty(self)?;
        std::fs::write(path, config_str)?;
        Ok(())
    }

    /// コマンドライン引数から設定を更新
    pub fn update_from_args(&mut self, args: &AppOptions) {
        // 基本設定
        self.node.data_dir = if args.data_dir.as_os_str().is_empty() {
            PathBuf::from("data")
        } else {
            args.data_dir.clone().into()
        };
        self.node.log_level = args.log_level.clone();

        // ネットワーク設定
        if let Some(ref addr) = args.external_addr {
            self.network.external_addr = Some(addr.clone());
        }
        if !args.bootstrap.is_empty() {
            self.network.bootstrap_nodes = args.bootstrap.clone();
        }

        // ポート設定
        self.network.port = args.base_port;
        self.web.port_offset = 1;
        self.api.port_offset = 2;
        self.websocket.port_offset = 3;

        // テストモード設定
        if args.test {
            // テストモードの設定
            self.dev.auto_mining = true;
            self.dev.nodes = args.nodes;
            self.dev.base_port = args.base_port;
            self.performance.block_time = 1000; // 1秒
            self.performance.max_peers = 10;
            self.performance.max_pending_tx = 1000;
        } else {
            // 本番モードの設定
            self.dev.auto_mining = false;
            self.performance.block_time = 2000; // 2秒
            self.performance.max_peers = 100;
            self.performance.max_pending_tx = 50000;
        }
    }

    /// Web UIのURL
    pub fn web_ui_url(&self) -> String {
        format!("http://localhost:{}", self.network.port + self.web.port_offset)
    }

    /// APIのURL
    pub fn api_url(&self) -> String {
        format!("http://localhost:{}", self.network.port + self.api.port_offset)
    }

    /// WebSocketのURL
    pub fn ws_url(&self) -> String {
        format!("ws://localhost:{}", self.network.port + self.websocket.port_offset)
    }

    /// ノードの役割を自動判定
    pub fn detect_role(&mut self) {
        // システム情報を取得
        let cpu_cores = sys_info::cpu_num().unwrap_or(1);
        let memory_gb = sys_info::mem_info().map(|m| m.total / 1024 / 1024).unwrap_or(0);

        // 役割を判定
        self.node.role = if memory_gb >= 16 && cpu_cores >= 4 {
            "validator".to_string()
        } else if memory_gb >= 8 && cpu_cores >= 2 {
            "full".to_string()
        } else {
            "light".to_string()
        };
    }

    /// 開発モードの設定を生成
    pub fn development() -> Self {
        let mut config = Self::default();
        config.node.name = "dev-node".to_string();
        config.node.data_dir = PathBuf::from("/tmp/rustorium/data");
        config.storage.path = PathBuf::from("/tmp/rustorium/data/storage");
        config.network.bootstrap_nodes.clear();
        config.dev.auto_mining = true;
        config.dev.block_time = 1000;
        config.performance.max_peers = 10;
        config.performance.max_pending_tx = 1000;
        config
    }

    /// 設定ファイルから読み込む
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        Self::load(path)
    }
}