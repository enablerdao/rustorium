use axum::{
    extract::{Json, Path},
    Extension,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::{
    core::{
        token::TokenManager,
        types::Address,
    },
    api::error::ApiError,
};

/// ウォレット作成リクエスト
#[derive(Debug, Deserialize)]
pub struct CreateWalletRequest {
    /// ウォレット名
    pub name: String,
}

/// ウォレット作成レスポンス
#[derive(Debug, Serialize)]
pub struct CreateWalletResponse {
    /// アドレス
    pub address: String,
    /// 初期残高
    pub initial_balance: u64,
}

/// ウォレット情報
#[derive(Debug, Serialize)]
pub struct WalletInfo {
    /// アドレス
    pub address: String,
    /// 残高
    pub balance: u64,
    /// トークン残高
    pub token_balances: Vec<TokenBalance>,
}

/// トークン残高
#[derive(Debug, Serialize)]
pub struct TokenBalance {
    /// トークンID
    pub token_id: String,
    /// シンボル
    pub symbol: String,
    /// 残高
    pub balance: u64,
}

/// ウォレットを作成
pub async fn create_wallet(
    Json(request): Json<CreateWalletRequest>,
    Extension(token_manager): Extension<Arc<TokenManager>>,
) -> Result<Json<CreateWalletResponse>, ApiError> {
    // 新しいアドレスを生成
    let address = Address::generate();

    // 初期トークンを付与（100トークン）
    let initial_balance = 100;
    token_manager.mint(&address, initial_balance).await?;

    // 自動的にBのアドレスに50トークンを送信
    let b_address = Address::from_hex("0xB000000000000000000000000000000000000000")?;
    token_manager.transfer(&address, &b_address, 50).await?;

    Ok(Json(CreateWalletResponse {
        address: address.to_hex(),
        initial_balance,
    }))
}

/// ウォレット情報を取得
pub async fn get_wallet(
    Path(address): Path<String>,
    Extension(token_manager): Extension<Arc<TokenManager>>,
) -> Result<Json<WalletInfo>, ApiError> {
    // アドレスをパース
    let address = Address::from_hex(&address)?;

    // 残高を取得
    let balance = token_manager.balance_of(&address).await?;

    // トークン残高を取得
    let tokens = token_manager.list_tokens().await?;
    let mut token_balances = Vec::new();
    for token_id in tokens {
        let token = token_manager.get_token(&token_id).await?
            .ok_or_else(|| ApiError::NotFound("Token not found".into()))?;
        let balance = token.balance_of(&address).await?;
        if balance > 0 {
            token_balances.push(TokenBalance {
                token_id: token_id.to_string(),
                symbol: token.symbol().to_string(),
                balance,
            });
        }
    }

    Ok(Json(WalletInfo {
        address: address.to_hex(),
        balance,
        token_balances,
    }))
}

/// 送金リクエスト
#[derive(Debug, Deserialize)]
pub struct TransferRequest {
    /// 送金先アドレス
    pub to: String,
    /// 金額
    pub amount: u64,
}

/// 送金レスポンス
#[derive(Debug, Serialize)]
pub struct TransferResponse {
    /// トランザクションID
    pub transaction_id: String,
    /// 新しい残高
    pub new_balance: u64,
}

/// トークンを送金
pub async fn transfer(
    Path(from): Path<String>,
    Json(request): Json<TransferRequest>,
    Extension(token_manager): Extension<Arc<TokenManager>>,
) -> Result<Json<TransferResponse>, ApiError> {
    // アドレスをパース
    let from = Address::from_hex(&from)?;
    let to = Address::from_hex(&request.to)?;

    // 送金を実行
    let tx = token_manager.transfer(&from, &to, request.amount).await?;

    // 新しい残高を取得
    let new_balance = token_manager.balance_of(&from).await?;

    Ok(Json(TransferResponse {
        transaction_id: tx.id.to_string(),
        new_balance,
    }))
}