// メトリクスの更新間隔（ミリ秒）
const UPDATE_INTERVAL = 1000;

// 要素の取得
const nodeStatus = document.getElementById('node-status');
const cpuCores = document.getElementById('cpu-cores');
const memory = document.getElementById('memory');
const nodeRole = document.getElementById('node-role');
const p2pPort = document.getElementById('p2p-port');
const webPort = document.getElementById('web-port');
const apiPort = document.getElementById('api-port');
const wsPort = document.getElementById('ws-port');
const maxPeers = document.getElementById('max-peers');
const pendingTx = document.getElementById('pending-tx');
const blockTime = document.getElementById('block-time');

// メトリクスの更新
async function updateMetrics() {
    try {
        const response = await fetch('/api/metrics');
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data = await response.json();

        // システムメトリクス
        cpuCores.textContent = data.system.cpu_cores;
        memory.textContent = `${data.system.memory_gb} GB`;
        nodeRole.textContent = data.system.role;

        // ネットワークメトリクス
        p2pPort.textContent = data.network.p2p_port;
        webPort.textContent = data.network.web_port;
        apiPort.textContent = data.network.api_port;
        wsPort.textContent = data.network.ws_port;

        // パフォーマンスメトリクス
        maxPeers.textContent = data.performance.max_peers;
        pendingTx.textContent = data.performance.max_pending_tx;
        blockTime.textContent = `${data.performance.block_time} ms`;

        // ステータスを更新
        nodeStatus.textContent = 'Connected';
        nodeStatus.classList.add('connected');
    } catch (error) {
        console.error('Failed to fetch metrics:', error);
        nodeStatus.textContent = 'Disconnected';
        nodeStatus.classList.remove('connected');
    }
}

// 定期的にメトリクスを更新
setInterval(updateMetrics, UPDATE_INTERVAL);

// 初回更新
updateMetrics();