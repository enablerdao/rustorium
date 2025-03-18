#!/bin/bash
set -e

# ログディレクトリの作成
mkdir -p /workspace/Rustorium/logs

# 既存のゾンビプロセスをクリーンアップ
echo "Cleaning up any existing processes..."
pkill -f "api" || true
pkill -f "web" || true
sleep 1

# APIサーバーを起動
echo "Starting API server..."
cd /workspace/Rustorium/api
nohup cargo run > /workspace/Rustorium/logs/api_server.log 2>&1 &
API_PID=$!
echo $API_PID > /workspace/Rustorium/logs/api_server.pid

# 少し待機
sleep 3

# APIサーバーのポートを取得
API_PORT=$(grep -o "API server listening on 0.0.0.0:[0-9]\+" /workspace/Rustorium/logs/api_server.log | grep -o "[0-9]\+$" || echo "51055")
echo "API server detected on port: $API_PORT"

# Webサーバーのポート
WEB_PORT=57620

# Webサーバーを起動
echo "Starting Web server..."
cd /workspace/Rustorium/web
nohup python -m http.server $WEB_PORT > /workspace/Rustorium/logs/web_server.log 2>&1 &
WEB_PID=$!
echo $WEB_PID > /workspace/Rustorium/logs/web_server.pid

# 少し待機
sleep 1

echo "Web server detected on port: $WEB_PORT"

echo "All services started!"
echo "API server running at http://localhost:$API_PORT (PID: $API_PID)"
echo "Web server running at http://localhost:$WEB_PORT (PID: $WEB_PID)"
echo ""
echo "To stop services, run: ./stop_services.sh"
echo "To view logs:"
echo "  API server: tail -f logs/api_server.log"
echo "  Web server: tail -f logs/web_server.log"