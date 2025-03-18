# Rustorium API リファレンス

このドキュメントでは、Rustorium APIの詳細なリファレンスを提供します。

## API概要

RustoriumのAPIは、RESTful原則に基づいて設計されています。すべてのAPIリクエストは以下のベースURLに対して行われます：

```
http://localhost:51055/api
```

レスポンスはJSON形式で返されます。

## 認証

現在、APIは認証なしでアクセス可能です。将来のバージョンでは、JWT（JSON Web Token）ベースの認証が実装される予定です。

## エンドポイント

### ブロック関連

#### ブロック一覧の取得

```
GET /api/blocks
```

**クエリパラメータ**:
- `limit`: 取得するブロック数（デフォルト: 10）
- `offset`: 開始オフセット（デフォルト: 0）

**レスポンス例**:
```json
{
  "blocks": [
    {
      "number": 10,
      "hash": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
      "timestamp": "2025-03-18T14:30:45Z",
      "transactions_count": 5,
      "size": 1024
    },
    {
      "number": 9,
      "hash": "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
      "timestamp": "2025-03-18T14:29:30Z",
      "transactions_count": 3,
      "size": 768
    }
  ],
  "total": 10
}
```

#### 最新ブロックの取得

```
GET /api/blocks/latest
```

**レスポンス例**:
```json
{
  "number": 10,
  "hash": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
  "parent_hash": "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
  "timestamp": "2025-03-18T14:30:45Z",
  "transactions_count": 5,
  "size": 1024,
  "gas_used": 125000,
  "gas_limit": 1000000,
  "validator": "0x0987654321fedcba0987654321fedcba09876543"
}
```

#### ブロック番号によるブロック取得

```
GET /api/blocks/{block_number}
```

**パスパラメータ**:
- `block_number`: ブロック番号

**レスポンス例**:
```json
{
  "number": 5,
  "hash": "0x9876543210abcdef9876543210abcdef9876543210abcdef9876543210abcdef",
  "parent_hash": "0xfedcba0987654321fedcba0987654321fedcba0987654321fedcba0987654321",
  "timestamp": "2025-03-18T14:25:15Z",
  "transactions_count": 2,
  "size": 512,
  "gas_used": 75000,
  "gas_limit": 1000000,
  "validator": "0x0987654321fedcba0987654321fedcba09876543"
}
```

#### ハッシュによるブロック取得

```
GET /api/blocks/by-hash/{block_hash}
```

**パスパラメータ**:
- `block_hash`: ブロックハッシュ

**レスポンス例**:
（上記と同様）

#### ブロック内のトランザクション取得

```
GET /api/blocks/{block_number}/transactions
```

**パスパラメータ**:
- `block_number`: ブロック番号

**クエリパラメータ**:
- `limit`: 取得するトランザクション数（デフォルト: 10）
- `offset`: 開始オフセット（デフォルト: 0）

**レスポンス例**:
```json
{
  "transactions": [
    {
      "id": "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
      "from": "0x1234567890abcdef1234567890abcdef12345678",
      "to": "0xabcdef1234567890abcdef1234567890abcdef12",
      "amount": "1000",
      "gas_price": "10",
      "gas_limit": "21000",
      "gas_used": "21000",
      "data": "",
      "nonce": 5,
      "status": "confirmed",
      "timestamp": "2025-03-18T14:30:15Z",
      "block_number": 10
    },
    {
      "id": "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
      "from": "0x9876543210fedcba9876543210fedcba98765432",
      "to": "0xfedcba9876543210fedcba9876543210fedcba98",
      "amount": "500",
      "gas_price": "10",
      "gas_limit": "21000",
      "gas_used": "21000",
      "data": "",
      "nonce": 3,
      "status": "confirmed",
      "timestamp": "2025-03-18T14:30:30Z",
      "block_number": 10
    }
  ],
  "total": 5
}
```

### トランザクション関連

#### トランザクションの送信

```
POST /api/transactions
```

**リクエスト例**:
```json
{
  "from": "0x1234567890abcdef1234567890abcdef12345678",
  "to": "0xabcdef1234567890abcdef1234567890abcdef12",
  "amount": "1000",
  "gas_price": "10",
  "gas_limit": "21000",
  "data": "",
  "nonce": 5
}
```

**レスポンス例**:
```json
{
  "id": "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
  "status": "pending"
}
```

#### トランザクションIDによるトランザクション取得

```
GET /api/transactions/{tx_id}
```

**パスパラメータ**:
- `tx_id`: トランザクションID

**レスポンス例**:
```json
{
  "id": "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
  "from": "0x1234567890abcdef1234567890abcdef12345678",
  "to": "0xabcdef1234567890abcdef1234567890abcdef12",
  "amount": "1000",
  "gas_price": "10",
  "gas_limit": "21000",
  "gas_used": "21000",
  "data": "",
  "nonce": 5,
  "status": "confirmed",
  "timestamp": "2025-03-18T14:30:15Z",
  "block_number": 10
}
```

