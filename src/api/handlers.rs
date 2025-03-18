use crate::api::models::{
    AccountResponse, ApiResponse, BlockResponse, CreateTransactionRequest, NodeStatusResponse,
    TransactionResponse,
};
use crate::common::errors::LedgerError;
use crate::common::types::{Address, Transaction, TransactionId, VmType};
use crate::common::utils;
use crate::storage::state::StateManager;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use tracing::{debug, error, info};

/// Application state
pub struct AppState {
    pub state_manager: Arc<StateManager>,
    pub start_time: std::time::Instant,
}

/// Get node status
pub async fn get_status(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let latest_block_height = state.state_manager.get_latest_block_height().await;
    let uptime_seconds = state.start_time.elapsed().as_secs();
    
    let status = NodeStatusResponse {
        node_id: "rustorium-node-1".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        network: "mainnet".to_string(),
        latest_block_height,
        connected_peers: 0, // TODO: Implement peer tracking
        uptime_seconds,
        pending_transactions: 0, // TODO: Implement pending tx tracking
    };
    
    (StatusCode::OK, Json(ApiResponse::success(status)))
}

/// Get block by height
pub async fn get_block(
    State(state): State<Arc<AppState>>,
    Path(height): Path<u64>,
) -> impl IntoResponse {
    match state.state_manager.get_block(height).await {
        Ok(Some(block)) => {
            let tx_ids = block
                .transactions
                .iter()
                .map(|tx| tx.id.to_string())
                .collect();
            
            let response = BlockResponse {
                height: block.header.height,
                hash: utils::bytes_to_hex(&utils::calculate_merkle_root(&[block.header.merkle_root])),
                prev_hash: utils::bytes_to_hex(&block.header.prev_hash),
                timestamp: block.header.timestamp,
                validator: block.header.validator.to_string(),
                transactions: tx_ids,
                merkle_root: utils::bytes_to_hex(&block.header.merkle_root),
            };
            
            (StatusCode::OK, Json(ApiResponse::success(response)))
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<BlockResponse>::error(format!(
                "Block at height {} not found",
                height
            ))),
        ),
        Err(e) => {
            error!("Error getting block: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<BlockResponse>::error(format!("Internal error: {}", e))),
            )
        }
    }
}

/// Get transaction by ID
pub async fn get_transaction(
    State(state): State<Arc<AppState>>,
    Path(tx_id_hex): Path<String>,
) -> impl IntoResponse {
    // Parse transaction ID
    let tx_id_bytes = match utils::hex_to_bytes(&tx_id_hex) {
        Ok(bytes) => {
            if bytes.len() != 32 {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<TransactionResponse>::error(
                        "Invalid transaction ID format".to_string(),
                    )),
                );
            }
            let mut array = [0u8; 32];
            array.copy_from_slice(&bytes);
            array
        }
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<TransactionResponse>::error(
                    "Invalid transaction ID format".to_string(),
                )),
            );
        }
    };
    
    let tx_id = TransactionId::new(tx_id_bytes);
    
    match state.state_manager.get_transaction(&tx_id).await {
        Ok(Some(tx)) => {
            let response = TransactionResponse::from(tx);
            (StatusCode::OK, Json(ApiResponse::success(response)))
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<TransactionResponse>::error(format!(
                "Transaction {} not found",
                tx_id
            ))),
        ),
        Err(e) => {
            error!("Error getting transaction: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<TransactionResponse>::error(format!("Internal error: {}", e))),
            )
        }
    }
}

/// Get account by address
pub async fn get_account(
    State(state): State<Arc<AppState>>,
    Path(address_hex): Path<String>,
) -> impl IntoResponse {
    // Parse address
    let address_bytes = match utils::hex_to_bytes(&address_hex) {
        Ok(bytes) => {
            if bytes.len() != 20 {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<AccountResponse>::error(
                        "Invalid address format".to_string(),
                    )),
                );
            }
            let mut array = [0u8; 20];
            array.copy_from_slice(&bytes);
            array
        }
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<AccountResponse>::error(
                    "Invalid address format".to_string(),
                )),
            );
        }
    };
    
    let address = Address(address_bytes);
    
    match state.state_manager.get_account(&address).await {
        Ok(Some(account)) => {
            let response = AccountResponse {
                address: account.address.to_string(),
                balance: account.balance,
                nonce: account.nonce,
                is_contract: !account.code.is_empty(),
            };
            
            (StatusCode::OK, Json(ApiResponse::success(response)))
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<AccountResponse>::error(format!(
                "Account {} not found",
                address
            ))),
        ),
        Err(e) => {
            error!("Error getting account: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<AccountResponse>::error(format!("Internal error: {}", e))),
            )
        }
    }
}

