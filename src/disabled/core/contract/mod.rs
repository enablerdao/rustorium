mod engine;
mod context;
mod state;
mod error;

pub use engine::ContractEngine;
pub use context::ContractContext;
pub use state::ContractState;
pub use error::ContractError;

use std::sync::Arc;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::core::types::Address;

/// スマートコントラクトのインターフェース
#[async_trait]
pub trait Contract: Send + Sync {
    /// コントラクトを初期化
    async fn initialize(&mut self, params: &[u8]) -> Result<()>;

    /// 関数を実行
    async fn execute(&mut self, function: &str, params: &[u8]) -> Result<Vec<u8>>;

    /// 状態を参照
    async fn view(&self, function: &str, params: &[u8]) -> Result<Vec<u8>>;

    /// 状態を取得
    fn state(&self) -> &ContractState;

    /// 状態を更新
    fn set_state(&mut self, state: ContractState);
}

/// コントラクトの種類
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContractType {
    /// マルチシグウォレット
    Multisig,
    /// ステーキング
    Staking,
    /// トークンスワップ
    Swap,
    /// カスタムコントラクト
    Custom(String),
}

/// コントラクトの設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractConfig {
    /// コントラクトの種類
    pub contract_type: ContractType,
    /// 所有者
    pub owner: Address,
    /// 初期パラメータ
    pub params: Vec<u8>,
}

/// コントラクトの情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractInfo {
    /// コントラクトアドレス
    pub address: Address,
    /// コントラクトの種類
    pub contract_type: ContractType,
    /// 所有者
    pub owner: Address,
    /// 作成日時
    pub created_at: u64,
    /// 最終更新日時
    pub updated_at: u64,
}

/// コントラクトのイベント
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractEvent {
    /// コントラクトアドレス
    pub contract_address: Address,
    /// イベント名
    pub name: String,
    /// パラメータ
    pub params: Vec<u8>,
    /// タイムスタンプ
    pub timestamp: u64,
}

/// コントラクトの実行結果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// 戻り値
    pub return_value: Vec<u8>,
    /// イベント
    pub events: Vec<ContractEvent>,
    /// ガス使用量
    pub gas_used: u64,
}

/// コントラクトの呼び出し情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallInfo {
    /// 呼び出し元
    pub caller: Address,
    /// 呼び出し先
    pub contract: Address,
    /// 関数名
    pub function: String,
    /// パラメータ
    pub params: Vec<u8>,
    /// 送金額
    pub value: u64,
    /// ガスリミット
    pub gas_limit: u64,
}

/// コントラクトの実行コンテキスト
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// 呼び出し情報
    pub call_info: CallInfo,
    /// ブロック情報
    pub block_info: BlockInfo,
    /// トランザクション情報
    pub tx_info: TxInfo,
}

/// ブロック情報
#[derive(Debug, Clone)]
pub struct BlockInfo {
    /// ブロック番号
    pub number: u64,
    /// タイムスタンプ
    pub timestamp: u64,
    /// ガス制限
    pub gas_limit: u64,
}

/// トランザクション情報
#[derive(Debug, Clone)]
pub struct TxInfo {
    /// トランザクションID
    pub id: Vec<u8>,
    /// 送信者
    pub sender: Address,
    /// ガス価格
    pub gas_price: u64,
}