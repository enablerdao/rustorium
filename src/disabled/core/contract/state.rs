use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::core::types::Address;

/// コントラクトの状態
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractState {
    /// ストレージ
    storage: HashMap<Vec<u8>, Vec<u8>>,
    /// バランス
    balances: HashMap<Address, u64>,
    /// メタデータ
    metadata: HashMap<String, Vec<u8>>,
}

impl ContractState {
    /// 新しい状態を作成
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
            balances: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    /// 値を設定
    pub fn set(&mut self, key: &[u8], value: Vec<u8>) {
        self.storage.insert(key.to_vec(), value);
    }

    /// 値を取得
    pub fn get(&self, key: &[u8]) -> Option<&Vec<u8>> {
        self.storage.get(key)
    }

    /// 値を削除
    pub fn remove(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        self.storage.remove(key)
    }

    /// バランスを設定
    pub fn set_balance(&mut self, address: &Address, amount: u64) {
        self.balances.insert(address.clone(), amount);
    }

    /// バランスを取得
    pub fn get_balance(&self, address: &Address) -> u64 {
        *self.balances.get(address).unwrap_or(&0)
    }

    /// バランスを加算
    pub fn add_balance(&mut self, address: &Address, amount: u64) {
        let balance = self.get_balance(address);
        self.set_balance(address, balance + amount);
    }

    /// バランスを減算
    pub fn sub_balance(&mut self, address: &Address, amount: u64) -> bool {
        let balance = self.get_balance(address);
        if balance >= amount {
            self.set_balance(address, balance - amount);
            true
        } else {
            false
        }
    }

    /// メタデータを設定
    pub fn set_metadata(&mut self, key: &str, value: Vec<u8>) {
        self.metadata.insert(key.to_string(), value);
    }

    /// メタデータを取得
    pub fn get_metadata(&self, key: &str) -> Option<&Vec<u8>> {
        self.metadata.get(key)
    }

    /// メタデータを削除
    pub fn remove_metadata(&mut self, key: &str) -> Option<Vec<u8>> {
        self.metadata.remove(key)
    }

    /// シリアライズ
    pub fn serialize(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(self)
    }

    /// デシリアライズ
    pub fn deserialize(bytes: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(bytes)
    }
}

impl Default for ContractState {
    fn default() -> Self {
        Self::new()
    }
}