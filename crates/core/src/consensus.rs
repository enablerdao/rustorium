//! GQT Core - コンセンサスモジュールインターフェース

use crate::{Module, ModuleConfig};
use async_trait::async_trait;

/// コンセンサスモジュールのインターフェース
#[async_trait]
pub trait ConsensusModule: Module {
    /// ブロックの提案
    async fn propose_block(&mut self, block: Vec<u8>) -> anyhow::Result<()>;
    /// ブロックの検証
    async fn verify_block(&self, block: Vec<u8>) -> anyhow::Result<bool>;
    /// ブロックの投票
    async fn vote_block(&mut self, block: Vec<u8>, vote: bool) -> anyhow::Result<()>;
    /// ブロックのファイナライズ
    async fn finalize_block(&mut self, block: Vec<u8>) -> anyhow::Result<()>;
}

/// コンセンサスモジュールのファクトリ
pub trait ConsensusModuleFactory {
    /// コンセンサスモジュールの作成
    fn create(config: ModuleConfig) -> Box<dyn ConsensusModule>;
}
