use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use serde_json;

// ブロック構造体
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub validator: String,
    pub difficulty: u32,
    pub size: usize,
    pub gas_used: u64,
    pub gas_limit: u64,
}

impl Block {
    // 新しいブロックを作成
    pub fn new(
        index: u64,
        transactions: Vec<Transaction>,
        previous_hash: String,
        validator: String,
        difficulty: u32,
    ) -> Self {
        let mut block = Block {
            index,
            timestamp: Utc::now(),
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
            validator,
            difficulty,
            size: 0,
            gas_used: 0,
            gas_limit: 10_000_000,
        };

        // ガス使用量を計算
        block.gas_used = block.transactions.iter().map(|tx| tx.gas_used).sum();

        // ブロックサイズを計算（シリアライズしたサイズ）
        let serialized = serde_json::to_string(&block).unwrap_or_default();
        block.size = serialized.len();

        // ハッシュを計算
        block.hash = block.calculate_hash();

        block
    }

    // ブロックのハッシュを計算
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let data = format!(
            "{}{}{}{}{}",
            self.index,
            self.timestamp.timestamp(),
            serde_json::to_string(&self.transactions).unwrap_or_default(),
            self.previous_hash,
            self.nonce
        );
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    // ブロックをマイニング
    pub fn mine_block(&mut self) {
        let target = "0".repeat(self.difficulty as usize);
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Block #{} mined: {}", self.index, self.hash);
    }
}

// トランザクション構造体
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
    pub fee: f64,
    pub data: Option<String>,
    pub nonce: u64,
    pub timestamp: DateTime<Utc>,
    pub status: TransactionStatus,
    pub block_number: Option<u64>,
    pub gas_price: u64,
    pub gas_limit: u64,
    pub gas_used: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

impl Transaction {
    // 新しいトランザクションを作成
    pub fn new(
        sender: String,
        recipient: String,
        amount: f64,
        data: Option<String>,
        gas_price: u64,
        gas_limit: u64,
    ) -> Self {
        let mut gas_used = 21000; // 基本的なトランザクションのガス使用量

        // データフィールドがある場合、追加のガスを使用
        if let Some(data_str) = &data {
            gas_used += data_str.len() as u64 * 68;
        }

        // 実際のガス使用量はガスリミットを超えない
        gas_used = gas_used.min(gas_limit);

        // 手数料を計算（gas_used * gas_price）
        let fee = (gas_used * gas_price) as f64 / 1_000_000_000.0; // GweiからETHに変換

        Transaction {
            id: format!("0x{}", Uuid::new_v4().to_string().replace("-", "")),
            sender,
            recipient,
            amount,
            fee,
            data,
            nonce: 0, // 初期値、後で更新される
            timestamp: Utc::now(),
            status: TransactionStatus::Pending,
            block_number: None,
            gas_price,
            gas_limit,
            gas_used,
        }
    }
}

// アカウント構造体
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    pub address: String,
    pub private_key: Option<String>,
    pub balance: f64,
    pub is_contract: bool,
    pub nonce: u64,
    pub transaction_count: u64,
    pub last_activity: DateTime<Utc>,
    pub tokens: HashMap<String, f64>,
}

impl Account {
    // 新しいアカウントを作成
    pub fn new(address: String, private_key: Option<String>, balance: f64) -> Self {
        Account {
            address,
            private_key,
            balance,
            is_contract: false,
            nonce: 0,
            transaction_count: 0,
            last_activity: Utc::now(),
            tokens: HashMap::new(),
        }
    }
}

// ブロックチェーン構造体
#[derive(Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    pub accounts: HashMap<String, Account>,
}

