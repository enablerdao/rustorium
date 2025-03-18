# Rustorium サービス管理ガイド

このガイドでは、Rustoriumのサービス（APIサーバーとWebUI）を管理するための方法について説明します。

## サービス管理スクリプト

Rustoriumには、サービスを簡単に管理するための3つのスクリプトが用意されています：

1. `start_services.sh` - サービスを起動する
2. `stop_services.sh` - サービスを停止する
3. `check_services.sh` - サービスの状態を確認する

これらのスクリプトを使用することで、サービスをバックグラウンドで継続的に実行し、必要に応じて停止することができます。

## サービスの起動

サービスを起動するには、以下のコマンドを実行します：

```bash
./start_services.sh
```

このスクリプトは以下の処理を行います：

1. 既存の実行中のサービスをクリーンアップ
2. APIサーバーをバックグラウンドで起動
3. WebUIサーバーをバックグラウンドで起動
4. 各サービスのポート情報を表示
5. PIDファイルを作成して、後でサービスを管理できるようにする

起動後、以下のURLでサービスにアクセスできます：

- APIサーバー: http://localhost:51055
- WebUI: http://localhost:57620

## サービスの状態確認

サービスの実行状態を確認するには、以下のコマンドを実行します：

```bash
./check_services.sh
```

このスクリプトは、各サービスが実行中かどうかを確認し、実行中の場合はPIDとポート番号を表示します。

## サービスの停止

サービスを停止するには、以下のコマンドを実行します：

```bash
./stop_services.sh
```

このスクリプトは、PIDファイルを使用して実行中のサービスを特定し、それらを適切に終了します。

## ログの確認

サービスのログを確認するには、以下のコマンドを実行します：

```bash
# APIサーバーのログを表示
tail -f logs/api_server.log

# WebUIサーバーのログを表示
tail -f logs/web_ui.log
```

## トラブルシューティング

### サービスが起動しない場合

1. ログファイルを確認して、エラーメッセージを確認します：
   ```bash
   cat logs/api_server.log
   cat logs/web_ui.log
   ```

2. ポートが既に使用されている可能性があります。以下のコマンドで確認できます：
   ```bash
   netstat -tuln | grep 51055
   netstat -tuln | grep 57620
   ```

3. 古いプロセスが残っている場合は、以下のコマンドで強制終了できます：
   ```bash
   pkill -f "standalone_api"
   pkill -f "web_ui"
   ```

### サービスが応答しない場合

1. サービスの状態を確認します：
   ```bash
   ./check_services.sh
   ```

2. サービスが実行中の場合は、再起動を試みます：
   ```bash
   ./stop_services.sh
   ./start_services.sh
   ```

3. ログを確認して、問題の原因を特定します。

## 自動起動の設定

システム起動時にRustoriumサービスを自動的に起動するには、以下の手順を実行します：

### systemdを使用する場合（Linux）

1. サービス定義ファイルを作成します：

```bash
sudo nano /etc/systemd/system/rustorium.service
```

2. 以下の内容を追加します：

```
[Unit]
Description=Rustorium Blockchain Services
After=network.target

[Service]
User=<your-username>
WorkingDirectory=/path/to/rustorium
ExecStart=/path/to/rustorium/start_services.sh
ExecStop=/path/to/rustorium/stop_services.sh
Restart=on-failure
RestartSec=5

[Install]
WantedBy=multi-user.target
```

3. サービスを有効にします：

```bash
sudo systemctl enable rustorium
sudo systemctl start rustorium
```

4. サービスの状態を確認します：

```bash
sudo systemctl status rustorium
```