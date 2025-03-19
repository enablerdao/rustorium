use anyhow::Result;
use super::super::dag::{Transaction, TxId};
use super::{AvalancheEngine, Vote};

impl AvalancheEngine {
    /// 投票を処理
    pub async fn process_vote(&mut self, tx_id: TxId, vote: Vote) -> Result<()> {
        // 1. コンフィデンス値を更新
        let mut confidence = self.confidence.write().await;
        let conf = confidence.entry(tx_id.clone()).or_insert_with(|| super::Confidence::new());
        conf.add_vote(vote);

        // 2. 投票チャネルに送信
        self.vote_sender.send(vote)?;

        Ok(())
    }

    /// 投票をリクエスト
    pub async fn request_vote(&self, tx: &Transaction) -> Result<Vote> {
        // 1. トランザクションを検証
        if !self.validate_transaction(tx).await? {
            return Ok(Vote::Reject);
        }

        // 2. メタスタビリティをチェック
        match self.resolve_metastability(tx).await? {
            super::super::dag::TxStatus::Confirmed => Ok(Vote::Accept),
            super::super::dag::TxStatus::Rejected => Ok(Vote::Reject),
            _ => {
                // 3. 現在のコンフィデンス値に基づいて投票
                let confidence = self.confidence.read().await;
                if let Some(conf) = confidence.get(&tx.id) {
                    if conf.get_confidence() >= self.params.threshold {
                        Ok(Vote::Accept)
                    } else {
                        Ok(Vote::Reject)
                    }
                } else {
                    // 4. 初めての投票の場合は検証結果に基づいて判断
                    if self.validate_transaction(tx).await? {
                        Ok(Vote::Accept)
                    } else {
                        Ok(Vote::Reject)
                    }
                }
            }
        }
    }

    /// 投票の集計
    pub async fn count_votes(&self, tx_id: &TxId) -> Result<(u32, u32)> {
        let confidence = self.confidence.read().await;
        if let Some(conf) = confidence.get(tx_id) {
            Ok((conf.accept_count, conf.reject_count))
        } else {
            Ok((0, 0))
        }
    }

    /// 投票の履歴をクリーンアップ
    pub async fn cleanup_votes(&mut self) -> Result<()> {
        // 1. 古い投票を削除
        let mut confidence = self.confidence.write().await;
        confidence.clear();

        // 2. チャネルをリセット
        let (vote_sender, vote_receiver) = tokio::sync::mpsc::unbounded_channel();
        self.vote_sender = vote_sender;
        self.vote_receiver = vote_receiver;

        Ok(())
    }

    /// 投票の統計情報を取得
    pub async fn get_vote_stats(&self, tx_id: &TxId) -> Result<VoteStats> {
        let confidence = self.confidence.read().await;
        if let Some(conf) = confidence.get(tx_id) {
            Ok(VoteStats {
                accept_count: conf.accept_count,
                reject_count: conf.reject_count,
                confidence: conf.get_confidence(),
            })
        } else {
            Ok(VoteStats {
                accept_count: 0,
                reject_count: 0,
                confidence: 0.0,
            })
        }
    }
}

/// 投票の統計情報
#[derive(Debug, Clone)]
pub struct VoteStats {
    /// 承認票数
    pub accept_count: u32,
    /// 拒否票数
    pub reject_count: u32,
    /// コンフィデンス値
    pub confidence: f64,
}