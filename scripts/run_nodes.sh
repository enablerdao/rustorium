#!/bin/bash

# デフォルト値
NODE_COUNT=10
BASE_PORT=40000
DATA_DIR="/tmp/rustorium"

# 引数の解析
while [[ $# -gt 0 ]]; do
    case $1 in
        --nodes)
            NODE_COUNT="$2"
            shift 2
            ;;
        --base-port)
            BASE_PORT="$2"
            shift 2
            ;;
        --data-dir)
            DATA_DIR="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# データディレクトリの作成
mkdir -p "$DATA_DIR"

# 既存のプロセスをクリーンアップ
echo "Cleaning up existing processes..."
pkill -f "rustorium-node"
sleep 2

# ブートストラップノードを起動
echo "Starting bootstrap node (node1)..."
cargo run --bin node -- \
    --node-id 1 \
    --data-dir "$DATA_DIR" \
    --port $BASE_PORT &

# ブートストラップノードのアドレスを取得
BOOTSTRAP_ADDR="/ip4/127.0.0.1/tcp/$BASE_PORT/p2p/$(cat "$DATA_DIR/node1/peer_id")"
echo "Bootstrap node address: $BOOTSTRAP_ADDR"

# 残りのノードを起動
for ((i=2; i<=NODE_COUNT; i++)); do
    echo "Starting node $i..."
    PORT=$((BASE_PORT + i - 1))
    cargo run --bin node -- \
        --node-id $i \
        --data-dir "$DATA_DIR" \
        --port $PORT \
        --bootstrap "$BOOTSTRAP_ADDR" &
    
    # 少し待機してノードが起動するのを待つ
    sleep 1
done

echo "All nodes started. Press Ctrl+C to stop all nodes."
wait