#### 保留中のトランザクション取得

```
GET /api/transactions/pending
```

**クエリパラメータ**:
- `limit`: 取得するトランザクション数（デフォルト: 10）
- `offset`: 開始オフセット（デフォルト: 0）

**レスポンス例**:
```json
{
  "transactions": [
    {
      "id": "0xfedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210",
      "from": "0x1234567890abcdef1234567890abcdef12345678",
      "to": "0xabcdef1234567890abcdef1234567890abcdef12",
      "amount": "2000",
      "gas_price": "10",
      "gas_limit": "21000",
      "data": "",
      "nonce": 6,
      "status": "pending",
      "timestamp": "2025-03-18T14:35:45Z"
    }
  ],
  "total": 1
}
```

### アカウント関連

#### アカウント情報の取得

```
GET /api/accounts/{address}
```

**パスパラメータ**:
- `address`: アカウントアドレス

**レスポンス例**:
```json
{
  "address": "0x1234567890abcdef1234567890abcdef12345678",
  "balance": "10000",
  "nonce": 6,
  "code_hash": null,
  "storage_root": null,
  "created_at": "2025-03-18T10:00:00Z",
  "last_activity": "2025-03-18T14:30:15Z"
}
```

#### アカウントのトランザクション履歴取得

```
GET /api/accounts/{address}/transactions
```

**パスパラメータ**:
- `address`: アカウントアドレス

**クエリパラメータ**:
- `limit`: 取得するトランザクション数（デフォルト: 10）
- `offset`: 開始オフセット（デフォルト: 0）
- `direction`: 方向フィルタ（`all`, `incoming`, `outgoing`）

**レスポンス例**:
```json
{
  "transactions": [
    {
      "id": "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
      "from": "0x1234567890abcdef1234567890abcdef12345678",
      "to": "0xabcdef1234567890abcdef1234567890abcdef12",
      "amount": "1000",
      "gas_price": "10",
      "gas_used": "21000",
      "status": "confirmed",
      "timestamp": "2025-03-18T14:30:15Z",
      "block_number": 10
    }
  ],
  "total": 5
}
```

### スマートコントラクト関連

#### コントラクトのデプロイ

```
POST /api/contracts
```

**リクエスト例**:
```json
{
  "from": "0x1234567890abcdef1234567890abcdef12345678",
  "code": "0x608060405234801561001057600080fd5b50610150806100206000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c80632e64cec11461003b5780636057361d14610059575b600080fd5b610043610075565b60405161005091906100a1565b60405180910390f35b610073600480360381019061006e91906100ed565b61007e565b005b60008054905090565b8060008190555050565b6000819050919050565b61009b81610088565b82525050565b60006020820190506100b66000830184610092565b92915050565b600080fd5b6100ca81610088565b81146100d557600080fd5b50565b6000813590506100e7816100c1565b92915050565b600060208284031215610103576101026100bc565b5b6000610111848285016100d8565b9150509291505056fea2646970667358221220ec5ef79ea9c3f806626466c24f736c1a5a5e3b8bc2fb7c814fa4ecc6ff3a9c4d64736f6c63430008120033",
  "constructor_args": "",
  "gas_limit": "1000000",
  "gas_price": "10"
}
```

**レスポンス例**:
```json
{
  "address": "0x0123456789abcdef0123456789abcdef01234567",
  "transaction_id": "0xfedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210",
  "gas_used": "500000"
}
```

#### コントラクトの呼び出し

```
POST /api/contracts/{address}/call
```

**パスパラメータ**:
- `address`: コントラクトアドレス

**リクエスト例**:
```json
{
  "from": "0x1234567890abcdef1234567890abcdef12345678",
  "method": "store",
  "args": "0x000000000000000000000000000000000000000000000000000000000000002a",
  "gas_limit": "100000",
  "gas_price": "10",
  "value": "0"
}
```

**レスポンス例**:
```json
{
  "transaction_id": "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
  "result": "0x",
  "gas_used": "35000"
}
```

#### コントラクト情報の取得

```
GET /api/contracts/{address}
```

**パスパラメータ**:
- `address`: コントラクトアドレス

**レスポンス例**:
```json
{
  "address": "0x0123456789abcdef0123456789abcdef01234567",
  "creator": "0x1234567890abcdef1234567890abcdef12345678",
  "creation_transaction": "0xfedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210",
  "creation_block": 8,
  "code_hash": "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
  "vm_type": "evm"
}
```

### ネットワーク関連

#### ネットワークステータスの取得

```
GET /api/network/status
```

**レスポンス例**:
```json
{
  "peers_count": 5,
  "latest_block": 10,
  "transactions_per_second": 18.5,
  "average_block_time": 2.1,
  "network_load": 45,
  "shards": 4,
  "uptime": 3600
}
```

