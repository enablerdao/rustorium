//! ブロック処理

use anyhow::Result;
use crate::types::{Block, BlockHash, Transaction};
use tracing::{info, warn, error};

/// ブロックチェーン
pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    /// 新しいブロックチェーンを作成
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
        }
    }
    
    /// ブロックを追加
    pub fn add_block(&mut self, block: Block) -> Result<BlockHash> {
        // ブロックの検証
        self.validate_block(&block)?;
        
        // チェーンに追加
        self.blocks.push(block.clone());
        
        Ok(block.hash())
    }
    
    /// ブロックを取得
    pub fn get_block(&self, hash: &BlockHash) -> Option<&Block> {
        self.blocks.iter().find(|b| b.hash() == *hash)
    }
    
    /// ブロックを検証
    fn validate_block(&self, block: &Block) -> Result<()> {
        // 前ブロックの存在確認
        if !self.blocks.is_empty() {
            let parent = self.get_block(&block.parent_hash)
                .ok_or_else(|| anyhow::anyhow!("Parent block not found"))?;
            
            // ブロック番号の検証
            if block.number != parent.number + 1 {
                return Err(anyhow::anyhow!("Invalid block number"));
            }
        }
        
        // トランザクションの検証
        for tx in &block.transactions {
            tx.verify()?;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_blockchain() -> Result<()> {
        let mut chain = Blockchain::new();
        
        // ブロックの追加
        let block = Block::new();
        let hash = chain.add_block(block.clone())?;
        
        // ブロックの取得
        let stored_block = chain.get_block(&hash).unwrap();
        assert_eq!(stored_block.hash(), hash);
        
        Ok(())
    }
}
