mod standard;
mod contract;
mod economics;
mod generator;

use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use super::types::Address;
use super::storage::StorageEngine;

pub use standard::{Token, TokenStandard};
pub use contract::{Contract, ContractModule};
pub use economics::{TokenEconomics, TokenomicsParams};
pub use generator::{TokenGenerator, TokenTemplate};

/// トークンID
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct TokenId(Vec<u8>);

impl TokenId {
    /// 新しいトークンIDを作成
    pub fn new(id: Vec<u8>) -> Self {
        Self(id)
    }

    /// トークンIDをバイト列として取得
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// トークンの種類
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenType {
    /// 標準トークン
    Standard,
    /// スマートコントラクト
    Contract,
    /// NFT
    NFT,
    /// ステーキングトークン
    Staking,
    /// ガバナンストークン
    Governance,
}

/// トークンの設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenConfig {
    /// トークン名
    pub name: String,
    /// シンボル
    pub symbol: String,
    /// 小数点以下の桁数
    pub decimals: u8,
    /// 総供給量
    pub total_supply: u64,
    /// トークンの種類
    pub token_type: TokenType,
    /// 経済パラメータ
    pub economics: TokenomicsParams,
    /// 追加のモジュール
    pub modules: Vec<ContractModule>,
}

/// トークンマネージャー
pub struct TokenManager {
    /// トークンストア
    tokens: Arc<RwLock<HashMap<TokenId, Arc<dyn Token>>>>,
    /// ストレージエンジン
    storage: Arc<dyn StorageEngine>,
    /// トークンジェネレーター
    generator: Arc<TokenGenerator>,
}

impl TokenManager {
    /// 新しいトークンマネージャーを作成
    pub fn new(storage: Arc<dyn StorageEngine>) -> Self {
        Self {
            tokens: Arc::new(RwLock::new(HashMap::new())),
            storage: storage.clone(),
            generator: Arc::new(TokenGenerator::new(storage)),
        }
    }

    /// トークンを作成
    pub async fn create_token(&self, config: TokenConfig) -> Result<TokenId> {
        // 1. トークンを生成
        let token = self.generator.generate_token(config).await?;
        let token_id = token.id();

        // 2. トークンを保存
        let mut tokens = self.tokens.write().await;
        tokens.insert(token_id.clone(), token.clone());
        self.storage.put_token(&token_id, &token).await?;

        Ok(token_id)
    }

    /// トークンを取得
    pub async fn get_token(&self, id: &TokenId) -> Result<Option<Arc<dyn Token>>> {
        // 1. メモリから検索
        let tokens = self.tokens.read().await;
        if let Some(token) = tokens.get(id) {
            return Ok(Some(token.clone()));
        }

        // 2. ストレージから読み込み
        if let Some(token) = self.storage.get_token(id).await? {
            let mut tokens = self.tokens.write().await;
            tokens.insert(id.clone(), token.clone());
            Ok(Some(token))
        } else {
            Ok(None)
        }
    }

    /// トークンを転送
    pub async fn transfer(
        &self,
        token_id: &TokenId,
        from: &Address,
        to: &Address,
        amount: u64,
    ) -> Result<()> {
        // 1. トークンを取得
        let token = self.get_token(token_id).await?
            .ok_or_else(|| anyhow::anyhow!("Token not found"))?;

        // 2. 転送を実行
        token.transfer(from, to, amount).await?;

        // 3. 状態を保存
        self.storage.put_token(token_id, &token).await?;

        Ok(())
    }

    /// 残高を取得
    pub async fn balance_of(
        &self,
        token_id: &TokenId,
        address: &Address,
    ) -> Result<u64> {
        // 1. トークンを取得
        let token = self.get_token(token_id).await?
            .ok_or_else(|| anyhow::anyhow!("Token not found"))?;

        // 2. 残高を取得
        Ok(token.balance_of(address).await?)
    }

    /// トークンを削除
    pub async fn delete_token(&self, id: &TokenId) -> Result<()> {
        // 1. トークンを削除
        let mut tokens = self.tokens.write().await;
        tokens.remove(id);
        self.storage.delete_token(id).await?;

        Ok(())
    }

    /// トークンの一覧を取得
    pub async fn list_tokens(&self) -> Result<Vec<TokenId>> {
        let tokens = self.tokens.read().await;
        Ok(tokens.keys().cloned().collect())
    }

    /// トークンの情報を更新
    pub async fn update_token(&self, id: &TokenId, config: TokenConfig) -> Result<()> {
        // 1. トークンを取得
        let token = self.get_token(id).await?
            .ok_or_else(|| anyhow::anyhow!("Token not found"))?;

        // 2. トークンを更新
        token.update_config(config).await?;

        // 3. 状態を保存
        self.storage.put_token(id, &token).await?;

        Ok(())
    }

    /// トークンの経済パラメータを更新
    pub async fn update_economics(
        &self,
        id: &TokenId,
        params: TokenomicsParams,
    ) -> Result<()> {
        // 1. トークンを取得
        let token = self.get_token(id).await?
            .ok_or_else(|| anyhow::anyhow!("Token not found"))?;

        // 2. 経済パラメータを更新
        token.update_economics(params).await?;

        // 3. 状態を保存
        self.storage.put_token(id, &token).await?;

        Ok(())
    }

    /// モジュールを追加
    pub async fn add_module(
        &self,
        id: &TokenId,
        module: ContractModule,
    ) -> Result<()> {
        // 1. トークンを取得
        let token = self.get_token(id).await?
            .ok_or_else(|| anyhow::anyhow!("Token not found"))?;

        // 2. モジュールを追加
        token.add_module(module).await?;

        // 3. 状態を保存
        self.storage.put_token(id, &token).await?;

        Ok(())
    }

    /// モジュールを削除
    pub async fn remove_module(
        &self,
        id: &TokenId,
        module_name: &str,
    ) -> Result<()> {
        // 1. トークンを取得
        let token = self.get_token(id).await?
            .ok_or_else(|| anyhow::anyhow!("Token not found"))?;

        // 2. モジュールを削除
        token.remove_module(module_name).await?;

        // 3. 状態を保存
        self.storage.put_token(id, &token).await?;

        Ok(())
    }
}