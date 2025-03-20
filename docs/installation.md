# インストールガイド

## 目次
- [自動インストール（推奨）](#自動インストール推奨)
- [手動インストール](#手動インストール)
  - [Linux](#linux)
  - [macOS](#macos)
  - [Windows](#windows)
- [トラブルシューティング](#トラブルシューティング)

## 自動インストール（推奨）

1行のコマンドでインストール：
```bash
curl -sSf https://raw.githubusercontent.com/enablerdao/rustorium/main/scripts/install.sh | bash
```

このスクリプトは以下の処理を自動的に行います：
- Rustのインストール（未インストールの場合）
- 必要な依存関係のインストール
- 環境変数の設定
- rustoriumのインストール

## 手動インストール

### Linux

#### 1. 依存関係のインストール
Ubuntu/Debian:
```bash
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev curl git
```

Fedora/RHEL:
```bash
sudo dnf groupinstall "Development Tools"
sudo dnf install openssl-devel pkg-config curl git
```

Arch Linux:
```bash
sudo pacman -Sy base-devel openssl pkg-config curl git
```

#### 2. Rustのインストール
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### 3. sccacheのインストール
```bash
cargo install sccache
echo 'export RUSTC_WRAPPER=sccache' >> ~/.bashrc  # または ~/.zshrc
source ~/.bashrc  # または source ~/.zshrc
```

#### 4. rustoriumのインストール
```bash
cargo install --git https://github.com/enablerdao/rustorium.git
```

### macOS

#### 1. Homebrewのインストール
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

#### 2. 依存関係のインストール
```bash
brew install openssl@3 pkg-config sccache
```

#### 3. OpenSSLの設定
```bash
cat << 'EOF' >> ~/.zshrc  # または ~/.bash_profile
# OpenSSL設定
export OPENSSL_DIR=/opt/homebrew/opt/openssl@3
export OPENSSL_ROOT_DIR=/opt/homebrew/opt/openssl@3
export OPENSSL_INCLUDE_DIR=/opt/homebrew/opt/openssl@3/include
export OPENSSL_LIB_DIR=/opt/homebrew/opt/openssl@3/lib
export PKG_CONFIG_PATH="/opt/homebrew/opt/openssl@3/lib/pkgconfig"

# sccache設定
export RUSTC_WRAPPER=sccache
EOF

source ~/.zshrc  # または source ~/.bash_profile
```

#### 4. 証明書の更新
```bash
/opt/homebrew/opt/openssl@3/bin/c_rehash
```

#### 5. Rustのインストール
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### 6. rustoriumのインストール
```bash
cargo install --git https://github.com/enablerdao/rustorium.git
```

### Windows

#### 1. 前提条件のインストール
1. [Git for Windows](https://gitforwindows.org/)をインストール
2. [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)をインストール
   - "C++ build tools"を選択

#### 2. Rustのインストール
1. [rustup-init.exe](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)をダウンロード
2. インストーラーを実行
3. 開発者用コマンドプロンプトを再起動

#### 3. sccacheのインストール
```bash
cargo install sccache
setx RUSTC_WRAPPER sccache
```

#### 4. rustoriumのインストール
```bash
cargo install --git https://github.com/enablerdao/rustorium.git
```

## トラブルシューティング

### OpenSSLの問題
- **症状**: `openssl`関連のエラー
- **解決**: 
  ```bash
  # macOS
  brew reinstall openssl@3
  # Linux
  sudo apt install libssl-dev  # または同等のパッケージ
  ```

### sccacheの問題
- **症状**: `sccache rustc -vV`エラー
- **解決**: 
  ```bash
  cargo uninstall sccache
  cargo install sccache
  ```

### ビルドエラー
- **症状**: `cargo install`失敗
- **解決**:
  ```bash
  rm -rf ~/.cargo/registry/cache
  cargo clean
  cargo install --force --git https://github.com/enablerdao/rustorium.git
  ```