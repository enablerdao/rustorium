use serde::{Deserialize, Serialize};
use crate::{
    core::{
        token::TokenManager,
        types::Address,
    },
    api::error::ApiError,
};
use super::super::session::Session;

/// ウォレットコマンド
#[derive(Debug, Deserialize)]
#[serde(tag = "command", content = "params")]
pub enum WalletCommand {
    /// ウォレットを作成
    Create {
        name: String,
    },
    /// ウォレット情報を取得
    GetInfo {
        address: String,
    },
    /// トークンを送金
    Transfer {
        from: String,
        to: String,
        amount: u64,
    },
    /// 残高の変更を購読
    SubscribeBalance {
        address: String,
    },
}

/// ウォレットイベント
#[derive(Debug, Serialize)]
#[serde(tag = "event", content = "data")]
pub enum WalletEvent {
    /// ウォレット作成完了
    Created {
        address: String,
        initial_balance: u64,
    },
    /// ウォレット情報
    Info {
        address: String,
        balance: u64,
        token_balances: Vec<TokenBalance>,
    },
    /// 送金完了
    Transferred {
        transaction_id: String,
        new_balance: u64,
    },
    /// 残高更新
    BalanceUpdated {
        address: String,
        balance: u64,
        token_balances: Vec<TokenBalance>,
    },
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

impl Session {
    /// ウォレットコマンドを処理
    pub async fn handle_wallet_command(
        &mut self,
        command: WalletCommand,
        token_manager: &TokenManager,
    ) -> Result<WalletEvent, ApiError> {
        match command {
            WalletCommand::Create { name: _ } => {
                // 新しいアドレスを生成
                let address = Address::generate();

                // 初期トークンを付与（100トークン）
                let initial_balance = 100;
                token_manager.mint(&address, initial_balance).await?;

                // 自動的にBのアドレスに50トークンを送信
                let b_address = Address::from_hex("0xB000000000000000000000000000000000000000")?;
                token_manager.transfer(&address, &b_address, 50).await?;

                Ok(WalletEvent::Created {
                    address: address.to_hex(),
                    initial_balance,
                })
            }

            WalletCommand::GetInfo { address } => {
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

                Ok(WalletEvent::Info {
                    address: address.to_hex(),
                    balance,
                    token_balances,
                })
            }

            WalletCommand::Transfer { from, to, amount } => {
                // アドレスをパース
                let from = Address::from_hex(&from)?;
                let to = Address::from_hex(&to)?;

                // 送金を実行
                let tx = token_manager.transfer(&from, &to, amount).await?;

                // 新しい残高を取得
                let new_balance = token_manager.balance_of(&from).await?;

                Ok(WalletEvent::Transferred {
                    transaction_id: tx.id.to_string(),
                    new_balance,
                })
            }

            WalletCommand::SubscribeBalance { address } => {
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

                // 購読を開始
                self.subscribe_balance(address.clone()).await?;

                Ok(WalletEvent::BalanceUpdated {
                    address: address.to_hex(),
                    balance,
                    token_balances,
                })
            }
        }
    }
}