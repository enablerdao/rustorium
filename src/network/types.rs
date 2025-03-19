use crate::core::{
    dag::{Transaction, TxId},
    avalanche::Vote,
    sharding::{CrossShardTx, ShardId, ShardState},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    // DAG関連
    NewTransaction(Transaction),
    TransactionConfirmation(TxId),
    
    // Avalanche関連
    Vote {
        tx_id: TxId,
        vote: Vote,
    },
    QueryTransaction {
        tx_id: TxId,
    },
    
    // シャーディング関連
    CrossShardTransaction(CrossShardTx),
    ShardState {
        shard_id: ShardId,
        state: ShardState,
    },
}