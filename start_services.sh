#!/bin/bash
set -e

# ログディレクトリの作成
mkdir -p /workspace/Rustorium/logs

# 既存のゾンビプロセスをクリーンアップ
echo "Cleaning up any existing processes..."
pkill -f "standalone_api" || true
pkill -f "web_ui" || true
sleep 1

# APIサーバーを起動
echo "Starting API server..."
cd /workspace/Rustorium/standalone_api
nohup cargo run > /workspace/Rustorium/logs/api_server.log 2>&1 &
API_PID=$!
echo $API_PID > /workspace/Rustorium/logs/api_server.pid

# 少し待機
sleep 3

# APIサーバーのポートを取得
API_PORT=$(grep -o "API server listening on 0.0.0.0:[0-9]\+" /workspace/Rustorium/logs/api_server.log | grep -o "[0-9]\+$" || echo "51055")
echo "API server detected on port: $API_PORT"

# WebUIサーバーを起動
echo "Starting Web UI server..."
cd /workspace/Rustorium/web_ui
nohup cargo run > /workspace/Rustorium/logs/web_ui.log 2>&1 &
WEB_PID=$!
echo $WEB_PID > /workspace/Rustorium/logs/web_ui.pid

# 少し待機
sleep 3

# WebUIサーバーのポートを取得
WEB_PORT=$(grep -o "Web UI server listening on 0.0.0.0:[0-9]\+" /workspace/Rustorium/logs/web_ui.log | grep -o "[0-9]\+$" || echo "57620")
echo "Web UI server detected on port: $WEB_PORT"

echo "All services started!"
echo "API server running at http://localhost:$API_PORT (PID: $API_PID)"
echo "Web UI running at http://localhost:$WEB_PORT (PID: $WEB_PID)"
echo ""
echo "To stop services, run: ./stop_services.sh"
echo "To view logs:"
echo "  API server: tail -f logs/api_server.log"
echo "  Web UI: tail -f logs/web_ui.log"