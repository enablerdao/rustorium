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
    pkill -f "api" || true
fi

# Webサーバーの停止
if [ -f /workspace/Rustorium/logs/web_server.pid ]; then
    WEB_PID=$(cat /workspace/Rustorium/logs/web_server.pid)
    if ps -p $WEB_PID > /dev/null; then
        echo "Stopping Web server (PID: $WEB_PID)..."
        kill $WEB_PID 2>/dev/null || true
    else
        echo "Web server is not running."
    fi
    rm -f /workspace/Rustorium/logs/web_server.pid
else
    echo "Web server PID file not found."
    pkill -f "python -m http.server" || true
fi

# 念のため、関連プロセスをすべて終了
echo "Cleaning up any remaining processes..."
pkill -f "api" || true
pkill -f "python -m http.server" || true

echo "All services stopped."