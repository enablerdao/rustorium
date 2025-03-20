# 🚀 クイックスタート

## 📦 インストール

### 1️⃣ バイナリインストール
```bash
curl -sSf https://raw.githubusercontent.com/enablerdao/rustorium/main/scripts/install.sh | bash
```

### 2️⃣ ソースからビルド
```bash
# リポジトリのクローン
git clone https://github.com/enablerdao/rustorium.git
cd rustorium

# ビルド
cargo build --release

# バイナリのインストール
sudo cp target/release/rustorium /usr/local/bin/
```

## 🎮 基本的な使用方法

### 1️⃣ ノードの起動
```bash
# 開発モードで起動
rustorium --dev

# 本番モードで起動
rustorium --config /path/to/config.toml
```

### 2️⃣ ステータス確認
```bash
# ノードの状態を確認
rustorium status

# メトリクスを表示
rustorium monitor
```

### 3️⃣ トランザクションの送信
```bash
# トランザクションの作成と送信
rustorium tx send \
  --to 0x1234... \
  --amount 100 \
  --data "Hello, World!"

# トランザクションの確認
rustorium tx status <TX_HASH>
```

### 4️⃣ ブロックの探索
```bash
# 最新ブロックの確認
rustorium block latest

# 特定のブロックの詳細を表示
rustorium block <BLOCK_NUMBER>
```

## 🌐 Web UI

### 1️⃣ ダッシュボード
- http://localhost:9070 - メインダッシュボード
- http://localhost:9071 - APIエンドポイント
- http://localhost:9072 - WebSocketインターフェース

### 2️⃣ APIの使用
```bash
# REST API
curl http://localhost:9071/api/v1/status

# WebSocket
wscat -c ws://localhost:9072
```

## 📊 モニタリング

### 1️⃣ メトリクス
```bash
# Prometheusメトリクス
curl http://localhost:9070/metrics

# ノード統計
rustorium stats
```

### 2️⃣ ログ
```bash
# ログの表示
rustorium logs

# デバッグログの有効化
rustorium --log-level debug
```

## 🔧 設定例

### config.toml
```toml
[node]
name = "my-node"
data_dir = "/var/lib/rustorium"

[network]
port = 9070
bootstrap_nodes = [
    "node1.rustorium.network:9070",
    "node2.rustorium.network:9070"
]

[storage]
path = "/var/lib/rustorium/data"
max_size = "1TB"
compression = true

[web]
enabled = true
cors_origins = ["*"]
```

## 🛠 開発者ツール

### 1️⃣ テストネットの使用
```bash
# テストネットの起動
rustorium testnet start

# テストトークンの取得
rustorium faucet request
```

### 2️⃣ デバッグツール
```bash
# デバッグコンソール
rustorium debug console

# ネットワーク診断
rustorium debug network
```

## 📚 次のステップ

- [アーキテクチャ概要](architecture/overview.md)
- [APIリファレンス](api/reference.md)
- [開発ガイド](guides/development.md)
- [運用ガイド](guides/operations.md)

## 🆘 トラブルシューティング

### よくある問題

1. **ノードが起動しない**
   ```bash
   # ポート使用状況の確認
   rustorium check ports
   
   # 設定の検証
   rustorium check config
   ```

2. **同期が遅い**
   ```bash
   # ネットワーク診断
   rustorium network diagnose
   
   # ピア接続の確認
   rustorium network peers
   ```

3. **メモリ使用量が高い**
   ```bash
   # メモリ使用状況の確認
   rustorium monitor memory
   
   # キャッシュのクリア
   rustorium cache clear
   ```

### サポート

- [Discord](https://discord.gg/rustorium)
- [GitHub Issues](https://github.com/enablerdao/rustorium/issues)
- [ドキュメント](https://docs.rustorium.dev)
