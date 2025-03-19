use super::{StorageEngine, CF_METADATA, CF_SHARD_STATE, CF_TRANSACTIONS};
use anyhow::{Context, Result};
use async_trait::async_trait;
use bytes::Bytes;
use rocksdb::{
    ColumnFamilyDescriptor, DBCompressionType, Options, DB, WriteBatch, WriteOptions,
};
use serde::{de::DeserializeOwned, Serialize};
use snap::raw::{Decoder, Encoder};
use std::{path::Path, sync::Arc};
use tokio::sync::Mutex;

/// RocksDBベースのストレージエンジン
pub struct RocksDBStorage {
    db: Arc<DB>,
    write_lock: Arc<Mutex<()>>,
}

impl RocksDBStorage {
    /// 新しいストレージエンジンを作成
    pub fn new(path: &Path) -> Result<Self> {
        // オプションの設定
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_compression_type(DBCompressionType::Lz4);
        opts.set_use_fsync(true);
        opts.set_keep_log_file_num(10);
        opts.set_max_total_wal_size(536_870_912); // 512MB
        opts.set_write_buffer_size(67_108_864); // 64MB
        opts.set_max_write_buffer_number(3);
        opts.set_target_file_size_base(67_108_864); // 64MB
        opts.set_level_zero_file_num_compaction_trigger(8);
        opts.set_level_zero_slowdown_writes_trigger(17);
        opts.set_level_zero_stop_writes_trigger(24);
        opts.set_max_bytes_for_level_base(536_870_912); // 512MB
        opts.set_max_bytes_for_level_multiplier(8.0);

        // カラムファミリーの設定
        let cf_opts = Options::default();
        let cf_descriptors = vec![
            ColumnFamilyDescriptor::new(CF_SHARD_STATE, cf_opts.clone()),
            ColumnFamilyDescriptor::new(CF_TRANSACTIONS, cf_opts.clone()),
            ColumnFamilyDescriptor::new(CF_METADATA, cf_opts),
        ];

        // DBを開く
        let db = DB::open_cf_descriptors(&opts, path, cf_descriptors)?;

        Ok(Self {
            db: Arc::new(db),
            write_lock: Arc::new(Mutex::new(())),
        })
    }

    /// 値を圧縮
    fn compress_value<V: Serialize>(value: &V) -> Result<Vec<u8>> {
        let serialized = bincode::serialize(value)?;
        let mut encoder = Encoder::new();
        Ok(encoder.compress_vec(&serialized)?)
    }

    /// 値を解凍
    fn decompress_value<V: DeserializeOwned>(bytes: &[u8]) -> Result<V> {
        let mut decoder = Decoder::new();
        let decompressed = decoder.decompress_vec(bytes)?;
        Ok(bincode::deserialize(&decompressed)?)
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
        let prefix_bytes = Bytes::copy_from_slice(prefix);
        let mut results = Vec::new();

        let iter = self.db.prefix_iterator_cf(cf_handle, prefix_bytes);
        for item in iter {
            let (key, value) = item?;
            let decompressed = Self::decompress_value(&value)?;
            results.push((key.to_vec(), decompressed));
        }

        Ok(results)
    }

    async fn batch_write_bytes(&self, cf: &str, pairs: Vec<(&[u8], &[u8])>) -> Result<()> {
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
        // DBを閉じる
        drop(self.db.clone());

        // スナップショットから復元
        let opts = Options::default();
        let cf_names = vec![CF_SHARD_STATE, CF_TRANSACTIONS, CF_METADATA];
        let cf_opts: Vec<_> = cf_names.iter().map(|_| Options::default()).collect();
        
        DB::open_cf_as_secondary(&opts, path, self.db.path(), cf_names.as_slice(), cf_opts.as_slice())?;
        
        Ok(())
    }
}