use std::sync::Arc;
use anyhow::Result;
use tokio::sync::RwLock;
use serde_json::json;
use reqwest::Client;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures::StreamExt;
use crate::{
    core::{
        token::TokenManager,
        storage::RocksDBStorage,
        types::Address,
    },
    api::{ApiServer, ApiConfig},
};

/// ウォレットのテスト
#[tokio::test]
async fn test_wallet_api() -> Result<()> {
    // テスト用のストレージを作成
    let storage = Arc::new(RocksDBStorage::new("/tmp/rustorium_test")?);

    // トークンマネージャーを初期化
    let token_manager = Arc::new(TokenManager::new(storage.clone()));

    // APIサーバーを起動
    let api_config = ApiConfig {
        host: "127.0.0.1".to_string(),
        rest_port: 8001,
        ws_port: 8002,
        graphql_port: 8003,
        cors_origin: "*".to_string(),
        rate_limit: 1000,
    };

    let api_server = ApiServer::new(
        api_config.clone(),
        token_manager.clone(),
        Arc::new(RwLock::new(vec![])),
        Arc::new(RwLock::new(vec![])),
    );
    api_server.start().await?;

    // RESTful APIのテスト
    let client = Client::new();

    // ウォレットを作成
    let response = client.post("http://localhost:8001/wallets")
        .json(&json!({
            "name": "Test Wallet"
        }))
        .send()
        .await?;
    assert!(response.status().is_success());
    let data = response.json::<serde_json::Value>().await?;
    let address = data["address"].as_str().unwrap();
    assert_eq!(data["initial_balance"].as_u64().unwrap(), 100);

    // ウォレット情報を取得
    let response = client.get(&format!("http://localhost:8001/wallets/{}", address))
        .send()
        .await?;
    assert!(response.status().is_success());
    let data = response.json::<serde_json::Value>().await?;
    assert_eq!(data["balance"].as_u64().unwrap(), 50); // Bに50送金済み

    // WebSocketのテスト
    let (mut ws_stream, _) = connect_async("ws://localhost:8002").await?;

    // ウォレットを作成
    ws_stream.send(Message::Text(json!({
        "command": "Create",
        "params": {
            "name": "Test Wallet 2"
        }
    }).to_string())).await?;

    let response = ws_stream.next().await.unwrap()?;
    let data = serde_json::from_str::<serde_json::Value>(response.to_text()?)?;
    let address2 = data["data"]["address"].as_str().unwrap();
    assert_eq!(data["data"]["initial_balance"].as_u64().unwrap(), 100);

    // 残高の変更を購読
    ws_stream.send(Message::Text(json!({
        "command": "SubscribeBalance",
        "params": {
            "address": address2
        }
    }).to_string())).await?;

    // GraphQLのテスト
    let query = r#"
        mutation {
            createWallet(name: "Test Wallet 3") {
                address
                balance
            }
        }
    "#;

    let response = client.post("http://localhost:8003/graphql")
        .json(&json!({
            "query": query
        }))
        .send()
        .await?;
    assert!(response.status().is_success());
    let data = response.json::<serde_json::Value>().await?;
    let address3 = data["data"]["createWallet"]["address"].as_str().unwrap();
    assert_eq!(data["data"]["createWallet"]["balance"].as_u64().unwrap(), 100);

    // 残高の変更を購読
    let subscription = r#"
        subscription {
            balanceUpdates(address: "%s") {
                address
                balance
                tokenBalances {
                    tokenId
                    symbol
                    balance
                }
            }
        }
    "#.replace("%s", address3);

    let (mut ws_stream, _) = connect_async("ws://localhost:8003/graphql").await?;
    ws_stream.send(Message::Text(json!({
        "type": "connection_init"
    }).to_string())).await?;

    ws_stream.send(Message::Text(json!({
        "type": "start",
        "id": "1",
        "payload": {
            "query": subscription
        }
    }).to_string())).await?;

    // APIサーバーを停止
    api_server.stop().await?;

    Ok(())
}