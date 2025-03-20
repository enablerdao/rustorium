# 🚀 Rustorium

<div align="center">

**次世代の分散型インフラストラクチャ**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

</div>

---

## 💫 Rustoriumとは

Rustoriumは、Rustで実装された高性能な分散型インフラストラクチャです。

### 🌟 主な特徴

- **⚡️ 高速処理**: 0.5秒での取引確定
- **🛡️ 堅牢なセキュリティ**: 異常検知と自動防御
- **🌈 開発者フレンドリー**: 直感的なAPI

### 🦀 Rustパワード
- 🔒 安全性と高速性の両立
- 🧬 最適化された並列処理
- 🌍 WebAssembly対応

## 🚀 クイックスタート

### 📦 インストール
```bash
curl -sSf https://raw.githubusercontent.com/enablerdao/rustorium/main/scripts/install.sh | bash
```

詳細なインストール手順は[こちら](docs/installation.md)をご覧ください。

### 🎮 ノードの管理
```bash
# 起動
rustorium

# 停止
rustorium stop

# 再起動
rustorium restart

# 状態確認
rustorium status

# オプション付きで起動
rustorium --base-port 8000          # ポート変更（デフォルト: 9070-9072）
rustorium --no-interactive          # バックグラウンドで実行
rustorium --data-dir my-node-data   # データディレクトリを指定
```

インストール後、サーバーは自動的にバックグラウンドで起動します。

詳細な使用方法は[こちら](docs/usage.md)をご覧ください。
```

### 🎛️ ノードの管理

ノードの起動後、以下のインターフェースを使用できます：

#### 🌐 Web UI
ノード起動時に表示されるURLにアクセスし、Web UIから操作できます：
- ノードの状態確認
- メトリクスの表示
- ネットワーク情報
- ブロックチェーン情報

#### 💻 CLI
インタラクティブモードでは、以下のメニューが利用可能です：
- 📊 Node Status
- 🌍 Network Information
- 📦 Blockchain Information
- 🔗 Peer Management
- ⚙️ Settings

#### 🔌 API/WebSocket
各インターフェースのデフォルトポート：
- Web UI: 9070
- REST API: 9071
- WebSocket: 9072

## 📚 ドキュメント

### 🎓 はじめに
- [アーキテクチャ概要](docs/architecture/overview.md)
- [開発環境の構築](docs/guides/installation.md)
- [APIリファレンス](docs/api/reference.md)

### 👨‍💻 開発者向け
- [コントリビューションガイド](CONTRIBUTING.md)
- [トラブルシューティング](docs/troubleshooting.md)

## 📜 ライセンス

Rustoriumは[MITライセンス](LICENSE)の下で公開されています。

---

<div align="center">

**[🌟 GitHubでスターを付ける](https://github.com/rustorium/rustorium)**

</div>