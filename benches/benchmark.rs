use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustorium::core::{
    transaction::TransactionManager,
    consensus::ConsensusManager,
    cache::CacheManager,
    storage::redb_storage::RedbStorage,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Duration;

fn transaction_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    // トランザクション処理のベンチマーク
    let mut group = c.benchmark_group("transactions");
    group.sample_size(100);
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("single_transaction", |b| {
        b.to_async(&rt).iter(|| async {
            let tx_manager = TransactionManager::new(Default::default());
            let tx = black_box(create_test_transaction());
            tx_manager.submit_transaction(tx).await.unwrap()
        });
    });

    group.bench_function("batch_transactions_1000", |b| {
        b.to_async(&rt).iter(|| async {
            let tx_manager = TransactionManager::new(Default::default());
            let txs = black_box((0..1000).map(|_| create_test_transaction()).collect::<Vec<_>>());
            for tx in txs {
                tx_manager.submit_transaction(tx).await.unwrap();
            }
        });
    });

    group.finish();
}

fn consensus_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("consensus");
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(15));

    group.bench_function("consensus_proposal", |b| {
        b.to_async(&rt).iter(|| async {
            let consensus_manager = ConsensusManager::new(Default::default());
            let proposal = black_box(create_test_proposal());
            consensus_manager.process_transaction(proposal).await.unwrap()
        });
    });

    group.finish();
}

fn cache_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("cache");
    group.sample_size(1000);
    group.measurement_time(Duration::from_secs(5));

    group.bench_function("cache_hit", |b| {
        b.to_async(&rt).iter(|| async {
            let cache_manager = CacheManager::new(Default::default());
            let key = black_box(b"test_key");
            let location = black_box(create_test_location());
            cache_manager.get(key, &location).await.unwrap()
        });
    });

    group.bench_function("cache_miss", |b| {
        b.to_async(&rt).iter(|| async {
            let cache_manager = CacheManager::new(Default::default());
            let key = black_box(generate_random_key());
            let location = black_box(create_test_location());
            cache_manager.get(key.as_bytes(), &location).await.unwrap()
        });
    });

    group.finish();
}

fn storage_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("storage");
    group.sample_size(100);
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("storage_write", |b| {
        b.to_async(&rt).iter(|| async {
            let storage = Arc::new(RedbStorage::new("/tmp/bench_db").unwrap());
            let key = black_box(generate_random_key());
            let value = black_box(generate_random_value());
            storage.write_with_proof(key.as_bytes(), value.as_bytes()).await.unwrap()
        });
    });

    group.bench_function("storage_read", |b| {
        b.to_async(&rt).iter(|| async {
            let storage = Arc::new(RedbStorage::new("/tmp/bench_db").unwrap());
            let key = black_box(b"test_key");
            storage.get(key).await.unwrap()
        });
    });

    group.finish();
}

// ヘルパー関数
fn create_test_transaction() -> Transaction {
    Transaction {
        id: uuid::Uuid::new_v4().to_string(),
        data: vec![0; 1024],  // 1KB of data
        client_info: ClientInfo {
            location: create_test_location(),
            client_id: "test_client".to_string(),
        },
    }
}

fn create_test_location() -> GeoLocation {
    GeoLocation {
        latitude: 35.6762,   // Tokyo
        longitude: 139.6503,
        region: "asia-northeast".to_string(),
    }
}

fn create_test_proposal() -> Transaction {
    create_test_transaction()
}

fn generate_random_key() -> String {
    uuid::Uuid::new_v4().to_string()
}

fn generate_random_value() -> String {
    let mut value = String::with_capacity(1024);
    for _ in 0..1024 {
        value.push(rand::random::<char>());
    }
    value
}

criterion_group!(
    benches,
    transaction_benchmark,
    consensus_benchmark,
    cache_benchmark,
    storage_benchmark
);
criterion_main!(benches);
