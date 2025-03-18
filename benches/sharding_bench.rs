use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustorium::common::types::{Address, Transaction, TransactionId, VmType};
use rustorium::common::utils;
use rustorium::sharding::ring::ShardRing;
use std::time::Duration;

fn bench_shard_assignment(c: &mut Criterion) {
    let mut group = c.benchmark_group("shard_assignment");
    group.measurement_time(Duration::from_secs(10));
    
    let shard_ring = ShardRing::new(16, 100);
    
    // Generate random transactions
    let mut transactions = Vec::new();
    for _ in 0..1000 {
        let tx_id = utils::random_transaction_id();
        transactions.push(tx_id);
    }
    
    group.bench_function("assign_1000_transactions", |b| {
        b.iter(|| {
            for tx_id in &transactions {
                black_box(shard_ring.get_shard_for_transaction(tx_id));
            }
        })
    });
    
    group.finish();
}

fn bench_shard_rebalancing(c: &mut Criterion) {
    let mut group = c.benchmark_group("shard_rebalancing");
    group.measurement_time(Duration::from_secs(10));
    
    group.bench_function("add_remove_shard", |b| {
        b.iter(|| {
            let mut ring = ShardRing::new(8, 100);
            
            // Add a shard
            ring.add_shard(rustorium::common::types::ShardId(8), 100);
            
            // Remove a shard
            ring.remove_shard(rustorium::common::types::ShardId(0));
            
            // Update weights
            for i in 1..8 {
                ring.update_shard_weight(rustorium::common::types::ShardId(i), 50 + i * 10);
            }
            
            black_box(ring)
        })
    });
    
    group.finish();
}

criterion_group!(benches, bench_shard_assignment, bench_shard_rebalancing);
criterion_main!(benches);