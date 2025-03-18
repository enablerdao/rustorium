#!/bin/bash

echo "Stopping Rustorium Python services..."

# APIサーバーの停止
API_PID=$(pgrep -f "uvicorn api:app")
if [ ! -z "$API_PID" ]; then
    echo "Stopping API server (PID: $API_PID)..."
    kill $API_PID
else
    echo "API server is not running."
fi

# WebUIサーバーの停止
WEB_UI_PID=$(pgrep -f "python -m http.server 57620")
if [ ! -z "$WEB_UI_PID" ]; then
    echo "Stopping Web UI server (PID: $WEB_UI_PID)..."
    kill $WEB_UI_PID
else
    echo "Web UI server is not running."
fi

# 残りのプロセスをクリーンアップ
echo "Cleaning up any remaining processes..."
pkill -f "python -m http.server 57620" 2>/dev/null || true
pkill -f "uvicorn api:app" 2>/dev/null || true

echo "All services stopped."