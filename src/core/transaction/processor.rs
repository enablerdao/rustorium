use anyhow::Result;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::time::SystemTime;
use tracing::{info, warn};
use crate::core::{
    storage::redb_storage::RedbStorage,
    cache::CacheManager,
};
use std::sync::Arc;

/// トランザクション処理エンジン
#[async_trait]
pub trait TransactionProcessor {
    /// トランザクションを処理
    async fn process_transaction(&self, tx: Transaction) -> Result<TxReceipt>;
    
    /// トランザクションの状態を取得
    async fn get_transaction(&self, tx_hash: &str) -> Result<Option<Transaction>>;
    
    /// トランザクションストリームを購読
    async fn subscribe_transactions(&self) -> Result<TransactionStream>;
}

/// トランザクション
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// トランザクションID
    pub id: String,
    
    /// 送信者の位置情報
    pub location: GeoLocation,
    
    /// トランザクションデータ
    pub data: Vec<u8>,
    
    /// タイムスタンプ
    pub timestamp: SystemTime,
    
    /// 署名
    pub signature: Option<Vec<u8>>,
}

impl Transaction {
    /// 新しいトランザクションを作成
    pub fn new() -> TransactionBuilder {
        TransactionBuilder::default()
    }
    
    /// トランザクションのハッシュを計算
    pub fn calculate_hash(&self) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(&self.id);
        hasher.update(&self.data);
        format!("{:x}", hasher.finalize())
    }
}

/// トランザクションビルダー
#[derive(Default)]
pub struct TransactionBuilder {
    data: Option<Vec<u8>>,
    location: Option<GeoLocation>,
    signature: Option<Vec<u8>>,
}

impl TransactionBuilder {
    pub fn with_data(mut self, data: Vec<u8>) -> Self {
        self.data = Some(data);
        self
    }
    
    pub fn with_location(mut self, location: GeoLocation) -> Self {
        self.location = Some(location);
        self
    }
    
    pub fn with_signature(mut self, signature: Vec<u8>) -> Self {
        self.signature = Some(signature);
        self
    }
    
    pub fn build(self) -> Result<Transaction> {
        Ok(Transaction {
            id: uuid::Uuid::new_v4().to_string(),
            data: self.data.unwrap_or_default(),
            location: self.location.unwrap_or_default(),
            timestamp: SystemTime::now(),
            signature: self.signature,
        })
    }
}

/// トランザクション処理の結果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxReceipt {
    /// トランザクションID
    pub tx_id: String,
    
    /// 処理状態
    pub status: TxStatus,
    
    /// ブロック番号
    pub block_number: Option<u64>,
    
    /// タイムスタンプ
    pub timestamp: SystemTime,
}

/// トランザクションの状態
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TxStatus {
    Pending,
    Confirmed,
    Failed(String),
}

/// トランザクションストリーム
pub struct TransactionStream {
    // TODO: 実装
}

/// デフォルトのトランザクション処理エンジン
pub struct DefaultTransactionProcessor {
    storage: Arc<RedbStorage>,
    cache: Arc<CacheManager>,
}

impl DefaultTransactionProcessor {
    pub fn new(storage: Arc<RedbStorage>, cache: Arc<CacheManager>) -> Self {
        Self { storage, cache }
    }
}

#[async_trait]
impl TransactionProcessor for DefaultTransactionProcessor {
    async fn process_transaction(&self, tx: Transaction) -> Result<TxReceipt> {
        info!("Processing transaction: {}", tx.id);
        
        // トランザクションの検証
        if tx.data.is_empty() {
            warn!("Empty transaction data");
            return Ok(TxReceipt {
                tx_id: tx.id,
                status: TxStatus::Failed("Empty transaction data".into()),
                block_number: None,
                timestamp: SystemTime::now(),
            });
        }
        
        // キャッシュの更新
        self.cache.set(tx.id.as_bytes(), &tx.data).await?;
        
        // ストレージへの永続化
        self.storage.write_with_proof(
            tx.id.as_bytes(),
            &tx.data,
        ).await?;
        
        Ok(TxReceipt {
            tx_id: tx.id,
            status: TxStatus::Confirmed,
            block_number: Some(0), // TODO: 実際のブロック番号
            timestamp: SystemTime::now(),
        })
    }
    
    async fn get_transaction(&self, tx_hash: &str) -> Result<Option<Transaction>> {
        // まずキャッシュを確認
        if let Some(data) = self.cache.get(tx_hash.as_bytes()).await? {
            return Ok(Some(Transaction {
                id: tx_hash.to_string(),
                data,
                location: GeoLocation::default(),
                timestamp: SystemTime::now(),
                signature: None,
            }));
        }
        
        // ストレージを確認
        if let Some(result) = self.storage.read(tx_hash.as_bytes()).await? {
            return Ok(Some(Transaction {
                id: tx_hash.to_string(),
                data: result.value,
                location: GeoLocation::default(),
                timestamp: SystemTime::now(),
                signature: None,
            }));
        }
        
        Ok(None)
    }
    
    async fn subscribe_transactions(&self) -> Result<TransactionStream> {
        // TODO: 実装
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    
    #[test]
    async fn test_transaction_creation() {
        let tx = Transaction::new()
            .with_data(b"test data".to_vec())
            .with_location(GeoLocation::default())
            .build()
            .unwrap();
            
        assert!(!tx.id.is_empty());
        assert_eq!(tx.data, b"test data");
    }
    
    #[test]
    async fn test_transaction_hash() {
        let tx = Transaction::new()
            .with_data(b"test data".to_vec())
            .build()
            .unwrap();
            
        let hash = tx.calculate_hash();
        assert!(!hash.is_empty());
    }
}
