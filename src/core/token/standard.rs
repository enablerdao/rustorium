use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::RwLock;
use super::{TokenId, TokenConfig, TokenType, TokenomicsParams, ContractModule};
use crate::core::types::Address;

/// トークンの標準インターフェース
#[async_trait]
pub trait Token: Send + Sync {
    /// トークンIDを取得
    fn id(&self) -> TokenId;

    /// トークン名を取得
    fn name(&self) -> &str;

    /// シンボルを取得
    fn symbol(&self) -> &str;

    /// 小数点以下の桁数を取得
    fn decimals(&self) -> u8;

    /// 総供給量を取得
    fn total_supply(&self) -> u64;

    /// トークンの種類を取得
    fn token_type(&self) -> TokenType;

    /// 残高を取得
    async fn balance_of(&self, address: &Address) -> Result<u64>;

    /// トークンを転送
    async fn transfer(&self, from: &Address, to: &Address, amount: u64) -> Result<()>;

    /// 設定を更新
    async fn update_config(&self, config: TokenConfig) -> Result<()>;

    /// 経済パラメータを更新
    async fn update_economics(&self, params: TokenomicsParams) -> Result<()>;

    /// モジュールを追加
    async fn add_module(&self, module: ContractModule) -> Result<()>;

    /// モジュールを削除
    async fn remove_module(&self, module_name: &str) -> Result<()>;
}

/// 標準トークンの実装
pub struct StandardToken {
    /// トークンID
    id: TokenId,
    /// トークン名
    name: String,
    /// シンボル
    symbol: String,
    /// 小数点以下の桁数
    decimals: u8,
    /// 総供給量
    total_supply: u64,
    /// トークンの種類
    token_type: TokenType,
    /// 残高
    balances: Arc<RwLock<HashMap<Address, u64>>>,
    /// 承認
    allowances: Arc<RwLock<HashMap<(Address, Address), u64>>>,
    /// モジュール
    modules: Arc<RwLock<HashMap<String, ContractModule>>>,
}

impl StandardToken {
    /// 新しい標準トークンを作成
    pub fn new(config: TokenConfig) -> Self {
        Self {
            id: TokenId::new(vec![0]), // TODO: 適切なIDを生成
            name: config.name,
            symbol: config.symbol,
            decimals: config.decimals,
            total_supply: config.total_supply,
            token_type: config.token_type,
            balances: Arc::new(RwLock::new(HashMap::new())),
            allowances: Arc::new(RwLock::new(HashMap::new())),
            modules: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 承認を設定
    pub async fn approve(&self, owner: &Address, spender: &Address, amount: u64) -> Result<()> {
        let mut allowances = self.allowances.write().await;
        allowances.insert((owner.clone(), spender.clone()), amount);
        Ok(())
    }

    /// 承認された金額を取得
    pub async fn allowance(&self, owner: &Address, spender: &Address) -> Result<u64> {
        let allowances = self.allowances.read().await;
        Ok(*allowances.get(&(owner.clone(), spender.clone())).unwrap_or(&0))
    }

    /// 承認された金額から転送
    pub async fn transfer_from(
        &self,
        owner: &Address,
        spender: &Address,
        to: &Address,
        amount: u64,
    ) -> Result<()> {
        // 1. 承認を確認
        let allowance = self.allowance(owner, spender).await?;
        if allowance < amount {
            anyhow::bail!("Insufficient allowance");
        }

        // 2. 転送を実行
        self.transfer(owner, to, amount).await?;

        // 3. 承認を更新
        let mut allowances = self.allowances.write().await;
        allowances.insert((owner.clone(), spender.clone()), allowance - amount);

        Ok(())
    }
}

#[async_trait]
impl Token for StandardToken {
    fn id(&self) -> TokenId {
        self.id.clone()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn symbol(&self) -> &str {
        &self.symbol
    }

    fn decimals(&self) -> u8 {
        self.decimals
    }

    fn total_supply(&self) -> u64 {
        self.total_supply
    }

    fn token_type(&self) -> TokenType {
        self.token_type.clone()
    }

    async fn balance_of(&self, address: &Address) -> Result<u64> {
        let balances = self.balances.read().await;
        Ok(*balances.get(address).unwrap_or(&0))
    }

    async fn transfer(&self, from: &Address, to: &Address, amount: u64) -> Result<()> {
        // 1. 残高を確認
        let mut balances = self.balances.write().await;
        let from_balance = *balances.get(from).unwrap_or(&0);
        if from_balance < amount {
            anyhow::bail!("Insufficient balance");
        }

        // 2. 残高を更新
        balances.insert(from.clone(), from_balance - amount);
        let to_balance = *balances.get(to).unwrap_or(&0);
        balances.insert(to.clone(), to_balance + amount);

        Ok(())
    }

    async fn update_config(&self, config: TokenConfig) -> Result<()> {
        // TODO: 設定の更新ロジックを実装
        Ok(())
    }

    async fn update_economics(&self, _params: TokenomicsParams) -> Result<()> {
        // TODO: 経済パラメータの更新ロジックを実装
        Ok(())
    }

    async fn add_module(&self, module: ContractModule) -> Result<()> {
        let mut modules = self.modules.write().await;
        modules.insert(module.name().to_string(), module);
        Ok(())
    }

    async fn remove_module(&self, module_name: &str) -> Result<()> {
        let mut modules = self.modules.write().await;
        modules.remove(module_name);
        Ok(())
    }
}