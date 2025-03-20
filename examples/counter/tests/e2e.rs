use rustorium_sdk::{Node, Config, testing::*};
use anyhow::Result;
use tokio;
use std::time::Duration;

#[tokio::test]
async fn test_counter_e2e() -> Result<()> {
    // テスト環境のセットアップ
    let config = Config::development()
        .with_data_dir("/tmp/rustorium-test/data")
        .with_port(9070)
        .build()?;

    let node = Node::new(config).await?;
    
    // コントラクトのデプロイ
    let counter = node.deploy_contract::<Counter>().await?;
    println!("Contract deployed at: {}", counter.address());

    // フロントエンドのセットアップ
    let browser = TestBrowser::new().await?;
    browser.goto(&format!("http://localhost:5173?contract={}", counter.address())).await?;

    // 接続ボタンのクリック
    browser.click(".connect-button").await?;
    tokio::time::sleep(Duration::from_secs(1)).await;

    // 初期値の確認
    let count_text = browser.text("h2").await?;
    assert_eq!(count_text, "Count: 0");

    // インクリメントのテスト
    browser.click(".increment").await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let count_text = browser.text("h2").await?;
    assert_eq!(count_text, "Count: 1");

    // デクリメントのテスト
    browser.click(".decrement").await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let count_text = browser.text("h2").await?;
    assert_eq!(count_text, "Count: 0");

    // エラーケースのテスト
    node.stop().await?;
    browser.click(".increment").await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let error_text = browser.text(".error").await?;
    assert!(error_text.contains("Failed to increment"));

    // ノードの再起動
    node.start().await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    browser.click(".increment").await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let count_text = browser.text("h2").await?;
    assert_eq!(count_text, "Count: 1");

    // クリーンアップ
    browser.close().await?;
    node.stop().await?;

    Ok(())
}

#[tokio::test]
async fn test_counter_events() -> Result<()> {
    // テスト環境のセットアップ
    let config = Config::development()
        .with_data_dir("/tmp/rustorium-test/data")
        .with_port(9070)
        .build()?;

    let node = Node::new(config).await?;
    let counter = node.deploy_contract::<Counter>().await?;

    // イベントの購読
    let mut events = counter.events().await?;
    let mut event_count = 0;

    // 複数の操作を実行
    for _ in 0..3 {
        counter.increment().await?;
        if let Some(event) = events.next().await {
            event_count += 1;
            match event {
                CounterChanged { old_value, new_value, .. } => {
                    assert_eq!(new_value, old_value + 1);
                }
            }
        }
    }

    assert_eq!(event_count, 3);
    assert_eq!(counter.get_count().await?, 3);

    // クリーンアップ
    node.stop().await?;

    Ok(())
}

#[tokio::test]
async fn test_counter_concurrent() -> Result<()> {
    // テスト環境のセットアップ
    let config = Config::development()
        .with_data_dir("/tmp/rustorium-test/data")
        .with_port(9070)
        .build()?;

    let node = Node::new(config).await?;
    let counter = node.deploy_contract::<Counter>().await?;

    // 複数のクライアントを同時に実行
    let mut handles = vec![];
    for _ in 0..10 {
        let counter_clone = counter.clone();
        handles.push(tokio::spawn(async move {
            counter_clone.increment().await
        }));
    }

    // すべての操作が完了するのを待つ
    for handle in handles {
        handle.await??;
    }

    // 最終的な値を確認
    assert_eq!(counter.get_count().await?, 10);

    // クリーンアップ
    node.stop().await?;

    Ok(())
}

#[tokio::test]
async fn test_counter_performance() -> Result<()> {
    // テスト環境のセットアップ
    let config = Config::development()
        .with_data_dir("/tmp/rustorium-test/data")
        .with_port(9070)
        .build()?;

    let node = Node::new(config).await?;
    let counter = node.deploy_contract::<Counter>().await?;

    // パフォーマンス測定
    let start = std::time::Instant::now();
    for _ in 0..100 {
        counter.increment().await?;
    }
    let duration = start.elapsed();

    println!("100 operations took: {:?}", duration);
    println!("Average time per operation: {:?}", duration / 100);

    // 結果の検証
    assert_eq!(counter.get_count().await?, 100);
    assert!(duration < Duration::from_secs(10), "Performance test took too long");

    // クリーンアップ
    node.stop().await?;

    Ok(())
}