/// Create a new transaction
pub async fn create_transaction(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateTransactionRequest>,
) -> impl IntoResponse {
    // Parse sender address
    let sender_bytes = match utils::hex_to_bytes(&request.sender) {
        Ok(bytes) => {
            if bytes.len() != 20 {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<TransactionResponse>::error(
                        "Invalid sender address format".to_string(),
                    )),
                );
            }
            let mut array = [0u8; 20];
            array.copy_from_slice(&bytes);
            array
        }
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<TransactionResponse>::error(
                    "Invalid sender address format".to_string(),
                )),
            );
        }
    };
    
    // Parse recipient address
    let recipient_bytes = match utils::hex_to_bytes(&request.recipient) {
        Ok(bytes) => {
            if bytes.len() != 20 {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<TransactionResponse>::error(
                        "Invalid recipient address format".to_string(),
                    )),
                );
            }
            let mut array = [0u8; 20];
            array.copy_from_slice(&bytes);
            array
        }
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<TransactionResponse>::error(
                    "Invalid recipient address format".to_string(),
                )),
            );
        }
    };
    
    // Parse data
    let data = match &request.data {
        Some(data_hex) => match utils::hex_to_bytes(data_hex) {
            Ok(bytes) => bytes,
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<TransactionResponse>::error(
                        "Invalid data format".to_string(),
                    )),
                );
            }
        },
        None => vec![],
    };
    
    // Parse VM type
    let vm_type = match &request.vm_type {
        Some(vm_type_str) => match vm_type_str.to_lowercase().as_str() {
            "evm" => VmType::Evm,
            "move" => VmType::MoveVm,
            "solana" => VmType::SolanaVm,
            "wasm" => VmType::Wasm,
            _ => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<TransactionResponse>::error(
                        "Invalid VM type".to_string(),
                    )),
                );
            }
        },
        None => VmType::Evm, // Default to EVM
    };
    
    // Get sender account to check nonce
    let sender = Address(sender_bytes);
    let nonce = match request.nonce {
        Some(nonce) => nonce,
        None => match state.state_manager.get_account(&sender).await {
            Ok(Some(account)) => account.nonce,
            Ok(None) => 0, // New account
            Err(e) => {
                error!("Error getting sender account: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::<TransactionResponse>::error(format!(
                        "Internal error: {}",
                        e
                    ))),
                );
            }
        },
    };
    
    // Create transaction
    let tx = Transaction::new(
        sender,
        Address(recipient_bytes),
        request.amount,
        request.fee,
        nonce,
        data,
        vm_type,
    );
    
    // TODO: In a real implementation, we would add the transaction to a pending pool
    // and broadcast it to the network. For now, we'll just return the created transaction.
    
    let response = TransactionResponse::from(tx);
    (StatusCode::CREATED, Json(ApiResponse::success(response)))
}

/// Query parameters for listing blocks
#[derive(Debug, Deserialize)]
pub struct ListBlocksQuery {
    pub start: Option<u64>,
    pub limit: Option<u64>,
}

/// List blocks
pub async fn list_blocks(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ListBlocksQuery>,
) -> impl IntoResponse {
    let latest_height = state.state_manager.get_latest_block_height().await;
    let start = params.start.unwrap_or(latest_height);
    let limit = params.limit.unwrap_or(10).min(100); // Max 100 blocks
    
    let mut blocks = Vec::new();
    
    for height in (0..=start).rev().take(limit as usize) {
        match state.state_manager.get_block(height).await {
            Ok(Some(block)) => {
                let tx_ids = block
                    .transactions
                    .iter()
                    .map(|tx| tx.id.to_string())
                    .collect();
                
                let response = BlockResponse {
                    height: block.header.height,
                    hash: utils::bytes_to_hex(&utils::calculate_merkle_root(&[block.header.merkle_root])),
                    prev_hash: utils::bytes_to_hex(&block.header.prev_hash),
                    timestamp: block.header.timestamp,
                    validator: block.header.validator.to_string(),
                    transactions: tx_ids,
                    merkle_root: utils::bytes_to_hex(&block.header.merkle_root),
                };
                
                blocks.push(response);
            }
            Ok(None) => {
                // Skip non-existent blocks
                continue;
            }
            Err(e) => {
                error!("Error getting block at height {}: {}", height, e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::<Vec<BlockResponse>>::error(format!(
                        "Internal error: {}",
                        e
                    ))),
                );
            }
        }
    }
    
    (StatusCode::OK, Json(ApiResponse::success(blocks)))
}