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
    
    # アーキテクチャとOSの検出
    ARCH=$(uname -m)
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    
    # 事前ビルド済みバイナリのダウンロードを試みる
    BINARY_URL="https://github.com/enablerdao/rustorium/releases/latest/download/rustorium-${OS}-${ARCH}.tar.gz"
    
    # バイナリのインストール（高速パス）
    install_binary() {
        local url=$1
        echo "Using pre-built binary for ${OS}-${ARCH}..."
        
        # 一時ディレクトリの作成
        TMP_DIR=$(mktemp -d)
        cd "$TMP_DIR"
        
        # バイナリのダウンロードと展開（rocksdbを含む）
        echo "Downloading pre-built binary (includes rocksdb)..."
        if curl -L "$url" | tar xz; then
            # バイナリのインストール
            echo "Installing binary..."
            mkdir -p "$HOME/.cargo/bin"
            cp rustorium "$HOME/.cargo/bin/"
            chmod +x "$HOME/.cargo/bin/rustorium"
            
            # クリーンアップ
            cd - > /dev/null
            rm -rf "$TMP_DIR"
            return 0
        else
            cd - > /dev/null
            rm -rf "$TMP_DIR"
            return 1
        fi
    }

    # ソースからのビルド（遅いパス）
    build_from_source() {
        echo "Building from source..."
        echo "This might take a few minutes..."
        
        # 一時ディレクトリの作成
        TMP_DIR=$(mktemp -d)
        cd "$TMP_DIR"
        
        # ソースコードの取得
        git clone https://github.com/enablerdao/rustorium.git .
        
        # バンドルされたrocksdbを使用（バイナリに含める）
        echo "Building with bundled rocksdb (will be included in binary)..."
        FEATURES="--features bundled-rocksdb"
        
        # 高速ビルドの設定
        export RUSTFLAGS="-C target-cpu=native"
        
        # リリースビルドの作成（高速プロファイル使用）
        echo "Building optimized version..."
        if cargo build --profile fast $FEATURES; then
            # バイナリのインストール
            echo "Installing binary..."
            mkdir -p "$HOME/.cargo/bin"
            cp target/fast/rustorium "$HOME/.cargo/bin/"
            chmod +x "$HOME/.cargo/bin/rustorium"
            
            # クリーンアップ
            cd - > /dev/null
            rm -rf "$TMP_DIR"
            return 0
        else
            cd - > /dev/null
            rm -rf "$TMP_DIR"
            return 1
        fi
    }

    # バックグラウンドでのソースコードのクローン
    clone_source_background() {
        (
            CLONE_DIR="$HOME/.rustorium/source"
            mkdir -p "$CLONE_DIR"
            cd "$CLONE_DIR"
            if [ ! -d ".git" ]; then
                echo "Cloning source code in background..."
                git clone https://github.com/enablerdao/rustorium.git . > /dev/null 2>&1 &
            fi
        ) &
    }

    # まずバイナリを試す
    if curl --output /dev/null --silent --head --fail "$BINARY_URL"; then
        if install_binary "$BINARY_URL"; then
            # バイナリのインストールに成功
            # バックグラウンドでソースをクローン
            clone_source_background
            return 0
        fi
    fi

    # バイナリが利用できないか、インストールに失敗した場合はソースからビルド
    build_from_source
    
    # キャッシュディレクトリの作成
    mkdir -p "$HOME/.rustorium/cache"
    
    # rocksdbのキャッシュディレクトリ
    mkdir -p "$HOME/.rustorium/cache/rocksdb"
    
    # 設定ファイルの作成
    mkdir -p "$HOME/.rustorium/config"
    cat > "$HOME/.rustorium/config/rocksdb.toml" << EOF
[storage]
cache_size_mb = 512
max_open_files = 1000
use_fsync = false
optimize_for_point_lookup = true
increase_parallelism = true
allow_concurrent_memtable_write = true
EOF
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
            # ソースディレクトリ内の場合（高速プロファイル使用）
            RUSTFLAGS="-C target-cpu=native" cargo run --profile fast -- "$@" &
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