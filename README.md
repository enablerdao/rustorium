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

#### Rustのインストール

##### Linux & macOS
```bash
# Rustupのインストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# 環境変数の設定
source $HOME/.cargo/env
```

##### Windows
1. [Rust インストーラー](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)をダウンロード
2. インストーラーを実行
3. 開発者用コマンドプロンプトを再起動

#### 必要なツール

##### Linux (Ubuntu/Debian)
```bash
# 開発ツールのインストール
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev
```

##### macOS
```bash
# Homebrewのインストール（未インストールの場合）
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
# 開発ツールのインストール
brew install openssl pkg-config
```

##### Windows
1. [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)をインストール
2. "C++ build tools"を選択してインストール

#### Rustoriumのインストール

```bash
# GitHubからソースコードを取得
git clone https://github.com/rustorium/rustorium.git
cd rustorium

# ビルドとインストール
cargo install --path .
```

### 🎮 ノードの起動

```bash
# シングルノード（最もシンプル）
cargo run

# リリースビルドで実行
cargo run --release

# データディレクトリを指定して起動
cargo run -- --data-dir my-node-data

# ポートを変更して起動（デフォルト: 9070）
cargo run -- --base-port 8000

# バックグラウンドで実行
cargo run -- --no-interactive
```

### 🔧 オプション
```bash
--data-dir        # データディレクトリ (default: data/<node-name>)
--base-port       # 基本ポート (default: 9070)
--no-interactive  # CUIを開かずにバックグラウンドで実行
```
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