use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::info;
use serde_json;

use crate::blockchain::{BlockchainState, Transaction};

// トランザクション作成リクエスト
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransactionRequest {
    pub from: String,
    pub to: String,
    pub amount: f64,
    #[serde(default)]
    pub data: Option<String>,
    #[serde(default = "default_gas_price")]
    pub gas_price: u64,
    #[serde(default = "default_gas_limit")]
    pub gas_limit: u64,
}

fn default_gas_price() -> u64 {
    5
}

fn default_gas_limit() -> u64 {
    21000
}

// APIレスポンス
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }
    
    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

// アプリケーション状態
pub struct AppState {
    pub blockchain_state: BlockchainState,
}

// ハンドラー関数
async fn get_status(State(state): State<Arc<Mutex<AppState>>>) -> (StatusCode, Json<ApiResponse<HashMap<String, serde_json::Value>>>) {
    let app_state = state.lock().unwrap();
    let blockchain = app_state.blockchain_state.blockchain.lock().unwrap();
    
    let stats = blockchain.get_network_stats();
    
    (StatusCode::OK, Json(ApiResponse::success(stats)))
}

async fn get_block(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(block_id): Path<String>,
) -> (StatusCode, Json<ApiResponse<crate::blockchain::Block>>) {
    let app_state = state.lock().unwrap();
    let blockchain = app_state.blockchain_state.blockchain.lock().unwrap();
    
    let block = if let Ok(number) = block_id.parse::<u64>() {
        blockchain.get_block_by_number(number).cloned()
    } else {
        blockchain.get_block_by_hash(&block_id).cloned()
    };
    
    match block {
        Some(block) => (StatusCode::OK, Json(ApiResponse::success(block))),
        None => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<crate::blockchain::Block>::error(format!(
                "Block {} not found",
                block_id
            ))),
        ),
    }
}

async fn get_transaction(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(tx_id): Path<String>,
) -> (StatusCode, Json<ApiResponse<Transaction>>) {
    let app_state = state.lock().unwrap();
    let blockchain = app_state.blockchain_state.blockchain.lock().unwrap();
    
    match blockchain.get_transaction(&tx_id) {
        Some(tx) => (StatusCode::OK, Json(ApiResponse::success(tx))),
        None => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<Transaction>::error(format!(
                "Transaction {} not found",
                tx_id
            ))),
        ),
    }
}

async fn get_account(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(address): Path<String>,
) -> (StatusCode, Json<ApiResponse<crate::blockchain::Account>>) {
    let app_state = state.lock().unwrap();
    let blockchain = app_state.blockchain_state.blockchain.lock().unwrap();
    
    match blockchain.accounts.get(&address) {
        Some(account) => (StatusCode::OK, Json(ApiResponse::success(account.clone()))),
        None => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<crate::blockchain::Account>::error(format!(
                "Account {} not found",
                address
            ))),
        ),
    }
}

async fn create_transaction(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(request): Json<CreateTransactionRequest>,
) -> (StatusCode, Json<ApiResponse<String>>) {
    let app_state = state.lock().unwrap();
    let mut blockchain = app_state.blockchain_state.blockchain.lock().unwrap();
    
    let result = blockchain.add_transaction(
        request.from.clone(),
        request.to.clone(),
        request.amount,
        request.data.clone(),
        request.gas_price,
        request.gas_limit,
    );
    
    match result {
        Ok(tx_id) => {
            // 自動マイニング（開発用）
            if !blockchain.pending_transactions.is_empty() {
                blockchain.mine_pending_transactions(request.from.clone());
            }
            
            (StatusCode::CREATED, Json(ApiResponse::success(tx_id)))
        }
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<String>::error(err)),
        ),
    }
}

// ブロックリスト取得のクエリパラメータ
#[derive(Debug, Deserialize)]
pub struct ListBlocksQuery {
    pub start: Option<u64>,
    pub limit: Option<u64>,
}

async fn list_blocks(
    State(state): State<Arc<Mutex<AppState>>>,
    Query(params): Query<ListBlocksQuery>,
) -> (StatusCode, Json<ApiResponse<Vec<crate::blockchain::Block>>>) {
    let app_state = state.lock().unwrap();
    let blockchain = app_state.blockchain_state.blockchain.lock().unwrap();
    
    let blocks = &blockchain.chain;
    let latest_height = blocks.len() as u64 - 1;
    let start = params.start.unwrap_or(latest_height);
    let limit = params.limit.unwrap_or(10).min(100); // 最大100ブロック
    
    let mut result = Vec::new();
    
    for i in (0..=start.min(latest_height)).rev().take(limit as usize) {
        if let Some(block) = blocks.get(i as usize) {
            result.push(block.clone());
        }
    }
    
    (StatusCode::OK, Json(ApiResponse::success(result)))
}

