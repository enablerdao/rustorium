use crate::common::errors::{ConsensusError, LedgerError};
use crate::common::types::{Address, Block, BlockHeader, Transaction};
use crate::common::utils;
use crate::consensus::avalanche::{AvalancheConsensus, AvalancheState};
use crate::storage::state::StateManager;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, RwLock};
use tokio::time;
use tracing::{debug, error, info, warn};

/// バリデータ
pub struct Validator {
    /// バリデータアドレス
    address: Address,
    /// 署名キー
    signing_key: SigningKey,
    /// 検証キー
    verifying_key: VerifyingKey,
    /// 状態マネージャー
    state_manager: Arc<StateManager>,
    /// Avalancheコンセンサス
    avalanche: Option<Arc<AvalancheConsensus<Transaction>>>,
    /// 保留中のトランザクション
    pending_transactions: Arc<RwLock<HashMap<[u8; 32], Transaction>>>,
    /// 最新のブロック高
    latest_block_height: Arc<RwLock<u64>>,
    /// ブロック生成間隔（ミリ秒）
    block_time_ms: u64,
    /// 実行中フラグ
    running: bool,
}

impl Validator {
    /// 新しいバリデータを作成
    pub fn new(
        address: Address,
        signing_key: SigningKey,
        state_manager: Arc<StateManager>,
        block_time_ms: u64,
    ) -> Self {
        let verifying_key = signing_key.verifying_key();
        
        Self {
            address,
            signing_key,
            verifying_key,
            state_manager,
            avalanche: None,
            pending_transactions: Arc::new(RwLock::new(HashMap::new())),
            latest_block_height: Arc::new(RwLock::new(0)),
            block_time_ms,
            running: false,
        }
    }
    
    /// ランダムなバリデータを作成
    pub fn random(state_manager: Arc<StateManager>, block_time_ms: u64) -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let public_key = signing_key.verifying_key();
        
        // アドレスを生成
        let mut address_bytes = [0u8; 20];
        address_bytes.copy_from_slice(&public_key.to_bytes()[0..20]);
        let address = Address(address_bytes);
        
