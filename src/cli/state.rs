use std::sync::Arc;
use crate::{
    core::{
        dag::DAGManager,
        avalanche::AvalancheEngine,
        sharding::ShardManager,
        token::TokenManager,
    },
    network::P2PNetwork,
    i18n::LocaleConfig,
};

/// アプリケーションの状態
#[derive(Debug)]
pub struct AppState {
    /// APIポート
    pub api_port: u16,
    /// WebSocketポート
    pub ws_port: u16,
    /// GraphQLポート
    pub graphql_port: u16,
    /// ロケール設定
    pub locale: LocaleConfig,
    /// API URL
    pub api_url: String,
    /// WebSocket URL
    pub ws_url: String,
    /// GraphQL URL
    pub graphql_url: String,
    /// DAGマネージャー
    pub dag_manager: Arc<DAGManager>,
    /// Avalancheエンジン
    pub avalanche: Arc<AvalancheEngine>,
    /// シャードマネージャー
    pub shard_manager: Arc<ShardManager>,
    /// トークンマネージャー
    pub token_manager: Arc<TokenManager>,
    /// P2Pネットワーク
    pub network: Arc<P2PNetwork>,
}