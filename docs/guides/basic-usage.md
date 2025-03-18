# Rustorium 基本的な使い方

このガイドでは、Rustoriumの基本的な使い方について説明します。

## 起動と停止

### システム全体の起動

Rustoriumのすべてのコンポーネントを一度に起動するには、以下のコマンドを実行します：

```bash
./run_all.sh
```

これにより、以下のサービスが起動します：
- APIサーバー（デフォルトポート: 51055）
- WebUI（デフォルトポート: 57620）

### システムの停止

実行中のRustoriumを停止するには、ターミナルで`Ctrl+C`を押します。

## WebUIの使用

WebUIにアクセスするには、ブラウザで以下のURLを開きます：

```
http://localhost:57620
```

### ダッシュボード

ダッシュボードでは、以下の情報を確認できます：

- 最新ブロック番号
- 接続ピア数
- 保留中のトランザクション数
- システム稼働時間
- 最近のトランザクション
- ネットワークステータス（TPS、ブロック時間、ネットワーク負荷、シャード数）

### ブロック情報

「Blocks」タブでは、ブロックチェーン上のブロック情報を閲覧できます：

- ブロック番号
- ブロックハッシュ
- タイムスタンプ
- トランザクション数
- ブロックサイズ
- バリデータ情報

### トランザクション情報

「Transactions」タブでは、トランザクションの詳細を確認できます：

- トランザクションID
- 送信者と受信者のアドレス
- 金額
- ガス使用量
- ステータス（保留中、確定、失敗）
- タイムスタンプ

### アカウント管理

「Accounts」タブでは、アカウント情報を管理できます：

- アカウントの作成
- アカウントのインポート
- 残高の確認
- トランザクション履歴の表示

### トランザクションの送信

「Send Transaction」タブでは、新しいトランザクションを送信できます：

1. 送信元アドレスを選択
2. 送信先アドレスを入力
3. 送信金額を入力
4. 必要に応じてデータフィールドを入力
5. 「Send」ボタンをクリック

### ネットワーク可視化

「Network」タブでは、ネットワークの状態を視覚的に確認できます：

- ノードのネットワークマップ
- シャード情報
- ピア接続状態

### スマートコントラクト管理

「Smart Contracts」タブでは、スマートコントラクトを管理できます：

- コントラクトのデプロイ
- コントラクトの呼び出し
- コントラクトコードの表示
- コントラクトの検証

### 分析ダッシュボード

「Analytics」タブでは、ブロックチェーンの分析情報を確認できます：

- トランザクションボリュームのグラフ
- ブロック時間の推移
- ガス使用量の統計
- アカウントアクティビティ

### AI分析

「AI Insights」タブでは、AI処理層による分析結果を確認できます：

- 異常トランザクションの検出
- ネットワーク負荷予測
- トランザクションパターン分析

## APIの使用

Rustoriumは、外部アプリケーションとの連携のためのRESTful APIを提供しています。

### APIエンドポイント

APIサーバーのベースURLは以下の通りです：

```
http://localhost:51055/api
```

### 主要APIエンドポイント

#### ブロック情報の取得

```
GET /api/blocks/latest
GET /api/blocks/{block_number}
GET /api/blocks/{block_hash}
```

#### トランザクション情報の取得

```
GET /api/transactions/{tx_id}
GET /api/transactions/pending
GET /api/blocks/{block_number}/transactions
```

#### アカウント情報の取得

```
GET /api/accounts/{address}
GET /api/accounts/{address}/balance
GET /api/accounts/{address}/transactions
```

#### トランザクションの送信

```
POST /api/transactions
```

リクエスト例：
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

#### スマートコントラクトのデプロイ

```
POST /api/contracts
```

#### スマートコントラクトの呼び出し

```
POST /api/contracts/{address}/call
```

#### ネットワーク情報の取得

```
GET /api/network/status
GET /api/network/peers
GET /api/network/shards
```

## コマンドラインインターフェース

Rustoriumは、コマンドラインからの操作もサポートしています。

### アカウントの作成

```bash
cargo run -- account create
```

### アカウントの一覧表示

```bash
cargo run -- account list
```

### トランザクションの送信

```bash
cargo run -- send-tx --from 0x1234567890abcdef1234567890abcdef12345678 --to 0xabcdef1234567890abcdef1234567890abcdef12 --amount 1000 --fee 10
```

### ブロック情報の表示

```bash
cargo run -- block info --number 12345
```

### ノード情報の表示

```bash
cargo run -- node info
```

## 次のステップ

- [設定ガイド](./configuration.md)で詳細な設定方法を学ぶ
- [開発ガイド](./development.md)でRustoriumの開発に参加する方法を学ぶ
- [APIリファレンス](../api/reference.md)でAPIの詳細を確認する