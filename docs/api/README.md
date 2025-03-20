<div align="center">

# 📡 API リファレンス

**Rustorium API v1.0**

<img src="../images/api-overview.png" alt="API Overview" width="800px">

</div>

---

## 📖 目次

- [概要](#-概要)
- [認証](#-認証)
- [エンドポイント](#-エンドポイント)
- [WebSocket](#-websocket)
- [GraphQL](#-graphql)
- [エラー処理](#-エラー処理)
- [レート制限](#-レート制限)
- [SDKs](#-sdks)

---

## 🌟 概要

Rustoriumは3つのAPIインターフェースを提供します：

1. **REST API** (HTTP/2)
   - 標準的なRESTful操作
   - OpenAPI (Swagger) 準拠
   - JSON/Protocol Buffers対応

2. **WebSocket API**
   - リアルタイムイベント
   - 双方向通信
   - 自動再接続

3. **GraphQL API**
   - 柔軟なクエリ
   - スキーマ駆動開発
   - 型安全

---

## 🔑 認証

### APIキーの取得

```bash
# APIキーの生成
rustorium key generate --name "My App"

# APIキーの一覧表示
rustorium key list

# APIキーの無効化
rustorium key revoke <KEY_ID>
```

### 認証ヘッダー

```http
Authorization: Bearer <API_KEY>
X-Rustorium-Version: 2024-01
```

### アクセス制御

```yaml
# アクセス制御の設定例
permissions:
  read:
    - transactions
    - blocks
    - state
  write:
    - transactions
  admin:
    - nodes
    - shards
```

---

## 🔌 エンドポイント

### トランザクション

```http
# トランザクションの送信
POST /api/v1/transactions
Content-Type: application/json

{
  "data": "base64_encoded_data",
  "location": {
    "latitude": 35.6895,
    "longitude": 139.6917,
    "region": "asia-northeast"
  }
}

# レスポンス
{
  "tx_hash": "0x1234...",
  "status": "pending",
  "timestamp": "2024-01-23T12:34:56Z"
}
```

### ブロック

```http
# 最新ブロックの取得
GET /api/v1/blocks/latest

# 特定のブロックの取得
GET /api/v1/blocks/{block_number}

# ブロック範囲の取得
GET /api/v1/blocks?start=1000&end=2000
```

### ステート

```http
# ステートの取得
GET /api/v1/state/{key}

# ステートの更新
PUT /api/v1/state/{key}
Content-Type: application/json

{
  "value": "new_value",
  "proof": {
    "root": "0x1234...",
    "path": ["0xabcd...", "0xef12..."]
  }
}
```

---

## 🔄 WebSocket

### 接続

```javascript
const ws = new WebSocket('ws://localhost:9072/ws');

ws.onopen = () => {
  console.log('Connected to Rustorium');
  
  // サブスクリプションの開始
  ws.send(JSON.stringify({
    type: 'subscribe',
    channel: 'blocks'
  }));
};
```

### イベント

```javascript
ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  
  switch (data.type) {
    case 'new_block':
      console.log('New block:', data.block);
      break;
      
    case 'state_update':
      console.log('State update:', data.update);
      break;
      
    case 'tx_confirmed':
      console.log('Transaction confirmed:', data.tx);
      break;
  }
};
```

### エラー処理

```javascript
ws.onerror = (error) => {
  console.error('WebSocket error:', error);
};

ws.onclose = (event) => {
  console.log('Connection closed:', event.code, event.reason);
  // 再接続ロジック
};
```

---

## 📊 GraphQL

### スキーマ

```graphql
type Query {
  # ブロック関連
  block(number: Int!): Block
  blocks(start: Int!, end: Int!): [Block!]!
  latestBlock: Block!
  
  # トランザクション関連
  transaction(hash: String!): Transaction
  transactions(
    address: String
    status: TxStatus
    limit: Int = 10
  ): [Transaction!]!
  
  # ステート関連
  state(key: String!): State
  proof(key: String!): MerkleProof
}

type Mutation {
  # トランザクション
  submitTransaction(input: TxInput!): TxReceipt!
  
  # ステート
  updateState(key: String!, value: String!): StateUpdate!
}

type Subscription {
  # リアルタイムイベント
  onNewBlock: Block!
  onStateUpdate(key: String): StateUpdate!
  onTxConfirmed(hash: String): Transaction!
}
```

### クエリ例

```graphql
query GetBlockWithTransactions($number: Int!) {
  block(number: $number) {
    hash
    number
    timestamp
    transactions {
      hash
      from
      to
      value
      status
    }
    proof {
      root
      path
    }
  }
}

mutation SubmitTx($input: TxInput!) {
  submitTransaction(input: $input) {
    hash
    status
    timestamp
  }
}

subscription WatchBlocks {
  onNewBlock {
    number
    hash
    transactionCount
  }
}
```

---

## ⚠️ エラー処理

### エラーコード

| コード | 説明 | 対処方法 |
|--------|------|----------|
| 1000 | 不正なリクエスト | リクエストパラメータを確認 |
| 1001 | 認証エラー | APIキーを確認 |
| 1002 | 権限エラー | 必要な権限を確認 |
| 2000 | トランザクションエラー | トランザクションの内容を確認 |
| 2001 | ステートエラー | ステートの整合性を確認 |
| 3000 | ネットワークエラー | ネットワーク接続を確認 |
| 5000 | 内部エラー | サポートに連絡 |

### エラーレスポンス

```json
{
  "error": {
    "code": 1000,
    "message": "Invalid transaction format",
    "details": {
      "field": "data",
      "reason": "Must be base64 encoded"
    }
  }
}
```

---

## 🚦 レート制限

### 制限値

| エンドポイント | 制限 | 期間 |
|--------------|------|------|
| GET /api/* | 1000 | 分 |
| POST /api/* | 100 | 分 |
| WebSocket | 10 | 接続/秒 |
| GraphQL | 500 | 分 |

### ヘッダー

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1706011200
```

---

## 📦 SDKs

### 公式SDK

- [Rust](https://github.com/enablerdao/rustorium-rs)
- [TypeScript](https://github.com/enablerdao/rustorium-ts)
- [Python](https://github.com/enablerdao/rustorium-py)
- [Go](https://github.com/enablerdao/rustorium-go)

### コード例

```rust
use rustorium_sdk::{Client, Transaction};

#[tokio::main]
async fn main() -> Result<()> {
    // クライアントの初期化
    let client = Client::new()
        .with_endpoint("http://localhost:9071")
        .with_api_key(env::var("RUSTORIUM_API_KEY")?)
        .build()?;

    // トランザクションの送信
    let tx = Transaction::new()
        .with_data(data)
        .with_location(location)
        .build()?;

    let receipt = client.submit_transaction(tx).await?;
    println!("Transaction submitted: {}", receipt.hash);

    // イベントの購読
    let mut events = client.subscribe_events().await?;
    while let Some(event) = events.next().await {
        match event {
            Event::NewBlock(block) => {
                println!("New block: {}", block.number);
            }
            Event::StateUpdate(update) => {
                println!("State update: {:?}", update);
            }
        }
    }

    Ok(())
}
```

---

## 📚 関連ドキュメント

- [アーキテクチャ概要](../architecture/README.md)
- [開発ガイド](../guides/development.md)
- [APIチュートリアル](tutorials/api.md)
- [トラブルシューティング](../guides/troubleshooting.md)

