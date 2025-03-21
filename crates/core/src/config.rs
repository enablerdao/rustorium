//! GQT Core - 設定定義

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// GQTノードの設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// ノードID
    pub node_id: String,
    /// ネットワーク設定
    pub network: NetworkConfig,
    /// コンセンサス設定
    pub consensus: ConsensusConfig,
    /// ストレージ設定
    pub storage: StorageConfig,
    /// ランタイム設定
    pub runtime: RuntimeConfig,
    /// API設定
    pub api: ApiConfig,
    /// メトリクス設定
    pub metrics: MetricsConfig,
    /// ログ設定
    pub logging: LoggingConfig,
}

/// ネットワーク設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// モジュール名
    pub module: String,
    /// リッスンアドレス
    pub listen_addr: String,
    /// 外部アドレス
    pub external_addr: String,
    /// ブートストラップノード
    pub bootstrap_nodes: Vec<String>,
    /// 最大ピア数
    pub max_peers: usize,
    /// その他の設定
    pub extra: HashMap<String, serde_json::Value>,
}

/// コンセンサス設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusConfig {
    /// モジュール名
    pub module: String,
    /// バリデーター数
    pub validators: usize,
    /// ブロック時間（秒）
    pub block_time: u64,
    /// その他の設定
    pub extra: HashMap<String, serde_json::Value>,
}

/// ストレージ設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// モジュール名
    pub module: String,
    /// データディレクトリ
    pub data_dir: String,
    /// その他の設定
    pub extra: HashMap<String, serde_json::Value>,
}

/// ランタイム設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    /// モジュール名
    pub module: String,
    /// メモリ制限（バイト）
    pub memory_limit: u64,
    /// その他の設定
    pub extra: HashMap<String, serde_json::Value>,
}

/// API設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// リッスンアドレス
    pub listen_addr: String,
    /// 有効なエンドポイント
    pub enabled_endpoints: Vec<String>,
    /// CORS設定
    pub cors: CorsConfig,
    /// その他の設定
    pub extra: HashMap<String, serde_json::Value>,
}

/// CORS設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    /// 許可するオリジン
    pub allowed_origins: Vec<String>,
    /// 許可するメソッド
    pub allowed_methods: Vec<String>,
    /// 許可するヘッダー
    pub allowed_headers: Vec<String>,
}

/// メトリクス設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// 有効化フラグ
    pub enabled: bool,
    /// リッスンアドレス
    pub listen_addr: String,
}

/// ログ設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// ログレベル
    pub level: String,
    /// ログファイル
    pub file: Option<String>,
    /// JSON形式フラグ
    pub json: bool,
}

impl Config {
    /// 設定ファイルを読み込む
    pub fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }

    /// デフォルト設定を生成
    pub fn default() -> Self {
        Self {
            node_id: "gqt-node-0".to_string(),
            network: NetworkConfig {
                module: "quic".to_string(),
                listen_addr: "0.0.0.0:9000".to_string(),
                external_addr: "127.0.0.1:9000".to_string(),
                bootstrap_nodes: vec![],
                max_peers: 50,
                extra: HashMap::new(),
            },
            consensus: ConsensusConfig {
                module: "hotstuff".to_string(),
                validators: 4,
                block_time: 1,
                extra: HashMap::new(),
            },
            storage: StorageConfig {
                module: "tikv".to_string(),
                data_dir: "data".to_string(),
                extra: HashMap::new(),
            },
            runtime: RuntimeConfig {
                module: "wasm".to_string(),
                memory_limit: 1024 * 1024 * 1024, // 1GB
                extra: HashMap::new(),
            },
            api: ApiConfig {
                listen_addr: "0.0.0.0:9001".to_string(),
                enabled_endpoints: vec!["rest".to_string(), "websocket".to_string()],
                cors: CorsConfig {
                    allowed_origins: vec!["*".to_string()],
                    allowed_methods: vec!["GET".to_string(), "POST".to_string()],
                    allowed_headers: vec!["*".to_string()],
                },
                extra: HashMap::new(),
            },
            metrics: MetricsConfig {
                enabled: true,
                listen_addr: "0.0.0.0:9002".to_string(),
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file: None,
                json: false,
            },
        }
    }
}
