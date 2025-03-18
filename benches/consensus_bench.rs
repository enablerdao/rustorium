use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustorium::common::types::{Block, BlockHeader, Transaction};
use rustorium::common::utils;
use std::time::Duration;
use rand::Rng;

fn bench_merkle_root_calculation(c: &mut Criterion) {
    let mut group = c.benchmark_group("merkle_root");
    group.measurement_time(Duration::from_secs(10));
    
    // Generate random hashes
    let mut hashes = Vec::new();
    for _ in 0..1000 {
        let mut hash = [0u8; 32];
        rand::thread_rng().fill(&mut hash);
        hashes.push(hash);
    }
    
    group.bench_function("calculate_1000_hashes", |b| {
        b.iter(|| {
            black_box(utils::calculate_merkle_root(&hashes));
        })
    });
    
    group.finish();
}

criterion_group!(benches, bench_merkle_root_calculation);
criterion_main!(benches);