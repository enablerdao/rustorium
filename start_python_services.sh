#!/bin/bash

# 既存のプロセスをクリーンアップ
echo "Cleaning up any existing processes..."
pkill -f "python -m http.server 57620" 2>/dev/null || true
pkill -f "uvicorn api:app" 2>/dev/null || true

# APIサーバーの起動
echo "Starting API server..."
mkdir -p logs
cd api_server && python -m uvicorn api:app --host 0.0.0.0 --port 51055 > ../logs/api_server.log 2>&1 &
API_PID=$!
cd ..

# APIサーバーが起動するまで待機
sleep 2

# APIサーバーのポートを確認
API_PORT=51055
echo "API server detected on port: $API_PORT"
echo $API_PORT

# WebUIサーバーの起動
echo "Starting Web UI server..."
cd web_ui && python -m http.server 57620 > ../logs/web_ui.log 2>&1 &
WEB_UI_PID=$!
cd ..

# WebUIサーバーが起動するまで待機
sleep 1

# WebUIサーバーのポートを確認
WEB_UI_PORT=57620
echo "Web UI server detected on port: $WEB_UI_PORT"
echo $WEB_UI_PORT

# サービス情報の表示
echo "All services started!"
echo "API server running at http://localhost:$API_PORT"
echo "$API_PORT (PID: $API_PID)"
echo "Web UI running at http://localhost:$WEB_UI_PORT"
echo "$WEB_UI_PORT (PID: $WEB_UI_PID)"
echo
echo "To stop services, run: ./stop_python_services.sh"
echo "To view logs:"
echo "  API server: tail -f logs/api_server.log"
echo "  Web UI: tail -f logs/web_ui.log"