#### ピア情報の取得

```
GET /api/network/peers
```

**レスポンス例**:
```json
{
  "peers": [
    {
      "id": "QmYyQSo1c1Ym7orWxLYvCrM2EmxFTANf8wXmmE7DWjhx5N",
      "address": "/ip4/192.168.1.1/tcp/30333",
      "connected_since": "2025-03-18T13:30:45Z",
      "version": "rustorium/0.1.0",
      "latency": 15
    },
    {
      "id": "QmZMxNdpMkewiVZLMRxaNxUeZpDUb34pWjZ1kZvsd16Zic",
      "address": "/ip4/192.168.1.2/tcp/30333",
      "connected_since": "2025-03-18T13:35:12Z",
      "version": "rustorium/0.1.0",
      "latency": 25
    }
  ],
  "total": 5
}
```

#### シャード情報の取得

```
GET /api/network/shards
```

**レスポンス例**:
```json
{
  "shards": [
    {
      "id": 0,
      "transactions_count": 1250,
      "accounts_count": 500,
      "load": 35
    },
    {
      "id": 1,
      "transactions_count": 980,
      "accounts_count": 450,
      "load": 28
    },
    {
      "id": 2,
      "transactions_count": 1500,
      "accounts_count": 600,
      "load": 42
    },
    {
      "id": 3,
      "transactions_count": 1100,
      "accounts_count": 520,
      "load": 31
    }
  ],
  "total": 4
}
```

### AI分析関連

#### 異常検出結果の取得

```
GET /api/ai/anomalies
```

**クエリパラメータ**:
- `limit`: 取得する異常数（デフォルト: 10）
- `offset`: 開始オフセット（デフォルト: 0）

**レスポンス例**:
```json
{
  "anomalies": [
    {
      "tx_id": "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
      "score": 0.92,
      "reason": "Unusual gas price and transaction value combination",
      "timestamp": "2025-03-18T14:15:30Z"
    }
  ],
  "total": 1
}
```

#### ネットワーク予測の取得

```
GET /api/ai/predictions
```

**クエリパラメータ**:
- `horizon`: 予測ホライズン（ブロック数、デフォルト: 100）

**レスポンス例**:
```json
{
  "predictions": {
    "transaction_volume": [120, 125, 130, 135, 140],
    "network_load": [40, 42, 45, 47, 50],
    "gas_price": [12, 12.5, 13, 13.5, 14]
  },
  "timestamp": "2025-03-18T14:40:00Z",
  "horizon": 5
}
```

## エラーレスポンス

エラーが発生した場合、APIは適切なHTTPステータスコードとともに以下の形式のJSONレスポンスを返します：

```json
{
  "error": {
    "code": "not_found",
    "message": "Block with number 9999 not found"
  }
}
```

## 実装状況

### 現在実装されているAPI

#### ブロック関連
- ✅ `GET /blocks` - ブロック一覧の取得
- ✅ `GET /blocks/{block_id}` - 特定のブロック情報の取得

#### トランザクション関連
- ✅ `GET /transactions` - トランザクション一覧の取得
- ✅ `GET /transactions/{tx_id}` - 特定のトランザクション情報の取得
- ✅ `POST /transactions` - 新規トランザクションの送信

#### アカウント関連
- ✅ `GET /accounts` - アカウント一覧の取得
- ✅ `GET /accounts/{address}` - 特定のアカウント情報の取得
- ✅ `GET /accounts/{address}/transactions` - アカウントのトランザクション履歴取得

#### ネットワーク関連
- ✅ `GET /network/status` - ネットワークステータスの取得

### 開発中のAPI

- 🔄 `POST /accounts` - 新規アカウントの作成
- 🔄 `GET /blocks/by-hash/{block_hash}` - ハッシュによるブロック取得
- 🔄 `GET /blocks/{block_number}/transactions` - ブロック内のトランザクション取得
- 🔄 `GET /transactions/pending` - 保留中のトランザクション取得

### 今後実装予定のAPI

#### スマートコントラクト関連
- ⏳ `POST /contracts` - コントラクトのデプロイ
- ⏳ `POST /contracts/{address}/call` - コントラクトの呼び出し
- ⏳ `GET /contracts/{address}` - コントラクト情報の取得

#### 拡張ネットワーク関連
- ⏳ `GET /network/peers` - ピア情報の取得
- ⏳ `GET /network/shards` - シャード情報の取得

#### AI分析関連
- ⏳ `GET /ai/anomalies` - 異常検出結果の取得
- ⏳ `GET /ai/predictions` - ネットワーク予測の取得

#### トークン関連
- ⏳ `POST /tokens` - 新規トークンの作成
- ⏳ `GET /tokens/{token_id}` - トークン情報の取得
- ⏳ `GET /accounts/{address}/tokens` - アカウントのトークン残高取得