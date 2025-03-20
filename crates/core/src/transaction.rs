//! トランザクション処理

use anyhow::Result;
use crate::types::{Transaction, Receipt, TxHash};
use tracing::{info, warn, error};

/// トランザクションプール
pub struct TransactionPool {
    pending: Vec<Transaction>,
}

impl TransactionPool {
    /// 新しいトランザクションプールを作成
    pub fn new() -> Self {
        Self {
            pending: Vec::new(),
        }
    }
    
    /// トランザクションを追加
    pub fn add_transaction(&mut self, tx: Transaction) -> Result<TxHash> {
        // トランザクションの検証
        self.validate_transaction(&tx)?;
        
        // プールに追加
        self.pending.push(tx.clone());
        
        Ok(tx.hash())
    }
    
    /// トランザクションを取得
    pub fn get_transaction(&self, hash: &TxHash) -> Option<&Transaction> {
        self.pending.iter().find(|tx| tx.hash() == *hash)
    }
    
    /// トランザクションを検証
    fn validate_transaction(&self, tx: &Transaction) -> Result<()> {
        // 署名の検証
        tx.verify_signature()?;
        
        // 重複チェック
        if self.pending.iter().any(|t| t.hash() == tx.hash()) {
            return Err(anyhow::anyhow!("Duplicate transaction"));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_transaction_pool() -> Result<()> {
        let mut pool = TransactionPool::new();
        
        // トランザクションの追加
        let tx = Transaction::new();
        let hash = pool.add_transaction(tx.clone())?;
        
        // トランザクションの取得
        let stored_tx = pool.get_transaction(&hash).unwrap();
        assert_eq!(stored_tx.hash(), hash);
        
        Ok(())
    }
}
