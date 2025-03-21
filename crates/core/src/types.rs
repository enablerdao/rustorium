//! 共通型定義

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ブロック
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// ブロックヘッダー
    pub header: BlockHeader,
    /// トランザクション一覧
    pub transactions: Vec<Transaction>,
    /// レシート一覧
    pub receipts: Vec<Receipt>,
}

/// ブロックヘッダー
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    /// ブロック番号
    pub number: u64,
    /// 親ブロックのハッシュ
    pub parent_hash: Hash,
    /// ステートルート
    pub state_root: Hash,
    /// トランザクションルート
    pub transactions_root: Hash,
    /// レシートルート
    pub receipts_root: Hash,
    /// タイムスタンプ
    pub timestamp: u64,
    /// バリデーター
    pub validator: Address,
    /// 署名
    pub signature: Signature,
}

/// トランザクション
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// 送信者アドレス
    pub from: Address,
    /// 受信者アドレス
    pub to: Address,
    /// 送金額
    pub value: u64,
    /// ガス価格
    pub gas_price: u64,
    /// ガスリミット
    pub gas_limit: u64,
    /// ノンス
    pub nonce: u64,
    /// データ
    pub data: Vec<u8>,
    /// 署名
    pub signature: Signature,
}

/// レシート
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    /// トランザクションハッシュ
    pub transaction_hash: Hash,
    /// ステータス
    pub status: ReceiptStatus,
    /// 使用したガス量
    pub gas_used: u64,
    /// ログ一覧
    pub logs: Vec<Log>,
}

/// レシートステータス
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReceiptStatus {
    /// 成功
    Success,
    /// 失敗
    Failure,
}

/// ログ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    /// コントラクトアドレス
    pub address: Address,
    /// トピック一覧
    pub topics: Vec<Hash>,
    /// データ
    pub data: Vec<u8>,
}

/// ハッシュ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hash(pub [u8; 32]);

/// アドレス
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address(pub [u8; 20]);

/// 署名
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Signature(pub [u8; 64]);

impl Transaction {
    /// トランザクションのハッシュを計算
    pub fn hash(&self) -> Hash {
        // TODO: 実際のハッシュ計算を実装
        Hash([0; 32])
    }

    /// 署名の検証
    pub fn verify_signature(&self) -> bool {
        // TODO: 実際の署名検証を実装
        true
    }
}

impl Block {
    /// ブロックのハッシュを計算
    pub fn hash(&self) -> Hash {
        // TODO: 実際のハッシュ計算を実装
        Hash([0; 32])
    }
}
