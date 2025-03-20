use std::sync::Arc;
use axum::{
    Router,
    routing::get,
    extract::{WebSocketUpgrade, State},
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Serialize, Deserialize};
use tokio::sync::broadcast;
use tracing::{info, error};

use super::{AppState, error::Result};

/// WebSocketメッセージ
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WsMessage {
    /// システムメトリクス
    Metrics(MetricsData),
    /// ブロックチェーン更新
    BlockUpdate(BlockData),
    /// ピア更新
    PeerUpdate(PeerData),
    /// エラー
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsData {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub network_in: u64,
    pub network_out: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockData {
    pub height: u64,
    pub hash: String,
    pub transactions: u32,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerData {
    pub connected: u32,
    pub total_known: u32,
    pub bandwidth: f64,
}

/// WebSocketルーターを作成
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/metrics", get(handle_metrics_ws))
        .route("/blocks", get(handle_blocks_ws))
        .route("/peers", get(handle_peers_ws))
        .with_state(state)
}

/// メトリクスWebSocketハンドラ
async fn handle_metrics_ws(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_metrics_socket(socket, state))
}

/// ブロックWebSocketハンドラ
async fn handle_blocks_ws(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_blocks_socket(socket, state))
}

/// ピアWebSocketハンドラ
async fn handle_peers_ws(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_peers_socket(socket, state))
}

/// メトリクスWebSocketの処理
async fn handle_metrics_socket(
    mut socket: axum::extract::ws::WebSocket,
    state: AppState,
) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.metrics.subscribe();

    // メトリクス更新を送信
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Err(e) = sender
                .send(axum::extract::ws::Message::Text(
                    serde_json::to_string(&WsMessage::Metrics(msg)).unwrap()
                ))
                .await
            {
                error!("Failed to send metrics: {}", e);
                break;
            }
        }
    });

    // クライアントメッセージを受信
    while let Some(Ok(_)) = receiver.next().await {
        // Pingなどの制御メッセージは無視
    }
}

/// ブロックWebSocketの処理
async fn handle_blocks_socket(
    mut socket: axum::extract::ws::WebSocket,
    state: AppState,
) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.metrics.subscribe_blocks();

    // ブロック更新を送信
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Err(e) = sender
                .send(axum::extract::ws::Message::Text(
                    serde_json::to_string(&WsMessage::BlockUpdate(msg)).unwrap()
                ))
                .await
            {
                error!("Failed to send block update: {}", e);
                break;
            }
        }
    });

    // クライアントメッセージを受信
    while let Some(Ok(_)) = receiver.next().await {
        // Pingなどの制御メッセージは無視
    }
}

/// ピアWebSocketの処理
async fn handle_peers_socket(
    mut socket: axum::extract::ws::WebSocket,
    state: AppState,
) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.metrics.subscribe_peers();

    // ピア更新を送信
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Err(e) = sender
                .send(axum::extract::ws::Message::Text(
                    serde_json::to_string(&WsMessage::PeerUpdate(msg)).unwrap()
                ))
                .await
            {
                error!("Failed to send peer update: {}", e);
                break;
            }
        }
    });

    // クライアントメッセージを受信
    while let Some(Ok(_)) = receiver.next().await {
        // Pingなどの制御メッセージは無視
    }
}