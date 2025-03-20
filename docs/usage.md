# 使用方法

## 目次
- [基本的な使用方法](#基本的な使用方法)
- [設定オプション](#設定オプション)
- [インターフェース](#インターフェース)
- [運用ガイド](#運用ガイド)

## 基本的な使用方法

### シンプルな起動
```bash
rustorium
```

デフォルトでは以下のポートが使用されます：
- Web UI: 9070
- REST API: 9071
- WebSocket: 9072

### ポートの変更
```bash
rustorium --base-port 8000
```
この場合：
- Web UI: 8000
- REST API: 8001
- WebSocket: 8002

### バックグラウンド実行
```bash
rustorium --no-interactive
```

### データディレクトリの指定
```bash
rustorium --data-dir my-node-data
```

## 設定オプション

### コマンドラインオプション
```bash
--data-dir <DIR>      # データディレクトリ (default: data/<node-name>)
--base-port <PORT>    # 基本ポート (default: 9070)
--no-interactive      # CUIを開かずにバックグラウンドで実行
--log-level <LEVEL>   # ログレベル (debug, info, warn, error)
```

### 環境変数
```bash
RUST_LOG=debug        # ログレベルの設定
RUSTORIUM_HOME        # ホームディレクトリの設定
```

## インターフェース

### Web UI
- URL: `http://localhost:<base-port>`
- 機能：
  - ノードの状態確認
  - メトリクスの表示
  - ネットワーク情報
  - ブロックチェーン情報

### REST API
- URL: `http://localhost:<base-port+1>`
- エンドポイント：
  - `/api/metrics`: システムメトリクス
  - `/api/health`: ヘルスチェック
  - `/api/config`: 設定情報

### WebSocket
- URL: `ws://localhost:<base-port+2>`
- イベント：
  - `block`: 新しいブロック
  - `transaction`: 新しいトランザクション
  - `peer`: ピア接続/切断

### CLI
インタラクティブモードでは以下のメニューが利用可能：
- 📊 Node Status
- 🌍 Network Information
- 📦 Blockchain Information
- 🔗 Peer Management
- ⚙️ Settings

## 運用ガイド

### システム要件
- CPU: 2コア以上
- メモリ: 4GB以上
- ストレージ: 50GB以上
- ネットワーク: 安定した接続

### セキュリティ
- ファイアウォール設定：
  ```bash
  # Web UI
  sudo ufw allow 9070
  # REST API
  sudo ufw allow 9071
  # WebSocket
  sudo ufw allow 9072
  ```

### バックアップ
データディレクトリのバックアップ：
```bash
# 停止
rustorium stop

# バックアップ
tar -czf backup.tar.gz data/

# 再起動
rustorium
```

### ログ
- 場所: `data/logs/`
- ローテーション: 日次、最大7日分
- フォーマット: JSON

### モニタリング
- メトリクスエンドポイント: `/api/metrics`
- Prometheusエクスポーター: 有効
- Grafanaダッシュボード: 利用可能