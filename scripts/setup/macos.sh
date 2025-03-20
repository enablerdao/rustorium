#!/bin/bash

set -e

# カラー出力の設定
GREEN='\033[0;32m'
NC='\033[0m'

echo -e "${GREEN}Setting up macOS environment...${NC}"

# 必要なパッケージのインストール
brew install openssl@3 pkg-config sccache

# OpenSSLの設定
OPENSSL_CONFIG="export OPENSSL_DIR=/opt/homebrew/opt/openssl@3
export OPENSSL_ROOT_DIR=/opt/homebrew/opt/openssl@3
export OPENSSL_INCLUDE_DIR=/opt/homebrew/opt/openssl@3/include
export OPENSSL_LIB_DIR=/opt/homebrew/opt/openssl@3/lib
export PKG_CONFIG_PATH=\"/opt/homebrew/opt/openssl@3/lib/pkgconfig\"

# sccache設定
export RUSTC_WRAPPER=sccache"

# シェルの検出と設定ファイルの選択
if [ -n "$ZSH_VERSION" ]; then
    SHELL_RC="$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ]; then
    SHELL_RC="$HOME/.bash_profile"
else
    SHELL_RC="$HOME/.profile"
fi

# 設定を追加（重複を避ける）
if ! grep -q "OPENSSL_DIR" "$SHELL_RC"; then
    echo -e "\n# Rustorium環境設定" >> "$SHELL_RC"
    echo "$OPENSSL_CONFIG" >> "$SHELL_RC"
fi

# 現在のシェルに設定を適用
eval "$OPENSSL_CONFIG"

# 証明書の更新
/opt/homebrew/opt/openssl@3/bin/c_rehash

echo -e "${GREEN}macOS setup complete!${NC}"