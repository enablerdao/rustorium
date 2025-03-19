use std::path::Path;
use std::sync::Arc;
use anyhow::{Context, Result};
use async_trait::async_trait;
use rocksdb::{DB, Options, ColumnFamilyDescriptor, WriteBatch, WriteOptions};
use tokio::sync::Mutex;
use snap::raw::Encoder;
use super::{StorageEngine, TypedStorage};

pub struct RocksDBStorage {
    db: Arc<DB>,
    write_lock: Arc<Mutex<()>>,
}

impl RocksDBStorage {
    pub fn new(path: &Path) -> Result<Self> {
        // データディレクトリを作成
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // カラムファミリーの設定
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);

        // カラムファミリーの定義
        let cf_opts = Options::default();
        let cf_descriptors = vec![
            ColumnFamilyDescriptor::new("default", cf_opts.clone()),
            ColumnFamilyDescriptor::new("shard_state", cf_opts.clone()),
            ColumnFamilyDescriptor::new("transactions", cf_opts.clone()),
            ColumnFamilyDescriptor::new("metadata", cf_opts),
        ];

        // データベースを開く
        let db = DB::open_cf_descriptors(&opts, path, cf_descriptors)?;

        Ok(Self {
            db: Arc::new(db),
            write_lock: Arc::new(Mutex::new(())),
        })
    }

    fn compress_value(value: &[u8]) -> Result<Vec<u8>> {
        let mut encoder = Encoder::new();
        Ok(encoder.compress_vec(value)?)
    }

    fn decompress_value(value: &[u8]) -> Result<Vec<u8>> {
        let mut encoder = Encoder::new();
        Ok(encoder.decompress_vec(value)?)
    }
}

#[async_trait]
impl StorageEngine for RocksDBStorage {
    async fn put_bytes(&self, cf: &str, key: &[u8], value: &[u8]) -> Result<()> {
        let cf_handle = self.db.cf_handle(cf).context("Column family not found")?;
        let compressed = Self::compress_value(value)?;
        
        let _lock = self.write_lock.lock().await;
        self.db.put_cf(cf_handle, key, compressed)?;
        Ok(())
    }

    async fn get_bytes(&self, cf: &str, key: &[u8]) -> Result<Option<Vec<u8>>> {
        let cf_handle = self.db.cf_handle(cf).context("Column family not found")?;
        if let Some(bytes) = self.db.get_cf(cf_handle, key)? {
            Ok(Some(Self::decompress_value(&bytes)?))
        } else {
            Ok(None)
        }
    }

    async fn delete_bytes(&self, cf: &str, key: &[u8]) -> Result<()> {
        let cf_handle = self.db.cf_handle(cf).context("Column family not found")?;
        let _lock = self.write_lock.lock().await;
        self.db.delete_cf(cf_handle, key)?;
        Ok(())
    }

    async fn scan_prefix_bytes(&self, cf: &str, prefix: &[u8]) -> Result<Vec<(Vec<u8>, Vec<u8>)>> {
        let cf_handle = self.db.cf_handle(cf).context("Column family not found")?;
        let mut results = Vec::new();

        let iter = self.db.prefix_iterator_cf(cf_handle, prefix);
        for item in iter {
            let (key, value) = item?;
            let decompressed = Self::decompress_value(&value)?;
            results.push((key.to_vec(), decompressed));
        }

        Ok(results)
    }

    async fn batch_write_bytes(&self, cf: &str, pairs: &[(&[u8], &[u8])]) -> Result<()> {
        let cf_handle = self.db.cf_handle(cf).context("Column family not found")?;
        let mut batch = WriteBatch::default();

        for (key, value) in pairs {
            let compressed = Self::compress_value(value)?;
            batch.put_cf(cf_handle, key, compressed);
        }

        let mut write_opts = WriteOptions::default();
        write_opts.set_sync(true);

        let _lock = self.write_lock.lock().await;
        self.db.write_opt(batch, &write_opts)?;
        Ok(())
    }

    async fn create_snapshot(&self, path: &Path) -> Result<()> {
        let checkpoint = rocksdb::checkpoint::Checkpoint::new(&self.db)?;
        checkpoint.create_checkpoint(path)?;
        Ok(())
    }

    async fn restore_from_snapshot(&self, path: &Path) -> Result<()> {
        // TODO: スナップショットからの復元を実装
        Ok(())
    }
}

#[async_trait]
impl TypedStorage for RocksDBStorage {
    async fn put<K, V>(&self, cf: &str, key: K, value: &V) -> Result<()>
    where
        K: AsRef<[u8]> + Send + Sync,
        V: serde::Serialize + Send + Sync,
    {
        let value_bytes = bincode::serialize(value)?;
        self.put_bytes(cf, key.as_ref(), &value_bytes).await
    }

    async fn get<K, V>(&self, cf: &str, key: K) -> Result<Option<V>>
    where
        K: AsRef<[u8]> + Send + Sync,
        V: serde::de::DeserializeOwned + Send + Sync,
    {
        if let Some(bytes) = self.get_bytes(cf, key.as_ref()).await? {
            Ok(Some(bincode::deserialize(&bytes)?))
        } else {
            Ok(None)
        }
    }

    async fn delete<K>(&self, cf: &str, key: K) -> Result<()>
    where
        K: AsRef<[u8]> + Send + Sync,
    {
        self.delete_bytes(cf, key.as_ref()).await
    }

    async fn scan_prefix<K, V>(&self, cf: &str, prefix: K) -> Result<Vec<(Vec<u8>, V)>>
    where
        K: AsRef<[u8]> + Send + Sync,
        V: serde::de::DeserializeOwned + Send + Sync,
    {
        let pairs = self.scan_prefix_bytes(cf, prefix.as_ref()).await?;
        let mut results = Vec::with_capacity(pairs.len());
        
        for (key, value) in pairs {
            results.push((key, bincode::deserialize(&value)?));
        }
        
        Ok(results)
    }

    async fn batch_write<K, V>(&self, cf: &str, pairs: &[(K, &V)]) -> Result<()>
    where
        K: AsRef<[u8]> + Send + Sync,
        V: serde::Serialize + Send + Sync,
    {
        let mut bytes_pairs = Vec::with_capacity(pairs.len());
        let mut value_bytes_vec = Vec::with_capacity(pairs.len());

        for (key, value) in pairs {
            let value_bytes = bincode::serialize(value)?;
            value_bytes_vec.push(value_bytes);
            bytes_pairs.push((key.as_ref(), value_bytes_vec.last().unwrap().as_slice()));
        }

        self.batch_write_bytes(cf, &bytes_pairs).await
    }
}