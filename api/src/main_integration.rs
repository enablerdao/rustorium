// 新機能の統合デモ
// 持続可能なコンセンサスメカニズム、動的報酬システム、リソース使用効率モニタリング、適応型スケーリングの基盤設計

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::consensus::{ConsensusConfig, ConsensusManager, ConsensusStatus, Validator};
use crate::scaling::{ScalingConfig, ScalingManager, ScalingStatus, ScalingMode};

// アプリケーション状態
pub struct AppState {
    // コンセンサスマネージャー
    pub consensus_manager: Arc<ConsensusManager>,
    
    // スケーリングマネージャー
    pub scaling_manager: Arc<Mutex<ScalingManager>>,
}

// APIレスポンス
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }
    
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

// バリデーター登録リクエスト
#[derive(Debug, Deserialize)]
pub struct RegisterValidatorRequest {
    pub address: String,
    pub stake: f64,
    pub public_key: Vec<u8>,
}

// シャード数設定リクエスト
#[derive(Debug, Deserialize)]
pub struct SetShardCountRequest {
    pub count: usize,
}

// メトリクス更新リクエスト
#[derive(Debug, Deserialize)]
pub struct UpdateMetricsRequest {
    pub tps: f64,
    pub node_count: usize,
}

// 統合デモ用のルーターを作成
pub fn create_integration_router() -> Router {
    // コンセンサス設定
    let consensus_config = ConsensusConfig::default();
    let consensus_manager = Arc::new(ConsensusManager::new(consensus_config));
    
    // スケーリング設定
    let scaling_config = ScalingConfig::default();
    let scaling_manager = Arc::new(Mutex::new(ScalingManager::new(scaling_config)));
    
    // アプリケーション状態
    let app_state = Arc::new(AppState {
        consensus_manager: consensus_manager.clone(),
        scaling_manager: scaling_manager.clone(),
    });
    
    // ルーターの作成
    Router::new()
        // コンセンサス関連のエンドポイント
        .route("/consensus/status", get(get_consensus_status))
        .route("/consensus/validators", get(get_validators))
        .route("/consensus/validators", post(register_validator))
        .route("/consensus/validators/:address", delete(unregister_validator))
        
        // スケーリング関連のエンドポイント
        .route("/scaling/status", get(get_scaling_status))
        .route("/scaling/shards", post(set_shard_count))
        .route("/scaling/metrics", post(update_metrics))
        
        // アプリケーション状態を共有
        .with_state(app_state)
}

// コンセンサスステータスの取得
async fn get_consensus_status(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let status = state.consensus_manager.get_status();
    
    (StatusCode::OK, Json(ApiResponse::success(status)))
}

// バリデーターリストの取得
async fn get_validators(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let validators = state.consensus_manager.get_validators();
    
    (StatusCode::OK, Json(ApiResponse::success(validators)))
}

// バリデーターの登録
async fn register_validator(
    State(state): State<Arc<AppState>>,
    Json(request): Json<RegisterValidatorRequest>,
) -> impl IntoResponse {
    let validator = Validator::new(
        request.address,
        request.stake,
        request.public_key,
    );
    
    match state.consensus_manager.register_validator(validator) {
        Ok(_) => (
            StatusCode::CREATED,
            Json(ApiResponse::<String>::success("Validator registered successfully".to_string())),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<String>::error(e)),
        ),
    }
}

// バリデーターの削除
async fn unregister_validator(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> impl IntoResponse {
    match state.consensus_manager.unregister_validator(&address) {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse::<String>::success("Validator unregistered successfully".to_string())),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<String>::error(e)),
        ),
    }
}

// スケーリングステータスの取得
async fn get_scaling_status(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let scaling_manager = state.scaling_manager.lock().unwrap();
    let status = scaling_manager.get_status();
    
    (StatusCode::OK, Json(ApiResponse::success(status)))
}

// シャード数の設定
async fn set_shard_count(
    State(state): State<Arc<AppState>>,
    Json(request): Json<SetShardCountRequest>,
) -> impl IntoResponse {
    let scaling_manager = state.scaling_manager.lock().unwrap();
    
    match scaling_manager.set_shard_count(request.count) {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse::<String>::success("Shard count updated successfully".to_string())),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<String>::error(e)),
        ),
    }
}

// メトリクスの更新
async fn update_metrics(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UpdateMetricsRequest>,
) -> impl IntoResponse {
    let scaling_manager = state.scaling_manager.lock().unwrap();
    scaling_manager.update_metrics(request.tps, request.node_count);
    
    // スケーリングの実行
    match scaling_manager.scale() {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse::<String>::success("Metrics updated and scaling executed".to_string())),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<String>::error(e)),
        ),
    }
}