mod handlers;
mod middleware;
mod routes;

use std::sync::Arc;
use anyhow::Result;
use axum::{
    Router,
    routing::{get, post},
    Extension,
};
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
use crate::core::{
    token::TokenManager,
    dag::DAGManager,
    sharding::ShardManager,
};
use super::ApiConfig;

/// RESTサーバー
pub struct RestServer {
    /// 設定
    config: ApiConfig,
    /// トークンマネージャー
    token_manager: Arc<TokenManager>,
    /// DAGマネージャー
    dag_manager: Arc<DAGManager>,
    /// シャードマネージャー
    shard_manager: Arc<ShardManager>,
}

impl RestServer {
    /// 新しいRESTサーバーを作成
    pub fn new(
        config: ApiConfig,
        token_manager: Arc<TokenManager>,
        dag_manager: Arc<DAGManager>,
        shard_manager: Arc<ShardManager>,
    ) -> Self {
        Self {
            config,
            token_manager,
            dag_manager,
            shard_manager,
        }
    }

    /// サーバーを起動
    pub async fn start(&self) -> Result<()> {
        // ルーターを作成
        let app = Router::new()
            // トークンエンドポイント
            .route("/tokens", get(handlers::token::list_tokens))
            .route("/tokens", post(handlers::token::create_token))
            .route("/tokens/:id", get(handlers::token::get_token))
            .route("/tokens/:id", post(handlers::token::update_token))
            .route("/tokens/:id/transfer", post(handlers::token::transfer_token))

            // トランザクションエンドポイント
            .route("/transactions", post(handlers::transaction::submit_transaction))
            .route("/transactions/:id", get(handlers::transaction::get_transaction))
            .route("/transactions", get(handlers::transaction::list_transactions))

            // アカウントエンドポイント
            .route("/accounts/:address/balance", get(handlers::account::get_balance))
            .route("/accounts/:address/transactions", get(handlers::account::get_transactions))

            // ミドルウェア
            .layer(Extension(self.token_manager.clone()))
            .layer(Extension(self.dag_manager.clone()))
            .layer(Extension(self.shard_manager.clone()))
            .layer(
                CorsLayer::new()
                    .allow_origin(self.config.cors_origin.parse()?)
                    .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allow_headers(vec!["Authorization", "Content-Type"])
            )
            .layer(TraceLayer::new_for_http())
            .layer(middleware::auth::AuthLayer::new())
            .layer(middleware::rate_limit::RateLimitLayer::new(
                self.config.rate_limit
            ));

        // サーバーを起動
        let addr = format!("{}:{}", self.config.host, self.config.rest_port);
        axum::Server::bind(&addr.parse()?)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}