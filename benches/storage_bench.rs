use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustorium::common::types::{Account, Address, Block, BlockHeader, Transaction, TransactionId};
use rustorium::common::utils;
use rustorium::storage::db::Database;
use std::time::Duration;
use tempfile::tempdir;

fn bench_block_storage(c: &mut Criterion) {
    let mut group = c.benchmark_group("block_storage");
    group.measurement_time(Duration::from_secs(10));
    
    // Create temporary directory for database
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path();
    
    // Open database
    let db = Database::open(db_path).unwrap();
    
    // Create test blocks
    let mut blocks = Vec::new();
    for i in 0..100 {
        let header = BlockHeader {
            height: i,
            prev_hash: [i as u8; 32],
            merkle_root: [0; 32],
            timestamp: utils::current_time_sec(),
            validator: Address([0; 20]),
            signature: None,
        };
        
        let block = Block {
            header,
            transactions: vec![],
        };
        
        blocks.push(block);
    }
    
    group.bench_function("write_100_blocks", |b| {
        b.iter(|| {
            for block in &blocks {
                black_box(db.put_block(block)).unwrap();
            }
        })
    });
    
    group.bench_function("read_100_blocks", |b| {
        b.iter(|| {
            for i in 0..100 {
                black_box(db.get_block(i)).unwrap();
            }
        })
    });
    
    group.finish();
}

fn bench_transaction_storage(c: &mut Criterion) {
    let mut group = c.benchmark_group("transaction_storage");
    group.measurement_time(Duration::from_secs(10));
    
    // Create temporary directory for database
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path();
    
    // Open database
    let db = Database::open(db_path).unwrap();
    
    // Create test transactions
    let mut transactions = Vec::new();
    for i in 0..100 {
        let sender = Address([i as u8; 20]);
        let recipient = Address([(i + 1) as u8; 20]);
        
        let tx = Transaction::new(
            sender,
            recipient,
            1000,
            10,
            i,
            vec![],
            rustorium::common::types::VmType::Evm,
        );
        
        transactions.push(tx);
    }
    
    group.bench_function("write_100_transactions", |b| {
        b.iter(|| {
            for tx in &transactions {
                black_box(db.put_transaction(tx)).unwrap();
            }
        })
    });
    
    group.bench_function("read_100_transactions", |b| {
        b.iter(|| {
            for tx in &transactions {
                black_box(db.get_transaction(&tx.id)).unwrap();
            }
        })
    });
    
    group.finish();
}

criterion_group!(benches, bench_block_storage, bench_transaction_storage);
criterion_main!(benches);