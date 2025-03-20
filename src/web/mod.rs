//! Webサーバーの実装
//! 
//! このモジュールは、RustoriumのWebサーバーを実装します。
//! 主な機能：
//! - HTTP/WebSocket サーバー
//! - 静的ファイルの提供
//! - CORS対応

pub mod api;

use std::sync::Arc;
use axum::{
    Router,
    routing::get,
    response::{IntoResponse, Response},
    http::{StatusCode, header},
    Json,
};
use tower::ServiceBuilder;
use tower_http::{
    trace::TraceLayer,
    cors::{CorsLayer, Any},
};

use tracing::info;
use rust_embed::RustEmbed;
use serde_json::json;
use thiserror::Error;
use crate::config::NodeConfig;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Authentication required")]
    Unauthorized,

    #[error("Permission denied: {0}")]
    Forbidden(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid request: {0}")]
    BadRequest(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            Self::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            Self::ServiceUnavailable(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg),
        };

        let body = Json(json!({
            "error": {
                "message": error_message,
                "code": status.as_u16(),
                "type": status.canonical_reason().unwrap_or("Unknown")
            }
        }));

        (status, body).into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self::Internal(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::Internal(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        Self::BadRequest(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;

/// アプリケーションの状態
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<NodeConfig>,
}

#[derive(RustEmbed)]
#[folder = "frontend/"]
struct Asset;

#[derive(Debug, Clone)]
pub struct WebServer {
    port: u16,
    config: Arc<NodeConfig>,
    shutdown: Arc<tokio::sync::Notify>,
}

impl WebServer {
    pub fn new(port: u16, config: NodeConfig) -> Self {
        Self {
            port,
            config: Arc::new(config),
            shutdown: Arc::new(tokio::sync::Notify::new()),
        }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        // APIルーターの作成
        let api_router = api::create_router(AppState { config: self.config.clone() });

        // 静的ファイルのハンドラー
        let static_handler = get(serve_static);

        // ルーターの作成
        let app = Router::new()
            .nest("/api", api_router)
            .route("/", static_handler.clone())
            .route("/index.html", static_handler.clone())
            .route("/manifest.json", static_handler.clone())
            .route("/sw.js", static_handler.clone())
            .route("/css/*file", static_handler.clone())
            .route("/js/*file", static_handler.clone())
            .route("/icons/*file", static_handler.clone())
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods(Any)
                        .allow_headers(Any))
                    .into_inner()
            );

        // サーバーの起動
        use std::net::{IpAddr, Ipv4Addr, SocketAddr};
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), self.port);
        info!("Starting web server on 0.0.0.0:{}", self.port);
        
        let shutdown_signal = self.shutdown.clone();
        let listener = tokio::net::TcpListener::bind(addr).await?;
        let server = axum::serve(listener, app);

        let graceful = server.with_graceful_shutdown(async move {
            shutdown_signal.notified().await;
        });

        graceful.await?;
        info!("Web server stopped");

        Ok(())
    }

    pub fn shutdown(&self) {
        self.shutdown.notify_one();
    }
}

async fn serve_static(uri: axum::http::Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };

    info!("Serving static file: {}", path);

    match Asset::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            info!("Found file: {} (mime: {})", path, mime);
            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(axum::body::Body::from(content.data))
                .unwrap()
        }
        None => {
            info!("File not found: {}", path);
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("404 Not Found".into())
                .unwrap()
        }
    }
}