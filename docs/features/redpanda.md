# 🐼 Redpanda統合

## 📖 概要

[Redpanda]は、超低遅延のストリーミング処理プラットフォームです。Rustoriumでは、トランザクション処理レイヤーとして使用し、地理的に分散したノード間での高速なメッセージング基盤を提供します。

## 🌟 主な特徴

### 1️⃣ 超低遅延
- **XRP（eXtreme Read Processing）**: マイクロ秒レベルの読み取り
- **ゼロコピー**: メモリ効率の最大化
- **SMP（Symmetric Multi-Processing）**: 並列処理の最適化

### 2️⃣ 高可用性
- **Raft合意**: 分散合意の保証
- **自動フェイルオーバー**: 無停止運用
- **地理的レプリケーション**: グローバル分散

### 3️⃣ 高性能
- **C++実装**: ネイティブパフォーマンス
- **DPDK**: カーネルバイパス
- **SPDK**: ストレージ最適化

## 💻 実装例

### 1️⃣ プロデューサー実装
```rust
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;

pub struct TransactionProducer {
    producer: FutureProducer,
    config: ProducerConfig,
}

impl TransactionProducer {
    pub fn new(config: ProducerConfig) -> Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &config.brokers.join(","))
            .set("message.timeout.ms", "5000")
            .set("compression.type", "lz4")
            .set("linger.ms", "5")
            .set("batch.size", "1048576")
            .create()?;

        Ok(Self { producer, config })
    }

    pub async fn send_transaction(&self, tx: Transaction) -> Result<TxReceipt> {
        let topic = self.determine_topic(&tx);
        let payload = tx.serialize()?;
        
        let record = FutureRecord::to(&topic)
            .key(&tx.id)
            .payload(&payload);

        let (partition, offset) = self.producer
            .send(record, Duration::from_secs(5))
            .await?;

        Ok(TxReceipt {
            id: tx.id,
            partition,
            offset,
            timestamp: SystemTime::now(),
        })
    }

    fn determine_topic(&self, tx: &Transaction) -> String {
        format!("transactions-{}", tx.shard_id())
    }
}
```

### 2️⃣ コンシューマー実装
```rust
use rdkafka::consumer::{StreamConsumer, Consumer};
use rdkafka::message::Message;

pub struct TransactionConsumer {
    consumer: StreamConsumer,
    config: ConsumerConfig,
}

impl TransactionConsumer {
    pub fn new(config: ConsumerConfig) -> Result<Self> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", &config.brokers.join(","))
            .set("group.id", &config.group_id)
            .set("enable.auto.commit", "false")
            .set("auto.offset.reset", "earliest")
            .set("fetch.min.bytes", "1")
            .set("fetch.max.wait.ms", "50")
            .create()?;

        consumer.subscribe(&[&config.topic])?;

        Ok(Self { consumer, config })
    }

    pub async fn process_transactions(&self) -> Result<()> {
        let mut stream = self.consumer.stream();

        while let Some(message) = stream.next().await {
            match message {
                Ok(msg) => {
                    let payload = msg.payload()
                        .ok_or_else(|| anyhow!("Empty payload"))?;
                    
                    let tx: Transaction = bincode::deserialize(payload)?;
                    self.process_transaction(tx).await?;
                    
                    self.consumer.commit_message(&msg, CommitMode::Async)?;
                }
                Err(e) => error!("Error processing message: {}", e),
            }
        }

        Ok(())
    }

    async fn process_transaction(&self, tx: Transaction) -> Result<()> {
        // トランザクションの処理ロジック
        Ok(())
    }
}
```

