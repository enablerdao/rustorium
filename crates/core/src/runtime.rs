//! GQT Core - ランタイムモジュールインターフェース

use crate::{Module, ModuleConfig};
use async_trait::async_trait;

/// ランタイムモジュールのインターフェース
#[async_trait]
pub trait RuntimeModule: Module {
    /// コントラクトのデプロイ
    async fn deploy(&mut self, code: Vec<u8>) -> anyhow::Result<Vec<u8>>;
    /// コントラクトの実行
    async fn execute(&mut self, contract: Vec<u8>, input: Vec<u8>) -> anyhow::Result<Vec<u8>>;
    /// コントラクトの状態取得
    async fn get_state(&self, contract: Vec<u8>) -> anyhow::Result<Vec<u8>>;
    /// コントラクトの削除
    async fn delete(&mut self, contract: Vec<u8>) -> anyhow::Result<()>;
}

/// ランタイムモジュールのファクトリ
pub trait RuntimeModuleFactory {
    /// ランタイムモジュールの作成
    fn create(config: ModuleConfig) -> Box<dyn RuntimeModule>;
}
