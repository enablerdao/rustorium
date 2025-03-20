//! ストレージ層
//! 
//! TiKV、Redb、Noriaを使用した高性能なストレージエンジンを提供します。

use anyhow::Result;
use tikv_client::{RawClient as TiKVClient, Config as TiKVConfig};
use redb::{Database as RedbDatabase, ReadableTable, TableDefinition};
use noria::{DataflowGraph, View};
use poseidon_rs::Poseidon;
use tracing::{info, warn, error};

/// ストレージエンジン
pub struct StorageEngine {
    tikv: TiKVClient,
    redb: RedbDatabase,
    noria: DataflowGraph,
    poseidon: Poseidon,
}

impl StorageEngine {
    /// 新しいストレージエンジンを作成
    pub async fn new() -> Result<Self> {
        info!("Initializing storage engine...");
        
        // TiKVの設定
        let tikv_config = TiKVConfig::default();
        let tikv = TiKVClient::new(vec!["127.0.0.1:2379"], tikv_config).await?;
        
        // Redbの設定
        let redb = RedbDatabase::create("data/redb")?;
        
        // Noriaの設定
        let noria = DataflowGraph::new();
        
        // Poseidonの設定
        let poseidon = Poseidon::new();
        
        Ok(Self {
            tikv,
            redb,
            noria,
            poseidon,
        })
    }
    
    /// データの保存
    pub async fn store(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        // TiKVに保存
        self.tikv.put(key.to_vec(), value.to_vec()).await?;
        
        // Redbに保存
        let tx = self.redb.begin_write()?;
        {
            let mut table = tx.open_table(TableDefinition::new("data"))?;
            table.insert(key, value)?;
        }
        tx.commit()?;
        
        // Noriaのビューを更新
        self.update_views(key, value).await?;
        
        Ok(())
    }
    
    /// データの取得
    pub async fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        // まずNoriaのキャッシュを確認
        if let Some(value) = self.get_from_cache(key).await? {
            return Ok(Some(value));
        }
        
        // Redbを確認
        if let Some(value) = self.get_from_redb(key)? {
            return Ok(Some(value));
        }
        
        // TiKVから取得
        self.tikv.get(key.to_vec()).await
    }
    
    /// Noriaのキャッシュから取得
    async fn get_from_cache(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        // TODO: Noriaのキャッシュ実装
        Ok(None)
    }
    
    /// Redbから取得
    fn get_from_redb(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        let tx = self.redb.begin_read()?;
        let table = tx.open_table(TableDefinition::new("data"))?;
        Ok(table.get(key)?.map(|v| v.to_vec()))
    }
    
    /// Noriaのビューを更新
    async fn update_views(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        // TODO: Noriaのビュー更新実装
        Ok(())
    }
    
    /// Poseidonハッシュの計算
    pub fn compute_hash(&self, data: &[u8]) -> Vec<u8> {
        self.poseidon.hash(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_storage_operations() -> Result<()> {
        let mut storage = StorageEngine::new().await?;
        
        // データの保存
        let key = b"test_key";
        let value = b"test_value";
        storage.store(key, value).await?;
        
        // データの取得
        let stored_value = storage.get(key).await?;
        assert_eq!(stored_value.as_deref(), Some(value.as_ref()));
        
        Ok(())
    }
    
    #[test]
    fn test_hash_computation() -> Result<()> {
        let storage = StorageEngine::new().await?;
        
        let data = b"test_data";
        let hash = storage.compute_hash(data);
        
        assert_eq!(hash.len(), 32);  // Poseidonは256ビットハッシュを生成
        
        Ok(())
    }
}
