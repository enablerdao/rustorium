#!/bin/bash

echo "Stopping Rustorium server..."

# サーバーの停止
if [ -f /workspace/Rustorium/logs/server.pid ]; then
    SERVER_PID=$(cat /workspace/Rustorium/logs/server.pid)
    if ps -p $SERVER_PID > /dev/null; then
        echo "Stopping Rustorium server (PID: $SERVER_PID)..."
        kill $SERVER_PID 2>/dev/null || true
    else
        echo "Rustorium server is not running."
    fi
    rm -f /workspace/Rustorium/logs/server.pid
else
    echo "Server PID file not found."
    pkill -f "rustorium" || true
fi

# 念のため、関連プロセスをすべて終了
echo "Cleaning up any remaining processes..."
pkill -f "rustorium" || true

echo "Server stopped."