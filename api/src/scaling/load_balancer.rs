// ロードバランサー実装
// トランザクションとブロックの分散処理

use crate::blockchain::Block;
use crate::scaling::Shard;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// ロードバランサー
/// トランザクションとブロックの分散処理を担当
pub struct LoadBalancer {
    /// シャードリスト
    shards: Arc<Mutex<Vec<Shard>>>,
    
    /// アドレスマッピング（アドレス -> シャードID）
    address_mapping: Arc<Mutex<HashMap<String, usize>>>,
    
    /// ラウンドロビンカウンター
    round_robin_counter: Arc<Mutex<usize>>,
}

/// 負荷分散戦略
pub enum BalancingStrategy {
    /// ハッシュベース（アドレスに基づく）
    Hash,
    /// ラウンドロビン
    RoundRobin,
    /// 最小負荷
    LeastLoaded,
}

impl LoadBalancer {
    /// 新しいロードバランサーを作成
    pub fn new() -> Self {
        Self {
            shards: Arc::new(Mutex::new(Vec::new())),
            address_mapping: Arc::new(Mutex::new(HashMap::new())),
            round_robin_counter: Arc::new(Mutex::new(0)),
        }
    }
    
    /// シャードリストを更新
    pub fn update_shards(&self, shards: Vec<Shard>) {
        let mut shards_lock = self.shards.lock().unwrap();
        *shards_lock = shards;
    }
    
    /// アドレスをシャードに割り当て
    pub fn assign_address(&self, address: &str, strategy: BalancingStrategy) -> Option<usize> {
        let mut address_mapping = self.address_mapping.lock().unwrap();
        
        // 既存のマッピングを確認
        if let Some(&shard_id) = address_mapping.get(address) {
            return Some(shard_id);
        }
        
        let shards = self.shards.lock().unwrap();
        
        if shards.is_empty() {
            return None;
        }
        
        // 新しいマッピングを作成
        let shard_id = match strategy {
            BalancingStrategy::Hash => self.hash_based_assignment(address, shards.len()),
            BalancingStrategy::RoundRobin => self.round_robin_assignment(shards.len()),
            BalancingStrategy::LeastLoaded => self.least_loaded_assignment(&shards),
        };
        
        address_mapping.insert(address.to_string(), shard_id);
        
        Some(shard_id)
    }
    
    /// ハッシュベースの割り当て
    fn hash_based_assignment(&self, address: &str, shard_count: usize) -> usize {
        let hash = address.bytes().fold(0u64, |acc, b| acc.wrapping_add(b as u64));
        (hash % shard_count as u64) as usize
    }
    
    /// ラウンドロビン割り当て
    fn round_robin_assignment(&self, shard_count: usize) -> usize {
        let mut counter = self.round_robin_counter.lock().unwrap();
        let shard_id = *counter % shard_count;
        *counter = (*counter + 1) % shard_count;
        shard_id
    }
    
    /// 最小負荷割り当て
    fn least_loaded_assignment(&self, shards: &[Shard]) -> usize {
        // アクティブなシャードのみを対象
        let active_shards: Vec<&Shard> = shards.iter()
            .filter(|s| s.active)
            .collect();
        
        if active_shards.is_empty() {
            return 0;
        }
        
        // 負荷の計算（アクティブトランザクション数 / ノード数）
        let loads: Vec<(usize, f64)> = active_shards.iter()
            .map(|shard| {
                let load = if shard.node_count == 0 {
                    f64::MAX // ノードがない場合は最大負荷とみなす
                } else {
                    shard.active_transactions as f64 / shard.node_count as f64
                };
                
                (shard.id, load)
            })
            .collect();
        
        // 最小負荷のシャードを選択
        loads.iter()
            .min_by(|(_, load1), (_, load2)| load1.partial_cmp(load2).unwrap())
            .map(|(id, _)| *id)
            .unwrap_or(0)
    }
    
