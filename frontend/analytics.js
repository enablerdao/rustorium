// 分析ダッシュボード用のスクリプト
let transactionChart = null;
let blockTimeChart = null;
let tpsChart = null;

// トランザクションデータを取得
async function fetchTransactionStats() {
    try {
        // 実際のAPIが実装されるまでダミーデータを使用
        const now = new Date();
        const hours = [];
        const txCounts = [];
        const volumes = [];
        
        for (let i = 23; i >= 0; i--) {
            const hour = new Date(now);
            hour.setHours(now.getHours() - i);
            hours.push(hour.getHours() + ':00');
            
            // ランダムなデータを生成
            const count = Math.floor(Math.random() * 50) + 10;
            txCounts.push(count);
            
            const volume = Math.floor(Math.random() * 10000) + 1000;
            volumes.push(volume);
        }
        
        return {
            labels: hours,
            txCounts: txCounts,
            volumes: volumes
        };
    } catch (error) {
        console.error('Error fetching transaction stats:', error);
        return { labels: [], txCounts: [], volumes: [] };
    }
}

// ブロック時間データを取得
async function fetchBlockTimeStats() {
    try {
        // 実際のAPIが実装されるまでダミーデータを使用
        const blockNumbers = [];
        const blockTimes = [];
        
        for (let i = 0; i < 20; i++) {
            blockNumbers.push(i + 1);
            // 1.5秒から2.5秒の間のランダムな値
            blockTimes.push(1.5 + Math.random());
        }
        
        return {
            labels: blockNumbers,
            blockTimes: blockTimes
        };
    } catch (error) {
        console.error('Error fetching block time stats:', error);
        return { labels: [], blockTimes: [] };
    }
}

// TPS（1秒あたりのトランザクション数）データを取得
async function fetchTpsStats() {
    try {
        // 実際のAPIが実装されるまでダミーデータを使用
        const timePoints = [];
        const tpsValues = [];
        
        const now = new Date();
        
        for (let i = 19; i >= 0; i--) {
            const time = new Date(now);
            time.setMinutes(now.getMinutes() - i);
            timePoints.push(time.getHours() + ':' + (time.getMinutes() < 10 ? '0' : '') + time.getMinutes());
            
            // 10から30の間のランダムな値
            tpsValues.push(10 + Math.floor(Math.random() * 20));
        }
        
        return {
            labels: timePoints,
            tps: tpsValues
        };
    } catch (error) {
        console.error('Error fetching TPS stats:', error);
        return { labels: [], tps: [] };
    }
}

// トランザクションチャートを描画
function drawTransactionChart(container, data) {
    if (transactionChart) {
        transactionChart.destroy();
    }
    
    const ctx = container.getContext('2d');
    
    transactionChart = new Chart(ctx, {
        type: 'bar',
        data: {
            labels: data.labels,
            datasets: [
                {
                    label: 'Transaction Count',
                    data: data.txCounts,
                    backgroundColor: 'rgba(54, 162, 235, 0.5)',
                    borderColor: 'rgba(54, 162, 235, 1)',
                    borderWidth: 1,
                    yAxisID: 'y'
                },
                {
                    label: 'Transaction Volume',
                    data: data.volumes,
                    type: 'line',
                    fill: false,
                    backgroundColor: 'rgba(255, 99, 132, 0.5)',
                    borderColor: 'rgba(255, 99, 132, 1)',
                    borderWidth: 2,
                    tension: 0.1,
                    yAxisID: 'y1'
                }
            ]
        },
        options: {
            responsive: true,
            interaction: {
                mode: 'index',
                intersect: false,
            },
            scales: {
                y: {
                    type: 'linear',
                    display: true,
                    position: 'left',
                    title: {
                        display: true,
                        text: 'Transaction Count'
                    }
                },
                y1: {
                    type: 'linear',
                    display: true,
                    position: 'right',
                    title: {
                        display: true,
                        text: 'Volume'
                    },
                    grid: {
                        drawOnChartArea: false
                    }
                }
            }
        }
    });
    
    return transactionChart;
}

// ブロック時間チャートを描画
function drawBlockTimeChart(container, data) {
    if (blockTimeChart) {
        blockTimeChart.destroy();
    }
    
    const ctx = container.getContext('2d');
    
    blockTimeChart = new Chart(ctx, {
        type: 'line',
        data: {
            labels: data.labels,
            datasets: [{
                label: 'Block Time (seconds)',
                data: data.blockTimes,
                backgroundColor: 'rgba(75, 192, 192, 0.5)',
                borderColor: 'rgba(75, 192, 192, 1)',
                borderWidth: 2,
                tension: 0.1
            }]
        },
        options: {
            responsive: true,
            scales: {
                y: {
                    beginAtZero: true,
                    title: {
                        display: true,
                        text: 'Seconds'
                    }
                },
                x: {
                    title: {
                        display: true,
                        text: 'Block Number'
                    }
                }
            }
        }
    });
    
    return blockTimeChart;
}

