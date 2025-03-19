use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod blockchain;
mod contracts;
use blockchain::{BlockchainState, Transaction};
use contracts::{Contract, DeployContractRequest, CallContractRequest, CallContractResult};
use uuid::Uuid;

// トランザクション作成リクエスト
#[derive(Debug, Serialize, Deserialize)]
struct CreateTransactionRequest {
    from: String,
    to: String,
    amount: f64,
    #[serde(default)]
    data: Option<String>,
    #[serde(default = "default_gas_price")]
    gas_price: u64,
    #[serde(default = "default_gas_limit")]
    gas_limit: u64,
}

fn default_gas_price() -> u64 {
    5
}

fn default_gas_limit() -> u64 {
    21000
}

// APIレスポンス
#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse<T: Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
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

// アプリケーション状態
struct AppState {
    blockchain_state: BlockchainState,
}

// ハンドラー関数
async fn get_status(State(state): State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let app_state = state.lock().unwrap();
    let blockchain = app_state.blockchain_state.blockchain.lock().unwrap();
    
    let stats = blockchain.get_network_stats();
    
    (StatusCode::OK, Json(ApiResponse::success(stats)))
}

async fn get_block(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(block_id): Path<String>,
) -> impl IntoResponse {
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
            Json(ApiResponse::<blockchain::Block>::error(format!(
                "Block {} not found",
                block_id
            ))),
        ),
    }
}

async fn get_transaction(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(tx_id): Path<String>,
) -> impl IntoResponse {
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
) -> impl IntoResponse {
    let app_state = state.lock().unwrap();
    let blockchain = app_state.blockchain_state.blockchain.lock().unwrap();
    
    match blockchain.accounts.get(&address) {
        Some(account) => (StatusCode::OK, Json(ApiResponse::success(account.clone()))),
        None => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<blockchain::Account>::error(format!(
                "Account {} not found",
                address
            ))),
        ),
    }
}

async fn create_transaction(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(request): Json<CreateTransactionRequest>,
) -> impl IntoResponse {
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
struct ListBlocksQuery {
    start: Option<u64>,
    limit: Option<u64>,
}

async fn list_blocks(
    State(state): State<Arc<Mutex<AppState>>>,
    Query(params): Query<ListBlocksQuery>,
) -> impl IntoResponse {
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
) -> impl IntoResponse {
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
) -> impl IntoResponse {
    let app_state = state.lock().unwrap();
    let blockchain = app_state.blockchain_state.blockchain.lock().unwrap();
    
    let accounts: Vec<_> = blockchain.accounts.values().cloned().collect();
    
    (StatusCode::OK, Json(ApiResponse::success(accounts)))
}

async fn create_account(
    State(state): State<Arc<Mutex<AppState>>>,
) -> impl IntoResponse {
    let app_state = state.lock().unwrap();
    let mut blockchain = app_state.blockchain_state.blockchain.lock().unwrap();
    
    let account = blockchain.create_account();
    
    (StatusCode::CREATED, Json(ApiResponse::success(account)))
}

async fn get_account_transactions(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(address): Path<String>,
) -> impl IntoResponse {
    let app_state = state.lock().unwrap();
    let blockchain = app_state.blockchain_state.blockchain.lock().unwrap();
    
    let transactions = blockchain.get_account_transactions(&address);
    
    (StatusCode::OK, Json(ApiResponse::success(transactions)))
}

// スマートコントラクト関連のハンドラー

// コントラクト一覧の取得
async fn list_contracts(
    State(state): State<Arc<Mutex<AppState>>>,
) -> impl IntoResponse {
    let app_state = state.lock().unwrap();
    let blockchain = app_state.blockchain_state.blockchain.lock().unwrap();
    
    let contracts = blockchain.contract_manager.get_all_contracts_cloned();
    
    (StatusCode::OK, Json(ApiResponse::success(contracts)))
}

// コントラクトのデプロイ
async fn deploy_contract(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(request): Json<DeployContractRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().unwrap();
    let mut blockchain = app_state.blockchain_state.blockchain.lock().unwrap();
    
    // 送信者アカウントの存在確認
    if !blockchain.accounts.contains_key(&request.from) {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<String>::error("Sender account not found".to_string())),
        );
    }
    
    // ガス代の計算
    let gas_cost = (request.gas_price * request.gas_limit) as f64 / 1_000_000.0;
    
    // 送信者の残高確認
    let sender_account = blockchain.accounts.get(&request.from).unwrap();
    if sender_account.balance < gas_cost {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<String>::error("Insufficient balance for gas".to_string())),
        );
    }
    
    // トランザクションの作成
    let tx_id = format!("0x{}", hex::encode(Uuid::new_v4().as_bytes())[..32].to_string());
    
    // ブロックチェーンの長さを取得
    let chain_len = blockchain.chain.len() as u64;
    
    // コントラクトのデプロイ
    let contract_address = blockchain.contract_manager.deploy_contract(
        request.from.clone(),
        request.bytecode.clone(),
        request.abi.clone(),
        tx_id.clone(),
        Some(chain_len),
    );
    
    // コントラクトアカウントの作成
    let contract_account = blockchain::Account::new_contract(
        contract_address.clone(),
        request.bytecode.clone(),
        request.abi.clone(),
    );
    
    // アカウントの追加
    blockchain.accounts.insert(contract_address.clone(), contract_account);
    
    // 送信者の残高を更新
    let sender_account = blockchain.accounts.get_mut(&request.from).unwrap();
    sender_account.balance -= gas_cost;
    sender_account.nonce += 1;
    
    // レスポンスの作成
    let response = format!(
        r#"{{
            "address": "{}",
            "transaction_id": "{}",
            "gas_used": {},
            "gas_cost": {}
        }}"#,
        contract_address, tx_id, request.gas_limit, gas_cost
    );
    
    (StatusCode::CREATED, Json(ApiResponse::success(response)))
}

