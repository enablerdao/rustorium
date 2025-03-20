//! 時刻同期モジュール
//! 
//! このモジュールは、ノード間の時刻同期を管理します。
//! 主な機能：
//! - NTPサーバーとの同期
//! - ノード間の時刻オフセット計算
//! - 時刻ドリフトの監視

use std::time::{SystemTime, Duration};
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use tracing::{info, warn, error};

/// 時刻同期の設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSyncConfig {
    /// NTPサーバーのリスト
    pub ntp_servers: Vec<String>,
    /// 同期間隔（秒）
    pub sync_interval: u64,
    /// 許容誤差（ミリ秒）
    pub max_offset_ms: u64,
    /// 再試行回数
    pub max_retries: u32,
}

impl Default for TimeSyncConfig {
    fn default() -> Self {
        Self {
            ntp_servers: vec![
                "pool.ntp.org".to_string(),
                "time.google.com".to_string(),
                "time.cloudflare.com".to_string(),
            ],
            sync_interval: 3600,  // 1時間
            max_offset_ms: 1000,  // 1秒
            max_retries: 3,
        }
    }
}

/// 時刻同期マネージャー
#[derive(Debug)]
pub struct TimeSyncManager {
    config: TimeSyncConfig,
    last_sync: Option<DateTime<Utc>>,
    current_offset: Duration,
}

impl TimeSyncManager {
    /// 新しい時刻同期マネージャーを作成
    pub fn new(config: TimeSyncConfig) -> Self {
        Self {
            config,
            last_sync: None,
            current_offset: Duration::from_secs(0),
        }
    }

    /// 時刻同期を実行
    pub async fn sync_time(&mut self) -> Result<()> {
        info!("Starting time synchronization...");

        let mut last_error = None;
        for _ in 0..self.config.max_retries {
            match self.try_sync().await {
                Ok(offset) => {
                    self.current_offset = offset;
                    self.last_sync = Some(Utc::now());

                    let offset_ms = offset.as_millis() as i64;
                    if offset_ms.abs() > self.config.max_offset_ms as i64 {
                        warn!("Large time offset detected: {}ms", offset_ms);
                    }

                    info!("Time synchronization successful. Offset: {}ms", offset_ms);
                    return Ok(());
                }
                Err(e) => {
                    last_error = Some(e);
                    warn!("Time sync attempt failed, retrying...");
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow!("Time sync failed after all retries")))
    }

    /// 単一のNTPサーバーとの同期を試行
    async fn try_sync(&self) -> Result<Duration> {
        // 各NTPサーバーを試行
        for server in &self.config.ntp_servers {
            match self.sync_with_server(server).await {
                Ok(offset) => return Ok(offset),
                Err(e) => {
                    warn!("Failed to sync with {}: {}", server, e);
                    continue;
                }
            }
        }

        Err(anyhow!("Failed to sync with any NTP server"))
    }

    /// 特定のNTPサーバーと同期
    async fn sync_with_server(&self, server: &str) -> Result<Duration> {
        use tokio::net::UdpSocket;


        // NTPパケットの準備
        let mut packet = [0u8; 48];
        packet[0] = 0x1B; // Version 3, Mode 3 (Client)

        // UDPソケットの作成
        let socket = UdpSocket::bind("0.0.0.0:0").await?;
        socket.connect((server, 123)).await?;

        // 送信時刻を記録
        let t1 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs() as f64;

        // パケットを送信
        socket.send(&packet).await?;

        // レスポンスを受信
        let mut response = [0u8; 48];
        socket.recv(&mut response).await?;

        // 受信時刻を記録
        let t4 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs() as f64;

        // NTPタイムスタンプを抽出
        let t2 = self.extract_timestamp(&response, 32);
        let t3 = self.extract_timestamp(&response, 40);

        // オフセットを計算
        let offset = ((t2 - t1) + (t3 - t4)) / 2.0;
        Ok(Duration::from_secs_f64(offset))
    }

    /// NTPパケットからタイムスタンプを抽出
    fn extract_timestamp(&self, packet: &[u8], offset: usize) -> f64 {
        let seconds = u32::from_be_bytes([
            packet[offset],
            packet[offset + 1],
            packet[offset + 2],
            packet[offset + 3],
        ]) as f64;
        let fraction = u32::from_be_bytes([
            packet[offset + 4],
            packet[offset + 5],
            packet[offset + 6],
            packet[offset + 7],
        ]) as f64 / 2f64.powi(32);
        seconds + fraction
    }

    /// 現在の時刻を取得（オフセットを考慮）
    pub fn now(&self) -> SystemTime {
        SystemTime::now()
            .checked_add(self.current_offset)
            .unwrap_or_else(|| {
                error!("Time overflow when applying offset");
                SystemTime::now()
            })
    }

    /// 同期が必要かどうかを確認
    pub fn needs_sync(&self) -> bool {
        match self.last_sync {
            Some(last) => {
                let elapsed = Utc::now()
                    .signed_duration_since(last)
                    .num_seconds() as u64;
                elapsed >= self.config.sync_interval
            }
            None => true,
        }
    }

    /// 最後の同期時刻を取得
    pub fn last_sync_time(&self) -> Option<DateTime<Utc>> {
        self.last_sync
    }

    /// 現在のオフセットを取得
    pub fn current_offset(&self) -> Duration {
        self.current_offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_time_sync() {
        let config = TimeSyncConfig::default();
        let mut manager = TimeSyncManager::new(config);

        // 初期状態の確認
        assert!(manager.needs_sync());
        assert_eq!(manager.last_sync_time(), None);
        assert_eq!(manager.current_offset(), Duration::from_secs(0));

        // 時刻同期の実行
        if let Err(e) = manager.sync_time().await {
            eprintln!("Time sync failed: {}", e);
            return;
        }

        // 同期後の状態確認
        assert!(!manager.needs_sync());
        assert!(manager.last_sync_time().is_some());
    }
}