#!/bin/bash

echo "Stopping Rustorium services..."

# APIサーバーの停止
if [ -f /workspace/Rustorium/logs/api_server.pid ]; then
    API_PID=$(cat /workspace/Rustorium/logs/api_server.pid)
    if ps -p $API_PID > /dev/null; then
        echo "Stopping API server (PID: $API_PID)..."
        kill $API_PID 2>/dev/null || true
    else
        echo "API server is not running."
    fi
    rm -f /workspace/Rustorium/logs/api_server.pid
else
    echo "API server PID file not found."
    pkill -f "standalone_api" || true
fi

# WebUIサーバーの停止
if [ -f /workspace/Rustorium/logs/web_ui.pid ]; then
    WEB_PID=$(cat /workspace/Rustorium/logs/web_ui.pid)
    if ps -p $WEB_PID > /dev/null; then
        echo "Stopping Web UI server (PID: $WEB_PID)..."
        kill $WEB_PID 2>/dev/null || true
    else
        echo "Web UI server is not running."
    fi
    rm -f /workspace/Rustorium/logs/web_ui.pid
else
    echo "Web UI server PID file not found."
    pkill -f "web_ui" || true
fi

# 念のため、関連プロセスをすべて終了
echo "Cleaning up any remaining processes..."
pkill -f "standalone_api" || true
pkill -f "web_ui" || true

echo "All services stopped."