    /// ブロックをシャードに割り当て
    pub fn assign_block(&self, block: &Block) -> usize {
        // ブロック内のトランザクションの送信者アドレスに基づいて割り当て
        if let Some(tx) = block.transactions.first() {
            if let Some(shard_id) = self.assign_address(&tx.from, BalancingStrategy::Hash) {
                return shard_id;
            }
        }
        
        // トランザクションがない場合はラウンドロビンで割り当て
        let shards = self.shards.lock().unwrap();
        if shards.is_empty() {
            return 0;
        }
        
        self.round_robin_assignment(shards.len())
    }
    
    /// トランザクションをシャードに割り当て
    pub fn assign_transaction(&self, from: &str, to: &str) -> usize {
        // 送信者アドレスに基づいて割り当て
        if let Some(shard_id) = self.assign_address(from, BalancingStrategy::Hash) {
            return shard_id;
        }
        
        // 送信者アドレスがない場合は受信者アドレスで割り当て
        if let Some(shard_id) = self.assign_address(to, BalancingStrategy::Hash) {
            return shard_id;
        }
        
        // どちらもない場合はラウンドロビンで割り当て
        let shards = self.shards.lock().unwrap();
        if shards.is_empty() {
            return 0;
        }
        
        self.round_robin_assignment(shards.len())
    }
    
    /// シャードの負荷状況を取得
    pub fn get_shard_loads(&self) -> Vec<(usize, f64)> {
        let shards = self.shards.lock().unwrap();
        
        shards.iter()
            .map(|shard| {
                let load = if shard.node_count == 0 {
                    0.0
                } else {
                    shard.active_transactions as f64 / shard.node_count as f64
                };
                
                (shard.id, load)
            })
            .collect()
    }
    
    /// アドレスのシャードIDを取得
    pub fn get_shard_for_address(&self, address: &str) -> Option<usize> {
        let address_mapping = self.address_mapping.lock().unwrap();
        address_mapping.get(address).cloned()
    }
    
    /// シャードリストを取得
    pub fn get_shards(&self) -> Vec<Shard> {
        let shards = self.shards.lock().unwrap();
        shards.clone()
    }
}

