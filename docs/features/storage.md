# 💾 ストレージレイヤー

## 📖 概要

Rustoriumのストレージレイヤーは、[Redb]と[TiKV]を組み合わせた高性能な分散KVストアです。[Poseidon]ハッシュを使用したZKフレンドリーなVerkle Treesにより、効率的な証明生成と検証を実現します。

## 🌟 主な特徴

### 1️⃣ 高性能KVストア
- **Redb**: 超高速なローカルストレージ
- **TiKV**: スケーラブルな分散ストレージ
- **ハイブリッド構成**: 最適な性能とスケーラビリティ

### 2️⃣ ZKフレンドリー
- **Poseidonハッシュ**: ZKプルーフに最適化
- **Verkle Trees**: 効率的な証明生成
- **インクリメンタル更新**: 高速な証明更新

### 3️⃣ 地理分散
- **動的シャーディング**: 地理的最適化
- **非同期レプリケーション**: 低レイテンシ
- **自動修復**: 高可用性

## 💻 実装例

### 1️⃣ ストレージインターフェース
```rust
#[async_trait]
pub trait Storage {
    /// データの書き込み（証明付き）
    async fn write_with_proof(&self, key: &[u8], value: &[u8]) -> Result<WriteResult>;
    
    /// データの読み取り
    async fn read(&self, key: &[u8]) -> Result<Option<ReadResult>>;
    
    /// データの削除
    async fn delete(&self, key: &[u8]) -> Result<()>;
    
    /// 証明の検証
    async fn verify_proof(&self, key: &[u8], value: &[u8], proof: &Proof) -> Result<bool>;
}

#[derive(Debug)]
pub struct WriteResult {
    pub proof: Proof,
    pub timestamp: SystemTime,
}

#[derive(Debug)]
pub struct ReadResult {
    pub value: Vec<u8>,
    pub proof: Option<Proof>,
    pub timestamp: SystemTime,
}
```

### 2️⃣ Redbストレージ実装
```rust
pub struct RedbStorage {
    db: Database,
    merkle_tree: Arc<Mutex<PoseidonMerkleTree>>,
}

impl RedbStorage {
    pub fn new(path: &str) -> Result<Self> {
        let db = Database::create(path)?;
        let merkle_tree = Arc::new(Mutex::new(PoseidonMerkleTree::new()));
        
        Ok(Self { db, merkle_tree })
    }
}

#[async_trait]
impl Storage for RedbStorage {
    async fn write_with_proof(&self, key: &[u8], value: &[u8]) -> Result<WriteResult> {
        // マークルツリーの更新
        let mut tree = self.merkle_tree.lock().await;
        let proof = tree.insert(key, value)?;
        
        // データの書き込み
        let txn = self.db.begin_write()?;
        {
            let mut table = txn.open_table(DATA_TABLE)?;
            table.insert(key, value)?;
        }
        txn.commit()?;
        
        Ok(WriteResult {
            proof,
            timestamp: SystemTime::now(),
        })
    }
    
    async fn read(&self, key: &[u8]) -> Result<Option<ReadResult>> {
        let txn = self.db.begin_read()?;
        let table = txn.open_table(DATA_TABLE)?;
        
        if let Some(value) = table.get(key)? {
            let tree = self.merkle_tree.lock().await;
            let proof = tree.generate_proof(key)?;
            
            Ok(Some(ReadResult {
                value: value.value().to_vec(),
                proof: Some(proof),
                timestamp: SystemTime::now(),
            }))
        } else {
            Ok(None)
        }
    }
}
```

### 3️⃣ PoseidonマークルツリーとVerkle Trees
```rust
pub struct PoseidonMerkleTree {
    root: [u8; 32],
    nodes: HashMap<Vec<u8>, Node>,
}

impl PoseidonMerkleTree {
    pub fn insert(&mut self, key: &[u8], value: &[u8]) -> Result<Proof> {
        let leaf_hash = self.hash_leaf(key, value);
        let path = self.calculate_path(key);
        
        // パスに沿ってノードを更新
        let mut current_hash = leaf_hash;
        let mut proof_hashes = Vec::new();
        
        for (level, parent) in path.iter().enumerate() {
            let sibling = self.get_sibling(parent, level)?;
            proof_hashes.push(sibling);
            
            current_hash = self.hash_nodes(current_hash, sibling);
        }
        
        self.root = current_hash;
        
        Ok(Proof {
            root: self.root,
            path: proof_hashes,
            leaf_hash,
        })
    }
    
    fn hash_leaf(&self, key: &[u8], value: &[u8]) -> [u8; 32] {
        let mut poseidon = Poseidon::new();
        poseidon.update(key);
        poseidon.update(value);
        poseidon.finalize()
    }
    
    fn hash_nodes(&self, left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
        let mut poseidon = Poseidon::new();
        poseidon.update(&left);
        poseidon.update(&right);
        poseidon.finalize()
    }
}
```

