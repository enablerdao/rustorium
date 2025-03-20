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
    routing::get_service,
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use tower_http::{
    services::ServeDir,
    cors::CorsLayer,
};
use tracing::{info, error};
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
        // 静的ファイルのハンドラー
        let serve_dir = ServeDir::new("frontend");

        // ルーターの作成
        let app = Router::new()
            .nest("/api", api::create_router(AppState { config: self.config.clone() }))
            .nest_service("/", get_service(serve_dir))
            .layer(CorsLayer::permissive());

        // サーバーの起動
        let addr = std::net::SocketAddr::from(([0, 0, 0, 0], self.port));
        info!("Starting web server on {}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await?;
        let server = axum::serve(listener, app);

        // シャットダウンシグナルを待機
        let shutdown_signal = self.shutdown.clone();
        tokio::select! {
            result = server => {
                if let Err(e) = result {
                    error!("Web server error: {}", e);
                }
            }
            _ = shutdown_signal.notified() => {
                info!("Shutdown signal received");
            }
        }

        info!("Web server stopped");
        Ok(())
    }

    pub fn shutdown(&self) {
        self.shutdown.notify_one();
    }
}