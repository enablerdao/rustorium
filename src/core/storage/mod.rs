use std::path::Path;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait StorageEngine: Send + Sync + std::fmt::Debug {
    async fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;
    async fn put(&self, key: &[u8], value: &[u8]) -> Result<()>;
    async fn delete(&self, key: &[u8]) -> Result<()>;
    async fn batch_write(&self, batch: Vec<(Vec<u8>, Option<Vec<u8>>)>) -> Result<()>;
}

#[derive(Debug)]
pub struct RocksDBStorage {
    db: rocksdb::DB,
}

impl RocksDBStorage {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let db = rocksdb::DB::open_default(path)?;
        Ok(Self { db })
    }
}

#[async_trait]
impl StorageEngine for RocksDBStorage {
    async fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(self.db.get(key)?)
    }

    async fn put(&self, key: &[u8], value: &[u8]) -> Result<()> {
        Ok(self.db.put(key, value)?)
    }

    async fn delete(&self, key: &[u8]) -> Result<()> {
        Ok(self.db.delete(key)?)
    }

    async fn batch_write(&self, batch: Vec<(Vec<u8>, Option<Vec<u8>>)>) -> Result<()> {
        let mut wb = rocksdb::WriteBatch::default();
        for (key, value) in batch {
            match value {
                Some(value) => {
                    wb.put(&key, &value);
                }
                None => {
                    wb.delete(&key);
                }
            }
        }
        self.db.write(wb)?;
        Ok(())
    }
}