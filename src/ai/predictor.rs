use crate::common::errors::LedgerError;
use crate::common::types::{Block, Transaction};
use ndarray::{Array1, Array2, Axis};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// 予測結果
#[derive(Debug, Clone)]
pub struct PredictionResult {
    /// 予測値
    pub value: f32,
    /// 信頼度
    pub confidence: f32,
}

/// 予測器
pub struct Predictor {
    /// 特徴量抽出器
    feature_extractor: PredictorFeatureExtractor,
    /// 予測モデル
    model: PredictionModel,
    /// 履歴データ
    history: Arc<RwLock<VecDeque<HistoryItem>>>,
    /// 履歴の最大サイズ
    max_history_size: usize,
}

/// 履歴アイテム
#[derive(Debug, Clone)]
struct HistoryItem {
    /// ブロック高
    height: u64,
    /// タイムスタンプ
    timestamp: u64,
    /// トランザクション数
    tx_count: usize,
    /// 平均ガス価格
    avg_gas_price: f32,
    /// 平均ブロック時間
    avg_block_time: f32,
}

impl Predictor {
    /// 新しい予測器を作成
    pub fn new(max_history_size: usize) -> Self {
        Self {
            feature_extractor: PredictorFeatureExtractor::new(),
            model: PredictionModel::new(),
            history: Arc::new(RwLock::new(VecDeque::with_capacity(max_history_size))),
            max_history_size,
        }
    }
    
    /// ブロック時間を予測
    pub async fn predict_block_time(&self) -> Result<PredictionResult, LedgerError> {
        // 特徴量を抽出
        let features = self.feature_extractor.extract_time_features(&self.history).await?;
        
        // 予測を実行
        let prediction = self.model.predict(&features);
        
        Ok(prediction)
    }
    
    /// ガス価格を予測
    pub async fn predict_gas_price(&self) -> Result<PredictionResult, LedgerError> {
        // 特徴量を抽出
        let features = self.feature_extractor.extract_gas_features(&self.history).await?;
        
        // 予測を実行
        let prediction = self.model.predict(&features);
        
        Ok(prediction)
    }
    
    /// 履歴にブロックを追加
    pub async fn add_block(&self, block: &Block) -> Result<(), LedgerError> {
        let mut history = self.history.write().await;
        
        // 前のブロックを取得
        let prev_timestamp = if let Some(prev) = history.back() {
            prev.timestamp
        } else {
            0
        };
        
        // ブロック時間を計算
        let block_time = if prev_timestamp > 0 {
            block.header.timestamp - prev_timestamp
        } else {
            0
        };
        
        // 平均ガス価格を計算（簡略化のため手数料を使用）
        let avg_gas_price = if !block.transactions.is_empty() {
            block.transactions.iter().map(|tx| tx.fee).sum::<u64>() as f32
                / block.transactions.len() as f32
        } else {
            0.0
        };
        
        // 履歴アイテムを作成
        let item = HistoryItem {
            height: block.header.height,
            timestamp: block.header.timestamp,
            tx_count: block.transactions.len(),
            avg_gas_price,
            avg_block_time: block_time as f32,
        };
        
        // 履歴に追加
        history.push_back(item);
        
        // 履歴サイズを制限
        if history.len() > self.max_history_size {
            history.pop_front();
        }
        
        Ok(())
    }
    
    /// モデルを更新
    pub async fn update_model(&mut self) -> Result<(), LedgerError> {
        let history = self.history.read().await;
        
        if history.len() < 10 {
            return Err(LedgerError::InvalidState(
                "Not enough history data to update model".to_string(),
            ));
        }
        
        // 時間予測用の特徴量とラベルを抽出
        let mut time_features = Vec::new();
        let mut time_labels = Vec::new();
        
        for i in 5..history.len() {
            let window: Vec<_> = history.range(i - 5..i).cloned().collect();
            let features = self.feature_extractor.extract_time_features_from_window(&window)?;
            time_features.push(features);
            time_labels.push(history[i].avg_block_time);
        }
        
        // ガス予測用の特徴量とラベルを抽出
        let mut gas_features = Vec::new();
        let mut gas_labels = Vec::new();
        
        for i in 5..history.len() {
            let window: Vec<_> = history.range(i - 5..i).cloned().collect();
            let features = self.feature_extractor.extract_gas_features_from_window(&window)?;
            gas_features.push(features);
            gas_labels.push(history[i].avg_gas_price);
        }
        
        // モデルを更新
        self.model.update(&time_features, &time_labels, &gas_features, &gas_labels);
        
        Ok(())
    }
}

