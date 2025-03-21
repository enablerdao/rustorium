//! GQT API - RESTful APIとWebSocketインターフェース

use axum::{
    routing::{get, post},
    Router,
    response::IntoResponse,
    Json,
    extract::State,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;
use gqt_core::{
    Block, Transaction, Hash, Address,
    NetworkModule, ConsensusModule, StorageModule, RuntimeModule,
};
use tower_http::cors::{CorsLayer, Any};

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

/// APIルーターの作成
pub fn create_api_router(state: Arc<ApiState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/blocks", get(get_blocks))
        .route("/api/transactions", get(get_transactions))
        .route("/api/validators", get(get_validators))
        .route("/api/metrics", get(get_metrics))
        .layer(cors)
        .with_state(state)
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
