use crate::common::errors::LedgerError;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get_service,
    Router,
};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tower_http::services::ServeDir;
use tracing::{error, info};

/// Web server configuration
pub struct WebServerConfig {
    pub port: u16,
    pub static_dir: PathBuf,
}

impl Default for WebServerConfig {
    fn default() -> Self {
        Self {
            port: 8081,
            static_dir: PathBuf::from("./web/dist"),
        }
    }
}

/// Start the web server
pub async fn start_web_server(config: WebServerConfig) -> Result<(), LedgerError> {
    info!("Starting web server on port {}", config.port);
    
    // Create static directory if it doesn't exist
    if !config.static_dir.exists() {
        fs::create_dir_all(&config.static_dir)
            .await
            .map_err(|e| LedgerError::Io(e))?;
    }
    
    // Create index.html if it doesn't exist
    let index_path = config.static_dir.join("index.html");
    if !index_path.exists() {
        let index_html = include_str!("../../web/index.html");
        fs::write(&index_path, index_html)
            .await
            .map_err(|e| LedgerError::Io(e))?;
    }
    
    // Serve static files
    let static_service = get_service(ServeDir::new(&config.static_dir))
        .handle_error(|error| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            )
        });
    
    // Create router
    let app = Router::new()
        .nest_service("/static", static_service.clone())
        .fallback_service(static_service);
    
    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    
    info!("Web server listening on {}", addr);
    
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app)
        .await
        .map_err(|e| LedgerError::Io(e.into()))?;
    
    Ok(())
}