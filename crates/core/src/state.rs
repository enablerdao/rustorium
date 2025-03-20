//! ステート管理

use anyhow::Result;
use crate::types::{Address, Transaction};
use tracing::{info, warn, error};

/// ステートマネージャ
pub struct StateManager {
    state: std::collections::HashMap<Address, Vec<u8>>,
}

impl StateManager {
    /// 新しいステートマネージャを作成
    pub fn new() -> Self {
        Self {
            state: std::collections::HashMap::new(),
        }
    }
    
    /// ステートを更新
    pub fn update_state(&mut self, tx: &Transaction) -> Result<()> {
        // トランザクションの実行
        let result = self.execute_transaction(tx)?;
        
        // ステートの更新
        self.state.insert(tx.to.clone(), result);
        
        Ok(())
    }
    
    /// ステートを取得
    pub fn get_state(&self, address: &Address) -> Option<&Vec<u8>> {
        self.state.get(address)
    }
    
    /// トランザクションを実行
    fn execute_transaction(&self, tx: &Transaction) -> Result<Vec<u8>> {
        // トランザクションの実行ロジック
        // TODO: 実際の実行エンジンを実装
        Ok(tx.data.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_state_manager() -> Result<()> {
        let mut manager = StateManager::new();
        
        // ステートの更新
        let tx = Transaction::new();
        manager.update_state(&tx)?;
        
        // ステートの取得
        let state = manager.get_state(&tx.to).unwrap();
        assert_eq!(state, &tx.data);
        
        Ok(())
    }
}
