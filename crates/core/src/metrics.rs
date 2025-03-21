//! GQT Core - メトリクス定義

use prometheus::{Counter, Gauge, Histogram, IntCounter, IntGauge, Registry};
use std::sync::Arc;

/// メトリクスレジストリ
#[derive(Clone)]
pub struct MetricsRegistry {
    /// Prometheusレジストリ
    registry: Arc<Registry>,
    /// トランザクション数
    pub transactions: IntCounter,
    /// ブロック数
    pub blocks: IntCounter,
    /// ピア数
    pub peers: IntGauge,
    /// トランザクション処理時間
    pub transaction_time: Histogram,
    /// ブロック処理時間
    pub block_time: Histogram,
    /// メモリ使用量
    pub memory_usage: Gauge,
    /// CPU使用率
    pub cpu_usage: Gauge,
}

impl MetricsRegistry {
    /// 新しいメトリクスレジストリを作成
    pub fn new() -> anyhow::Result<Self> {
        let registry = Registry::new();

        let transactions = IntCounter::new("gqt_transactions_total", "Total number of transactions")?;
        let blocks = IntCounter::new("gqt_blocks_total", "Total number of blocks")?;
        let peers = IntGauge::new("gqt_peers", "Number of connected peers")?;
        let transaction_time = Histogram::new("gqt_transaction_time_seconds", "Transaction processing time")?;
        let block_time = Histogram::new("gqt_block_time_seconds", "Block processing time")?;
        let memory_usage = Gauge::new("gqt_memory_usage_bytes", "Memory usage in bytes")?;
        let cpu_usage = Gauge::new("gqt_cpu_usage_percent", "CPU usage percentage")?;

        registry.register(Box::new(transactions.clone()))?;
        registry.register(Box::new(blocks.clone()))?;
        registry.register(Box::new(peers.clone()))?;
        registry.register(Box::new(transaction_time.clone()))?;
        registry.register(Box::new(block_time.clone()))?;
        registry.register(Box::new(memory_usage.clone()))?;
        registry.register(Box::new(cpu_usage.clone()))?;

        Ok(Self {
            registry: Arc::new(registry),
            transactions,
            blocks,
            peers,
            transaction_time,
            block_time,
            memory_usage,
            cpu_usage,
        })
    }

    /// メトリクスを収集
    pub fn gather(&self) -> Vec<prometheus::proto::MetricFamily> {
        self.registry.gather()
    }
}
