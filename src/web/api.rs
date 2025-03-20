use axum::{
    Router,
    routing::{get, post},
    extract::State,
    response::{IntoResponse, Json},
};
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

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

/// APIルートページを表示
#[utoipa::path(
    get,
    path = "/",
    tag = "root",
    responses(
        (status = 200, description = "API information", body = ApiRootResponse)
    )
)]
async fn api_root() -> Result<Json<ApiRootResponse>> {
    Ok(Json(ApiRootResponse {
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
            Endpoint {
                path: "/ws/metrics".to_string(),
                method: "WebSocket".to_string(),
                description: "Real-time metrics updates".to_string(),
            },
            Endpoint {
                path: "/ws/blocks".to_string(),
                method: "WebSocket".to_string(),
                description: "Real-time block updates".to_string(),
            },
            Endpoint {
                path: "/ws/peers".to_string(),
                method: "WebSocket".to_string(),
                description: "Real-time peer updates".to_string(),
            },
        ],
    }))
}

/// HTMLレスポンスを返す
async fn api_root_html() -> impl IntoResponse {
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Rustorium Node API</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            line-height: 1.6;
            color: #2c3e50;
            max-width: 1200px;
            margin: 0 auto;
            padding: 2rem;
            background-color: #f5f6fa;
        }}
        .container {{
            background: white;
            border-radius: 8px;
            padding: 2rem;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }}
        h1 {{
            color: #2c3e50;
            margin-bottom: 1rem;
        }}
        .version {{
            color: #7f8c8d;
            font-size: 0.9rem;
        }}
        .description {{
            margin: 2rem 0;
            padding: 1rem;
            background: #f8f9fa;
            border-radius: 4px;
        }}
        .links {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 1rem;
            margin: 2rem 0;
        }}
        .link-card {{
            padding: 1rem;
            border: 1px solid #ddd;
            border-radius: 4px;
            text-align: center;
        }}
        .link-card a {{
            color: #3498db;
            text-decoration: none;
            font-weight: 500;
        }}
        .link-card a:hover {{
            text-decoration: underline;
        }}
        .endpoints {{
            margin-top: 2rem;
        }}
        .endpoint {{
            padding: 1rem;
            border: 1px solid #ddd;
            border-radius: 4px;
            margin-bottom: 1rem;
        }}
        .method {{
            display: inline-block;
            padding: 0.25rem 0.5rem;
            border-radius: 4px;
            font-weight: bold;
            margin-right: 0.5rem;
        }}
        .get {{ background: #dff0d8; color: #3c763d; }}
        .post {{ background: #d9edf7; color: #31708f; }}
        .ws {{ background: #fcf8e3; color: #8a6d3b; }}
    </style>
</head>
<body>
    <div class="container">
        <h1>Rustorium Node API <span class="version">v{}</span></h1>
        
        <div class="description">
            <p>Rustorium is a next-generation blockchain infrastructure built with Rust.
            This API provides access to node operations, metrics, and configuration.</p>
        </div>

        <div class="links">
            <div class="link-card">
                <a href="/api/docs">API Documentation (Swagger UI)</a>
            </div>
            <div class="link-card">
                <a href="/api/api-docs/openapi.json">OpenAPI Specification</a>
            </div>
            <div class="link-card">
                <a href="https://github.com/rustorium/rustorium">GitHub Repository</a>
            </div>
            <div class="link-card">
                <a href="https://rustorium.org">Website</a>
            </div>
        </div>

        <div class="endpoints">
            <h2>Available Endpoints</h2>
            
            <div class="endpoint">
                <span class="method get">GET</span>
                <code>/api/health</code>
                <p>Health check endpoint</p>
            </div>

            <div class="endpoint">
                <span class="method get">GET</span>
                <code>/api/metrics</code>
                <p>Get system metrics</p>
            </div>

            <div class="endpoint">
                <span class="method get">GET</span>
                <code>/api/config</code>
                <p>Get node configuration</p>
            </div>

            <div class="endpoint">
                <span class="method post">POST</span>
                <code>/api/config</code>
                <p>Update node configuration</p>
            </div>

            <div class="endpoint">
                <span class="method ws">WS</span>
                <code>/ws/metrics</code>
                <p>Real-time metrics updates</p>
            </div>

            <div class="endpoint">
                <span class="method ws">WS</span>
                <code>/ws/blocks</code>
                <p>Real-time block updates</p>
            </div>

            <div class="endpoint">
                <span class="method ws">WS</span>
                <code>/ws/peers</code>
                <p>Real-time peer updates</p>
            </div>
        </div>
    </div>
</body>
</html>"#,
        env!("CARGO_PKG_VERSION")
    );

    axum::response::Html(html)
}

/// APIルーターを作成
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(api_root_html))
        .route("/json", get(api_root))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/health", get(health_check))
        .route("/metrics", get(get_metrics))
        .route("/config", get(get_config))
        .route("/config", post(update_config))
        .with_state(state)
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

/// ヘルスチェック
#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "Health check successful", body = HealthResponse)
    )
)]
async fn health_check() -> Result<Json<HealthResponse>> {
    Ok(Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: chrono::Utc::now().timestamp(),
    }))
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
async fn get_metrics(
    State(state): State<AppState>,
) -> Result<Json<MetricsResponse>> {
    let config = &state.config;
    let cpu_cores = sys_info::cpu_num().unwrap_or(1) as i32;

    Ok(Json(MetricsResponse {
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
    }))
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
async fn get_config(
    State(state): State<AppState>,
) -> Result<Json<NodeConfig>> {
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
) -> Result<Json<serde_json::Value>> {
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