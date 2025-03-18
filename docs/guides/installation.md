# Rustorium インストールガイド

このガイドでは、Rustoriumをインストールして実行するための手順を説明します。

## 必要条件

Rustoriumを実行するには、以下のソフトウェアが必要です：

- **Rust 1.70以上**: 最新の安定版Rustを推奨します
- **Cargo**: Rustのパッケージマネージャー（通常はRustとともにインストールされます）
- **libclang-dev**: RocksDBの依存関係
- **CMake**: ビルドシステム
- **Git**: ソースコード管理

## インストール手順

### 1. Rustのインストール

まだRustをインストールしていない場合は、以下のコマンドでインストールできます：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

インストール後、以下のコマンドでRustのバージョンを確認します：

```bash
rustc --version
cargo --version
```

### 2. 依存関係のインストール

#### Debian/Ubuntu

```bash
sudo apt update
sudo apt install -y build-essential libclang-dev cmake pkg-config libssl-dev
```

#### macOS

```bash
brew install cmake llvm openssl
```

#### Windows

Windows上での開発は、Windows Subsystem for Linux (WSL)の使用を推奨します。WSL内でDebian/Ubuntuの手順に従ってください。

### 3. Rustoriumのソースコードを取得

```bash
git clone https://github.com/enablerdao/rustorium.git
cd rustorium
```

### 4. Rustoriumのビルド

```bash
# リリースビルド
cargo build --release

# または開発ビルド
cargo build
```

ビルドが完了すると、バイナリは`target/release/`または`target/debug/`ディレクトリに生成されます。

### 5. WebUIのビルド

WebUIをビルドするには、以下のコマンドを実行します：

```bash
./build_web.sh
```

## 実行方法

### スタンドアロンモードでの実行

すべてのコンポーネントを一度に起動するには、以下のコマンドを実行します：

```bash
./run_all.sh
```

これにより、APIサーバーとWebUIの両方が起動します。デフォルトでは、以下のポートが使用されます：

- APIサーバー: http://localhost:51055
- WebUI: http://localhost:57620

### 個別コンポーネントの実行

#### APIサーバーのみ起動

```bash
cd standalone_api
cargo run
```

#### WebUIのみ起動

```bash
cd web_ui
cargo run
```

## Docker環境での実行

Dockerを使用してRustoriumを実行することもできます。

### Dockerイメージのビルド

```bash
docker build -t rustorium .
```

### Dockerコンテナの実行

```bash
docker run -p 51055:51055 -p 57620:57620 rustorium
```

## トラブルシューティング

### ビルドエラー

ビルド中にエラーが発生した場合は、以下を確認してください：

1. Rustのバージョンが最新であることを確認：
   ```bash
   rustup update
   ```

2. 依存関係が正しくインストールされていることを確認

3. キャッシュをクリアして再ビルド：
   ```bash
   cargo clean
   cargo build
   ```

### 実行時エラー

1. ポートが既に使用されている場合は、設定ファイルで別のポートを指定してください。

2. パーミッションエラーが発生した場合は、適切な権限があることを確認してください。

3. ログを確認して詳細なエラーメッセージを確認してください。

## 次のステップ

- [基本的な使い方](./basic-usage.md)を確認する
- [設定ガイド](./configuration.md)で詳細な設定方法を学ぶ
- [開発ガイド](./development.md)でRustoriumの開発に参加する方法を学ぶ