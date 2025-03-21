//! Rustorium API - RESTful APIとWebSocketインターフェース

use axum::{
    routing::{get, post, get_service},
    Router,
    response::IntoResponse,
    Json,
    extract::State,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{sync::Arc, net::SocketAddr, path::PathBuf};
use tokio::sync::RwLock;
use tower_http::{
    cors::{CorsLayer, Any},
    services::ServeDir,
};
use rustorium_core::{
    Block, Transaction, Hash, Address,
    NetworkModule, ConsensusModule, StorageModule, RuntimeModule,
};

/// APIの状態
pub struct ApiState {
    /// ネットワークモジュール
    pub network: Arc<RwLock<Box<dyn NetworkModule>>>,
    /// コンセンサスモジュール
    pub consensus: Arc<RwLock<Box<dyn ConsensusModule>>>,
    /// ストレージモジュール
    pub storage: Arc<RwLock<Box<dyn StorageModule>>>,
    /// ランタイムモジュール
    pub runtime: Arc<RwLock<Box<dyn RuntimeModule>>>,
}

/// APIサーバーの設定
#[derive(Debug, Clone)]
pub struct ApiConfig {
    /// Web UIのポート
    pub web_port: u16,
    /// REST APIのポート
    pub api_port: u16,
    /// WebSocketのポート
    pub ws_port: u16,
    /// 静的ファイルのディレクトリ
    pub static_dir: PathBuf,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            web_port: 9070,
            api_port: 9071,
            ws_port: 9072,
            static_dir: PathBuf::from("frontend/dist"),
        }
    }
}

/// APIサーバー
pub struct ApiServer {
    /// API状態
    state: Arc<ApiState>,
    /// 設定
    config: ApiConfig,
}

impl ApiServer {
    /// 新しいAPIサーバーを作成
    pub fn new(state: Arc<ApiState>, config: ApiConfig) -> Self {
        Self { state, config }
    }

    /// サーバーを起動
    pub async fn start(&self) -> anyhow::Result<()> {
        // CORSの設定
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        // Web UIサーバー
        let web_app = Router::new()
            .fallback_service(get_service(ServeDir::new(&self.config.static_dir)))
            .layer(cors.clone());

        // REST APIサーバー
        let api_app = Router::new()
            .route("/api/blocks", get(get_blocks))
            .route("/api/transactions", get(get_transactions))
            .route("/api/validators", get(get_validators))
            .route("/api/metrics", get(get_metrics))
            .layer(cors.clone())
            .with_state(self.state.clone());

        // WebSocketサーバー
        let ws_app = Router::new()
            .route("/ws", get(ws_handler))
            .layer(cors)
            .with_state(self.state.clone());

        // 各サーバーを起動
        let web_addr = SocketAddr::from(([127, 0, 0, 1], self.config.web_port));
        let api_addr = SocketAddr::from(([127, 0, 0, 1], self.config.api_port));
        let ws_addr = SocketAddr::from(([127, 0, 0, 1], self.config.ws_port));

        tracing::info!("Starting Web UI server on {}", web_addr);
        tracing::info!("Starting REST API server on {}", api_addr);
        tracing::info!("Starting WebSocket server on {}", ws_addr);

        tokio::try_join!(
            axum::Server::bind(&web_addr).serve(web_app.into_make_service()),
            axum::Server::bind(&api_addr).serve(api_app.into_make_service()),
            axum::Server::bind(&ws_addr).serve(ws_app.into_make_service()),
        )?;

        Ok(())
    }
}

async fn get_blocks(
    State(state): State<Arc<ApiState>>,
) -> impl IntoResponse {
    let storage = state.storage.read().await;
    let latest_block = storage.get_latest_block().await.unwrap_or(None);

    if let Some(block) = latest_block {
        Json(json!([{
            "number": block.header.number,
            "hash": hex::encode(block.hash().0),
            "transactions": block.transactions.iter().map(|tx| {
                hex::encode(tx.hash().0)
            }).collect::<Vec<_>>(),
            "timestamp": block.header.timestamp,
            "validator": hex::encode(block.header.validator.0),
        }]))
    } else {
        Json(json!([]))
    }
}

async fn get_transactions(
    State(state): State<Arc<ApiState>>,
) -> impl IntoResponse {
    let storage = state.storage.read().await;
    let latest_block = storage.get_latest_block().await.unwrap_or(None);

    if let Some(block) = latest_block {
        Json(json!(block.transactions.iter().map(|tx| {
            json!({
                "hash": hex::encode(tx.hash().0),
                "from": hex::encode(tx.from.0),
                "to": hex::encode(tx.to.0),
                "value": tx.value,
                "timestamp": block.header.timestamp,
            })
        }).collect::<Vec<_>>()))
    } else {
        Json(json!([]))
    }
}

async fn get_validators(
    State(state): State<Arc<ApiState>>,
) -> impl IntoResponse {
    let consensus = state.consensus.read().await;
    let metrics = consensus.metrics().await.unwrap_or_default();

    Json(json!([{
        "address": "0x1234...",
        "stake": "1000000",
        "isOnline": true,
        "lastVote": 12345,
    }]))
}

async fn get_metrics(
    State(state): State<Arc<ApiState>>,
) -> impl IntoResponse {
    let network = state.network.read().await;
    let consensus = state.consensus.read().await;
    let storage = state.storage.read().await;

    let network_metrics = network.metrics().await.unwrap_or_default();
    let consensus_metrics = consensus.metrics().await.unwrap_or_default();
    let storage_metrics = storage.metrics().await.unwrap_or_default();

    Json(json!({
        "tps": 1234,
        "blockTime": 1.2,
        "validatorCount": 21,
        "networkSize": 1234567890,
    }))
}

async fn ws_handler(
    ws: axum::extract::WebSocketUpgrade,
    State(state): State<Arc<ApiState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_ws_connection(socket, state))
}

async fn handle_ws_connection(
    mut socket: axum::extract::ws::WebSocket,
    state: Arc<ApiState>,
) {
    use axum::extract::ws::Message;
    use futures::{sink::SinkExt, stream::StreamExt};

    // 1秒ごとにメトリクスを送信
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));

    loop {
        tokio::select! {
            _ = interval.tick() => {
                let network = state.network.read().await;
                let consensus = state.consensus.read().await;
                let storage = state.storage.read().await;

                let metrics = json!({
                    "type": "metrics",
                    "data": {
                        "tps": 1234,
                        "blockTime": 1.2,
                        "validatorCount": 21,
                        "networkSize": 1234567890,
                    }
                });

                if let Err(e) = socket.send(Message::Text(metrics.to_string())).await {
                    tracing::error!("Failed to send metrics: {}", e);
                    break;
                }
            }
            Some(msg) = socket.next() => {
                match msg {
                    Ok(Message::Text(text)) => {
                        tracing::debug!("Received message: {}", text);
                        // TODO: メッセージの処理
                    }
                    Ok(Message::Close(_)) => break,
                    Err(e) => {
                        tracing::error!("WebSocket error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
}
