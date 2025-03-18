// WebSocketクライアント
class RustoriumWebSocket {
    constructor(url) {
        this.url = url;
        this.socket = null;
        this.isConnected = false;
        this.callbacks = {
            onOpen: [],
            onClose: [],
            onError: [],
            onMessage: [],
            onStatus: [],
            onTransactions: []
        };
        this.reconnectAttempts = 0;
        this.maxReconnectAttempts = 5;
        this.reconnectDelay = 2000; // 2秒
        
        // 自動接続
        this.connect();
    }

    // 接続
    connect() {
        if (this.socket && (this.socket.readyState === WebSocket.OPEN || this.socket.readyState === WebSocket.CONNECTING)) {
            console.log('WebSocket already connected or connecting');
            return;
        }

        console.log(`Connecting to WebSocket at ${this.url}`);
        this.socket = new WebSocket(this.url);

        this.socket.onopen = (event) => {
            console.log('WebSocket connected');
            this.isConnected = true;
            this.reconnectAttempts = 0;
            this.callbacks.onOpen.forEach(callback => callback(event));
        };

        this.socket.onclose = (event) => {
            console.log('WebSocket disconnected');
            this.isConnected = false;
            this.callbacks.onClose.forEach(callback => callback(event));
            
            // 自動再接続
            if (this.reconnectAttempts < this.maxReconnectAttempts) {
                this.reconnectAttempts++;
                console.log(`Attempting to reconnect (${this.reconnectAttempts}/${this.maxReconnectAttempts})...`);
                setTimeout(() => this.connect(), this.reconnectDelay);
            }
        };

        this.socket.onerror = (error) => {
            console.error('WebSocket error:', error);
            this.callbacks.onError.forEach(callback => callback(error));
        };

        this.socket.onmessage = (event) => {
            try {
                const data = JSON.parse(event.data);
                this.callbacks.onMessage.forEach(callback => callback(data));
                
                // 特定のデータタイプに対するコールバック
                if (data.success && data.data) {
                    // ネットワークステータスデータ
                    if (data.data.block_count !== undefined) {
                        this.callbacks.onStatus.forEach(callback => callback(data.data));
                    }
                    
                    // トランザクションデータ（配列の場合）
                    if (Array.isArray(data.data) && data.data.length > 0 && data.data[0].id) {
                        this.callbacks.onTransactions.forEach(callback => callback(data.data));
                    }
                }
            } catch (error) {
                console.error('Error parsing WebSocket message:', error);
            }
        };
    }

    // 切断
    disconnect() {
        if (this.socket) {
            this.socket.close();
            this.socket = null;
            this.isConnected = false;
        }
    }

    // メッセージ送信
    send(message) {
        if (this.isConnected) {
            this.socket.send(message);
        } else {
            console.error('Cannot send message: WebSocket not connected');
        }
    }

    // ネットワークステータスを取得
    getStatus() {
        this.send('get_status');
    }

    // トランザクションを取得
    getTransactions() {
        this.send('get_transactions');
    }

    // イベントリスナー追加
    on(event, callback) {
        if (this.callbacks[event]) {
            this.callbacks[event].push(callback);
        }
        return this;
    }

    // イベントリスナー削除
    off(event, callback) {
        if (this.callbacks[event]) {
            this.callbacks[event] = this.callbacks[event].filter(cb => cb !== callback);
        }
        return this;
    }
}

// グローバルインスタンス
const wsClient = new RustoriumWebSocket(`ws://${window.location.host}/ws`);

// デバッグ用のログ
console.log(`WebSocket URL: ws://${window.location.host}/ws`);
console.log('WebSocket client initialized');

// 自動接続
document.addEventListener('DOMContentLoaded', () => {
    wsClient.connect();
    
    // 定期的なデータ更新
    wsClient.on('onOpen', () => {
        // 接続時に初期データを取得
        wsClient.getStatus();
        wsClient.getTransactions();
        
        // 5秒ごとに更新
        setInterval(() => {
            if (wsClient.isConnected) {
                wsClient.getStatus();
                wsClient.getTransactions();
            }
        }, 5000);
    });
});