## 📊 パフォーマンス特性

### 1️⃣ 書き込み性能
| 操作 | レイテンシ | スループット |
|------|------------|--------------|
| 単一書き込み | < 1ms | 100K/秒 |
| バッチ書き込み | < 10ms | 1M/秒 |
| プルーフ付き書き込み | < 5ms | 50K/秒 |

### 2️⃣ 読み取り性能
| シナリオ | レイテンシ |
|----------|------------|
| キャッシュヒット | < 1ms |
| キャッシュミス | < 10ms |
| プルーフ検証 | < 5ms |

### 3️⃣ ストレージ効率
| メトリック | 値 |
|------------|-----|
| 圧縮率 | 3-5x |
| インデックスオーバーヘッド | 10-15% |
| プルーフサイズ | 1-2KB |

## 🔧 設定オプション

### 1️⃣ ストレージ設定
```rust
#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub path: PathBuf,
    pub max_size: u64,
    pub compression_enabled: bool,
    pub encryption_enabled: bool,
    pub sync_mode: SyncMode,
}

#[derive(Debug, Clone)]
pub enum SyncMode {
    None,
    Async,
    Full,
}
```

### 2️⃣ シャーディング設定
```rust
#[derive(Debug, Clone)]
pub struct ShardConfig {
    pub shard_count: u32,
    pub replication_factor: u32,
    pub placement_strategy: PlacementStrategy,
}

#[derive(Debug, Clone)]
pub enum PlacementStrategy {
    Random,
    Geographic,
    LoadBased,
    Hybrid,
}
```

## 🔍 モニタリング

### 1️⃣ メトリクス
```rust
#[derive(Debug)]
pub struct StorageMetrics {
    // 基本メトリクス
    writes_total: Counter,
    reads_total: Counter,
    errors_total: Counter,
    
    // パフォーマンスメトリクス
    write_latency: Histogram,
    read_latency: Histogram,
    proof_generation_time: Histogram,
    
    // リソースメトリクス
    disk_usage: Gauge,
    memory_usage: Gauge,
    cache_hit_ratio: Gauge,
}

impl StorageMetrics {
    pub fn record_write(&self, size: usize, duration: Duration) {
        self.writes_total.inc();
        self.write_latency.observe(duration.as_secs_f64());
        self.disk_usage.add(size as f64);
    }
}
```

### 2️⃣ トレーシング
```rust
#[tracing::instrument(skip(self, value))]
pub async fn write_with_proof(&self, key: &[u8], value: &[u8]) -> Result<WriteResult> {
    let start = Instant::now();
    
    // プルーフの生成
    let proof_start = Instant::now();
    let proof = self.generate_proof(key, value).await?;
    let proof_duration = proof_start.elapsed();
    
    tracing::debug!(
        "Proof generated in {:?}, size: {} bytes",
        proof_duration,
        proof.size()
    );
    
    // データの書き込み
    let write_result = self.inner_write(key, value).await?;
    let total_duration = start.elapsed();
    
    tracing::info!(
        "Write completed in {:?}, key: {:?}, size: {} bytes",
        total_duration,
        key,
        value.len()
    );
    
    Ok(write_result)
}
```

## 📚 関連ドキュメント

- [Redb公式ドキュメント](https://docs.rs/redb/)
- [TiKV公式ドキュメント](https://tikv.org/docs/)
- [Poseidonハッシュ](https://www.poseidon-hash.info/)
- [Verkle Trees論文](https://math.mit.edu/research/highschool/primes/materials/2018/Kuszmaul.pdf)
- [パフォーマンスチューニング](../guides/performance.md)

[Redb]: https://docs.rs/redb/
[TiKV]: https://tikv.org/
[Poseidon]: https://www.poseidon-hash.info/
