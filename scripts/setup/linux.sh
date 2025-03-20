#!/bin/bash

set -e

# カラー出力の設定
GREEN='\033[0;32m'
NC='\033[0m'

echo -e "${GREEN}Setting up Linux environment...${NC}"

# パッケージマネージャの検出とインストール
if command -v apt-get &> /dev/null; then
    sudo apt-get update
    sudo apt-get install -y build-essential pkg-config libssl-dev
elif command -v yum &> /dev/null; then
    sudo yum groupinstall -y "Development Tools"
    sudo yum install -y openssl-devel pkg-config
elif command -v pacman &> /dev/null; then
    sudo pacman -Sy base-devel openssl pkg-config
else
    echo "Unsupported package manager. Please install the following packages manually:"
    echo "- C/C++ compiler (gcc/clang)"
    echo "- pkg-config"
    echo "- OpenSSL development files"
    exit 1
fi

# sccacheのインストールと設定
cargo install sccache

# シェルの設定
SHELL_CONFIG="# Rustorium環境設定
export RUSTC_WRAPPER=sccache"

if [ -f "$HOME/.bashrc" ]; then
    if ! grep -q "RUSTC_WRAPPER" "$HOME/.bashrc"; then
        echo "$SHELL_CONFIG" >> "$HOME/.bashrc"
    fi
fi

if [ -f "$HOME/.zshrc" ]; then
    if ! grep -q "RUSTC_WRAPPER" "$HOME/.zshrc"; then
        echo "$SHELL_CONFIG" >> "$HOME/.zshrc"
    fi
fi

# 現在のシェルに設定を適用
export RUSTC_WRAPPER=sccache

echo -e "${GREEN}Linux setup complete!${NC}"