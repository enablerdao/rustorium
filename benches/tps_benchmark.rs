use criterion::{criterion_group, criterion_main, Criterion};
use rustorium::core::{
    transaction::TransactionManager,
    consensus::ConsensusManager,
    cache::CacheManager,
    storage::redb_storage::RedbStorage,
};
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use std::time::{Duration, Instant};
use futures::future::join_all;

async fn run_tps_test(
    num_transactions: usize,
    concurrent_limit: usize,
    tx_size_bytes: usize,
) -> (f64, Duration) {
    let tx_manager = Arc::new(TransactionManager::new(Default::default()));
    let consensus = Arc::new(ConsensusManager::new(Default::default()));
    let cache = Arc::new(Mutex::new(CacheManager::new(Default::default())));
    let storage = Arc::new(RedbStorage::new("/tmp/bench_db").unwrap());
    
    // 同時実行数を制限するセマフォ
    let semaphore = Arc::new(Semaphore::new(concurrent_limit));
    
    let start = Instant::now();
    
    // トランザクションを生成
    let mut handles = Vec::with_capacity(num_transactions);
    
    for i in 0..num_transactions {
        let tx_manager = tx_manager.clone();
        let consensus = consensus.clone();
        let cache = cache.clone();
        let storage = storage.clone();
        let semaphore = semaphore.clone();
        
        let handle = tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            
            // トランザクションを作成
            let tx = Transaction {
                id: format!("tx-{}", i),
                data: vec![0u8; tx_size_bytes],
                client_info: ClientInfo {
                    location: GeoLocation {
                        latitude: 35.6762,
                        longitude: 139.6503,
                        region: "asia-northeast".to_string(),
                    },
                    client_id: format!("client-{}", i % 100),
                },
            };
            
            // トランザクションを処理
            let receipt = tx_manager.submit_transaction(tx.clone()).await?;
            
            // コンセンサスを取得
            let consensus_result = consensus.process_transaction(tx.clone()).await?;
            
            // キャッシュに保存
            let mut cache = cache.lock().await;
            cache.set(tx.id.as_bytes(), &receipt.to_bytes()).await?;
            
            // ストレージに保存
            storage.write_with_proof(tx.id.as_bytes(), &consensus_result.to_bytes()).await?;
            
            Ok::<_, anyhow::Error>(())
        });
        
        handles.push(handle);
    }
    
    // すべてのトランザクションの完了を待機
    join_all(handles).await;
    
    let duration = start.elapsed();
    let tps = num_transactions as f64 / duration.as_secs_f64();
    
    (tps, duration)
}

fn tps_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("tps");
    group.sample_size(10);  // TPSテストは時間がかかるので少なめに
    group.measurement_time(Duration::from_secs(30));
    
    // シナリオ1: 低負荷（1KB取引、100同時実行）
    group.bench_function("low_load", |b| {
        b.to_async(&rt).iter(|| async {
            run_tps_test(10_000, 100, 1024).await
        });
    });
    
    // シナリオ2: 中負荷（1KB取引、500同時実行）
    group.bench_function("medium_load", |b| {
        b.to_async(&rt).iter(|| async {
            run_tps_test(50_000, 500, 1024).await
        });
    });
    
    // シナリオ3: 高負荷（1KB取引、1000同時実行）
    group.bench_function("high_load", |b| {
        b.to_async(&rt).iter(|| async {
            run_tps_test(100_000, 1000, 1024).await
        });
    });
    
    // シナリオ4: 大容量取引（10KB取引、500同時実行）
    group.bench_function("large_tx", |b| {
        b.to_async(&rt).iter(|| async {
            run_tps_test(50_000, 500, 10 * 1024).await
        });
    });
    
    // シナリオ5: 極限テスト（1KB取引、2000同時実行）
    group.bench_function("extreme_load", |b| {
        b.to_async(&rt).iter(|| async {
            run_tps_test(200_000, 2000, 1024).await
        });
    });
    
    group.finish();
}

criterion_group!(benches, tps_benchmark);
criterion_main!(benches);