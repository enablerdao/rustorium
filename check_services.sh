#!/bin/bash

echo "Checking Rustorium services status..."

# APIサーバーのステータス確認
if [ -f /workspace/Rustorium/logs/api_server.pid ]; then
    API_PID=$(cat /workspace/Rustorium/logs/api_server.pid)
    if ps -p $API_PID > /dev/null; then
        API_PORT=$(grep -o "API server listening on 0.0.0.0:[0-9]\+" /workspace/Rustorium/logs/api_server.log | grep -o "[0-9]\+$" || echo "unknown")
        echo "✅ API server is running (PID: $API_PID, Port: $API_PORT)"
    else
        echo "❌ API server is not running (PID file exists but process is dead)"
    fi
else
    echo "❌ API server is not running (No PID file)"
fi

# WebUIサーバーのステータス確認
if [ -f /workspace/Rustorium/logs/web_ui.pid ]; then
    WEB_PID=$(cat /workspace/Rustorium/logs/web_ui.pid)
    if ps -p $WEB_PID > /dev/null; then
        WEB_PORT=$(grep -o "Web UI server listening on 0.0.0.0:[0-9]\+" /workspace/Rustorium/logs/web_ui.log | grep -o "[0-9]\+$" || echo "unknown")
        echo "✅ Web UI server is running (PID: $WEB_PID, Port: $WEB_PORT)"
    else
        echo "❌ Web UI server is not running (PID file exists but process is dead)"
    fi
else
    echo "❌ Web UI server is not running (No PID file)"
fi

echo ""
echo "To start services: ./start_services.sh"
echo "To stop services: ./stop_services.sh"
echo "To view logs:"
echo "  API server: tail -f logs/api_server.log"
echo "  Web UI: tail -f logs/web_ui.log"