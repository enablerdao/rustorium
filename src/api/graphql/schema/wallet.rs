use async_graphql::*;
use crate::{
    core::{
        token::TokenManager,
        types::Address,
    },
    api::error::ApiError,
};

/// ウォレット
#[derive(SimpleObject)]
pub struct Wallet {
    /// アドレス
    pub address: String,
    /// 残高
    pub balance: u64,
    /// トークン残高
    pub token_balances: Vec<TokenBalance>,
}

/// トークン残高
#[derive(SimpleObject)]
pub struct TokenBalance {
    /// トークンID
    pub token_id: String,
    /// シンボル
    pub symbol: String,
    /// 残高
    pub balance: u64,
}

/// ウォレットクエリ
#[derive(Default)]
pub struct WalletQuery;

#[Object]
impl WalletQuery {
    /// ウォレット情報を取得
    async fn wallet(&self, ctx: &Context<'_>, address: String) -> Result<Wallet, Error> {
        let token_manager = ctx.data::<TokenManager>()?;

        // アドレスをパース
        let address = Address::from_hex(&address)
            .map_err(|e| Error::new(format!("Invalid address: {}", e)))?;

        // 残高を取得
        let balance = token_manager.balance_of(&address).await
            .map_err(|e| Error::new(format!("Failed to get balance: {}", e)))?;

        // トークン残高を取得
        let tokens = token_manager.list_tokens().await
            .map_err(|e| Error::new(format!("Failed to list tokens: {}", e)))?;
        let mut token_balances = Vec::new();
        for token_id in tokens {
            let token = token_manager.get_token(&token_id).await
                .map_err(|e| Error::new(format!("Failed to get token: {}", e)))?
                .ok_or_else(|| Error::new("Token not found"))?;
            let balance = token.balance_of(&address).await
                .map_err(|e| Error::new(format!("Failed to get token balance: {}", e)))?;
            if balance > 0 {
                token_balances.push(TokenBalance {
                    token_id: token_id.to_string(),
                    symbol: token.symbol().to_string(),
                    balance,
                });
            }
        }

        Ok(Wallet {
            address: address.to_hex(),
            balance,
            token_balances,
        })
    }
}

/// ウォレットミューテーション
#[derive(Default)]
pub struct WalletMutation;

#[Object]
impl WalletMutation {
    /// ウォレットを作成
    async fn create_wallet(&self, ctx: &Context<'_>, name: String) -> Result<Wallet, Error> {
        let token_manager = ctx.data::<TokenManager>()?;

        // 新しいアドレスを生成
        let address = Address::generate();

        // 初期トークンを付与（100トークン）
        let initial_balance = 100;
        token_manager.mint(&address, initial_balance).await
            .map_err(|e| Error::new(format!("Failed to mint tokens: {}", e)))?;

        // 自動的にBのアドレスに50トークンを送信
        let b_address = Address::from_hex("0xB000000000000000000000000000000000000000")
            .map_err(|e| Error::new(format!("Invalid B address: {}", e)))?;
        token_manager.transfer(&address, &b_address, 50).await
            .map_err(|e| Error::new(format!("Failed to transfer tokens: {}", e)))?;

        Ok(Wallet {
            address: address.to_hex(),
            balance: initial_balance,
            token_balances: vec![],
        })
    }

    /// トークンを送金
    async fn transfer(
        &self,
        ctx: &Context<'_>,
        from: String,
        to: String,
        amount: u64,
    ) -> Result<Wallet, Error> {
        let token_manager = ctx.data::<TokenManager>()?;

        // アドレスをパース
        let from = Address::from_hex(&from)
            .map_err(|e| Error::new(format!("Invalid from address: {}", e)))?;
        let to = Address::from_hex(&to)
            .map_err(|e| Error::new(format!("Invalid to address: {}", e)))?;

        // 送金を実行
        token_manager.transfer(&from, &to, amount).await
            .map_err(|e| Error::new(format!("Failed to transfer tokens: {}", e)))?;

        // 新しい残高を取得
        let balance = token_manager.balance_of(&from).await
            .map_err(|e| Error::new(format!("Failed to get balance: {}", e)))?;

        // トークン残高を取得
        let tokens = token_manager.list_tokens().await
            .map_err(|e| Error::new(format!("Failed to list tokens: {}", e)))?;
        let mut token_balances = Vec::new();
        for token_id in tokens {
            let token = token_manager.get_token(&token_id).await
                .map_err(|e| Error::new(format!("Failed to get token: {}", e)))?
                .ok_or_else(|| Error::new("Token not found"))?;
            let balance = token.balance_of(&from).await
                .map_err(|e| Error::new(format!("Failed to get token balance: {}", e)))?;
            if balance > 0 {
                token_balances.push(TokenBalance {
                    token_id: token_id.to_string(),
                    symbol: token.symbol().to_string(),
                    balance,
                });
            }
        }

        Ok(Wallet {
            address: from.to_hex(),
            balance,
            token_balances,
        })
    }
}

/// ウォレットサブスクリプション
#[derive(Default)]
pub struct WalletSubscription;

#[Subscription]
impl WalletSubscription {
    /// 残高の変更を購読
    async fn balance_updates(
        &self,
        ctx: &Context<'_>,
        address: String,
    ) -> impl Stream<Item = Result<Wallet, Error>> {
        let token_manager = ctx.data::<TokenManager>()?.clone();
        let address = Address::from_hex(&address)
            .map_err(|e| Error::new(format!("Invalid address: {}", e)))?;

        // 残高の変更を監視
        let mut balance_stream = token_manager.subscribe_balance(&address).await
            .map_err(|e| Error::new(format!("Failed to subscribe to balance updates: {}", e)))?;

        async_stream::stream! {
            while let Some(_) = balance_stream.next().await {
                // 残高を取得
                let balance = token_manager.balance_of(&address).await
                    .map_err(|e| Error::new(format!("Failed to get balance: {}", e)))?;

                // トークン残高を取得
                let tokens = token_manager.list_tokens().await
                    .map_err(|e| Error::new(format!("Failed to list tokens: {}", e)))?;
                let mut token_balances = Vec::new();
                for token_id in tokens {
                    let token = token_manager.get_token(&token_id).await
                        .map_err(|e| Error::new(format!("Failed to get token: {}", e)))?
                        .ok_or_else(|| Error::new("Token not found"))?;
                    let balance = token.balance_of(&address).await
                        .map_err(|e| Error::new(format!("Failed to get token balance: {}", e)))?;
                    if balance > 0 {
                        token_balances.push(TokenBalance {
                            token_id: token_id.to_string(),
                            symbol: token.symbol().to_string(),
                            balance,
                        });
                    }
                }

                yield Ok(Wallet {
                    address: address.to_hex(),
                    balance,
                    token_balances,
                });
            }
        }
    }
}