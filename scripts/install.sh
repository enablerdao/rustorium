#!/bin/bash

set -e

# カラー出力の設定
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}
╭────────────────────────────────────────────────────────────────────╮
│                                                                    │
│  ██████╗ ██╗   ██╗███████╗████████╗ ██████╗ ██████╗ ██╗██╗   ██╗███╗╗╗█╗╗│
│  ██╔══██╗██║   ██║██╔════╝╚══██╔══╝██╔═══██╗██╔══██╗██║██║   ██║████╗ ██║│
│  ██████╔╝██║   ██║███████╗   ██║   ██║   ██║██████╔╝██║██║   ██║██╔████║│
│  ██╔══██╗██║   ██║╚════██║   ██║   ██║   ██║██╔══██╗██║██║   ██║██║╚██╔╝│
│  ██║  ██║╚██████╔╝███████║   ██║   ╚██████╔╝██║  ██║██║╚██████╔╝██║ ╚═╝ │
│  ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝╚═╝ ╚═════╝ ╚═╝     │
│                                                                    │
│                          R U S T O R I U M v0.1.0                  │
│                                                                    │
╰────────────────────────────────────────────────────────────────────╯${NC}
"

# OSの検出
detect_os() {
    case "$(uname -s)" in
        Darwin*)
            echo "macos"
            ;;
        Linux*)
            echo "linux"
            ;;
        MINGW*|MSYS*|CYGWIN*)
            echo "windows"
            ;;
        *)
            echo "unknown"
            ;;
    esac
}

# 必要なコマンドの確認
check_command() {
    if ! command -v "$1" &> /dev/null; then
        echo -e "${RED}Error: $1 is not installed${NC}"
        return 1
    fi
}

# 必要なツールのインストール
install_prerequisites() {
    local os=$1
    case "$os" in
        macos)
            if ! command -v brew &> /dev/null; then
                echo "Installing Homebrew..."
                /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
            fi
            brew install curl git
            ;;
        linux)
            if command -v apt-get &> /dev/null; then
                sudo apt-get update
                sudo apt-get install -y curl git
            elif command -v yum &> /dev/null; then
                sudo yum install -y curl git
            elif command -v pacman &> /dev/null; then
                sudo pacman -Sy curl git
            fi
            ;;
        windows)
            echo "Please install Git for Windows from https://gitforwindows.org/"
            echo "Then run this script again from Git Bash"
            exit 1
            ;;
    esac
}

# Rustのインストール
install_rust() {
    if ! command -v rustup &> /dev/null; then
        echo "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
}

# OS固有のセットアップスクリプトを実行
run_os_setup() {
    local os=$1
    local setup_script="https://raw.githubusercontent.com/enablerdao/rustorium/main/scripts/setup/$os.sh"
    
    echo "Running $os-specific setup..."
    curl -sSf "$setup_script" | bash
}

# rustoriumのインストール
install_rustorium() {
    echo "Installing Rustorium..."
    cargo install --git https://github.com/enablerdao/rustorium.git
}

# メイン処理
main() {
    local os
    os=$(detect_os)
    
    if [ "$os" = "unknown" ]; then
        echo -e "${RED}Error: Unsupported operating system${NC}"
        exit 1
    fi

    echo -e "${GREEN}Installing Rustorium for $os...${NC}"

    # 必要なツールの確認とインストール
    install_prerequisites "$os"

    # 基本コマンドの確認
    check_command curl
    check_command git

    # Rustのインストール
    install_rust

    # OS固有のセットアップ
    run_os_setup "$os"

    # rustoriumのインストール
    install_rustorium

    echo -e "${GREEN}Installation complete!${NC}"
    echo -e "${BLUE}To start Rustorium, run: rustorium${NC}"
}

main