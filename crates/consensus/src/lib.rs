//! コンセンサス層
//! 
//! GluonとTendermintを使用した高性能なコンセンサスエンジンを提供します。

use anyhow::Result;
use gluon_consensus::{Node as GluonNode, Config as GluonConfig};
use tendermint::{Node as TendermintNode, Config as TendermintConfig};
use tracing::{info, warn, error};

/// コンセンサスエンジン
pub struct ConsensusEngine {
    gluon: GluonNode,
    tendermint: TendermintNode,
    state: ConsensusState,
}

/// コンセンサスの状態
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsensusState {
    Initializing,
    Running,
    Stopped,
}

impl ConsensusEngine {
    /// 新しいコンセンサスエンジンを作成
    pub async fn new() -> Result<Self> {
        info!("Initializing consensus engine...");
        
        // Gluonの設定
        let gluon_config = GluonConfig {
            validators: vec![],  // バリデータリスト
            threshold: 2,        // BFTしきい値
            block_time: 1000,    // ブロック生成時間（ミリ秒）
        };
        
        // Tendermintの設定
        let tendermint_config = TendermintConfig {
            chain_id: "rustorium".to_string(),
            genesis_time: chrono::Utc::now(),
            consensus_params: Default::default(),
        };
        
        // ノードの作成
        let gluon = GluonNode::new(gluon_config).await?;
        let tendermint = TendermintNode::new(tendermint_config).await?;
        
        Ok(Self {
            gluon,
            tendermint,
            state: ConsensusState::Initializing,
        })
    }
    
    /// コンセンサスを開始
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting consensus engine...");
        
        // Gluonの開始
        self.gluon.start().await?;
        
        // Tendermintの開始
        self.tendermint.start().await?;
        
        self.state = ConsensusState::Running;
        info!("Consensus engine started successfully");
        Ok(())
    }
    
    /// コンセンサスを停止
    pub async fn stop(&mut self) -> Result<()> {
        info!("Stopping consensus engine...");
        
        // Tendermintの停止
        self.tendermint.stop().await?;
        
        // Gluonの停止
        self.gluon.stop().await?;
        
        self.state = ConsensusState::Stopped;
        info!("Consensus engine stopped successfully");
        Ok(())
    }
    
    /// コンセンサスの状態を取得
    pub fn state(&self) -> ConsensusState {
        self.state
    }
    
    /// バリデータの追加
    pub async fn add_validator(&mut self, validator: Validator) -> Result<()> {
        // Gluonにバリデータを追加
        self.gluon.add_validator(validator.clone()).await?;
        
        // Tendermintにバリデータを追加
        self.tendermint.add_validator(validator).await?;
        
        Ok(())
    }
    
    /// バリデータの削除
    pub async fn remove_validator(&mut self, validator_id: &str) -> Result<()> {
        // Gluonからバリデータを削除
        self.gluon.remove_validator(validator_id).await?;
        
        // Tendermintからバリデータを削除
        self.tendermint.remove_validator(validator_id).await?;
        
        Ok(())
    }
}

/// バリデータ情報
#[derive(Debug, Clone)]
pub struct Validator {
    /// バリデータID
    pub id: String,
    /// 公開鍵
    pub public_key: Vec<u8>,
    /// 投票力
    pub voting_power: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_consensus_lifecycle() -> Result<()> {
        let mut consensus = ConsensusEngine::new().await?;
        assert_eq!(consensus.state(), ConsensusState::Initializing);
        
        // 起動テスト
        consensus.start().await?;
        assert_eq!(consensus.state(), ConsensusState::Running);
        
        // 停止テスト
        consensus.stop().await?;
        assert_eq!(consensus.state(), ConsensusState::Stopped);
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_validator_management() -> Result<()> {
        let mut consensus = ConsensusEngine::new().await?;
        
        // バリデータの追加
        let validator = Validator {
            id: "validator1".to_string(),
            public_key: vec![1, 2, 3, 4],
            voting_power: 100,
        };
        consensus.add_validator(validator.clone()).await?;
        
        // バリデータの削除
        consensus.remove_validator(&validator.id).await?;
        
        Ok(())
    }
}
