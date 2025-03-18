use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use uuid::Uuid;

// モデル定義
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Transaction {
    id: String,
    sender: String,
    recipient: String,
    amount: u64,
    fee: u64,
    nonce: u64,
    timestamp: u64,
    data: String,
    status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Block {
    height: u64,
    hash: String,
    prev_hash: String,
    timestamp: u64,
    validator: String,
    transactions: Vec<String>,
    merkle_root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Account {
    address: String,
    balance: u64,
    nonce: u64,
    is_contract: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NodeStatus {
    node_id: String,
    version: String,
    network: String,
    latest_block_height: u64,
    connected_peers: usize,
    uptime_seconds: u64,
    pending_transactions: usize,
}

// APIレスポンス
#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

impl<T> ApiResponse<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }
    
    fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

// トランザクション作成リクエスト
#[derive(Debug, Serialize, Deserialize)]
struct CreateTransactionRequest {
    sender: String,
    recipient: String,
    amount: u64,
    fee: u64,
    nonce: Option<u64>,
    data: Option<String>,
}

// アプリケーション状態
struct AppState {
    transactions: RwLock<HashMap<String, Transaction>>,
    blocks: RwLock<HashMap<u64, Block>>,
    accounts: RwLock<HashMap<String, Account>>,
    start_time: std::time::Instant,
}

// ハンドラー関数
async fn get_status(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let uptime_seconds = state.start_time.elapsed().as_secs();
    
    let status = NodeStatus {
        node_id: "rustorium-node-1".to_string(),
        version: "0.1.0".to_string(),
        network: "testnet".to_string(),
        latest_block_height: 10,
        connected_peers: 5,
        uptime_seconds,
        pending_transactions: state.transactions.read().unwrap().len(),
    };
    
    (StatusCode::OK, Json(ApiResponse::success(status)))
}

async fn get_block(
    State(state): State<Arc<AppState>>,
    Path(height): Path<u64>,
) -> impl IntoResponse {
    let blocks = state.blocks.read().unwrap();
    
    match blocks.get(&height) {
        Some(block) => (StatusCode::OK, Json(ApiResponse::success(block.clone()))),
        None => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<Block>::error(format!(
                "Block at height {} not found",
                height
            ))),
        ),
    }
}

async fn get_transaction(
    State(state): State<Arc<AppState>>,
    Path(tx_id): Path<String>,
) -> impl IntoResponse {
    let transactions = state.transactions.read().unwrap();
    
    match transactions.get(&tx_id) {
        Some(tx) => (StatusCode::OK, Json(ApiResponse::success(tx.clone()))),
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
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> impl IntoResponse {
    let accounts = state.accounts.read().unwrap();
    
    match accounts.get(&address) {
        Some(account) => (StatusCode::OK, Json(ApiResponse::success(account.clone()))),
        None => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<Account>::error(format!(
                "Account {} not found",
                address
            ))),
        ),
    }
}

async fn create_transaction(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateTransactionRequest>,
) -> impl IntoResponse {
    // 新しいトランザクションを作成
    let tx_id = Uuid::new_v4().to_string();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let nonce = request.nonce.unwrap_or(0);
    
    let transaction = Transaction {
        id: tx_id.clone(),
        sender: request.sender,
        recipient: request.recipient,
        amount: request.amount,
        fee: request.fee,
        nonce,
        timestamp,
        data: request.data.unwrap_or_default(),
        status: "Pending".to_string(),
    };
    
    // トランザクションを保存
    state.transactions.write().unwrap().insert(tx_id, transaction.clone());
    
    (StatusCode::CREATED, Json(ApiResponse::success(transaction)))
}

// ブロックリスト取得のクエリパラメータ
#[derive(Debug, Deserialize)]
struct ListBlocksQuery {
    start: Option<u64>,
    limit: Option<u64>,
}

async fn list_blocks(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ListBlocksQuery>,
) -> impl IntoResponse {
    let blocks = state.blocks.read().unwrap();
    let latest_height = 10; // 仮の最新ブロック高
    let start = params.start.unwrap_or(latest_height);
    let limit = params.limit.unwrap_or(10).min(100); // 最大100ブロック
    
    let mut result = Vec::new();
    
    for height in (0..=start).rev().take(limit as usize) {
        if let Some(block) = blocks.get(&height) {
            result.push(block.clone());
        }
    }
    
    (StatusCode::OK, Json(ApiResponse::success(result)))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ロギングの設定
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
    
    // サンプルデータの作成
    let mut transactions = HashMap::new();
    let mut blocks = HashMap::new();
    let mut accounts = HashMap::new();
    
    // サンプルアカウント
    let account1 = Account {
        address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        balance: 1000000,
        nonce: 5,
        is_contract: false,
    };
    
    let account2 = Account {
        address: "0xabcdef1234567890abcdef1234567890abcdef12".to_string(),
        balance: 500000,
        nonce: 3,
        is_contract: false,
    };
    
    accounts.insert(account1.address.clone(), account1);
    accounts.insert(account2.address.clone(), account2);
    
    // サンプルトランザクション
    let tx1 = Transaction {
        id: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
        sender: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        recipient: "0xabcdef1234567890abcdef1234567890abcdef12".to_string(),
        amount: 1000,
        fee: 10,
        nonce: 5,
        timestamp: 1677721600,
        data: "".to_string(),
        status: "Confirmed".to_string(),
    };
    
    transactions.insert(tx1.id.clone(), tx1.clone());
    
    // サンプルブロック
    for i in 0..10 {
        let block = Block {
            height: i,
            hash: format!("0x{:064x}", i),
            prev_hash: if i > 0 { format!("0x{:064x}", i - 1) } else { "0x0".to_string() },
            timestamp: 1677721600 + i * 15,
            validator: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
            transactions: if i == 5 { vec![tx1.id.clone()] } else { vec![] },
            merkle_root: format!("0x{:064x}", i * 10),
        };
        
        blocks.insert(i, block);
    }
    
    // アプリケーション状態の作成
    let app_state = Arc::new(AppState {
        transactions: RwLock::new(transactions),
        blocks: RwLock::new(blocks),
        accounts: RwLock::new(accounts),
        start_time: std::time::Instant::now(),
    });
    
    // CORSの設定
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    // ルーターの構築
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
    
    // サーバーの起動
    let port = 51055; // APIサーバーのポート
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("API server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}