impl Blockchain {
    // 新しいブロックチェーンを作成
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            pending_transactions: Vec::new(),
            accounts: HashMap::new(),
        };

        // ジェネシスブロックを作成
        blockchain.create_genesis_block();

        // 初期アカウントを作成
        blockchain.create_initial_accounts();

        blockchain
    }

    // ジェネシスブロックを作成
    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(
            0,
            Vec::new(),
            "0".to_string(),
            "0x0000000000000000000000000000000000000000".to_string(),
            4,
        );
        self.chain.push(genesis_block);
        println!("Genesis block created");
    }

    // 初期アカウントを作成（開発用）
    fn create_initial_accounts(&mut self) {
        let initial_accounts = vec![
            (
                "0x1234567890abcdef1234567890abcdef12345678".to_string(),
                Some("0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890".to_string()),
                1_000_000.0,
            ),
            (
                "0xabcdef1234567890abcdef1234567890abcdef12".to_string(),
                Some("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string()),
                500_000.0,
            ),
            (
                "0x9876543210fedcba9876543210fedcba98765432".to_string(),
                Some("0xfedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210".to_string()),
                750_000.0,
            ),
        ];

        for (address, private_key, balance) in initial_accounts {
            let account = Account::new(address.clone(), private_key, balance);
            self.accounts.insert(address, account);
        }
    }

    // 最新のブロックを取得
    pub fn get_latest_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    // トランザクションを追加
    pub fn add_transaction(
        &mut self,
        sender: String,
        recipient: String,
        amount: f64,
        data: Option<String>,
        gas_price: u64,
        gas_limit: u64,
    ) -> Result<String, String> {
        // 送信者アカウントの存在確認
        if !self.accounts.contains_key(&sender) {
            return Err(format!("Sender account {} does not exist", sender));
        }

        // 受信者アカウントの存在確認（存在しない場合は作成）
        if !self.accounts.contains_key(&recipient) {
            let account = Account::new(recipient.clone(), None, 0.0);
            self.accounts.insert(recipient.clone(), account);
        }

        // 送信者アカウントを取得
        let sender_account = self.accounts.get(&sender).unwrap().clone();

        // トランザクションを作成
        let mut transaction = Transaction::new(
            sender.clone(),
            recipient,
            amount,
            data,
            gas_price,
            gas_limit,
        );

        // 残高チェック
        if sender_account.balance < (transaction.amount + transaction.fee) {
            return Err(format!(
                "Insufficient balance: {} < {}",
                sender_account.balance,
                transaction.amount + transaction.fee
            ));
        }

        // ノンス値を設定
        transaction.nonce = sender_account.nonce;

        // ペンディングトランザクションに追加
        let tx_id = transaction.id.clone();
        self.pending_transactions.push(transaction);

        // 送信者のノンス値を増加
        let sender_account = self.accounts.get_mut(&sender).unwrap();
        sender_account.nonce += 1;

        println!("Transaction added: {}", tx_id);
        Ok(tx_id)
    }

    // ペンディングトランザクションをマイニング
    pub fn mine_pending_transactions(&mut self, miner_address: String) -> Option<Block> {
        if self.pending_transactions.is_empty() {
            println!("No transactions to mine");
            return None;
        }

        // マイニング報酬トランザクションを追加
        let mut reward_tx = Transaction::new(
            "0x0000000000000000000000000000000000000000".to_string(), // システムアドレス
            miner_address.clone(),
            5.0, // マイニング報酬
            None,
            0,
            21000,
        );
        reward_tx.status = TransactionStatus::Confirmed;
        self.pending_transactions.push(reward_tx);

        // 最新のブロックを取得
        let latest_block = self.get_latest_block().unwrap();

        // 新しいブロックを作成
        let mut new_block = Block::new(
            latest_block.index + 1,
            self.pending_transactions.clone(),
            latest_block.hash.clone(),
            miner_address,
            4, // 難易度
        );

        // ブロックをマイニング
        new_block.mine_block();

        // ブロックをチェーンに追加
        self.chain.push(new_block.clone());

        // トランザクションの処理（残高の更新など）
        let pending_txs = self.pending_transactions.clone();
        for tx in pending_txs {
            self.process_transaction(&tx, new_block.index);
        }

        // ペンディングトランザクションをクリア
        self.pending_transactions.clear();

        println!("Block #{} mined and added to the chain", new_block.index);
        Some(new_block)
    }

    // トランザクションを処理
    fn process_transaction(&mut self, transaction: &Transaction, block_number: u64) {
        // システムアドレスからの送金（マイニング報酬など）の場合は残高チェックをスキップ
        if transaction.sender != "0x0000000000000000000000000000000000000000" {
            // 送信者の残高を減少
            if let Some(sender) = self.accounts.get_mut(&transaction.sender) {
                sender.balance -= transaction.amount + transaction.fee;
                sender.transaction_count += 1;
                sender.last_activity = Utc::now();
            }
        }

        // 受信者の残高を増加
        if let Some(recipient) = self.accounts.get_mut(&transaction.recipient) {
            recipient.balance += transaction.amount;
            recipient.last_activity = Utc::now();
        } else {
            // 受信者アカウントが存在しない場合は作成
            let mut account = Account::new(transaction.recipient.clone(), None, transaction.amount);
            account.last_activity = Utc::now();
            self.accounts.insert(transaction.recipient.clone(), account);
        }
    }

    // 新しいアカウントを作成
    pub fn create_account(&mut self) -> Account {
        // アドレスと秘密鍵を生成
        let private_key = format!("0x{}", Uuid::new_v4().to_string().replace("-", ""));
        let mut hasher = Sha256::new();
        hasher.update(private_key.as_bytes());
        let address = format!("0x{:x}", hasher.finalize());
        let address = address[0..42].to_string(); // 0xを含めて42文字

        // アカウントを作成
        let account = Account::new(address.clone(), Some(private_key), 0.0);
        self.accounts.insert(address.clone(), account.clone());

        println!("New account created: {}", address);
        account
    }

    // アカウントのトランザクション履歴を取得
    pub fn get_account_transactions(&self, address: &str) -> Vec<Transaction> {
        let mut transactions = Vec::new();

        // チェーン内のすべてのブロックを検索
        for block in &self.chain {
            for tx in &block.transactions {
                if tx.sender == address || tx.recipient == address {
                    transactions.push(tx.clone());
                }
            }
        }

        // ペンディングトランザクションも検索
        for tx in &self.pending_transactions {
            if tx.sender == address || tx.recipient == address {
                transactions.push(tx.clone());
            }
        }

        // タイムスタンプの降順でソート
        transactions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        transactions
    }

    // ブロック番号からブロックを取得
    pub fn get_block_by_number(&self, number: u64) -> Option<&Block> {
        self.chain.iter().find(|block| block.index == number)
    }

    // ハッシュからブロックを取得
    pub fn get_block_by_hash(&self, block_hash: &str) -> Option<&Block> {
        self.chain.iter().find(|block| block.hash == block_hash)
    }

    // トランザクションIDからトランザクションを取得
    pub fn get_transaction(&self, tx_id: &str) -> Option<Transaction> {
        // ペンディングトランザクションを検索
        for tx in &self.pending_transactions {
            if tx.id == tx_id {
                return Some(tx.clone());
            }
        }

        // チェーン内のすべてのブロックを検索
        for block in &self.chain {
            for tx in &block.transactions {
                if tx.id == tx_id {
                    let mut tx_clone = tx.clone();
                    tx_clone.block_number = Some(block.index);
                    return Some(tx_clone);
                }
            }
        }

        None
    }

    // ネットワーク統計情報を取得
    pub fn get_network_stats(&self) -> HashMap<String, serde_json::Value> {
        let mut stats = HashMap::new();

        // ブロック数
        stats.insert(
            "block_count".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.chain.len())),
        );

        // 最新ブロック
        if let Some(latest_block) = self.get_latest_block() {
            stats.insert(
                "latest_block".to_string(),
                serde_json::to_value(latest_block).unwrap_or(serde_json::Value::Null),
            );
        }

        // ペンディングトランザクション数
        stats.insert(
            "pending_transactions".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.pending_transactions.len())),
        );

        // 平均ブロック時間を計算
        let mut block_times = Vec::new();
        for i in 1..self.chain.len() {
            let time_diff = (self.chain[i].timestamp - self.chain[i - 1].timestamp).num_seconds();
            block_times.push(time_diff as f64);
        }

        let avg_block_time = if !block_times.is_empty() {
            block_times.iter().sum::<f64>() / block_times.len() as f64
        } else {
            0.0
        };

        stats.insert(
            "average_block_time".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(avg_block_time).unwrap_or(serde_json::Number::from(0))),
        );

        // TPS（1秒あたりのトランザクション数）を計算
        let latest_tx_count = self
            .get_latest_block()
            .map(|block| block.transactions.len())
            .unwrap_or(0);

        let tps = if avg_block_time > 0.0 {
            latest_tx_count as f64 / avg_block_time
        } else {
            0.0
        };

        stats.insert(
            "tps".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(tps).unwrap_or(serde_json::Number::from(0))),
        );

        // アカウント数
        stats.insert(
            "account_count".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.accounts.len())),
        );

        // 難易度
        if let Some(latest_block) = self.get_latest_block() {
            stats.insert(
                "difficulty".to_string(),
                serde_json::Value::Number(serde_json::Number::from(latest_block.difficulty)),
            );
        }

        stats
    }
}

// スレッドセーフなブロックチェーンシングルトン
pub struct BlockchainState {
    pub blockchain: Arc<Mutex<Blockchain>>,
}

impl BlockchainState {
    pub fn new() -> Self {
        BlockchainState {
            blockchain: Arc::new(Mutex::new(Blockchain::new())),
        }
    }

    pub fn get_instance() -> Self {
        Self::new()
    }
}