//! HotStuffコンセンサスモジュール

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use rustorium_core::{
    Module, ModuleConfig, ModuleStatus, ModuleMetrics,
    consensus::ConsensusModule,
};
use async_trait::async_trait;
use tracing::info;

/// HotStuffの状態
#[derive(Debug, Clone, PartialEq, Eq)]
enum HotStuffPhase {
    /// 新ブロック
    New,
    /// 準備フェーズ
    Prepare,
    /// プリコミットフェーズ
    PreCommit,
    /// コミットフェーズ
    Commit,
    /// 決定フェーズ
    Decide,
}

/// HotStuffコンセンサスモジュール
pub struct HotStuffModule {
    /// 設定
    config: ModuleConfig,
    /// ステータス
    status: ModuleStatus,
    /// 現在のフェーズ
    phase: HotStuffPhase,
    /// 投票
    votes: Arc<RwLock<HashMap<Vec<u8>, Vec<bool>>>>,
}

impl HotStuffModule {
    /// 新しいHotStuffコンセンサスモジュールを作成
    pub fn new(config: ModuleConfig) -> Self {
        Self {
            config,
            status: ModuleStatus::Uninitialized,
            phase: HotStuffPhase::New,
            votes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 投票の集計
    async fn count_votes(&self, block: &[u8]) -> (usize, usize) {
        let votes = self.votes.read().await;
        if let Some(block_votes) = votes.get(block) {
            let yes_votes = block_votes.iter().filter(|&&v| v).count();
            let no_votes = block_votes.len() - yes_votes;
            (yes_votes, no_votes)
        } else {
            (0, 0)
        }
    }

    /// フェーズの進行
    async fn advance_phase(&mut self, block: &[u8]) -> anyhow::Result<()> {
        let (yes_votes, _) = self.count_votes(block).await;
        let threshold = self.config.config
            .get("vote_threshold")
            .and_then(|v| v.as_u64())
            .unwrap_or(2) as usize;

        if yes_votes >= threshold {
            self.phase = match self.phase {
                HotStuffPhase::New => HotStuffPhase::Prepare,
                HotStuffPhase::Prepare => HotStuffPhase::PreCommit,
                HotStuffPhase::PreCommit => HotStuffPhase::Commit,
                HotStuffPhase::Commit => HotStuffPhase::Decide,
                HotStuffPhase::Decide => HotStuffPhase::New,
            };
            info!("Advanced to phase: {:?}", self.phase);
        }

        Ok(())
    }
}

#[async_trait]
impl Module for HotStuffModule {
    async fn init(&mut self) -> anyhow::Result<()> {
        info!("Initializing HotStuff consensus module...");
        self.status = ModuleStatus::Initialized;
        info!("HotStuff consensus module initialized");
        Ok(())
    }

    async fn start(&mut self) -> anyhow::Result<()> {
        info!("Starting HotStuff consensus module...");
        self.status = ModuleStatus::Running;
        info!("HotStuff consensus module started");
        Ok(())
    }

    async fn stop(&mut self) -> anyhow::Result<()> {
        info!("Stopping HotStuff consensus module...");
        self.status = ModuleStatus::Stopped;
        info!("HotStuff consensus module stopped");
        Ok(())
    }

    async fn status(&self) -> anyhow::Result<ModuleStatus> {
        Ok(self.status.clone())
    }

    async fn metrics(&self) -> anyhow::Result<ModuleMetrics> {
        let mut metrics = HashMap::new();
        metrics.insert("current_phase".to_string(), match self.phase {
            HotStuffPhase::New => 0.0,
            HotStuffPhase::Prepare => 1.0,
            HotStuffPhase::PreCommit => 2.0,
            HotStuffPhase::Commit => 3.0,
            HotStuffPhase::Decide => 4.0,
        });

        Ok(ModuleMetrics {
            timestamp: std::time::SystemTime::now(),
            metrics,
        })
    }
}

#[async_trait]
impl ConsensusModule for HotStuffModule {
    async fn propose_block(&mut self, block: Vec<u8>) -> anyhow::Result<()> {
        info!("Proposing block: {}", hex::encode(&block));
        self.phase = HotStuffPhase::New;
        self.votes.write().await.insert(block, Vec::new());
        Ok(())
    }

    async fn verify_block(&self, block: Vec<u8>) -> anyhow::Result<bool> {
        // TODO: 実際のブロック検証を実装
        Ok(true)
    }

    async fn vote_block(&mut self, block: Vec<u8>, vote: bool) -> anyhow::Result<()> {
        info!("Voting {} for block: {}", vote, hex::encode(&block));
        self.votes.write().await.entry(block.clone()).or_insert_with(Vec::new).push(vote);
        self.advance_phase(&block).await?;
        Ok(())
    }

    async fn finalize_block(&mut self, block: Vec<u8>) -> anyhow::Result<()> {
        if self.phase == HotStuffPhase::Decide {
            info!("Finalizing block: {}", hex::encode(&block));
            self.votes.write().await.remove(&block);
            self.phase = HotStuffPhase::New;
        }
        Ok(())
    }
}