// TPSチャートを描画
function drawTpsChart(container, data) {
    if (tpsChart) {
        tpsChart.destroy();
    }
    
    const ctx = container.getContext('2d');
    
    tpsChart = new Chart(ctx, {
        type: 'line',
        data: {
            labels: data.labels,
            datasets: [{
                label: 'Transactions Per Second',
                data: data.tps,
                backgroundColor: 'rgba(153, 102, 255, 0.5)',
                borderColor: 'rgba(153, 102, 255, 1)',
                borderWidth: 2,
                tension: 0.1,
                fill: true
            }]
        },
        options: {
            responsive: true,
            scales: {
                y: {
                    beginAtZero: true,
                    title: {
                        display: true,
                        text: 'TPS'
                    }
                }
            }
        }
    });
    
    return tpsChart;
}

// 分析ダッシュボードを表示
function showAnalyticsPage() {
    contentArea.innerHTML = `
        <h1 class="mb-4">Analytics Dashboard</h1>
        
        <div class="row mb-4">
            <div class="col-md-4">
                <div class="card stats-card">
                    <div class="card-body">
                        <div class="text-primary mb-2">
                            <i class="bi bi-arrow-left-right fs-3"></i>
                        </div>
                        <div class="value" id="total-tx">...</div>
                        <div class="label">Total Transactions</div>
                    </div>
                </div>
            </div>
            <div class="col-md-4">
                <div class="card stats-card">
                    <div class="card-body">
                        <div class="text-success mb-2">
                            <i class="bi bi-lightning-charge fs-3"></i>
                        </div>
                        <div class="value" id="avg-tps">...</div>
                        <div class="label">Average TPS</div>
                    </div>
                </div>
            </div>
            <div class="col-md-4">
                <div class="card stats-card">
                    <div class="card-body">
                        <div class="text-info mb-2">
                            <i class="bi bi-clock fs-3"></i>
                        </div>
                        <div class="value" id="avg-block-time">...</div>
                        <div class="label">Avg Block Time</div>
                    </div>
                </div>
            </div>
        </div>
        
        <div class="row mb-4">
            <div class="col-md-12">
                <div class="card">
                    <div class="card-header">
                        <h5 class="mb-0">Transaction Activity (24h)</h5>
                    </div>
                    <div class="card-body">
                        <canvas id="transaction-chart" height="300"></canvas>
                    </div>
                </div>
            </div>
        </div>
        
        <div class="row">
            <div class="col-md-6">
                <div class="card">
                    <div class="card-header">
                        <h5 class="mb-0">Block Time</h5>
                    </div>
                    <div class="card-body">
                        <canvas id="block-time-chart" height="250"></canvas>
                    </div>
                </div>
            </div>
            <div class="col-md-6">
                <div class="card">
                    <div class="card-header">
                        <h5 class="mb-0">Transactions Per Second</h5>
                    </div>
                    <div class="card-body">
                        <canvas id="tps-chart" height="250"></canvas>
                    </div>
                </div>
            </div>
        </div>
    `;

    // Chart.jsライブラリが読み込まれているか確認
    if (typeof Chart === 'undefined') {
        // Chart.jsを動的に読み込む
        const script = document.createElement('script');
        script.src = 'https://cdn.jsdelivr.net/npm/chart.js';
        script.onload = async () => {
            await loadAnalyticsData();
        };
        document.head.appendChild(script);
    } else {
        // すでに読み込まれている場合は直接実行
        loadAnalyticsData();
    }
}

// 分析データを読み込む
async function loadAnalyticsData() {
    // 統計データを表示
    document.getElementById('total-tx').textContent = '1,245';
    document.getElementById('avg-tps').textContent = '18.5';
    document.getElementById('avg-block-time').textContent = '2.1s';
    
    // トランザクションチャートを描画
    const txData = await fetchTransactionStats();
    const txChartContainer = document.getElementById('transaction-chart');
    if (txChartContainer) {
        drawTransactionChart(txChartContainer, txData);
    }
    
    // ブロック時間チャートを描画
    const blockTimeData = await fetchBlockTimeStats();
    const blockTimeChartContainer = document.getElementById('block-time-chart');
    if (blockTimeChartContainer) {
        drawBlockTimeChart(blockTimeChartContainer, blockTimeData);
    }
    
    // TPSチャートを描画
    const tpsData = await fetchTpsStats();
    const tpsChartContainer = document.getElementById('tps-chart');
    if (tpsChartContainer) {
        drawTpsChart(tpsChartContainer, tpsData);
    }
}