async fn list_transactions(
    State(state): State<Arc<Mutex<AppState>>>,
    Query(params): Query<ListBlocksQuery>,
) -> (StatusCode, Json<ApiResponse<Vec<Transaction>>>) {
    let app_state = state.lock().unwrap();
    let blockchain = app_state.blockchain_state.blockchain.lock().unwrap();
    
    // すべてのトランザクションを収集
    let mut all_transactions = Vec::new();
    
    // ペンディングトランザクション
    for tx in &blockchain.pending_transactions {
        all_transactions.push(tx.clone());
    }
    
    // 確認済みトランザクション
    for block in &blockchain.chain {
        for tx in &block.transactions {
            all_transactions.push(tx.clone());
        }
    }
    
    // タイムスタンプの降順でソート
    all_transactions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    
    let limit = params.limit.unwrap_or(10).min(100); // 最大100トランザクション
    let start = params.start.unwrap_or(0);
    
    let result = all_transactions
        .into_iter()
        .skip(start as usize)
        .take(limit as usize)
        .collect::<Vec<_>>();
    
    (StatusCode::OK, Json(ApiResponse::success(result)))
}

async fn list_accounts(
    State(state): State<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<ApiResponse<Vec<crate::blockchain::Account>>>) {
    let app_state = state.lock().unwrap();
    let blockchain = app_state.blockchain_state.blockchain.lock().unwrap();
    
    let accounts: Vec<_> = blockchain.accounts.values().cloned().collect();
    
    (StatusCode::OK, Json(ApiResponse::success(accounts)))
}

async fn create_account(
    State(state): State<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<ApiResponse<crate::blockchain::Account>>) {
    let app_state = state.lock().unwrap();
    let mut blockchain = app_state.blockchain_state.blockchain.lock().unwrap();
    
    let account = blockchain.create_account();
    
    (StatusCode::CREATED, Json(ApiResponse::success(account)))
}

async fn get_account_transactions(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(address): Path<String>,
) -> (StatusCode, Json<ApiResponse<Vec<Transaction>>>) {
    let app_state = state.lock().unwrap();
    let blockchain = app_state.blockchain_state.blockchain.lock().unwrap();
    
    let transactions = blockchain.get_account_transactions(&address);
    
    (StatusCode::OK, Json(ApiResponse::success(transactions)))
}

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::Response;
use std::time::Duration;
use tokio::time::interval;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite;

