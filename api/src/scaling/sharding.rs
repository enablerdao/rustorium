// シャーディング実装
// 水平スケーリングによるスループット向上

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// シャード
/// データの水平分割単位
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shard {
    /// シャードID
    pub id: usize,
    
    /// シャード名
    pub name: String,
    
    /// 担当ノード数
    pub node_count: usize,
    
    /// 処理中のトランザクション数
    pub active_transactions: usize,
    
    /// 累計処理トランザクション数
    pub total_transactions: usize,
    
    /// 状態（アクティブ/非アクティブ）
    pub active: bool,
}

/// シャードマネージャー
/// シャードの作成、管理、割り当てを担当
pub struct ShardManager {
    /// シャードリスト
    shards: Arc<Mutex<Vec<Shard>>>,
    
    /// シャードマッピング（アドレス -> シャードID）
    address_mapping: Arc<Mutex<HashMap<String, usize>>>,
}

impl ShardManager {
    /// 新しいシャードマネージャーを作成
    pub fn new(initial_shards: usize) -> Self {
        let mut shards = Vec::with_capacity(initial_shards);
        
        // 初期シャードを作成
        for i in 0..initial_shards {
            shards.push(Shard {
                id: i,
                name: format!("shard-{}", i),
                node_count: 0,
                active_transactions: 0,
                total_transactions: 0,
                active: true,
            });
        }
        
        Self {
            shards: Arc::new(Mutex::new(shards)),
            address_mapping: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// シャード数を設定
    pub fn set_shard_count(&self, count: usize) -> Result<(), String> {
        let mut shards = self.shards.lock().unwrap();
        let current_count = shards.len();
        
        if count == current_count {
            return Ok(());
        }
        
        if count > current_count {
            // シャードを追加
            for i in current_count..count {
                shards.push(Shard {
                    id: i,
                    name: format!("shard-{}", i),
                    node_count: 0,
                    active_transactions: 0,
                    total_transactions: 0,
                    active: true,
                });
            }
        } else {
            // シャードを削除
            // 注意: 実際の実装では、削除前にシャードのデータを移行する必要がある
            shards.truncate(count);
        }
        
        // アドレスマッピングを更新
        self.rebalance_addresses()?;
        
        Ok(())
    }
    
    /// アドレスマッピングを再調整
    fn rebalance_addresses(&self) -> Result<(), String> {
        let shards = self.shards.lock().unwrap();
        let mut address_mapping = self.address_mapping.lock().unwrap();
        
        if shards.is_empty() {
            return Err("No shards available".to_string());
        }
        
        // 全てのアドレスを再マッピング
        for (address, _) in address_mapping.iter_mut() {
            let shard_id = self.calculate_shard_id(address, shards.len());
            *_ = shard_id;
        }
        
        Ok(())
    }
    
    /// アドレスからシャードIDを計算
    fn calculate_shard_id(&self, address: &str, shard_count: usize) -> usize {
        // 単純なハッシュベースのシャーディング
        // 実際の実装では、より洗練されたアルゴリズムを使用する
        let hash = address.bytes().fold(0u64, |acc, b| acc.wrapping_add(b as u64));
        (hash % shard_count as u64) as usize
    }
    
    /// アドレスをシャードに割り当て
    pub fn assign_address(&self, address: &str) -> usize {
        let mut address_mapping = self.address_mapping.lock().unwrap();
        
        // 既存のマッピングを確認
        if let Some(&shard_id) = address_mapping.get(address) {
            return shard_id;
        }
        
        // 新しいマッピングを作成
        let shards = self.shards.lock().unwrap();
        let shard_id = self.calculate_shard_id(address, shards.len());
        
        address_mapping.insert(address.to_string(), shard_id);
        
        shard_id
    }
    
    /// シャードにノードを追加
    pub fn add_node_to_shard(&self, shard_id: usize) -> Result<(), String> {
        let mut shards = self.shards.lock().unwrap();
        
        if shard_id >= shards.len() {
            return Err(format!("Shard ID {} out of range", shard_id));
        }
        
        shards[shard_id].node_count += 1;
        
        Ok(())
    }
    
    /// シャードからノードを削除
    pub fn remove_node_from_shard(&self, shard_id: usize) -> Result<(), String> {
        let mut shards = self.shards.lock().unwrap();
        
        if shard_id >= shards.len() {
            return Err(format!("Shard ID {} out of range", shard_id));
        }
        
        if shards[shard_id].node_count == 0 {
            return Err(format!("No nodes in shard {}", shard_id));
        }
        
        shards[shard_id].node_count -= 1;
        
        Ok(())
    }
    
    /// トランザクションをシャードに追加
    pub fn add_transaction_to_shard(&self, shard_id: usize) -> Result<(), String> {
        let mut shards = self.shards.lock().unwrap();
        
        if shard_id >= shards.len() {
            return Err(format!("Shard ID {} out of range", shard_id));
        }
        
        shards[shard_id].active_transactions += 1;
        shards[shard_id].total_transactions += 1;
        
        Ok(())
    }
    
    /// トランザクションをシャードから削除
    pub fn remove_transaction_from_shard(&self, shard_id: usize) -> Result<(), String> {
        let mut shards = self.shards.lock().unwrap();
        
        if shard_id >= shards.len() {
            return Err(format!("Shard ID {} out of range", shard_id));
        }
        
        if shards[shard_id].active_transactions == 0 {
            return Err(format!("No active transactions in shard {}", shard_id));
        }
        
        shards[shard_id].active_transactions -= 1;
        
        Ok(())
    }
    
    /// シャードの状態を切り替え
    pub fn toggle_shard_active(&self, shard_id: usize, active: bool) -> Result<(), String> {
        let mut shards = self.shards.lock().unwrap();
        
        if shard_id >= shards.len() {
            return Err(format!("Shard ID {} out of range", shard_id));
        }
        
        shards[shard_id].active = active;
        
        Ok(())
    }
    
    /// シャードリストを取得
    pub fn get_shards(&self) -> Vec<Shard> {
        let shards = self.shards.lock().unwrap();
        shards.clone()
    }
    
    /// シャード数を取得
    pub fn get_shard_count(&self) -> usize {
        let shards = self.shards.lock().unwrap();
        shards.len()
    }
    
    /// アドレスのシャードIDを取得
    pub fn get_shard_for_address(&self, address: &str) -> Option<usize> {
        let address_mapping = self.address_mapping.lock().unwrap();
        address_mapping.get(address).cloned()
    }
    
    /// シャードの負荷状況を取得
    pub fn get_shard_load(&self) -> Vec<(usize, f64)> {
        let shards = self.shards.lock().unwrap();
        
        shards.iter().map(|shard| {
            let load = if shard.node_count == 0 {
                0.0
            } else {
                shard.active_transactions as f64 / shard.node_count as f64
            };
            
            (shard.id, load)
        }).collect()
    }
    
    /// 最も負荷の低いシャードを取得
    pub fn get_least_loaded_shard(&self) -> Option<usize> {
        let loads = self.get_shard_load();
        
        if loads.is_empty() {
            return None;
        }
        
        loads.iter()
            .min_by(|(_, load1), (_, load2)| load1.partial_cmp(load2).unwrap())
            .map(|(id, _)| *id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_shard_manager_creation() {
        let manager = ShardManager::new(4);
        let shards = manager.get_shards();
        
        assert_eq!(shards.len(), 4);
        
        for (i, shard) in shards.iter().enumerate() {
            assert_eq!(shard.id, i);
            assert_eq!(shard.name, format!("shard-{}", i));
            assert_eq!(shard.node_count, 0);
            assert_eq!(shard.active_transactions, 0);
            assert_eq!(shard.total_transactions, 0);
            assert!(shard.active);
        }
    }
    
    #[test]
    fn test_set_shard_count() {
        let manager = ShardManager::new(2);
        
        // シャード数を増やす
        manager.set_shard_count(4).unwrap();
        assert_eq!(manager.get_shard_count(), 4);
        
        // シャード数を減らす
        manager.set_shard_count(3).unwrap();
        assert_eq!(manager.get_shard_count(), 3);
    }
    
    #[test]
    fn test_address_assignment() {
        let manager = ShardManager::new(4);
        
        // アドレスをシャードに割り当て
        let shard_id1 = manager.assign_address("address1");
        let shard_id2 = manager.assign_address("address2");
        
        // 同じアドレスは同じシャードに割り当てられる
        let shard_id1_again = manager.assign_address("address1");
        assert_eq!(shard_id1, shard_id1_again);
        
        // シャードIDを取得
        let shard_id1_get = manager.get_shard_for_address("address1").unwrap();
        let shard_id2_get = manager.get_shard_for_address("address2").unwrap();
        
        assert_eq!(shard_id1, shard_id1_get);
        assert_eq!(shard_id2, shard_id2_get);
    }
    
    #[test]
    fn test_node_management() {
        let manager = ShardManager::new(2);
        
        // ノードを追加
        manager.add_node_to_shard(0).unwrap();
        manager.add_node_to_shard(0).unwrap();
        manager.add_node_to_shard(1).unwrap();
        
        let shards = manager.get_shards();
        assert_eq!(shards[0].node_count, 2);
        assert_eq!(shards[1].node_count, 1);
        
        // ノードを削除
        manager.remove_node_from_shard(0).unwrap();
        
        let shards = manager.get_shards();
        assert_eq!(shards[0].node_count, 1);
        assert_eq!(shards[1].node_count, 1);
        
        // 範囲外のシャードIDを指定
        assert!(manager.add_node_to_shard(2).is_err());
        assert!(manager.remove_node_from_shard(2).is_err());
    }
    
    #[test]
    fn test_transaction_management() {
        let manager = ShardManager::new(2);
        
        // トランザクションを追加
        manager.add_transaction_to_shard(0).unwrap();
        manager.add_transaction_to_shard(0).unwrap();
        manager.add_transaction_to_shard(1).unwrap();
        
        let shards = manager.get_shards();
        assert_eq!(shards[0].active_transactions, 2);
        assert_eq!(shards[0].total_transactions, 2);
        assert_eq!(shards[1].active_transactions, 1);
        assert_eq!(shards[1].total_transactions, 1);
        
        // トランザクションを削除
        manager.remove_transaction_from_shard(0).unwrap();
        
        let shards = manager.get_shards();
        assert_eq!(shards[0].active_transactions, 1);
        assert_eq!(shards[0].total_transactions, 2); // 累計は変わらない
        
        // 範囲外のシャードIDを指定
        assert!(manager.add_transaction_to_shard(2).is_err());
        assert!(manager.remove_transaction_from_shard(2).is_err());
    }
    
    #[test]
    fn test_shard_load() {
        let manager = ShardManager::new(2);
        
        // ノードを追加
        manager.add_node_to_shard(0).unwrap();
        manager.add_node_to_shard(0).unwrap();
        manager.add_node_to_shard(1).unwrap();
        
        // トランザクションを追加
        manager.add_transaction_to_shard(0).unwrap();
        manager.add_transaction_to_shard(0).unwrap();
        manager.add_transaction_to_shard(1).unwrap();
        manager.add_transaction_to_shard(1).unwrap();
        
        // 負荷状況を取得
        let loads = manager.get_shard_load();
        assert_eq!(loads.len(), 2);
        
        // シャード0: 2トランザクション / 2ノード = 1.0
        assert_eq!(loads[0].0, 0);
        assert_eq!(loads[0].1, 1.0);
        
        // シャード1: 2トランザクション / 1ノード = 2.0
        assert_eq!(loads[1].0, 1);
        assert_eq!(loads[1].1, 2.0);
        
        // 最も負荷の低いシャードを取得
        let least_loaded = manager.get_least_loaded_shard().unwrap();
        assert_eq!(least_loaded, 0);
    }
}