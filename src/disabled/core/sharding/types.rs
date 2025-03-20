use serde::{Deserialize, Serialize};
use super::super::types::Address;

/// シャードID
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShardId(pub Vec<u8>);

/// クロスシャードトランザクションの状態
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrossShardTxStatus {
    /// 保留中
    Pending,
    /// 送信シャードで承認済み
    SourceApproved,
    /// 受信シャードで承認済み
    DestApproved,
    /// 確定済み
    Confirmed,
    /// 拒否
    Rejected,
}

/// クロスシャードトランザクション
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossShardTx {
    /// トランザクションID
    pub id: Vec<u8>,
    /// 送信シャード
    pub source_shard: ShardId,
    /// 受信シャード
    pub dest_shard: ShardId,
    /// 送信者
    pub from: Address,
    /// 受信者
    pub to: Address,
    /// 金額
    pub amount: u64,
    /// 状態
    pub status: CrossShardTxStatus,
    /// タイムスタンプ
    pub timestamp: i64,
}

pub use self::CrossShardTxStatus as CrossShardTxStatus;