        Self::new(address, signing_key, state_manager, block_time_ms)
    }
    
    /// Avalancheコンセンサスを設定
    pub fn set_avalanche(&mut self, avalanche: Arc<AvalancheConsensus<Transaction>>) {
        self.avalanche = Some(avalanche);
    }
    
    /// バリデータを開始
    pub async fn start(&mut self) -> Result<(), LedgerError> {
        if self.running {
            return Err(LedgerError::InvalidState(
                "Validator is already running".to_string(),
            ));
        }
        
        self.running = true;
        
        // 最新のブロック高を取得
        let latest_height = self.state_manager.get_latest_block_height().await;
        *self.latest_block_height.write().await = latest_height;
        
        info!(
            "Validator started with address {} at block height {}",
            self.address, latest_height
        );
        
        // ブロック生成ループを開始
        self.block_production_loop().await?;
        
        self.running = false;
        info!("Validator stopped");
        
        Ok(())
    }
    
    /// ブロック生成ループ
    async fn block_production_loop(&self) -> Result<(), LedgerError> {
        let mut interval = time::interval(Duration::from_millis(self.block_time_ms));
        
        while self.running {
            interval.tick().await;
            
            // ブロックを生成
            match self.produce_block().await {
                Ok(block) => {
                    info!("Produced block at height {}", block.header.height);
                    
                    // ブロックを適用
                    if let Err(e) = self.state_manager.apply_block(&block).await {
                        error!("Failed to apply block: {}", e);
                    } else {
                        // 最新のブロック高を更新
                        *self.latest_block_height.write().await = block.header.height;
                    }
                }
                Err(e) => {
                    error!("Failed to produce block: {}", e);
                }
            }
        }
        
        Ok(())
    }
    
    /// ブロックを生成
    async fn produce_block(&self) -> Result<Block, LedgerError> {
        let latest_height = *self.latest_block_height.read().await;
        let new_height = latest_height + 1;
        
        // 前のブロックを取得
        let prev_block = if latest_height > 0 {
            self.state_manager
                .get_block(latest_height)
                .await?
                .ok_or_else(|| {
                    LedgerError::NotFound(format!("Previous block at height {} not found", latest_height))
                })?
        } else {
            // ジェネシスブロックの場合
            Block {
                header: BlockHeader {
                    height: 0,
                    prev_hash: [0; 32],
                    merkle_root: [0; 32],
                    timestamp: 0,
                    validator: Address([0; 20]),
                    signature: None,
                },
                transactions: vec![],
            }
        };
        
        // トランザクションを選択
        let transactions = self.select_transactions().await?;
        
        // マークルルートを計算
        let tx_hashes: Vec<[u8; 32]> = transactions
            .iter()
            .map(|tx| *tx.id.as_bytes())
            .collect();
        
        let merkle_root = utils::calculate_merkle_root(&tx_hashes);
        
        // ブロックヘッダーを作成
        let mut header = BlockHeader {
            height: new_height,
            prev_hash: utils::calculate_merkle_root(&[prev_block.header.merkle_root]),
            merkle_root,
            timestamp: utils::current_time_sec(),
            validator: self.address,
            signature: None,
        };
        
        // ヘッダーに署名
        let header_bytes = bincode::serialize(&header)?;
        let signature = self.signing_key.sign(&header_bytes);
        
        // 署名をヘッダーに設定
        header.signature = Some(crate::common::types::Signature(signature.to_bytes()));
        
        // ブロックを作成
        let block = Block {
            header,
            transactions,
        };
        
        Ok(block)
    }
    
    /// トランザクションを選択
    async fn select_transactions(&self) -> Result<Vec<Transaction>, LedgerError> {
        let mut selected = Vec::new();
        let mut selected_ids = HashSet::new();
        
        // 保留中のトランザクションを取得
        let pending = self.pending_transactions.read().await;
        
        // Avalancheコンセンサスで確定したトランザクションを優先
        if let Some(avalanche) = &self.avalanche {
            for (tx_id, tx) in pending.iter() {
                if selected.len() >= 100 {
                    break; // 最大100トランザクション
                }
                
                if selected_ids.contains(tx_id) {
                    continue;
                }
                
                if let Some(state) = avalanche.get_item_state(&tx.id) {
                    if state == AvalancheState::Accepted {
                        selected.push(tx.clone());
                        selected_ids.insert(*tx_id);
                    }
                }
            }
        }
        
        // 残りのスロットを他のトランザクションで埋める
        for (tx_id, tx) in pending.iter() {
            if selected.len() >= 100 {
                break; // 最大100トランザクション
            }
            
            if selected_ids.contains(tx_id) {
                continue;
            }
            
            selected.push(tx.clone());
            selected_ids.insert(*tx_id);
        }
        
        Ok(selected)
    }
    
    /// トランザクションを追加
    pub async fn add_transaction(&self, transaction: Transaction) -> Result<(), LedgerError> {
        // トランザクションを検証
        self.verify_transaction(&transaction).await?;
        
        // Avalancheコンセンサスに追加
        if let Some(avalanche) = &self.avalanche {
            avalanche.add_item(transaction.id, transaction.clone()).await?;
        }
        
        // 保留中のトランザクションに追加
        let mut pending = self.pending_transactions.write().await;
        pending.insert(*transaction.id.as_bytes(), transaction);
        
        Ok(())
    }
    
    /// トランザクションを検証
    async fn verify_transaction(&self, transaction: &Transaction) -> Result<(), LedgerError> {
        // 署名を検証
        if let Some(signature) = &transaction.signature {
            // TODO: 実際の署名検証を実装
        }
        
        // 送信者アカウントを取得
        let sender = match self.state_manager.get_account(&transaction.sender).await? {
            Some(account) => account,
            None => {
                return Err(LedgerError::Transaction(
                    crate::common::errors::TransactionError::InvalidFormat(
                        "Sender account not found".to_string(),
                    ),
                ));
            }
        };
        
        // ノンスを検証
        if transaction.nonce != sender.nonce {
            return Err(LedgerError::Transaction(
                crate::common::errors::TransactionError::InvalidNonce,
            ));
        }
        
        // 残高を検証
        if sender.balance < transaction.amount + transaction.fee {
            return Err(LedgerError::Transaction(
                crate::common::errors::TransactionError::InsufficientFunds,
            ));
        }
        
        Ok(())
    }
    
    /// ブロックを検証
    pub async fn verify_block(&self, block: &Block) -> Result<(), LedgerError> {
        // ブロック高を検証
        let latest_height = *self.latest_block_height.read().await;
        if block.header.height != latest_height + 1 {
            return Err(LedgerError::Consensus(ConsensusError::InvalidBlock(format!(
                "Invalid block height: expected {}, got {}",
                latest_height + 1,
                block.header.height
            ))));
        }
        
        // 前のブロックハッシュを検証
        let prev_block = self
            .state_manager
            .get_block(latest_height)
            .await?
            .ok_or_else(|| {
                LedgerError::NotFound(format!("Previous block at height {} not found", latest_height))
            })?;
        
        let expected_prev_hash = utils::calculate_merkle_root(&[prev_block.header.merkle_root]);
        if block.header.prev_hash != expected_prev_hash {
            return Err(LedgerError::Consensus(ConsensusError::InvalidBlock(
                "Invalid previous block hash".to_string(),
            )));
        }
        
        // マークルルートを検証
        let tx_hashes: Vec<[u8; 32]> = block
            .transactions
            .iter()
            .map(|tx| *tx.id.as_bytes())
            .collect();
        
        let expected_merkle_root = utils::calculate_merkle_root(&tx_hashes);
        if block.header.merkle_root != expected_merkle_root {
            return Err(LedgerError::Consensus(ConsensusError::InvalidBlock(
                "Invalid merkle root".to_string(),
            )));
        }
        
        // 署名を検証
        if let Some(signature) = &block.header.signature {
            // 署名を検証するためにはバリデータの公開鍵が必要
            // TODO: バリデータの公開鍵を取得して署名を検証
        } else {
            return Err(LedgerError::Consensus(ConsensusError::InvalidBlock(
                "Block has no signature".to_string(),
            )));
        }
        
        // トランザクションを検証
        for tx in &block.transactions {
            self.verify_transaction(tx).await?;
        }
        
        Ok(())
    }
    
    /// バリデータを停止
    pub fn stop(&mut self) {
        self.running = false;
    }
}