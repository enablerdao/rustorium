//! ネットワーク層
//! 
//! QUICとRedpandaを使用した高性能なネットワーク通信を提供します。

use anyhow::Result;
use quinn::{Endpoint, ServerConfig, ClientConfig};
use redpanda::client::{Producer, Consumer};
use tracing::{info, warn, error};

/// ネットワークマネージャ
pub struct NetworkManager {
    quic_endpoint: Endpoint,
    producer: Producer,
    consumer: Consumer,
}

impl NetworkManager {
    /// 新しいネットワークマネージャを作成
    pub async fn new() -> Result<Self> {
        info!("Initializing network manager...");
        
        // QUICエンドポイントの設定
        let (endpoint, _server_cert) = Self::configure_quic().await?;
        
        // Redpandaクライアントの設定
        let producer = Producer::new().await?;
        let consumer = Consumer::new().await?;
        
        Ok(Self {
            quic_endpoint: endpoint,
            producer,
            consumer,
        })
    }
    
    /// ネットワークを開始
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting network...");
        
        // QUICリスナーの開始
        self.start_quic_listener().await?;
        
        // Redpandaの接続開始
        self.connect_redpanda().await?;
        
        info!("Network started successfully");
        Ok(())
    }
    
    /// ネットワークを停止
    pub async fn stop(&mut self) -> Result<()> {
        info!("Stopping network...");
        
        // Redpandaの切断
        self.disconnect_redpanda().await?;
        
        // QUICの停止
        self.stop_quic().await?;
        
        info!("Network stopped successfully");
        Ok(())
    }
    
    /// QUICエンドポイントの設定
    async fn configure_quic() -> Result<(Endpoint, Vec<u8>)> {
        // 証明書の生成
        let cert = rcgen::generate_simple_self_signed(vec!["rustorium".into()])?;
        let cert_der = cert.serialize_der()?;
        let priv_key = cert.serialize_private_key_der();
        
        // サーバー設定
        let mut server_config = ServerConfig::with_single_cert(
            vec![rustls::Certificate(cert_der.clone())],
            rustls::PrivateKey(priv_key)
        )?;
        
        // クライアント設定
        let client_config = ClientConfig::new(Arc::new(
            rustls::ClientConfig::builder()
                .with_safe_defaults()
                .with_custom_certificate_verifier(Arc::new(SkipServerVerification))
                .with_no_client_auth()
        ));
        
        // エンドポイントの作成
        let mut endpoint = Endpoint::server(server_config, "0.0.0.0:0".parse()?)?;
        endpoint.set_default_client_config(client_config);
        
        Ok((endpoint, cert_der))
    }
    
    /// QUICリスナーの開始
    async fn start_quic_listener(&mut self) -> Result<()> {
        // 接続ハンドラーの設定
        tokio::spawn(async move {
            while let Some(conn) = self.quic_endpoint.accept().await {
                let conn = conn.await.expect("Connection failed");
                tokio::spawn(handle_connection(conn));
            }
        });
        
        Ok(())
    }
    
    /// QUICの停止
    async fn stop_quic(&mut self) -> Result<()> {
        // エンドポイントの停止
        self.quic_endpoint.close(0u32.into(), b"Shutdown");
        Ok(())
    }
    
    /// Redpandaへの接続
    async fn connect_redpanda(&mut self) -> Result<()> {
        // プロデューサーの接続
        self.producer.connect().await?;
        
        // コンシューマーの接続
        self.consumer.connect().await?;
        
        Ok(())
    }
    
    /// Redpandaからの切断
    async fn disconnect_redpanda(&mut self) -> Result<()> {
        // プロデューサーの切断
        self.producer.disconnect().await?;
        
        // コンシューマーの切断
        self.consumer.disconnect().await?;
        
        Ok(())
    }
}

/// 接続ハンドラー
async fn handle_connection(conn: quinn::Connection) {
    while let Ok((mut send, mut recv)) = conn.accept_bi().await {
        // データの受信
        let mut data = Vec::new();
        if let Err(e) = recv.read_to_end(1024 * 1024).await.map(|d| data = d) {
            error!("Failed to read from stream: {}", e);
            continue;
        }
        
        // メッセージの処理
        match handle_message(&data).await {
            Ok(response) => {
                if let Err(e) = send.write_all(&response).await {
                    error!("Failed to send response: {}", e);
                }
            }
            Err(e) => {
                error!("Failed to handle message: {}", e);
            }
        }
    }
}

/// メッセージ処理
async fn handle_message(data: &[u8]) -> Result<Vec<u8>> {
    // TODO: メッセージ処理の実装
    Ok(data.to_vec())
}

/// 証明書検証をスキップするための実装
struct SkipServerVerification;

impl rustls::client::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_network_lifecycle() -> Result<()> {
        let mut network = NetworkManager::new().await?;
        
        // 起動テスト
        network.start().await?;
        
        // 停止テスト
        network.stop().await?;
        
        Ok(())
    }
}
