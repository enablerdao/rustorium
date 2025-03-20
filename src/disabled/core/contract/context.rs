use std::sync::Arc;
use crate::core::{
    token::TokenManager,
    types::Address,
};

/// コントラクトのコンテキスト
pub struct ContractContext {
    /// トークンマネージャー
    pub token_manager: Arc<TokenManager>,
    /// 呼び出し元アドレス
    pub caller: Address,
    /// コントラクトアドレス
    pub contract: Address,
    /// ブロック番号
    pub block_number: u64,
    /// タイムスタンプ
    pub timestamp: u64,
    /// ガス制限
    pub gas_limit: u64,
    /// ガス価格
    pub gas_price: u64,
}

impl ContractContext {
    /// 新しいコンテキストを作成
    pub fn new(
        token_manager: Arc<TokenManager>,
        caller: Address,
        contract: Address,
        block_number: u64,
        timestamp: u64,
        gas_limit: u64,
        gas_price: u64,
    ) -> Self {
        Self {
            token_manager,
            caller,
            contract,
            block_number,
            timestamp,
            gas_limit,
            gas_price,
        }
    }

    /// トークンを転送
    pub async fn transfer_token(
        &self,
        token_id: &str,
        from: &Address,
        to: &Address,
        amount: u64,
    ) -> anyhow::Result<()> {
        self.token_manager.transfer(token_id, from, to, amount).await
    }

    /// トークンの残高を取得
    pub async fn get_token_balance(
        &self,
        token_id: &str,
        address: &Address,
    ) -> anyhow::Result<u64> {
        self.token_manager.balance_of(token_id, address).await
    }

    /// ガスを消費
    pub fn consume_gas(&mut self, amount: u64) -> bool {
        if self.gas_limit >= amount {
            self.gas_limit -= amount;
            true
        } else {
            false
        }
    }

    /// ガスコストを計算
    pub fn calculate_gas_cost(&self, amount: u64) -> u64 {
        amount * self.gas_price
    }

    /// 呼び出し元かどうかを確認
    pub fn is_caller(&self, address: &Address) -> bool {
        self.caller == *address
    }

    /// コントラクトアドレスかどうかを確認
    pub fn is_contract(&self, address: &Address) -> bool {
        self.contract == *address
    }

    /// 現在のタイムスタンプを取得
    pub fn now(&self) -> u64 {
        self.timestamp
    }

    /// 現在のブロック番号を取得
    pub fn block_number(&self) -> u64 {
        self.block_number
    }

    /// 残りのガス制限を取得
    pub fn gas_remaining(&self) -> u64 {
        self.gas_limit
    }
}