/// 予測器用特徴量抽出器
struct PredictorFeatureExtractor {
    /// 特徴量の次元数
    feature_dim: usize,
}

impl PredictorFeatureExtractor {
    /// 新しい特徴量抽出器を作成
    fn new() -> Self {
        Self { feature_dim: 5 }
    }
    
    /// 時間予測用の特徴量を抽出
    async fn extract_time_features(
        &self,
        history: &Arc<RwLock<VecDeque<HistoryItem>>>,
    ) -> Result<Array1<f32>, LedgerError> {
        let history = history.read().await;
        
        if history.len() < 5 {
            return Err(LedgerError::InvalidState(
                "Not enough history data to extract features".to_string(),
            ));
        }
        
        let window: Vec<_> = history.iter().rev().take(5).cloned().collect();
        self.extract_time_features_from_window(&window)
    }
    
    /// 時間予測用の特徴量をウィンドウから抽出
    fn extract_time_features_from_window(
        &self,
        window: &[HistoryItem],
    ) -> Result<Array1<f32>, LedgerError> {
        let mut features = Array1::zeros(self.feature_dim);
        
        // 特徴量1: 直近のブロック時間
        features[0] = window[0].avg_block_time;
        
        // 特徴量2: 直近5ブロックの平均ブロック時間
        features[1] = window.iter().map(|item| item.avg_block_time).sum::<f32>() / window.len() as f32;
        
        // 特徴量3: 直近のトランザクション数
        features[2] = window[0].tx_count as f32;
        
        // 特徴量4: 直近5ブロックの平均トランザクション数
        features[3] = window.iter().map(|item| item.tx_count as f32).sum::<f32>() / window.len() as f32;
        
        // 特徴量5: トランザクション数の変化率
        let tx_count_prev = window[1].tx_count as f32;
        features[4] = if tx_count_prev > 0.0 {
            (window[0].tx_count as f32 - tx_count_prev) / tx_count_prev
        } else {
            0.0
        };
        
        Ok(features)
    }
    
    /// ガス予測用の特徴量を抽出
    async fn extract_gas_features(
        &self,
        history: &Arc<RwLock<VecDeque<HistoryItem>>>,
    ) -> Result<Array1<f32>, LedgerError> {
        let history = history.read().await;
        
        if history.len() < 5 {
            return Err(LedgerError::InvalidState(
                "Not enough history data to extract features".to_string(),
            ));
        }
        
        let window: Vec<_> = history.iter().rev().take(5).cloned().collect();
        self.extract_gas_features_from_window(&window)
    }
    
    /// ガス予測用の特徴量をウィンドウから抽出
    fn extract_gas_features_from_window(
        &self,
        window: &[HistoryItem],
    ) -> Result<Array1<f32>, LedgerError> {
        let mut features = Array1::zeros(self.feature_dim);
        
        // 特徴量1: 直近のガス価格
        features[0] = window[0].avg_gas_price;
        
        // 特徴量2: 直近5ブロックの平均ガス価格
        features[1] = window.iter().map(|item| item.avg_gas_price).sum::<f32>() / window.len() as f32;
        
        // 特徴量3: 直近のトランザクション数
        features[2] = window[0].tx_count as f32;
        
        // 特徴量4: 直近5ブロックの平均トランザクション数
        features[3] = window.iter().map(|item| item.tx_count as f32).sum::<f32>() / window.len() as f32;
        
        // 特徴量5: ガス価格の変化率
        let gas_price_prev = window[1].avg_gas_price;
        features[4] = if gas_price_prev > 0.0 {
            (window[0].avg_gas_price - gas_price_prev) / gas_price_prev
        } else {
            0.0
        };
        
        Ok(features)
    }
}

/// 予測モデル
struct PredictionModel {
    /// 時間予測用の重み
    time_weights: Option<Array1<f32>>,
    /// ガス予測用の重み
    gas_weights: Option<Array1<f32>>,
    /// 時間予測用のバイアス
    time_bias: f32,
    /// ガス予測用のバイアス
    gas_bias: f32,
}