### 3️⃣ シャーディング実装
```rust
pub struct ShardManager {
    shards: HashMap<ShardId, ShardInfo>,
    config: ShardConfig,
}

impl ShardManager {
    pub fn determine_shard(&self, tx: &Transaction) -> ShardId {
        let location = tx.client_location();
        let mut min_distance = f64::MAX;
        let mut selected_shard = None;

        for (shard_id, info) in &self.shards {
            let distance = self.calculate_distance(&location, &info.location);
            if distance < min_distance {
                min_distance = distance;
                selected_shard = Some(shard_id);
            }
        }

        selected_shard.unwrap_or(&self.config.default_shard).clone()
    }

    fn calculate_distance(&self, a: &GeoLocation, b: &GeoLocation) -> f64 {
        // ハーバーサイン公式による距離計算
        let lat1 = a.latitude.to_radians();
        let lat2 = b.latitude.to_radians();
        let dlat = (b.latitude - a.latitude).to_radians();
        let dlon = (b.longitude - a.longitude).to_radians();

        let a = (dlat/2.0).sin().powi(2) + 
                lat1.cos() * lat2.cos() * (dlon/2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().asin();
        
        6371.0 * c // 地球の半径（km）
    }
}
```

## 📊 パフォーマンス特性

### 1️⃣ レイテンシ
| シナリオ | レイテンシ |
|---------|------------|
| プロデュース | < 1ms |
| コンシューム | < 5ms |
| エンドツーエンド | < 10ms |

### 2️⃣ スループット
| シナリオ | スループット |
|---------|-------------|
| 単一パーティション | 1M msg/s |
| 複数パーティション | 10M msg/s |
| 複数トピック | 100M msg/s |

### 3️⃣ リソース使用
| メトリック | 値 |
|-----------|-----|
| メモリ/パーティション | ~100MB |
| CPU/コア | 60-80% |
| ディスクIO | 1GB/s |

## 🔧 設定オプション

### 1️⃣ プロデューサー設定
```rust
#[derive(Debug, Clone)]
pub struct ProducerConfig {
    pub brokers: Vec<String>,
    pub batch_size: usize,
    pub linger_ms: u64,
    pub compression_type: CompressionType,
    pub acks: Acks,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone)]
pub enum CompressionType {
    None,
    Gzip,
    Snappy,
    Lz4,
    Zstd,
}

#[derive(Debug, Clone)]
pub enum Acks {
    None,
    Leader,
    All,
}
```

### 2️⃣ コンシューマー設定
```rust
#[derive(Debug, Clone)]
pub struct ConsumerConfig {
    pub brokers: Vec<String>,
    pub group_id: String,
    pub topics: Vec<String>,
    pub fetch_min_bytes: u32,
    pub fetch_max_wait_ms: u32,
    pub max_poll_records: u32,
    pub isolation_level: IsolationLevel,
}

#[derive(Debug, Clone)]
pub enum IsolationLevel {
    ReadUncommitted,
    ReadCommitted,
}
```

## 🔍 モニタリング

### 1️⃣ メトリクス
```rust
#[derive(Debug)]
pub struct RedpandaMetrics {
    // プロデューサーメトリクス
    messages_sent: Counter,
    send_errors: Counter,
    batch_size_avg: Gauge,
    compression_ratio: Gauge,

    // コンシューマーメトリクス
    messages_received: Counter,
    processing_errors: Counter,
    consumer_lag: Gauge,
    processing_time: Histogram,

    // パーティションメトリクス
    partition_count: Gauge,
    leader_count: Gauge,
    replica_count: Gauge,
}

impl RedpandaMetrics {
    pub fn record_send(&self, size: usize, duration: Duration) {
        self.messages_sent.inc();
        self.batch_size_avg.set(size as f64);
        self.processing_time.observe(duration.as_secs_f64());
    }
}
```

### 2️⃣ トレーシング
```rust
#[tracing::instrument(skip(self, tx))]
pub async fn send_transaction(&self, tx: Transaction) -> Result<TxReceipt> {
    let start = Instant::now();
    
    let shard = self.determine_shard(&tx);
    tracing::debug!("Selected shard: {}", shard);

    let result = self.inner_send_transaction(tx, shard).await;
    let duration = start.elapsed();

    tracing::info!(
        "Transaction processed in {:?} on shard {}",
        duration,
        shard
    );

    result
}
```

## 📚 関連ドキュメント

- [Redpanda公式ドキュメント](https://docs.redpanda.com/)
- [rdkafkaクレート](https://docs.rs/rdkafka/)
- [トランザクション処理](../architecture/transactions.md)
- [シャーディング](../architecture/sharding.md)
- [パフォーマンスチューニング](../guides/performance.md)

[Redpanda]: https://redpanda.com/