impl Default for LoadBalancer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::Transaction;
    
    fn create_test_shards(count: usize) -> Vec<Shard> {
        let mut shards = Vec::with_capacity(count);
        
        for i in 0..count {
            shards.push(Shard {
                id: i,
                name: format!("shard-{}", i),
                node_count: i + 1, // 各シャードに異なるノード数を設定
                active_transactions: i * 10, // 各シャードに異なるトランザクション数を設定
                total_transactions: i * 100,
                active: true,
            });
        }
        
        shards
    }
    
    #[test]
    fn test_load_balancer_creation() {
        let balancer = LoadBalancer::new();
        let shards = balancer.get_shards();
        
        assert!(shards.is_empty());
    }
    
    #[test]
    fn test_update_shards() {
        let balancer = LoadBalancer::new();
        let test_shards = create_test_shards(3);
        
        balancer.update_shards(test_shards.clone());
        
        let shards = balancer.get_shards();
        assert_eq!(shards.len(), 3);
        
        for (i, shard) in shards.iter().enumerate() {
            assert_eq!(shard.id, test_shards[i].id);
            assert_eq!(shard.name, test_shards[i].name);
            assert_eq!(shard.node_count, test_shards[i].node_count);
            assert_eq!(shard.active_transactions, test_shards[i].active_transactions);
        }
    }
    
    #[test]
    fn test_hash_based_assignment() {
        let balancer = LoadBalancer::new();
        let test_shards = create_test_shards(4);
        
        balancer.update_shards(test_shards);
        
        // 同じアドレスは同じシャードに割り当てられる
        let shard_id1 = balancer.assign_address("address1", BalancingStrategy::Hash).unwrap();
        let shard_id1_again = balancer.assign_address("address1", BalancingStrategy::Hash).unwrap();
        
        assert_eq!(shard_id1, shard_id1_again);
        
        // 異なるアドレスは異なるシャードに割り当てられる可能性がある
        let shard_id2 = balancer.assign_address("address2", BalancingStrategy::Hash).unwrap();
        let shard_id3 = balancer.assign_address("address3", BalancingStrategy::Hash).unwrap();
        
        // 注意: ハッシュ関数の性質上、異なるアドレスが同じシャードに割り当てられる可能性もある
        // そのため、この部分のテストは確率的
    }
    
    #[test]
    fn test_round_robin_assignment() {
        let balancer = LoadBalancer::new();
        let test_shards = create_test_shards(3);
        
        balancer.update_shards(test_shards);
        
        // ラウンドロビンで順番に割り当てられる
        let shard_id1 = balancer.assign_address("address1", BalancingStrategy::RoundRobin).unwrap();
        let shard_id2 = balancer.assign_address("address2", BalancingStrategy::RoundRobin).unwrap();
        let shard_id3 = balancer.assign_address("address3", BalancingStrategy::RoundRobin).unwrap();
        let shard_id4 = balancer.assign_address("address4", BalancingStrategy::RoundRobin).unwrap();
        
        assert_eq!(shard_id1, 0);
        assert_eq!(shard_id2, 1);
        assert_eq!(shard_id3, 2);
        assert_eq!(shard_id4, 0); // 一周して最初に戻る
    }
    
    #[test]
    fn test_least_loaded_assignment() {
        let balancer = LoadBalancer::new();
        let mut test_shards = create_test_shards(3);
        
        // シャード0: 0トランザクション / 1ノード = 0.0
        // シャード1: 10トランザクション / 2ノード = 5.0
        // シャード2: 20トランザクション / 3ノード = 6.67
        test_shards[0].active_transactions = 0;
        test_shards[1].active_transactions = 10;
        test_shards[2].active_transactions = 20;
        
        balancer.update_shards(test_shards);
        
        // 最も負荷の低いシャード（シャード0）に割り当てられる
        let shard_id = balancer.assign_address("address1", BalancingStrategy::LeastLoaded).unwrap();
        assert_eq!(shard_id, 0);
    }
    
    #[test]
    fn test_assign_block() {
        let balancer = LoadBalancer::new();
        let test_shards = create_test_shards(3);
        
        balancer.update_shards(test_shards);
        
        // トランザクションを含むブロックを作成
        let tx = Transaction {
            id: "tx1".to_string(),
            from: "address1".to_string(),
            to: "address2".to_string(),
            value: 10.0,
            data: None,
            gas_price: 1,
            gas_limit: 21000,
            nonce: 0,
            timestamp: 0,
            signature: None,
            status: "pending".to_string(),
            gas_used: 0,
            block_id: None,
        };
        
        let block = Block {
            hash: "block1".to_string(),
            previous_hash: "block0".to_string(),
            timestamp: 0,
            nonce: 0,
            transactions: vec![tx],
            miner: "miner1".to_string(),
            difficulty: 1,
            height: 1,
        };
        
        // ブロックをシャードに割り当て
        let shard_id = balancer.assign_block(&block);
        
        // 送信者アドレス（address1）に基づいて割り当てられる
        let expected_shard_id = balancer.get_shard_for_address("address1").unwrap();
        assert_eq!(shard_id, expected_shard_id);
    }
    
    #[test]
    fn test_assign_transaction() {
        let balancer = LoadBalancer::new();
        let test_shards = create_test_shards(3);
        
        balancer.update_shards(test_shards);
        
        // トランザクションをシャードに割り当て
        let shard_id = balancer.assign_transaction("address1", "address2");
        
        // 送信者アドレス（address1）に基づいて割り当てられる
        let expected_shard_id = balancer.get_shard_for_address("address1").unwrap();
        assert_eq!(shard_id, expected_shard_id);
    }
}