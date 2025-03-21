//! ネットワークモジュールのインターフェース

use crate::{Module, ModuleConfig};
use async_trait::async_trait;
use std::net::SocketAddr;

/// ネットワークモジュールのインターフェース
#[async_trait]
pub trait NetworkModule: Module {
    /// ピアに接続
    async fn connect(&mut self, addr: SocketAddr) -> anyhow::Result<()>;
    /// ピアから切断
    async fn disconnect(&mut self, addr: SocketAddr) -> anyhow::Result<()>;
    /// メッセージの送信
    async fn send(&self, addr: SocketAddr, data: Vec<u8>) -> anyhow::Result<()>;
    /// メッセージの受信
    async fn receive(&self) -> anyhow::Result<(SocketAddr, Vec<u8>)>;
    /// ピア一覧の取得
    async fn peers(&self) -> anyhow::Result<Vec<SocketAddr>>;
}

/// ネットワークモジュールのファクトリ
pub trait NetworkModuleFactory {
    /// ネットワークモジュールの作成
    fn create(config: ModuleConfig) -> Box<dyn NetworkModule>;
}