// WebSocketハンドラー
async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<Mutex<AppState>>>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<Mutex<AppState>>) {
    // ソケットを送信と受信に分割
    let (mut sender, mut receiver) = socket.split();
    
    // 接続時に初期データを送信
    let initial_stats = {
        let app_state = state.lock().unwrap();
        let blockchain = app_state.blockchain_state.blockchain.lock().unwrap();
        blockchain.get_network_stats()
    };
    
    if let Ok(json) = serde_json::to_string(&ApiResponse::success(initial_stats)) {
        if sender.send(Message::Text(json)).await.is_err() {
            return;
        }
    }
    
    // 定期的なステータス更新を行う
    let mut update_interval = interval(Duration::from_secs(5));
    
    // 別スレッドでメッセージ処理と並行して実行
    let state_clone = state.clone();
    tokio::spawn(async move {
        loop {
            // 5秒ごとに実行
            update_interval.tick().await;
            
            // ネットワーク状態を取得
            let stats = {
                let app_state = state_clone.lock().unwrap();
                let blockchain = app_state.blockchain_state.blockchain.lock().unwrap();
                blockchain.get_network_stats()
            };
            
            // WebSocketメッセージを作成して送信
            if let Ok(json) = serde_json::to_string(&ApiResponse::success(stats)) {
                // 新しいWebSocketコネクションを作成して送信
                if let Ok(socket) = tokio::net::TcpStream::connect("localhost:57620").await {
                    if let Ok(ws_stream) = tokio_tungstenite::client_async("ws://localhost:57620/ws", socket).await {
                        let (mut ws_sender, _) = ws_stream.0.split();
                        let _ = ws_sender.send(tungstenite::Message::Text(json.into())).await;
                    }
                }
            }
            
            // トランザクション情報も送信
            let transactions = {
                let app_state = state_clone.lock().unwrap();
                let blockchain = app_state.blockchain_state.blockchain.lock().unwrap();
                
                // すべてのトランザクションを収集
                let mut all_transactions = Vec::new();
                
                // ペンディングトランザクション
                for tx in &blockchain.pending_transactions {
                    all_transactions.push(tx.clone());
                }
                
                // 確認済みトランザクション
                for block in &blockchain.chain {
                    for tx in &block.transactions {
                        all_transactions.push(tx.clone());
                    }
                }
                
                // タイムスタンプの降順でソート
                all_transactions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
                
                // 最新の10件を取得
                all_transactions.into_iter().take(10).collect::<Vec<_>>()
            };
            
            if let Ok(json) = serde_json::to_string(&ApiResponse::success(transactions)) {
                // 新しいWebSocketコネクションを作成して送信
                if let Ok(socket) = tokio::net::TcpStream::connect("localhost:57620").await {
                    if let Ok(ws_stream) = tokio_tungstenite::client_async("ws://localhost:57620/ws", socket).await {
                        let (mut ws_sender, _) = ws_stream.0.split();
                        let _ = ws_sender.send(tungstenite::Message::Text(json.into())).await;
                    }
                }
            }
        }
    });
    
    // クライアントからのメッセージを処理
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(text) = message {
            match text.as_str() {
                "get_status" => {
                    // ネットワーク状態を取得して送信
                    let stats = {
                        let app_state = state.lock().unwrap();
                        let blockchain = app_state.blockchain_state.blockchain.lock().unwrap();
                        blockchain.get_network_stats()
                    };
                    
                    if let Ok(json) = serde_json::to_string(&ApiResponse::success(stats)) {
                        if sender.send(Message::Text(json)).await.is_err() {
                            break;
                        }
                    }
                },
                "get_transactions" => {
                    // 最新のトランザクションを取得して送信
                    let result = {
                        let app_state = state.lock().unwrap();
                        let blockchain = app_state.blockchain_state.blockchain.lock().unwrap();
                        
                        // すべてのトランザクションを収集
                        let mut all_transactions = Vec::new();
                        
                        // ペンディングトランザクション
                        for tx in &blockchain.pending_transactions {
                            all_transactions.push(tx.clone());
                        }
                        
                        // 確認済みトランザクション
                        for block in &blockchain.chain {
                            for tx in &block.transactions {
                                all_transactions.push(tx.clone());
                            }
                        }
                        
                        // タイムスタンプの降順でソート
                        all_transactions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
                        
                        // 最新の10件を取得
                        all_transactions.into_iter().take(10).collect::<Vec<_>>()
                    };
                    
                    if let Ok(json) = serde_json::to_string(&ApiResponse::success(result)) {
                        if sender.send(Message::Text(json)).await.is_err() {
                            break;
                        }
                    }
                },
                _ => {
                    // 不明なコマンド
                    if sender.send(Message::Text(format!("{{\"success\":false,\"error\":\"Unknown command: {}\"}}", text))).await.is_err() {
                        break;
                    }
                }
            }
        }
    }
}

pub async fn start_integrated_server(port: u16) -> anyhow::Result<()> {
    // ブロックチェーンの初期化
    let blockchain_state = BlockchainState::new();
    
    // アプリケーション状態の作成
    let app_state = Arc::new(Mutex::new(AppState {
        blockchain_state,
    }));
    
    // 静的ファイルのディレクトリ
    let static_dir = PathBuf::from("./frontend");
    
    // 静的ファイルを提供するサービス
    let static_service = get_service(ServeDir::new(&static_dir))
        .handle_error(|error| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            )
        });
    
    // CORSの設定
    let cors = CorsLayer::permissive();
    
    // ルーターの構築
    let app = Router::new()
        // API エンドポイント
        .route("/blocks", get(list_blocks))
        .route("/blocks/:id", get(get_block))
        .route("/transactions", get(list_transactions))
        .route("/transactions", post(create_transaction))
        .route("/transactions/:id", get(get_transaction))
        .route("/accounts", get(list_accounts))
        .route("/accounts", post(create_account))
        .route("/accounts/:address", get(get_account))
        .route("/accounts/:address/transactions", get(get_account_transactions))
        .route("/network/status", get(get_status))
        // WebSocket エンドポイント
        .route("/ws", get(ws_handler))
        // 静的ファイル
        .nest_service("/", static_service.clone())
        .fallback_service(static_service)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);
    
    // サーバーの起動
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Rustorium integrated server listening on {}", addr);
    println!("Rustorium integrated server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}