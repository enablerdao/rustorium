mod rest;
mod websocket;
mod graphql;
mod auth;
mod rate_limit;
mod error;

use std::sync::Arc;
use anyhow::Result;
use tokio::sync::RwLock;
use crate::core::{
    token::TokenManager,
    dag::DAGManager,
    sharding::ShardManager,
};

/// APIサーバーの設定
#[derive(Debug, Clone)]
pub struct ApiConfig {
    /// ホスト
    pub host: String,
    /// RESTポート
    pub rest_port: u16,
    /// WebSocketポート
    pub ws_port: u16,
    /// GraphQLポート
    pub graphql_port: u16,
    /// CORSオリジン
    pub cors_origin: String,
    /// レート制限（リクエスト/分）
    pub rate_limit: u32,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            rest_port: 8001,
            ws_port: 8002,
            graphql_port: 8003,
            cors_origin: "*".to_string(),
            rate_limit: 1000,
        }
    }
}

/// APIサーバー
pub struct ApiServer {
    /// 設定
    config: ApiConfig,
    /// トークンマネージャー
    token_manager: Arc<TokenManager>,
    /// DAGマネージャー
    dag_manager: Arc<DAGManager>,
    /// シャードマネージャー
    shard_manager: Arc<ShardManager>,
    /// 接続管理
    connections: Arc<RwLock<ConnectionManager>>,
}

impl ApiServer {
    /// 新しいAPIサーバーを作成
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
            connections: Arc::new(RwLock::new(ConnectionManager::new())),
        }
    }

    /// サーバーを起動
    pub async fn start(&self) -> Result<()> {
        // RESTサーバーを起動
        let rest_server = rest::RestServer::new(
            self.config.clone(),
            self.token_manager.clone(),
            self.dag_manager.clone(),
            self.shard_manager.clone(),
        );
        tokio::spawn(rest_server.start());

        // WebSocketサーバーを起動
        let ws_server = websocket::WebSocketServer::new(
            self.config.clone(),
            self.token_manager.clone(),
            self.dag_manager.clone(),
            self.shard_manager.clone(),
            self.connections.clone(),
        );
        tokio::spawn(ws_server.start());

        // GraphQLサーバーを起動
        let graphql_server = graphql::GraphQLServer::new(
            self.config.clone(),
            self.token_manager.clone(),
            self.dag_manager.clone(),
            self.shard_manager.clone(),
        );
        tokio::spawn(graphql_server.start());

        Ok(())
    }

    /// サーバーを停止
    pub async fn stop(&self) -> Result<()> {
        // 接続を切断
        let mut connections = self.connections.write().await;
        connections.disconnect_all().await?;

        Ok(())
    }
}

/// 接続管理
struct ConnectionManager {
    /// WebSocket接続
    ws_connections: Vec<websocket::Connection>,
    /// GraphQL Subscription接続
    graphql_connections: Vec<graphql::Connection>,
}

impl ConnectionManager {
    /// 新しい接続管理を作成
    fn new() -> Self {
        Self {
            ws_connections: Vec::new(),
            graphql_connections: Vec::new(),
        }
    }

    /// WebSocket接続を追加
    async fn add_ws_connection(&mut self, conn: websocket::Connection) {
        self.ws_connections.push(conn);
    }

    /// GraphQL接続を追加
    async fn add_graphql_connection(&mut self, conn: graphql::Connection) {
        self.graphql_connections.push(conn);
    }

    /// 全ての接続を切断
    async fn disconnect_all(&mut self) -> Result<()> {
        // WebSocket接続を切断
        for conn in &mut self.ws_connections {
            conn.disconnect().await?;
        }
        self.ws_connections.clear();

        // GraphQL接続を切断
        for conn in &mut self.graphql_connections {
            conn.disconnect().await?;
        }
        self.graphql_connections.clear();

        Ok(())
    }
}