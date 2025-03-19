mod schema;
mod resolvers;
mod subscriptions;
mod context;

use std::sync::Arc;
use anyhow::Result;
use async_graphql::{
    Schema, EmptySubscription,
    http::{GraphQLPlaygroundConfig, playground_source},
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    Router,
    routing::get,
    response::Html,
    Extension,
};
use crate::core::{
    token::TokenManager,
    dag::DAGManager,
    sharding::ShardManager,
};
use super::ApiConfig;

/// GraphQLサーバー
pub struct GraphQLServer {
    /// 設定
    config: ApiConfig,
    /// トークンマネージャー
    token_manager: Arc<TokenManager>,
    /// DAGマネージャー
    dag_manager: Arc<DAGManager>,
    /// シャードマネージャー
    shard_manager: Arc<ShardManager>,
}

impl GraphQLServer {
    /// 新しいGraphQLサーバーを作成
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
        // スキーマを作成
        let schema = Schema::build(
            schema::QueryRoot,
            schema::MutationRoot,
            EmptySubscription,
        )
        .data(context::Context {
            token_manager: self.token_manager.clone(),
            dag_manager: self.dag_manager.clone(),
            shard_manager: self.shard_manager.clone(),
        })
        .finish();

        // ルーターを作成
        let app = Router::new()
            .route("/", get(graphql_playground).post(graphql_handler))
            .layer(Extension(schema));

        // サーバーを起動
        let addr = format!("{}:{}", self.config.host, self.config.graphql_port);
        axum::Server::bind(&addr.parse()?)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}

/// GraphQL接続
pub struct Connection {
    /// 購読ID
    subscription_id: String,
    /// コールバック
    callback: Box<dyn Fn(String) -> Result<()> + Send + Sync>,
}

impl Connection {
    /// 新しい接続を作成
    pub fn new(
        subscription_id: String,
        callback: Box<dyn Fn(String) -> Result<()> + Send + Sync>,
    ) -> Self {
        Self {
            subscription_id,
            callback,
        }
    }

    /// 接続を切断
    pub async fn disconnect(&mut self) -> Result<()> {
        // 購読を解除
        // TODO: 購読解除の実装
        Ok(())
    }
}

/// GraphQLプレイグラウンドハンドラー
async fn graphql_playground() -> Html<String> {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/")
            .subscription_endpoint("/ws"),
    ))
}

/// GraphQLハンドラー
async fn graphql_handler(
    schema: Extension<Schema<schema::QueryRoot, schema::MutationRoot, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}