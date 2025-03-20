use axum::{
    Router,
    routing::{get, post},
    extract::State,
    response::{IntoResponse, Json},
};
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};
use chrono::Utc;

use super::{AppState, AppError, Result};
use crate::config::NodeConfig;

#[derive(OpenApi)]
#[openapi(
    paths(
        api_root,
        health_check,
        get_metrics,
        get_config,
        update_config,
    ),
    components(
        schemas(
            ApiRootResponse,
            Documentation,
            Endpoint,
            HealthResponse,
            MetricsResponse,
            NodeConfig
        )
    ),
    tags(
        (name = "root", description = "API root information"),
        (name = "health", description = "Health check endpoints"),
        (name = "metrics", description = "System metrics endpoints"),
        (name = "config", description = "Configuration endpoints")
    )
)]
#[allow(dead_code)]
struct ApiDoc;

/// APIルートページのレスポンス
#[derive(Debug, Serialize, ToSchema)]
pub struct ApiRootResponse {
    name: String,
    version: String,
    description: String,
    documentation: Documentation,
    endpoints: Vec<Endpoint>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Documentation {
    swagger_ui: String,
    openapi_json: String,
    github_repo: String,
    website: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Endpoint {
    path: String,
    method: String,
    description: String,
}

/// ヘルスチェックレスポンス
#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    status: String,
    version: String,
    timestamp: i64,
}

/// メトリクスレスポンス
#[derive(Debug, Serialize, ToSchema)]
pub struct MetricsResponse {
    system: SystemMetrics,
    network: NetworkMetrics,
    performance: PerformanceMetrics,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SystemMetrics {
    cpu_cores: i32,
    memory_gb: u64,
    role: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct NetworkMetrics {
    p2p_port: u16,
    web_port: u16,
    api_port: u16,
    ws_port: u16,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PerformanceMetrics {
    max_peers: u32,
    max_pending_tx: u32,
    block_time: u64,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(api_root))
        .route("/health", get(health_check))
        .route("/metrics", get(get_metrics))
        .route("/config", get(get_config))
        .route("/config", post(update_config))
        .with_state(state)
}

/// APIルートページを表示
#[utoipa::path(
    get,
    path = "/",
    tag = "root",
    responses(
        (status = 200, description = "API information", body = ApiRootResponse)
    )
)]
async fn api_root() -> Result<impl IntoResponse> {
    let response = ApiRootResponse {
        name: "Rustorium Node API".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        description: "Rustorium is a next-generation blockchain infrastructure built with Rust. \
            This API provides access to node operations, metrics, and configuration.".to_string(),
        documentation: Documentation {
            swagger_ui: "/api/docs".to_string(),
            openapi_json: "/api/api-docs/openapi.json".to_string(),
            github_repo: "https://github.com/rustorium/rustorium".to_string(),
            website: "https://rustorium.org".to_string(),
        },
        endpoints: vec![
            Endpoint {
                path: "/api/health".to_string(),
                method: "GET".to_string(),
                description: "Health check endpoint".to_string(),
            },
            Endpoint {
                path: "/api/metrics".to_string(),
                method: "GET".to_string(),
                description: "Get system metrics".to_string(),
            },
            Endpoint {
                path: "/api/config".to_string(),
                method: "GET".to_string(),
                description: "Get node configuration".to_string(),
            },
            Endpoint {
                path: "/api/config".to_string(),
                method: "POST".to_string(),
                description: "Update node configuration".to_string(),
            },
        ],
    };

    Ok(Json(response))
}

/// ヘルスチェック
#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "Health check successful", body = HealthResponse)
    )
)]
async fn health_check() -> Result<impl IntoResponse> {
    let response = HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: Utc::now().timestamp(),
    };

    Ok(Json(response))
}

/// メトリクスを取得
#[utoipa::path(
    get,
    path = "/metrics",
    tag = "metrics",
    responses(
        (status = 200, description = "Metrics retrieved successfully", body = MetricsResponse),
        (status = 500, description = "Internal server error")
    )
)]
async fn get_metrics(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let config = &state.config;
    let cpu_cores = sys_info::cpu_num().unwrap_or(1) as i32;

    let response = MetricsResponse {
        system: SystemMetrics {
            cpu_cores,
            memory_gb: sys_info::mem_info()
                .map(|m| m.total / 1024 / 1024)
                .unwrap_or(0),
            role: config.node.role.clone(),
        },
        network: NetworkMetrics {
            p2p_port: config.network.port,
            web_port: config.network.port + config.web.port_offset,
            api_port: config.network.port + config.api.port_offset,
            ws_port: config.network.port + config.websocket.port_offset,
        },
        performance: PerformanceMetrics {
            max_peers: config.performance.max_peers,
            max_pending_tx: config.performance.max_pending_tx,
            block_time: config.performance.block_time,
        },
    };

    Ok(Json(response))
}

/// 設定を取得
#[utoipa::path(
    get,
    path = "/config",
    tag = "config",
    responses(
        (status = 200, description = "Configuration retrieved successfully", body = NodeConfig),
        (status = 500, description = "Internal server error")
    )
)]
async fn get_config(State(state): State<AppState>) -> Result<impl IntoResponse> {
    Ok(Json((*state.config).clone()))
}

/// 設定を更新
#[utoipa::path(
    post,
    path = "/config",
    tag = "config",
    request_body = NodeConfig,
    responses(
        (status = 200, description = "Configuration updated successfully"),
        (status = 400, description = "Invalid configuration"),
        (status = 500, description = "Internal server error")
    )
)]
async fn update_config(
    State(state): State<AppState>,
    Json(new_config): Json<NodeConfig>,
) -> Result<impl IntoResponse> {
    // 設定ファイルのパスを取得
    let config_path = std::path::PathBuf::from(&state.config.node.data_dir)
        .join("config.toml");

    // 設定を保存
    new_config.save(config_path.to_str().unwrap())
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Configuration updated successfully"
    })))
}