// コントラクト情報の取得
async fn get_contract(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(address): Path<String>,
) -> impl IntoResponse {
    let app_state = state.lock().unwrap();
    let blockchain = app_state.blockchain_state.blockchain.lock().unwrap();
    
    // コントラクトの存在確認
    match blockchain.contract_manager.get_contract(&address) {
        Some(contract) => {
            let contract_clone = contract.clone();
            (StatusCode::OK, Json(ApiResponse::success(contract_clone)))
        },
        None => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<Contract>::error(format!(
                "Contract not found: {}",
                address
            ))),
        ),
    }
}

// コントラクトの呼び出し
async fn call_contract(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(address): Path<String>,
    Json(request): Json<CallContractRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().unwrap();
    let mut blockchain = app_state.blockchain_state.blockchain.lock().unwrap();
    
    // 送信者アカウントの存在確認
    if !blockchain.accounts.contains_key(&request.from) {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<CallContractResult>::error("Sender account not found".to_string())),
        );
    }
    
    // コントラクトの存在確認
    if blockchain.contract_manager.get_contract(&address).is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<CallContractResult>::error(format!(
                "Contract not found: {}",
                address
            ))),
        );
    }
    
    // ガス代の計算
    let gas_cost = (request.gas_price * request.gas_limit) as f64 / 1_000_000.0;
    
    // 送信者の残高確認
    let sender_account = blockchain.accounts.get(&request.from).unwrap();
    if sender_account.balance < gas_cost + request.value {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<CallContractResult>::error("Insufficient balance".to_string())),
        );
    }
    
    // トランザクションの作成
    let tx_id = format!("0x{}", hex::encode(Uuid::new_v4().as_bytes())[..32].to_string());
    
    // コントラクトの呼び出し
    let result = match blockchain.contract_manager.call_contract(
        &address,
        &request.method,
        request.args.as_deref(),
        &request.from,
    ) {
        Ok(result) => result,
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<CallContractResult>::error(err)),
            );
        }
    };
    
    // 送信者の残高を更新
    let sender_account = blockchain.accounts.get_mut(&request.from).unwrap();
    sender_account.balance -= gas_cost + request.value;
    sender_account.nonce += 1;
    
    // コントラクトアカウントの残高を更新（もし値が送信された場合）
    if request.value > 0.0 {
        let contract_account = blockchain.accounts.get_mut(&address).unwrap();
        contract_account.balance += request.value;
    }
    
    // レスポンスの作成
    let response = CallContractResult {
        transaction_id: tx_id,
        result: Some(result),
        gas_used: request.gas_limit,
    };
    
    (StatusCode::OK, Json(ApiResponse::success(response)))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ロギングの設定
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
    
    // ブロックチェーンの初期化
    let blockchain_state = BlockchainState::new();
    
    // アプリケーション状態の作成
    let app_state = Arc::new(Mutex::new(AppState {
        blockchain_state,
    }));
    
    // CORSの設定
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    // ルーターの構築
    let app = Router::new()
        .route("/", get(|| async { "Rustorium API Server" }))
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
        // スマートコントラクト関連のエンドポイント
        .route("/contracts", get(list_contracts))
        .route("/contracts", post(deploy_contract))
        .route("/contracts/:address", get(get_contract))
        .route("/contracts/:address/call", post(call_contract))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);
    
    // サーバーの起動
    let port = 50128; // APIサーバーのポート
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("API server listening on {}", addr);
    println!("API server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}