impl PredictionModel {
    /// 新しい予測モデルを作成
    fn new() -> Self {
        Self {
            time_weights: None,
            gas_weights: None,
            time_bias: 0.0,
            gas_bias: 0.0,
        }
    }
    
    /// 予測を実行
    fn predict(&self, features: &Array1<f32>) -> PredictionResult {
        // 線形回帰モデルで予測
        let value = match &self.time_weights {
            Some(weights) => {
                let dot_product = weights.dot(features);
                dot_product + self.time_bias
            }
            None => {
                // モデルが初期化されていない場合はデフォルト値を返す
                15.0 // デフォルトのブロック時間（秒）
            }
        };
        
        // 信頼度は固定値（実際のモデルでは計算が必要）
        let confidence = 0.8;
        
        PredictionResult { value, confidence }
    }
    
    /// モデルを更新
    fn update(
        &mut self,
        time_features: &[Array1<f32>],
        time_labels: &[f32],
        gas_features: &[Array1<f32>],
        gas_labels: &[f32],
    ) {
        if time_features.is_empty() || gas_features.is_empty() {
            return;
        }
        
        // 時間予測モデルを更新
        self.update_time_model(time_features, time_labels);
        
        // ガス予測モデルを更新
        self.update_gas_model(gas_features, gas_labels);
    }
    
    /// 時間予測モデルを更新
    fn update_time_model(&mut self, features: &[Array1<f32>], labels: &[f32]) {
        if features.is_empty() || features.len() != labels.len() {
            return;
        }
        
        let n_samples = features.len();
        let n_features = features[0].len();
        
        // 特徴量を行列に変換
        let mut x = Array2::zeros((n_samples, n_features));
        for (i, feat) in features.iter().enumerate() {
            x.row_mut(i).assign(feat);
        }
        
        // ラベルをベクトルに変換
        let y = Array1::from_vec(labels.to_vec());
        
        // 線形回帰の重みを計算（簡略化のため単純な方法を使用）
        let x_mean = x.mean_axis(Axis(0)).unwrap();
        let y_mean = y.mean().unwrap();
        
        let mut numerator = Array1::zeros(n_features);
        let mut denominator = Array1::zeros(n_features);
        
        for i in 0..n_samples {
            let x_diff = &x.row(i) - &x_mean;
            let y_diff = y[i] - y_mean;
            
            numerator = numerator + &(&x_diff * y_diff);
            denominator = denominator + &(&x_diff * &x_diff);
        }
        
        // ゼロ除算を防ぐ
        for i in 0..n_features {
            if denominator[i] == 0.0 {
                denominator[i] = 1.0;
            }
        }
        
        let weights = numerator / denominator;
        let bias = y_mean - weights.dot(&x_mean);
        
        self.time_weights = Some(weights);
        self.time_bias = bias;
    }
    
    /// ガス予測モデルを更新
    fn update_gas_model(&mut self, features: &[Array1<f32>], labels: &[f32]) {
        if features.is_empty() || features.len() != labels.len() {
            return;
        }
        
        let n_samples = features.len();
        let n_features = features[0].len();
        
        // 特徴量を行列に変換
        let mut x = Array2::zeros((n_samples, n_features));
        for (i, feat) in features.iter().enumerate() {
            x.row_mut(i).assign(feat);
        }
        
        // ラベルをベクトルに変換
        let y = Array1::from_vec(labels.to_vec());
        
        // 線形回帰の重みを計算（簡略化のため単純な方法を使用）
        let x_mean = x.mean_axis(Axis(0)).unwrap();
        let y_mean = y.mean().unwrap();
        
        let mut numerator = Array1::zeros(n_features);
        let mut denominator = Array1::zeros(n_features);
        
        for i in 0..n_samples {
            let x_diff = &x.row(i) - &x_mean;
            let y_diff = y[i] - y_mean;
            
            numerator = numerator + &(&x_diff * y_diff);
            denominator = denominator + &(&x_diff * &x_diff);
        }
        
        // ゼロ除算を防ぐ
        for i in 0..n_features {
            if denominator[i] == 0.0 {
                denominator[i] = 1.0;
            }
        }
        
        let weights = numerator / denominator;
        let bias = y_mean - weights.dot(&x_mean);
        
        self.gas_weights = Some(weights);
        self.gas_bias = bias;
    }
}