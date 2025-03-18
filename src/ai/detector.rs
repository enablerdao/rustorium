use crate::common::errors::LedgerError;
use crate::common::types::{Transaction, TransactionId};
use ndarray::{Array, Array1, Array2, Axis};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// 異常検出結果
#[derive(Debug, Clone)]
pub enum AnomalyResult {
    /// 正常
    Normal,
    /// 異常（スコアと理由）
    Anomalous(f32, String),
}

/// 異常検出器
pub struct AnomalyDetector {
    /// 特徴量抽出器
    feature_extractor: FeatureExtractor,
    /// 異常検出モデル
    model: AnomalyModel,
    /// 検出閾値
    threshold: f32,
    /// 検出結果キャッシュ
    result_cache: Arc<RwLock<HashMap<TransactionId, AnomalyResult>>>,
}

impl AnomalyDetector {
    /// 新しい異常検出器を作成
    pub fn new(threshold: f32) -> Self {
        Self {
            feature_extractor: FeatureExtractor::new(),
            model: AnomalyModel::new(),
            threshold,
            result_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// トランザクションの異常を検出
    pub async fn detect_anomaly(&self, transaction: &Transaction) -> Result<AnomalyResult, LedgerError> {
        let tx_id = transaction.id;
        
        // キャッシュをチェック
        {
            let cache = self.result_cache.read().await;
            if let Some(result) = cache.get(&tx_id) {
                return Ok(result.clone());
            }
        }
        
        // 特徴量を抽出
        let features = self.feature_extractor.extract_features(transaction);
        
        // 異常スコアを計算
        let anomaly_score = self.model.compute_anomaly_score(&features);
        
        // 結果を判定
        let result = if anomaly_score > self.threshold {
            AnomalyResult::Anomalous(
                anomaly_score,
                format!("Anomaly score {:.4} exceeds threshold {:.4}", anomaly_score, self.threshold),
            )
        } else {
            AnomalyResult::Normal
        };
        
        // キャッシュに保存
        {
            let mut cache = self.result_cache.write().await;
            cache.insert(tx_id, result.clone());
        }
        
        Ok(result)
    }
    
    /// 検出閾値を設定
    pub fn set_threshold(&mut self, threshold: f32) {
        self.threshold = threshold;
    }
    
    /// モデルを更新
    pub async fn update_model(&mut self, transactions: &[Transaction]) -> Result<(), LedgerError> {
        // 特徴量を抽出
        let features = transactions
            .iter()
            .map(|tx| self.feature_extractor.extract_features(tx))
            .collect::<Vec<_>>();
        
        // モデルを更新
        self.model.update(&features);
        
        // キャッシュをクリア
        {
            let mut cache = self.result_cache.write().await;
            cache.clear();
        }
        
        Ok(())
    }
}

/// 特徴量抽出器
struct FeatureExtractor {
    /// 特徴量の次元数
    feature_dim: usize,
}

impl FeatureExtractor {
    /// 新しい特徴量抽出器を作成
    fn new() -> Self {
        Self { feature_dim: 10 }
    }
    
    /// トランザクションから特徴量を抽出
    fn extract_features(&self, transaction: &Transaction) -> Array1<f32> {
        let mut features = Array::zeros(self.feature_dim);
        
        // 特徴量1: 金額（正規化）
        features[0] = normalize_amount(transaction.amount);
        
        // 特徴量2: 手数料（正規化）
        features[1] = normalize_amount(transaction.fee);
        
        // 特徴量3: 手数料/金額の比率
        features[2] = if transaction.amount > 0 {
            transaction.fee as f32 / transaction.amount as f32
        } else {
            0.0
        };
        
        // 特徴量4: データサイズ（正規化）
        features[3] = normalize_data_size(transaction.data.len());
        
        // 特徴量5: ノンス（正規化）
        features[4] = normalize_nonce(transaction.nonce);
        
        // 特徴量6-10: 送信者アドレスのハッシュ特徴
        let sender_hash = compute_address_hash(&transaction.sender.0);
        for i in 0..5 {
            features[5 + i] = sender_hash[i];
        }
        
        features
    }
}

/// 金額を正規化
fn normalize_amount(amount: u64) -> f32 {
    // 簡単な対数正規化
    if amount == 0 {
        0.0
    } else {
        (amount as f32).log10() / 10.0
    }
}

/// データサイズを正規化
fn normalize_data_size(size: usize) -> f32 {
    // 簡単な対数正規化
    if size == 0 {
        0.0
    } else {
        (size as f32).log10() / 5.0
    }
}

/// ノンスを正規化
fn normalize_nonce(nonce: u64) -> f32 {
    // 簡単な対数正規化
    if nonce == 0 {
        0.0
    } else {
        (nonce as f32).log10() / 5.0
    }
}

/// アドレスからハッシュ特徴を計算
fn compute_address_hash(address: &[u8; 20]) -> [f32; 5] {
    let mut result = [0.0; 5];
    
    // アドレスを5つのチャンクに分割して各チャンクの合計を計算
    for i in 0..5 {
        let chunk_sum: u32 = address[i * 4..(i + 1) * 4]
            .iter()
            .map(|&b| b as u32)
            .sum();
        result[i] = (chunk_sum % 1000) as f32 / 1000.0;
    }
    
    result
}

/// 異常検出モデル
struct AnomalyModel {
    /// 平均ベクトル
    mean: Option<Array1<f32>>,
    /// 共分散行列
    cov_inv: Option<Array2<f32>>,
}

impl AnomalyModel {
    /// 新しい異常検出モデルを作成
    fn new() -> Self {
        Self {
            mean: None,
            cov_inv: None,
        }
    }
    
    /// 異常スコアを計算
    fn compute_anomaly_score(&self, features: &Array1<f32>) -> f32 {
        match (&self.mean, &self.cov_inv) {
            (Some(mean), Some(cov_inv)) => {
                // マハラノビス距離を計算
                let diff = features - mean;
                let dist = diff.dot(cov_inv).dot(&diff);
                dist
            }
            _ => {
                // モデルが初期化されていない場合は0を返す
                0.0
            }
        }
    }
    
    /// モデルを更新
    fn update(&mut self, features: &[Array1<f32>]) {
        if features.is_empty() {
            return;
        }
        
        let n_samples = features.len();
        let n_features = features[0].len();
        
        // 特徴量を行列に変換
        let mut data = Array2::zeros((n_samples, n_features));
        for (i, feat) in features.iter().enumerate() {
            data.row_mut(i).assign(feat);
        }
        
        // 平均を計算
        let mean = data.mean_axis(Axis(0)).unwrap();
        
        // 共分散行列を計算
        let mut cov = Array2::zeros((n_features, n_features));
        for i in 0..n_samples {
            let diff = &data.row(i) - &mean;
            let outer = diff.clone().insert_axis(Axis(1)).dot(&diff.clone().insert_axis(Axis(0)));
            cov = cov + outer;
        }
        cov = cov / (n_samples as f32 - 1.0);
        
        // 共分散行列の逆行列を計算（簡略化のため単位行列で代用）
        let cov_inv = Array2::eye(n_features);
        
        self.mean = Some(mean);
        self.cov_inv = Some(cov_inv);
    }
}