# Quick Start Tutorial

このチュートリアルでは、Rustoriumを使って最初のブロックチェーンアプリケーションを構築する方法を説明します。

## 前提条件

- Rust 1.75.0以上
- Node.js 18以上（フロントエンド用）
- Docker（オプション）

## 1. プロジェクトのセットアップ

### 1.1 開発環境の準備

```bash
# Rustoriumのインストール
curl -sSf https://raw.githubusercontent.com/enablerdao/rustorium/main/scripts/install.sh | bash

# 開発用ディレクトリの作成
mkdir my-dapp
cd my-dapp

# フロントエンドの初期化
npm create vite@latest frontend -- --template react-ts
cd frontend
npm install
```

### 1.2 スマートコントラクトの作成

`contracts/counter.rs`:
```rust
use rustorium_sdk::{Contract, State};

#[derive(Contract)]
pub struct Counter {
    count: State<i32>,
}

#[contract]
impl Counter {
    pub fn new() -> Self {
        Self {
            count: State::new(0),
        }
    }

    pub fn increment(&mut self) {
        *self.count += 1;
    }

    pub fn decrement(&mut self) {
        *self.count -= 1;
    }

    pub fn get_count(&self) -> i32 {
        *self.count
    }
}
```

### 1.3 バックエンドの設定

`src/main.rs`:
```rust
use rustorium_sdk::{Node, Config};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // ノードの設定
    let config = Config::development()
        .with_data_dir("/tmp/my-dapp/data")
        .with_port(9070)
        .build()?;

    // ノードの起動
    let node = Node::new(config).await?;
    
    // コントラクトのデプロイ
    let counter = node.deploy_contract::<Counter>().await?;
    println!("Contract deployed at: {}", counter.address());

    // ノードの実行
    node.run().await?;

    Ok(())
}
```

## 2. フロントエンドの実装

### 2.1 依存関係のインストール

```bash
cd frontend
npm install @rustorium/sdk @rustorium/react ethers
```

### 2.2 Reactコンポーネントの作成

`src/App.tsx`:
```tsx
import { useState, useEffect } from 'react';
import { useRustorium, useContract } from '@rustorium/react';
import { Counter } from '../contracts/counter';

function App() {
  const { isConnected, connect } = useRustorium();
  const [count, setCount] = useState<number>(0);
  const counter = useContract<Counter>(COUNTER_ADDRESS);

  useEffect(() => {
    if (counter) {
      updateCount();
    }
  }, [counter]);

  async function updateCount() {
    const value = await counter.get_count();
    setCount(value);
  }

  async function handleIncrement() {
    await counter.increment();
    await updateCount();
  }

  async function handleDecrement() {
    await counter.decrement();
    await updateCount();
  }

  if (!isConnected) {
    return (
      <button onClick={connect}>
        Connect to Rustorium
      </button>
    );
  }

  return (
    <div>
      <h1>Counter: {count}</h1>
      <button onClick={handleIncrement}>+</button>
      <button onClick={handleDecrement}>-</button>
    </div>
  );
}

export default App;
```

### 2.3 環境設定

`.env`:
```env
VITE_RUSTORIUM_NODE=http://localhost:9070
VITE_COUNTER_ADDRESS=0x1234...  # デプロイ後のアドレス
```

## 3. アプリケーションの実行

### 3.1 ノードの起動

```bash
# 開発モードでノードを起動
cargo run -- --dev

# または、Dockerを使用
docker run -d \
  -p 9070:9070 \
  -p 9071:9071 \
  -p 9072:9072 \
  rustorium/node:latest --dev
```

### 3.2 フロントエンドの起動

```bash
cd frontend
npm run dev
```

これで、以下のURLでアプリケーションにアクセスできます：
- フロントエンド: http://localhost:5173
- ノードAPI: http://localhost:9071
- WebSocket: ws://localhost:9072

## 4. 機能の拡張

### 4.1 イベントの追加

`contracts/counter.rs`:
```rust
#[derive(Event)]
pub struct CounterChanged {
    pub old_value: i32,
    pub new_value: i32,
    pub changed_by: Address,
}

#[contract]
impl Counter {
    pub fn increment(&mut self) {
        let old_value = *self.count;
        *self.count += 1;
        self.emit(CounterChanged {
            old_value,
            new_value: *self.count,
            changed_by: msg::sender(),
        });
    }
}
```

### 4.2 イベントの購読

`src/App.tsx`:
```tsx
useEffect(() => {
  if (counter) {
    const subscription = counter.events.CounterChanged.subscribe(
      (event) => {
        console.log('Counter changed:', {
          oldValue: event.old_value,
          newValue: event.new_value,
          changedBy: event.changed_by,
        });
        setCount(event.new_value);
      }
    );

    return () => subscription.unsubscribe();
  }
}, [counter]);
```

## 5. テストの追加

### 5.1 コントラクトテスト

`tests/counter.rs`:
```rust
use rustorium_sdk::testing::*;

#[tokio::test]
async fn test_counter() {
    // テスト環境のセットアップ
    let env = TestEnv::new().await;
    
    // コントラクトのデプロイ
    let counter = env.deploy_contract::<Counter>().await;
    
    // 初期値の確認
    assert_eq!(counter.get_count().await, 0);
    
    // インクリメントのテスト
    counter.increment().await;
    assert_eq!(counter.get_count().await, 1);
    
    // デクリメントのテスト
    counter.decrement().await;
    assert_eq!(counter.get_count().await, 0);
}
```

### 5.2 フロントエンドテスト

`src/App.test.tsx`:
```tsx
import { render, screen, fireEvent } from '@testing-library/react';
import { RustoriumProvider } from '@rustorium/react';
import App from './App';

test('counter interactions', async () => {
  render(
    <RustoriumProvider>
      <App />
    </RustoriumProvider>
  );

  // 接続ボタンのテスト
  const connectButton = screen.getByText('Connect to Rustorium');
  await fireEvent.click(connectButton);

  // カウンターの操作テスト
  const incrementButton = screen.getByText('+');
  const decrementButton = screen.getByText('-');

  await fireEvent.click(incrementButton);
  expect(screen.getByText('Counter: 1')).toBeInTheDocument();

  await fireEvent.click(decrementButton);
  expect(screen.getByText('Counter: 0')).toBeInTheDocument();
});
```

## 6. デプロイ

### 6.1 コントラクトのデプロイ

```bash
# 本番環境へのデプロイ
rustorium contract deploy \
  --network mainnet \
  --contract Counter \
  --args '[]'
```

### 6.2 フロントエンドのデプロイ

```bash
# ビルド
cd frontend
npm run build

# Vercelへのデプロイ
vercel deploy dist
```

## 次のステップ

1. [高度な機能](../advanced/features.md)の追加
   - アクセス制御
   - アップグレード可能なコントラクト
   - 複雑なステート管理

2. [セキュリティ](../advanced/security.md)の強化
   - 入力バリデーション
   - エラー処理
   - 監査

3. [パフォーマンス](../advanced/performance.md)の最適化
   - キャッシュの活用
   - バッチ処理
   - 非同期処理

4. [モニタリング](../user-guide/monitoring.md)の設定
   - メトリクスの収集
   - アラートの設定
   - ログ管理

## サポート

問題が発生した場合は：

1. [FAQ](../appendix/faq.md)を確認
2. [Discord](https://discord.gg/rustorium)に参加
3. [GitHub](https://github.com/enablerdao/rustorium/issues)でissueを作成

