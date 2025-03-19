// ネットワークビジュアライゼーション用のスクリプト
let networkChart = null;

// CSS for network page
const networkStyles = `
.network-icon {
    width: 60px;
    height: 60px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    font-size: 24px;
}
`;

// スタイルを追加
(function() {
    const style = document.createElement('style');
    style.textContent = networkStyles;
    document.head.appendChild(style);
})();

// ネットワークデータを取得
async function fetchNetworkData() {
    try {
        // 実際のAPIが実装されるまでダミーデータを使用
        return {
            nodes: [
                { id: 'node-1', label: 'Node 1 (This Node)', group: 'validator', x: 0, y: 0 },
                { id: 'node-2', label: 'Node 2', group: 'validator', x: -100, y: -50 },
                { id: 'node-3', label: 'Node 3', group: 'validator', x: 100, y: -50 },
                { id: 'node-4', label: 'Node 4', group: 'validator', x: 0, y: 100 },
                { id: 'node-5', label: 'Node 5', group: 'peer', x: -150, y: 50 },
                { id: 'node-6', label: 'Node 6', group: 'peer', x: 150, y: 50 },
                { id: 'node-7', label: 'Node 7', group: 'peer', x: -50, y: 150 },
                { id: 'node-8', label: 'Node 8', group: 'peer', x: 50, y: 150 }
            ],
            edges: [
                { from: 'node-1', to: 'node-2', value: 5, label: '10ms' },
                { from: 'node-1', to: 'node-3', value: 3, label: '15ms' },
                { from: 'node-1', to: 'node-4', value: 4, label: '12ms' },
                { from: 'node-1', to: 'node-5', value: 2, label: '20ms' },
                { from: 'node-1', to: 'node-6', value: 2, label: '18ms' },
                { from: 'node-2', to: 'node-3', value: 1, label: '25ms' },
                { from: 'node-2', to: 'node-7', value: 1, label: '30ms' },
                { from: 'node-3', to: 'node-6', value: 1, label: '22ms' },
                { from: 'node-4', to: 'node-8', value: 1, label: '14ms' },
                { from: 'node-5', to: 'node-7', value: 1, label: '28ms' }
            ]
        };
    } catch (error) {
        console.error('Error fetching network data:', error);
        return { nodes: [], edges: [] };
    }
}

// ネットワークグラフを描画
function drawNetworkGraph(container, data) {
    if (networkChart) {
        networkChart.destroy();
    }

    const nodes = new vis.DataSet(data.nodes);
    const edges = new vis.DataSet(data.edges);

    const options = {
        nodes: {
            shape: 'dot',
            size: 16,
            font: {
                size: 12,
                face: 'Tahoma'
            },
            borderWidth: 2,
            shadow: true
        },
        edges: {
            width: 2,
            shadow: true,
            smooth: {
                type: 'continuous'
            },
            font: {
                size: 12,
                align: 'middle'
            },
            color: {
                inherit: 'from'
            }
        },
        groups: {
            validator: {
                color: { background: '#4CAF50', border: '#388E3C' },
                borderWidth: 2
            },
            peer: {
                color: { background: '#2196F3', border: '#1565C0' },
                borderWidth: 1
            }
        },
        physics: {
            stabilization: false,
            barnesHut: {
                gravitationalConstant: -2000,
                centralGravity: 0.1,
                springLength: 150,
                springConstant: 0.05,
                damping: 0.09
            }
        }
    };

    const network = new vis.Network(container, { nodes, edges }, options);
    
    // ノードをクリックしたときの処理
    network.on('click', function(params) {
        if (params.nodes.length > 0) {
            const nodeId = params.nodes[0];
            const node = nodes.get(nodeId);
            showNodeDetails(node);
        }
    });

    return network;
}

// ノードの詳細を表示
function showNodeDetails(node) {
    const nodeDetailsEl = document.getElementById('node-details');
    if (!nodeDetailsEl) return;

    nodeDetailsEl.innerHTML = `
        <div class="card mb-3">
            <div class="card-header">
                <h5 class="mb-0">Node Details: ${node.label}</h5>
            </div>
            <div class="card-body">
                <div class="row mb-2">
                    <div class="col-md-4 fw-bold">Node ID:</div>
                    <div class="col-md-8">${node.id}</div>
                </div>
                <div class="row mb-2">
                    <div class="col-md-4 fw-bold">Type:</div>
                    <div class="col-md-8">
                        <span class="badge ${node.group === 'validator' ? 'bg-success' : 'bg-primary'}">
                            ${node.group === 'validator' ? 'Validator' : 'Peer'}
                        </span>
                    </div>
                </div>
                <div class="row mb-2">
                    <div class="col-md-4 fw-bold">Status:</div>
                    <div class="col-md-8">
                        <span class="badge bg-success">Online</span>
                    </div>
                </div>
                <div class="row mb-2">
                    <div class="col-md-4 fw-bold">Uptime:</div>
                    <div class="col-md-8">3h 45m</div>
                </div>
                <div class="row">
                    <div class="col-md-4 fw-bold">Version:</div>
                    <div class="col-md-8">Rustorium v0.1.0</div>
                </div>
            </div>
        </div>
    `;
}

// シャードデータを取得
async function fetchShardData() {
    try {
        // 実際のAPIが実装されるまでダミーデータを使用
        return [
            { id: 0, size: 2500, transactions: 120, nodes: ['node-1', 'node-2'] },
            { id: 1, size: 1800, transactions: 95, nodes: ['node-3', 'node-4'] },
            { id: 2, size: 3200, transactions: 145, nodes: ['node-5', 'node-6'] },
            { id: 3, size: 2100, transactions: 110, nodes: ['node-7', 'node-8'] }
        ];
    } catch (error) {
        console.error('Error fetching shard data:', error);
        return [];
    }
}

