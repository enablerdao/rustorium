//! 共通の型定義

use serde::{Serialize, Deserialize};

/// トランザクションハッシュ
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TxHash([u8; 32]);

/// ブロックハッシュ
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockHash([u8; 32]);

/// アドレス
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address([u8; 20]);

/// 署名
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signature([u8; 64]);

/// トランザクション
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// 送信者アドレス
    pub from: Address,
    /// 受信者アドレス
    pub to: Address,
    /// データ
    pub data: Vec<u8>,
    /// 署名
    pub signature: Signature,
}

/// ブロック
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// ブロック番号
    pub number: u64,
    /// 前ブロックのハッシュ
    pub parent_hash: BlockHash,
    /// タイムスタンプ
    pub timestamp: u64,
    /// トランザクションリスト
    pub transactions: Vec<Transaction>,
    /// ステートルート
    pub state_root: [u8; 32],
}

/// トランザクション実行結果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    /// トランザクションハッシュ
    pub tx_hash: TxHash,
    /// ステータス
    pub status: Status,
    /// ガス使用量
    pub gas_used: u64,
    /// ログ
    pub logs: Vec<Log>,
}

/// ステータス
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Success,
    Failure,
}

/// ログ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    /// アドレス
    pub address: Address,
    /// トピック
    pub topics: Vec<[u8; 32]>,
    /// データ
    pub data: Vec<u8>,
}
