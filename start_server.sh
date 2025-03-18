#!/bin/bash
set -e

# ログディレクトリの作成
mkdir -p /workspace/Rustorium/logs

# 既存のゾンビプロセスをクリーンアップ
echo "Cleaning up any existing processes..."
pkill -f "rustorium" || true
sleep 1

# 統合サーバーを起動
echo "Starting Rustorium integrated server..."
cd /workspace/Rustorium
nohup cargo run > /workspace/Rustorium/logs/server.log 2>&1 &
SERVER_PID=$!
echo $SERVER_PID > /workspace/Rustorium/logs/server.pid

# 少し待機
sleep 3

# サーバーのポートを取得
SERVER_PORT=57620
echo "Rustorium server detected on port: $SERVER_PORT"

echo "Rustorium server started!"
echo "Server running at http://localhost:$SERVER_PORT (PID: $SERVER_PID)"
echo ""
echo "To stop the server, run: ./stop_server.sh"
echo "To view logs: tail -f logs/server.log"