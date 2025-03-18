#!/bin/bash
set -e

# 既存のゾンビプロセスをクリーンアップ
echo "Cleaning up any existing processes..."
pkill -f "standalone_api" || true
pkill -f "web_ui" || true
sleep 1

# APIサーバーを起動
echo "Starting API server..."
cd /workspace/Rustorium/standalone_api
cargo run > api_output.log 2>&1 &
API_PID=$!

# 少し待機
sleep 3

# APIサーバーのポートを取得
API_PORT=$(grep -o "API server listening on 0.0.0.0:[0-9]\+" api_output.log | grep -o "[0-9]\+$" || echo "51055")
echo "API server detected on port: $API_PORT"

# WebUIサーバーを起動
echo "Starting Web UI server..."
cd /workspace/Rustorium/web_ui
cargo run > web_output.log 2>&1 &
WEB_PID=$!

# 少し待機
sleep 3

# WebUIサーバーのポートを取得
WEB_PORT=$(grep -o "Web UI server listening on 0.0.0.0:[0-9]\+" web_output.log | grep -o "[0-9]\+$" || echo "57620")
echo "Web UI server detected on port: $WEB_PORT"

echo "All services started!"
echo "API server running at http://localhost:$API_PORT"
echo "Web UI running at http://localhost:$WEB_PORT"
echo ""
echo "Press Ctrl+C to stop all services"

# 終了時に子プロセスを終了
cleanup() {
    echo "Stopping services..."
    kill $API_PID $WEB_PID 2>/dev/null || true
    exit
}

trap cleanup INT TERM EXIT

# 親プロセスが終了するまで待機
wait