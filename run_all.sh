#!/bin/bash
set -e

# APIサーバーを起動
echo "Starting API server..."
cd /workspace/RustLedger/standalone_api
cargo run &
API_PID=$!

# 少し待機
sleep 2

# WebUIサーバーを起動
echo "Starting Web UI server..."
cd /workspace/RustLedger/web_ui
cargo run &
WEB_PID=$!

echo "All services started!"
echo "API server running at http://localhost:51055"
echo "Web UI running at http://localhost:57620"
echo ""
echo "Press Ctrl+C to stop all services"

# 終了時に子プロセスを終了
trap "kill $API_PID $WEB_PID; exit" INT TERM EXIT

# 親プロセスが終了するまで待機
wait