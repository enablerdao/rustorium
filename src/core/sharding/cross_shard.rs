//! クロスシャード通信の実装
//! 
//! このモジュールは、シャード間の通信を管理します。
//! 主な機能：
//! - 2段階コミットプロトコル
//! - メッセージルーティング
//! - 失敗時のリカバリ

use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use crate::core::sharding::{ShardId, Timestamp};

/// メッセージの優先度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    High,
    Medium,
    Low,
}

/// メッセージの状態
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageStatus {
    Created,
    Preparing,
    Prepared,
    Committing,
    Committed,
    Failed,
    Rolled,
}

/// クロスシャードメッセージ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossShardMessage {
    // ルーティング情報
    pub routing: MessageRouting,
    // ペイロード
    pub payload: MessagePayload,
    // メタデータ
    pub metadata: MessageMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageRouting {
    pub from_shard: ShardId,
    pub to_shard: ShardId,
    pub priority: Priority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePayload {
    pub data: Vec<u8>,
    pub size: usize,
    pub checksum: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageMetadata {
    pub timestamp: Timestamp,
    pub ttl: std::time::Duration,
    pub retry_count: u8,
}

impl CrossShardMessage {
    /// 新しいメッセージを作成
    pub fn new(
        from_shard: ShardId,
        to_shard: ShardId,
        data: Vec<u8>,
        priority: Priority,
    ) -> Self {
        let size = data.len();
        let checksum = blake3::hash(&data).as_bytes().clone();
        
        Self {
            routing: MessageRouting {
                from_shard,
                to_shard,
                priority,
            },
            payload: MessagePayload {
                data,
                size,
                checksum,
            },
            metadata: MessageMetadata {
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                ttl: std::time::Duration::from_secs(300), // 5分
                retry_count: 0,
            },
        }
    }

    /// メッセージを送信
    pub async fn send(&self) -> Result<MessageReceipt> {
        // 1. 最適ルート選択
        let route = self.find_optimal_route()?;
        
        // 2. 2段階コミット
        self.prepare(route).await?;
        self.commit(route).await?;
        
        // 3. 確認と再試行
        self.verify_and_retry().await?;
        
        Ok(MessageReceipt::new())
    }

    /// 最適なルートを見つける
    fn find_optimal_route(&self) -> Result<MessageRoute> {
        // TODO: 実際のルート検索ロジックを実装
        Ok(MessageRoute {
            hops: vec![
                self.routing.from_shard,
                self.routing.to_shard,
            ],
            estimated_latency: std::time::Duration::from_millis(100),
        })
    }

    /// プリペアフェーズ
    async fn prepare(&self, route: MessageRoute) -> Result<()> {
        // TODO: 2段階コミットのプリペアフェーズを実装
        Ok(())
    }

    /// コミットフェーズ
    async fn commit(&self, route: MessageRoute) -> Result<()> {
        // TODO: 2段階コミットのコミットフェーズを実装
        Ok(())
    }

    /// 確認と再試行
    async fn verify_and_retry(&self) -> Result<()> {
        // TODO: 確認と再試行ロジックを実装
        Ok(())
    }
}

/// メッセージのルート
#[derive(Debug, Clone)]
pub struct MessageRoute {
    pub hops: Vec<ShardId>,
    pub estimated_latency: std::time::Duration,
}

/// メッセージの受領証
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageReceipt {
    pub message_id: [u8; 32],
    pub status: MessageStatus,
    pub timestamp: Timestamp,
}

impl MessageReceipt {
    pub fn new() -> Self {
        Self {
            message_id: [0; 32],
            status: MessageStatus::Created,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

/// クロスシャード通信マネージャー
#[derive(Debug)]
pub struct CrossShardManager {
    pending_messages: Arc<RwLock<Vec<CrossShardMessage>>>,
    completed_messages: Arc<RwLock<Vec<MessageReceipt>>>,
}

impl CrossShardManager {
    /// 新しいマネージャーを作成
    pub fn new() -> Self {
        Self {
            pending_messages: Arc::new(RwLock::new(Vec::new())),
            completed_messages: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// メッセージを送信
    pub async fn send_message(&self, message: CrossShardMessage) -> Result<MessageReceipt> {
        // メッセージをペンディングリストに追加
        {
            let mut pending = self.pending_messages.write().await;
            pending.push(message.clone());
        }

        // メッセージを送信
        let receipt = message.send().await?;

        // 完了リストに追加
        {
            let mut completed = self.completed_messages.write().await;
            completed.push(receipt.clone());
        }

        // ペンディングリストから削除
        {
            let mut pending = self.pending_messages.write().await;
            pending.retain(|m| {
                m.routing.from_shard != message.routing.from_shard ||
                m.routing.to_shard != message.routing.to_shard ||
                m.metadata.timestamp != message.metadata.timestamp
            });
        }

        Ok(receipt)
    }

    /// 保留中のメッセージを取得
    pub async fn get_pending_messages(&self) -> Result<Vec<CrossShardMessage>> {
        Ok(self.pending_messages.read().await.clone())
    }

    /// 完了したメッセージを取得
    pub async fn get_completed_messages(&self) -> Result<Vec<MessageReceipt>> {
        Ok(self.completed_messages.read().await.clone())
    }

    /// 期限切れのメッセージをクリーンアップ
    pub async fn cleanup_expired_messages(&self) -> Result<()> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // 期限切れのペンディングメッセージを削除
        {
            let mut pending = self.pending_messages.write().await;
            pending.retain(|m| {
                now - m.metadata.timestamp < m.metadata.ttl.as_secs()
            });
        }

        // 古い完了メッセージを削除（24時間以上前）
        {
            let mut completed = self.completed_messages.write().await;
            completed.retain(|r| {
                now - r.timestamp < 24 * 60 * 60
            });
        }

        Ok(())
    }
}