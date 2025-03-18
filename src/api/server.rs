use crate::api::handlers::{
    create_transaction, get_account, get_block, get_status, get_transaction, list_blocks, AppState,
};
use crate::common::config::ApiConfig;
use crate::storage::state::StateManager;
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{info, warn};

/// Start the API server
pub async fn start(port: u16) -> Result<(), anyhow::Error> {
    // Create a dummy state manager for now
    // In a real implementation, this would be passed from the node
    let db = Arc::new(crate::storage::db::Database::open("./data/db")?);
    let cache = Arc::new(crate::storage::cache::StorageCache::new(
        &crate::common::config::StorageConfig::default(),
    ));
    let state_manager = Arc::new(StateManager::new(db, cache)?);
    
    start_with_state(port, state_manager).await
}

/// Start the API server with a provided state manager
pub async fn start_with_state(port: u16, state_manager: Arc<StateManager>) -> Result<(), anyhow::Error> {
    let app_state = Arc::new(AppState {
        state_manager,
        start_time: std::time::Instant::now(),
    });
    
    // Set up CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    // Build router
    let app = Router::new()
        .route("/api/status", get(get_status))
        .route("/api/blocks", get(list_blocks))
        .route("/api/blocks/:height", get(get_block))
        .route("/api/transactions/:tx_id", get(get_transaction))
        .route("/api/transactions", post(create_transaction))
        .route("/api/accounts/:address", get(get_account))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);
    
    // Bind to address
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("API server listening on {}", addr);
    
    // Start server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    
    Ok(())
}

/// Start the API server with a custom configuration
pub async fn start_with_config(
    config: &ApiConfig,
    state_manager: Arc<StateManager>,
) -> Result<(), anyhow::Error> {
    if !config.enabled {
        warn!("API server is disabled in configuration");
        return Ok(());
    }
    
    let app_state = Arc::new(AppState {
        state_manager,
        start_time: std::time::Instant::now(),
    });
    
    // Set up CORS
    let cors = if config.cors_allowed_origins.contains(&"*".to_string()) {
        CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any)
    } else {
        let origins = config
            .cors_allowed_origins
            .iter()
            .filter_map(|origin| origin.parse().ok())
            .collect::<Vec<_>>();
        
        CorsLayer::new()
            .allow_origin(origins)
            .allow_methods(Any)
            .allow_headers(Any)
    };
    
    // Build router
    let app = Router::new()
        .route("/api/status", get(get_status))
        .route("/api/blocks", get(list_blocks))
        .route("/api/blocks/:height", get(get_block))
        .route("/api/transactions/:tx_id", get(get_transaction))
        .route("/api/transactions", post(create_transaction))
        .route("/api/accounts/:address", get(get_account))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);
    
    // Bind to address
    let addr = SocketAddr::new(config.listen_addr, config.listen_port);
    info!("API server listening on {}", addr);
    
    // Start server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    
    Ok(())
}