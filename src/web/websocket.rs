use anyhow::Result;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};
use futures::{stream::StreamExt, SinkExt};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{info, warn, error};

/// WebSocketハンドラー
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    state: Arc<WebSocketState>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

/// WebSocketの状態
pub struct WebSocketState {
    tx: broadcast::Sender<Event>,
}

impl WebSocketState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1000);
        Self { tx }
    }
    
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.tx.subscribe()
    }
    
    pub fn broadcast(&self, event: Event) -> Result<()> {
        self.tx.send(event)?;
        Ok(())
    }
}

/// WebSocketイベント
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Event {
    #[serde(rename = "new_block")]
    NewBlock(BlockEvent),
    
    #[serde(rename = "tx_confirmed")]
    TransactionConfirmed(TransactionEvent),
    
    #[serde(rename = "state_update")]
    StateUpdate(StateEvent),
}

/// ブロックイベント
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockEvent {
    pub number: u64,
    pub hash: String,
    pub timestamp: u64,
    pub tx_count: usize,
}

/// トランザクションイベント
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionEvent {
    pub hash: String,
    pub status: String,
    pub block_number: Option<u64>,
    pub timestamp: u64,
}

/// ステート更新イベント
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateEvent {
    pub key: String,
    pub value: String,
    pub timestamp: u64,
}

/// WebSocket接続の処理
async fn handle_socket(socket: WebSocket, state: Arc<WebSocketState>) {
    let (mut sender, mut receiver) = socket.split();
    
    // イベント購読
    let mut rx = state.subscribe();
    
    // 送信タスク
    let mut send_task = tokio::spawn(async move {
        while let Ok(event) = rx.recv().await {
            let msg = serde_json::to_string(&event)?;
            if let Err(e) = sender.send(Message::Text(msg)).await {
                error!("Failed to send message: {}", e);
                break;
            }
        }
        Ok::<_, anyhow::Error>(())
    });
    
    // 受信タスク
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    info!("Received message: {}", text);
                    // TODO: メッセージの処理
                }
                Message::Close(_) => {
                    info!("Client disconnected");
                    break;
                }
                _ => {}
            }
        }
    });
    
    // タスクの終了を待機
    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort();
        }
        _ = (&mut recv_task) => {
            send_task.abort();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    use tokio_tungstenite::connect_async;
    use url::Url;
    
    #[test]
    async fn test_websocket_connection() {
        // WebSocketサーバーの起動
        let state = Arc::new(WebSocketState::new());
        let server = axum::Server::bind(&"127.0.0.1:0".parse().unwrap())
            .serve(axum::Router::new()
                .route("/ws", axum::routing::get(ws_handler))
                .with_state(state.clone())
                .into_make_service());
            
        let addr = server.local_addr();
        tokio::spawn(server);
        
        // クライアントの接続
        let url = Url::parse(&format!("ws://127.0.0.1:{}/ws", addr.port())).unwrap();
        let (mut ws_stream, _) = connect_async(url).await.unwrap();
        
        // イベントの送信
        let event = Event::NewBlock(BlockEvent {
            number: 1,
            hash: "0x1234".into(),
            timestamp: 1234567890,
            tx_count: 10,
        });
        state.broadcast(event.clone()).unwrap();
        
        // イベントの受信
        if let Some(Ok(msg)) = ws_stream.next().await {
            let received: Event = serde_json::from_str(msg.to_text().unwrap()).unwrap();
            match received {
                Event::NewBlock(block) => {
                    assert_eq!(block.number, 1);
                    assert_eq!(block.hash, "0x1234");
                }
                _ => panic!("Unexpected event type"),
            }
        }
    }
}
