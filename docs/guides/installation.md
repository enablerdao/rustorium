# 🛠️ 開発環境の構築

## 🚀 クイックスタート

### 1. CLIツールのインストール
```bash
# インストールスクリプトを実行
curl -L https://get.rustorium.org | bash

# パスを設定
export PATH="$HOME/.rustorium/bin:$PATH"

# インストールを確認
rustorium --version
```

### 2. 開発環境の準備
```bash
# 新しいプロジェクトを作成
rustorium init my-project
cd my-project

# 依存関係をインストール
rustorium install
```

## 💻 必要な環境

### システム要件
- **OS**: Ubuntu 20.04+ / macOS 12+
- **CPU**: 4コア以上推奨
- **メモリ**: 8GB以上推奨
- **ディスク**: 50GB以上の空き容量

### 必須ソフトウェア
- **Rust**: 1.70.0以上
- **Node.js**: 18.0.0以上
- **Docker**: 20.10.0以上
- **Git**: 2.30.0以上

## 🔧 詳細なセットアップ

### 1. Rustのインストール
```bash
# Rustupをインストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 必要なツールチェインを追加
rustup target add wasm32-unknown-unknown
rustup component add rustfmt clippy
```

### 2. Node.jsのインストール
```bash
# nvmを使用してインストール
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
source ~/.bashrc

# Node.jsをインストール
nvm install 18
nvm use 18
```

### 3. Dockerのセットアップ
```bash
# Dockerをインストール
curl -fsSL https://get.docker.com | sh

# ユーザーをdockerグループに追加
sudo usermod -aG docker $USER
```

## 🎮 開発環境の設定

### VSCode拡張機能
1. **必須拡張機能**
   - Rust Analyzer
   - WebAssembly
   - Solidity
   - Docker

2. **推奨設定**
   ```json
   {
     "rust-analyzer.checkOnSave.command": "clippy",
     "editor.formatOnSave": true
   }
   ```

### 開発用の設定ファイル
```yaml
# config.dev.yaml
network:
  name: "testnet"
  port: 8545

sharding:
  enabled: true
  initial_shards: 2

consensus:
  validator_count: 4
  block_time: 1000
```

## 🚀 テスト環境の起動

### 1. ローカルノードの起動
```bash
# シングルノードの開発環境
rustorium dev

# マルチノードのテストネット
rustorium testnet --nodes 4
```

### 2. スマートコントラクトのデプロイ
```bash
# コントラクトをコンパイル
rustorium contract compile

# テストネットにデプロイ
rustorium contract deploy --network testnet
```

### 3. テストの実行
```bash
# ユニットテスト
cargo test

# 統合テスト
rustorium test integration

# 負荷テスト
rustorium test benchmark
```

## 🔍 トラブルシューティング

### よくある問題

1. **ビルドエラー**
   ```bash
   # 依存関係をクリーン
   cargo clean
   
   # キャッシュを更新
   rustorium update
   ```

2. **ネットワークエラー**
   ```bash
   # ポートの使用状況を確認
   netstat -tulpn | grep 8545
   
   # ファイアウォール設定を確認
   sudo ufw status
   ```

3. **パフォーマンス問題**
   ```bash
   # システムリソースを確認
   rustorium metrics
   
   # ログを確認
   rustorium logs --level debug
   ```

## 📚 次のステップ

1. [クイックスタートガイド](../quickstart.md)を読む
2. [サンプルアプリ](../examples/)を試す
3. [API リファレンス](../api/reference.md)を確認

## 🆘 サポート

- **Discord**: [Rustoriumコミュニティ](https://discord.gg/rustorium)
- **Forum**: [開発者フォーラム](https://forum.rustorium.org)
- **GitHub**: [Issue Tracker](https://github.com/rustorium/rustorium/issues)

---

<div align="center">

**[📚 ドキュメントTOP](../README.md)** | **[💻 サンプルコード](../examples/)** | **[❓ FAQ](../faq.md)**

</div>