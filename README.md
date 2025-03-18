# RustLedger

RustLedgerは、Rustで実装された高性能な分散型台帳システムです。シャーディング、Avalanche型ゴシッププロトコル、マルチVM実行環境、DAGベース並列処理、AI処理層などの先進的な機能を備えています。

## 主な機能

- **シャーディング**: 一貫性ハッシュリングを使用したトランザクションの効率的な分散処理
- **Avalanche型ゴシッププロトコル**: 高速なコンセンサスと耐障害性
- **マルチVM実行環境**: EVM、WASM対応の柔軟な実行環境
- **ストレージエンジン**: RocksDBベースの高性能ストレージ
- **DAGベース並列処理**: 依存関係を考慮した効率的なトランザクション処理
- **AI処理層**: 異常検出と予測機能
- **Webインターフェース**: Rustで実装されたモダンなUI

## 必要条件

- Rust 1.70以上
- Cargo
- wasm-pack (WebUI用)
- libclang-dev (RocksDB依存)

## インストール

```bash
# リポジトリをクローン
git clone https://github.com/yourusername/rustledger.git
cd rustledger

# ビルド
cargo build --release

# WebUIをビルド
./build_web.sh
```

## 使用方法

### 新しい台帳の初期化

```bash
cargo run -- init --path ./data
```

### フルノードの起動

```bash
cargo run -- node --config config.toml
```

### APIサーバーのみ起動

```bash
cargo run -- api --port 8080
```

### Webインターフェースのみ起動

```bash
cargo run -- web --port 8081 --api-url http://localhost:8080
```

### カスタム起動オプション

```bash
# APIサーバーなしでノードを起動
cargo run -- node --config config.toml --api false

# WebUIなしでノードを起動
cargo run -- node --config config.toml --web false

# カスタムポートでノードを起動
cargo run -- node --config config.toml --api-port 9090 --web-port 9091
```

### トランザクションの送信

```bash
cargo run -- send-tx --from 0x1234567890abcdef1234567890abcdef12345678 --to 0xabcdef1234567890abcdef1234567890abcdef12 --amount 1000 --fee 10
```

## 設定

`config.toml`ファイルで以下の設定が可能です：

```toml
[node]
id = "node-1"
data_dir = "./data"
log_level = "info"

[network]
listen_addr = "0.0.0.0"
listen_port = 30333
bootstrap_nodes = []
max_peers = 50

[sharding]
shard_count = 4
rebalance_interval_sec = 3600

[consensus]
algorithm = "avalanche"
block_time_ms = 2000
validators = []
min_validators = 4
threshold_percentage = 67

[api]
enabled = true
listen_addr = "0.0.0.0"
listen_port = 8080
```

## アーキテクチャ

RustLedgerは以下のコンポーネントで構成されています：

1. **シャーディング層**: トランザクションを複数のシャードに分散
2. **ネットワーク層**: P2P通信とゴシッププロトコル
3. **コンセンサス層**: Avalancheベースのコンセンサスアルゴリズム
4. **実行層**: マルチVM対応のトランザクション実行環境
5. **ストレージ層**: 効率的なデータ永続化
6. **API層**: RESTful APIとWebインターフェース
7. **AI層**: 異常検出と予測機能

## ライセンス

MIT