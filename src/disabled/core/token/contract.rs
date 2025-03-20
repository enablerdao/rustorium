use std::collections::HashMap;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use super::TokenId;
use crate::core::types::Address;

/// コントラクトの状態
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractState {
    /// 状態変数
    pub variables: HashMap<String, Vec<u8>>,
    /// イベントログ
    pub events: Vec<ContractEvent>,
}

/// コントラクトイベント
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractEvent {
    /// イベント名
    pub name: String,
    /// パラメータ
    pub params: HashMap<String, Vec<u8>>,
    /// タイムスタンプ
    pub timestamp: u64,
}

/// コントラクトモジュール
#[derive(Debug, Clone)]
pub struct ContractModule {
    /// モジュール名
    name: String,
    /// コントラクトの実装
    contract: Box<dyn Contract>,
}

impl ContractModule {
    /// 新しいコントラクトモジュールを作成
    pub fn new(name: String, contract: Box<dyn Contract>) -> Self {
        Self { name, contract }
    }

    /// モジュール名を取得
    pub fn name(&self) -> &str {
        &self.name
    }

    /// コントラクトを取得
    pub fn contract(&self) -> &dyn Contract {
        self.contract.as_ref()
    }
}

/// コントラクトの標準インターフェース
#[async_trait]
pub trait Contract: Send + Sync {
    /// コントラクトを初期化
    async fn initialize(&mut self, params: HashMap<String, Vec<u8>>) -> Result<()>;

    /// 関数を実行
    async fn execute(&mut self, function: &str, params: HashMap<String, Vec<u8>>) -> Result<Vec<u8>>;

    /// 状態を取得
    fn state(&self) -> &ContractState;

    /// 状態を更新
    fn set_state(&mut self, state: ContractState);
}

/// ステーキングコントラクト
pub struct StakingContract {
    /// トークンID
    token_id: TokenId,
    /// 状態
    state: ContractState,
    /// ステーキング期間
    staking_period: u64,
    /// 報酬レート
    reward_rate: f64,
}

impl StakingContract {
    /// 新しいステーキングコントラクトを作成
    pub fn new(token_id: TokenId) -> Self {
        Self {
            token_id,
            state: ContractState {
                variables: HashMap::new(),
                events: Vec::new(),
            },
            staking_period: 0,
            reward_rate: 0.0,
        }
    }

    /// ステーキングを開始
    async fn stake(&mut self, address: &Address, amount: u64) -> Result<()> {
        // 1. ステーキング情報を保存
        let mut stakes = self.get_stakes()?;
        stakes.insert(address.clone(), (amount, self.current_time()?));
        self.set_stakes(stakes)?;

        // 2. イベントを記録
        self.state.events.push(ContractEvent {
            name: "Stake".to_string(),
            params: {
                let mut params = HashMap::new();
                params.insert("address".to_string(), address.as_bytes().to_vec());
                params.insert("amount".to_string(), amount.to_le_bytes().to_vec());
                params
            },
            timestamp: self.current_time()?,
        });

        Ok(())
    }

    /// ステーキングを解除
    async fn unstake(&mut self, address: &Address) -> Result<()> {
        // 1. ステーキング情報を取得
        let mut stakes = self.get_stakes()?;
        let (amount, start_time) = stakes.remove(address)
            .ok_or_else(|| anyhow::anyhow!("No stake found"))?;

        // 2. 報酬を計算
        let duration = self.current_time()? - start_time;
        let reward = if duration >= self.staking_period {
            (amount as f64 * self.reward_rate) as u64
        } else {
            0
        };

        // 3. イベントを記録
        self.state.events.push(ContractEvent {
            name: "Unstake".to_string(),
            params: {
                let mut params = HashMap::new();
                params.insert("address".to_string(), address.as_bytes().to_vec());
                params.insert("amount".to_string(), amount.to_le_bytes().to_vec());
                params.insert("reward".to_string(), reward.to_le_bytes().to_vec());
                params
            },
            timestamp: self.current_time()?,
        });

        // 4. ステーキング情報を更新
        self.set_stakes(stakes)?;

        Ok(())
    }

    /// ステーキング情報を取得
    fn get_stakes(&self) -> Result<HashMap<Address, (u64, u64)>> {
        let stakes = self.state.variables.get("stakes")
            .map(|bytes| bincode::deserialize(bytes))
            .transpose()?
            .unwrap_or_default();
        Ok(stakes)
    }

    /// ステーキング情報を設定
    fn set_stakes(&mut self, stakes: HashMap<Address, (u64, u64)>) -> Result<()> {
        let bytes = bincode::serialize(&stakes)?;
        self.state.variables.insert("stakes".to_string(), bytes);
        Ok(())
    }

    /// 現在時刻を取得
    fn current_time(&self) -> Result<u64> {
        use std::time::{SystemTime, UNIX_EPOCH};
        Ok(SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs())
    }
}

#[async_trait]
impl Contract for StakingContract {
    async fn initialize(&mut self, params: HashMap<String, Vec<u8>>) -> Result<()> {
        // パラメータを解析
        self.staking_period = u64::from_le_bytes(
            params.get("staking_period")
                .ok_or_else(|| anyhow::anyhow!("Missing staking_period"))?
                .try_into()?
        );
        self.reward_rate = f64::from_le_bytes(
            params.get("reward_rate")
                .ok_or_else(|| anyhow::anyhow!("Missing reward_rate"))?
                .try_into()?
        );
        Ok(())
    }

    async fn execute(&mut self, function: &str, params: HashMap<String, Vec<u8>>) -> Result<Vec<u8>> {
        match function {
            "stake" => {
                let address = Address::new(
                    params.get("address")
                        .ok_or_else(|| anyhow::anyhow!("Missing address"))?
                        .try_into()?
                );
                let amount = u64::from_le_bytes(
                    params.get("amount")
                        .ok_or_else(|| anyhow::anyhow!("Missing amount"))?
                        .try_into()?
                );
                self.stake(&address, amount).await?;
                Ok(Vec::new())
            }
            "unstake" => {
                let address = Address::new(
                    params.get("address")
                        .ok_or_else(|| anyhow::anyhow!("Missing address"))?
                        .try_into()?
                );
                self.unstake(&address).await?;
                Ok(Vec::new())
            }
            _ => anyhow::bail!("Unknown function: {}", function),
        }
    }

    fn state(&self) -> &ContractState {
        &self.state
    }

    fn set_state(&mut self, state: ContractState) {
        self.state = state;
    }
}