// シャードデータを表示
function displayShardData(shards) {
    const shardListEl = document.getElementById('shard-list');
    if (!shardListEl) return;

    if (shards.length === 0) {
        shardListEl.innerHTML = `
            <div class="alert alert-info" role="alert">
                <i class="bi bi-info-circle me-2"></i>
                No shard data available.
            </div>
        `;
        return;
    }

    let html = `
        <div class="table-responsive">
            <table class="table table-hover">
                <thead>
                    <tr>
                        <th>Shard ID</th>
                        <th>Size</th>
                        <th>Transactions</th>
                        <th>Nodes</th>
                        <th>Status</th>
                    </tr>
                </thead>
                <tbody>
    `;

    for (const shard of shards) {
        html += `
            <tr>
                <td>${shard.id}</td>
                <td>${formatSize(shard.size)}</td>
                <td>${shard.transactions}</td>
                <td>${shard.nodes.length}</td>
                <td><span class="badge bg-success">Active</span></td>
            </tr>
        `;
    }

    html += `
                </tbody>
            </table>
        </div>
    `;

    shardListEl.innerHTML = html;
}

// サイズをフォーマット
function formatSize(bytes) {
    if (bytes < 1024) {
        return bytes + ' B';
    } else if (bytes < 1024 * 1024) {
        return (bytes / 1024).toFixed(2) + ' KB';
    } else {
        return (bytes / (1024 * 1024)).toFixed(2) + ' MB';
    }
}

// ネットワークページを表示
function showNetworkPage() {
    contentArea.innerHTML = `
        <h1 class="mb-4">Network</h1>
        
        <div class="row mb-4">
            <div class="col-md-8">
                <div class="card">
                    <div class="card-header">
                        <h5 class="mb-0">Network Map</h5>
                    </div>
                    <div class="card-body">
                        <div id="network-graph" style="height: 500px;"></div>
                    </div>
                </div>
            </div>
            <div class="col-md-4">
                <div id="node-details">
                    <div class="alert alert-info" role="alert">
                        <i class="bi bi-info-circle me-2"></i>
                        Click on a node to see details.
                    </div>
                </div>
                
                <div class="card mt-3">
                    <div class="card-header">
                        <h5 class="mb-0">Network Stats</h5>
                    </div>
                    <div class="card-body">
                        <div class="row mb-2">
                            <div class="col-6 fw-bold">Total Nodes:</div>
                            <div class="col-6">8</div>
                        </div>
                        <div class="row mb-2">
                            <div class="col-6 fw-bold">Validators:</div>
                            <div class="col-6">4</div>
                        </div>
                        <div class="row mb-2">
                            <div class="col-6 fw-bold">Peers:</div>
                            <div class="col-6">4</div>
                        </div>
                        <div class="row">
                            <div class="col-6 fw-bold">Avg Latency:</div>
                            <div class="col-6">18ms</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        
        <div class="card">
            <div class="card-header d-flex justify-content-between align-items-center">
                <h5 class="mb-0">Shards</h5>
                <button class="btn btn-sm btn-outline-primary" id="rebalance-btn">
                    <i class="bi bi-arrow-repeat me-1"></i>
                    Rebalance Shards
                </button>
            </div>
            <div class="card-body">
                <div id="shard-list">
                    <div class="d-flex justify-content-center my-5">
                        <div class="spinner-border text-primary" role="status">
                            <span class="visually-hidden">Loading...</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    `;

    // vis.jsライブラリが読み込まれているか確認
    if (typeof vis === 'undefined') {
        // vis.jsを動的に読み込む
        const script = document.createElement('script');
        script.src = 'https://unpkg.com/vis-network@9.1.2/dist/vis-network.min.js';
        script.onload = async () => {
            const data = await fetchNetworkData();
            const container = document.getElementById('network-graph');
            networkChart = drawNetworkGraph(container, data);
            
            const shards = await fetchShardData();
            displayShardData(shards);
            
            // リバランスボタンのイベントリスナーを追加
            document.getElementById('rebalance-btn').addEventListener('click', handleShardRebalance);
        };
        document.head.appendChild(script);
    } else {
        // すでに読み込まれている場合は直接実行
        (async () => {
            const data = await fetchNetworkData();
            const container = document.getElementById('network-graph');
            networkChart = drawNetworkGraph(container, data);
            
            const shards = await fetchShardData();
            displayShardData(shards);
            
            // リバランスボタンのイベントリスナーを追加
            document.getElementById('rebalance-btn').addEventListener('click', handleShardRebalance);
        })();
    }
}

// シャードリバランスを処理
function handleShardRebalance() {
    const btn = document.getElementById('rebalance-btn');
    const originalText = btn.innerHTML;
    
    btn.disabled = true;
    btn.innerHTML = `
        <span class="spinner-border spinner-border-sm me-1" role="status" aria-hidden="true"></span>
        Rebalancing...
    `;
    
    // 実際のAPIが実装されるまで、タイマーでシミュレート
    setTimeout(() => {
        btn.disabled = false;
        btn.innerHTML = originalText;
        
        // 成功メッセージを表示
        const shardListEl = document.getElementById('shard-list');
        shardListEl.innerHTML = `
            <div class="alert alert-success mb-3" role="alert">
                <i class="bi bi-check-circle me-2"></i>
                Shard rebalancing completed successfully!
            </div>
        ` + shardListEl.innerHTML;
        
        // 3秒後にメッセージを消す
        setTimeout(() => {
            const alert = shardListEl.querySelector('.alert');
            if (alert) {
                alert.remove();
            }
        }, 3000);
    }, 2000);
}