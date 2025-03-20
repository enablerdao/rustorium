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

    # rustoriumコマンドをグローバルに利用可能にする
    if [ -d "$HOME/.cargo/bin" ]; then
        if ! command -v rustorium &> /dev/null; then
            echo "Creating rustorium command..."
            cat > "$HOME/.cargo/bin/rustorium" << 'EOF'
#!/bin/bash
if [ -f "$(dirname $(which cargo))/../lib/rustlib/uninstall.sh" ]; then
    RUST_DIR="$(dirname $(which cargo))/.."
else
    RUST_DIR="$HOME/.cargo"
fi

# サーバーの状態確認
check_server() {
    if command -v pgrep &> /dev/null; then
        pgrep -f "rustorium.*--base-port" > /dev/null
        return $?
    else
        ps aux | grep -v grep | grep "rustorium.*--base-port" > /dev/null
        return $?
    fi
}

# サーバーの起動
start_server() {
    if ! check_server; then
        echo "Starting Rustorium server..."
        if [ -f "$PWD/Cargo.toml" ]; then
            # ソースディレクトリ内の場合
            cargo run -- "$@" &
        else
            # インストール済みバイナリを使用
            "$RUST_DIR/bin/rustorium" "$@" &
        fi
        sleep 2
        if check_server; then
            echo "Server started successfully"
        else
            echo "Failed to start server"
            return 1
        fi
    else
        echo "Server is already running"
    fi
}

# メイン処理
if [ "$1" = "stop" ]; then
    if check_server; then
        pkill -f "rustorium.*--base-port"
        echo "Server stopped"
    else
        echo "Server is not running"
    fi
elif [ "$1" = "restart" ]; then
    if check_server; then
        pkill -f "rustorium.*--base-port"
        echo "Server stopped"
    fi
    start_server "$@"
elif [ "$1" = "status" ]; then
    if check_server; then
        echo "Server is running"
        ps aux | grep -v grep | grep "rustorium.*--base-port"
    else
        echo "Server is not running"
    fi
else
    start_server "$@"
fi
EOF
            chmod +x "$HOME/.cargo/bin/rustorium"
        fi
    fi

    echo -e "${GREEN}Installation complete!${NC}"
    
    # サーバーが実行中でなければ起動
    if ! pgrep -f "rustorium.*--base-port" > /dev/null; then
        echo -e "${BLUE}Starting Rustorium server...${NC}"
        rustorium --no-interactive &
        sleep 2
        echo -e "${GREEN}Server started successfully${NC}"
    else
        echo -e "${BLUE}Rustorium server is already running${NC}"
    fi

    echo -e "\n${GREEN}Available commands:${NC}"
    echo -e "  ${BLUE}rustorium${NC}          - Start the server"
    echo -e "  ${BLUE}rustorium stop${NC}     - Stop the server"
    echo -e "  ${BLUE}rustorium restart${NC}  - Restart the server"
    echo -e "  ${BLUE}rustorium status${NC}   - Check server status"
}

main