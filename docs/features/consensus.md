# Avalancheコンセンサス

Rustoriumは、高速なトランザクション確定と優れた耐障害性を実現するために、Avalancheコンセンサスプロトコルを採用しています。この文書では、その実装詳細について説明します。

## 概要

Avalancheは、確率的サンプリングと複数ラウンドの投票を使用する革新的なコンセンサスプロトコルです。このプロトコルは、高いスループット、低いレイテンシー、強い安全性保証を提供します。

## 主要コンポーネント

### Avalancheプロトコル実装

Avalancheプロトコルの中核は、ランダムなピアサンプリングと繰り返しクエリによる投票メカニズムです。

```rust
pub struct AvalancheConsensus {
    params: AvalancheParams,
    network: Arc<NetworkService>,
    pending_transactions: DashMap<TransactionId, TransactionState>,
    validators: Arc<RwLock<Vec<NodeId>>>,
}

impl AvalancheConsensus {
    pub async fn start_consensus_for_transaction(
        &self,
        tx_id: TransactionId,
    ) -> Result<(), ConsensusError> {
        // トランザクションを保留中リストに追加
        self.pending_transactions.insert(
            tx_id.clone(),
            TransactionState::new(self.params.confidence_threshold),
        );
        
        // コンセンサスプロセスを開始
        self.run_consensus_rounds(tx_id).await
    }
    
    async fn run_consensus_rounds(
        &self,
        tx_id: TransactionId,
    ) -> Result<(), ConsensusError> {
        let mut round = 0;
        
        while round < self.params.max_rounds {
            // ランダムなバリデータをサンプリング
            let validators = self.sample_validators(self.params.sample_size).await?;
            
            // クエリを送信
            let responses = self.query_validators(&validators, &tx_id).await?;
            
            // 応答を処理
            self.process_responses(&tx_id, responses).await?;
            
            // 信頼度をチェック
            if self.check_confidence(&tx_id).await? {
                return Ok(());
            }
            
            round += 1;
        }
        
        Err(ConsensusError::MaxRoundsExceeded)
    }
    
    // 他のメソッド...
}
```

### バリデータサンプリング

Avalancheプロトコルでは、各ラウンドでランダムなバリデータのサブセットをサンプリングします。

```rust
async fn sample_validators(&self, sample_size: usize) -> Result<Vec<NodeId>, ConsensusError> {
    let validators = self.validators.read().await;
    
    if validators.len() < sample_size {
        return Err(ConsensusError::InsufficientValidators);
    }
    
    // ランダムなバリデータをサンプリング
    let mut rng = rand::thread_rng();
    let sampled_indices: Vec<usize> = (0..validators.len())
        .choose_multiple(&mut rng, sample_size)
        .collect();
    
    Ok(sampled_indices.iter().map(|&i| validators[i].clone()).collect())
}
```

### クエリと応答処理

バリデータへのクエリ送信と応答処理の実装です。

```rust
async fn query_validators(
    &self,
    validators: &[NodeId],
    tx_id: &TransactionId,
) -> Result<Vec<bool>, ConsensusError> {
    let mut responses = Vec::with_capacity(validators.len());
    
    for validator in validators {
        // クエリメッセージを作成
        let query_type = QueryType::GetTransaction(format!("0x{}", hex::encode(tx_id.as_bytes())));
        let query = Query::new(query_type);
        
        // クエリを送信し応答を待機
        match self.network.send_query(validator, query).await {
            Ok(response) => {
                // 応答を処理
                let vote = self.process_response(response).await?;
                responses.push(vote);
            },
            Err(_) => {
                // タイムアウトまたは接続エラー
                responses.push(false);
            }
        }
    }
    
    Ok(responses)
}

async fn process_responses(
    &self,
    tx_id: &TransactionId,
    responses: Vec<bool>,
) -> Result<(), ConsensusError> {
    if let Some(mut tx_state) = self.pending_transactions.get_mut(tx_id) {
        // 肯定的な応答の数をカウント
        let positive_votes = responses.iter().filter(|&&vote| vote).count();
        
        // 信頼度を更新
        tx_state.update_confidence(positive_votes, responses.len());
        
        Ok(())
    } else {
        Err(ConsensusError::TransactionNotFound)
    }
}
```

### 信頼度計算

トランザクションの信頼度を計算し、確定条件を満たしているかチェックします。

```rust
async fn check_confidence(&self, tx_id: &TransactionId) -> Result<bool, ConsensusError> {
    if let Some(tx_state) = self.pending_transactions.get(tx_id) {
        // 信頼度が閾値を超えているかチェック
        if tx_state.confidence >= self.params.confidence_threshold {
            // トランザクションを確定
            self.finalize_transaction(tx_id).await?;
            return Ok(true);
        }
        
        Ok(false)
    } else {
        Err(ConsensusError::TransactionNotFound)
    }
}

async fn finalize_transaction(&self, tx_id: &TransactionId) -> Result<(), ConsensusError> {
    // トランザクションを確定済みとしてマーク
    if let Some((id, mut state)) = self.pending_transactions.remove(tx_id) {
        state.status = TransactionStatus::Finalized;
        // 確定イベントを発行
        self.emit_finalization_event(&id).await;
    }
    
    Ok(())
}
```

## Avalancheパラメータ

Avalancheプロトコルの動作は、以下のパラメータによって調整できます。

```rust
pub struct AvalancheParams {
    // 各ラウンドでサンプリングするバリデータの数
    pub sample_size: usize,
    
    // トランザクション確定に必要な信頼度の閾値
    pub confidence_threshold: f64,
    
    // 最大ラウンド数
    pub max_rounds: usize,
    
    // クエリタイムアウト（ミリ秒）
    pub query_timeout_ms: u64,
}
```

## 設定例

```toml
[consensus]
# コンセンサスアルゴリズム
algorithm = "avalanche"

# サンプルサイズ
sample_size = 20

# 信頼度閾値
confidence_threshold = 0.8

# 最大ラウンド数
max_rounds = 10

# クエリタイムアウト（ミリ秒）
query_timeout_ms = 500

# 最小バリデータ数
min_validators = 4

# 閾値パーセンテージ
threshold_percentage = 67
```

## 耐障害性

Avalancheプロトコルは、ネットワーク内のノードの一部が悪意を持っていたり、障害を起こしていたりしても、正しく動作するように設計されています。具体的には、ネットワークの2/3以上のノードが正直である限り、安全性と活性が保証されます。

## パフォーマンス特性

Rustoriumの実装では、以下のパフォーマンス特性を実現しています：

- トランザクション確定時間: 1-2秒
- スループット: 1,000-10,000 TPS（ネットワーク条件による）
- スケーラビリティ: バリデータ数に対して対数的にスケール

## 今後の改善点

1. 動的なサンプルサイズ調整: ネットワーク条件に基づいてサンプルサイズを自動調整
2. 重み付きサンプリング: バリデータの評判や過去のパフォーマンスに基づいたサンプリング
3. マルチレベルコンセンサス: 異なる重要度のトランザクションに対して異なるコンセンサスパラメータを適用