mod handlers;
mod messages;
mod session;

use std::sync::Arc;
use anyhow::Result;
use tokio::sync::RwLock;
use tokio_tungstenite::tungstenite::Message;
use futures::{StreamExt, SinkExt};
use crate::core::{
    token::TokenManager,
    dag::DAGManager,
    sharding::ShardManager,
};
use super::{ApiConfig, ConnectionManager};

/// WebSocketサーバー
pub struct WebSocketServer {
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

impl WebSocketServer {
    /// 新しいWebSocketサーバーを作成
    pub fn new(
        config: ApiConfig,
        token_manager: Arc<TokenManager>,
        dag_manager: Arc<DAGManager>,
        shard_manager: Arc<ShardManager>,
        connections: Arc<RwLock<ConnectionManager>>,
    ) -> Self {
        Self {
            config,
            token_manager,
            dag_manager,
            shard_manager,
            connections,
        }
    }

    /// サーバーを起動
    pub async fn start(&self) -> Result<()> {
        let addr = format!("{}:{}", self.config.host, self.config.ws_port);
        let listener = tokio::net::TcpListener::bind(&addr).await?;

        while let Ok((stream, _)) = listener.accept().await {
            let ws_stream = tokio_tungstenite::accept_async(stream).await?;
            let (mut write, mut read) = ws_stream.split();

            // セッションを作成
            let session = session::Session::new(
                self.token_manager.clone(),
                self.dag_manager.clone(),
                self.shard_manager.clone(),
            );

            // メッセージ処理ループを開始
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        // メッセージを処理
                        let response = session.handle_message(&text).await?;
                        write.send(Message::Text(response)).await?;
                    }
                    Ok(Message::Close(_)) => {
                        break;
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}

/// WebSocket接続
pub struct Connection {
    /// セッション
    session: session::Session,
    /// 購読チャネル
    subscriptions: Vec<String>,
}

impl Connection {
    /// 新しい接続を作成
    pub fn new(session: session::Session) -> Self {
        Self {
            session,
            subscriptions: Vec::new(),
        }
    }

    /// 接続を切断
    pub async fn disconnect(&mut self) -> Result<()> {
        // 購読を解除
        for channel in &self.subscriptions {
            self.session.unsubscribe(channel).await?;
        }
        self.subscriptions.clear();

        Ok(())
    }
}