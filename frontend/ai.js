// AI分析機能用のスクリプト
let anomalyChart = null;
let predictionChart = null;

// 異常検出データを取得
async function fetchAnomalyData() {
    try {
        // 実際のAPIが実装されるまでダミーデータを使用
        const dates = [];
        const anomalyScores = [];
        const threshold = 0.7;
        
        const now = new Date();
        
        for (let i = 29; i >= 0; i--) {
            const date = new Date(now);
            date.setDate(now.getDate() - i);
            dates.push(date.toLocaleDateString());
            
            // ランダムなスコアを生成（0.1〜0.9）
            let score = 0.1 + Math.random() * 0.8;
            
            // 時々異常値を入れる
            if (i % 7 === 0) {
                score = 0.75 + Math.random() * 0.15;
            }
            
            anomalyScores.push(score);
        }
        
        return {
            dates,
            anomalyScores,
            threshold
        };
    } catch (error) {
        console.error('Error fetching anomaly data:', error);
        return { dates: [], anomalyScores: [], threshold: 0.7 };
    }
}

// 予測データを取得
async function fetchPredictionData() {
    try {
        // 実際のAPIが実装されるまでダミーデータを使用
        const dates = [];
        const actualValues = [];
        const predictedValues = [];
        
        const now = new Date();
        
        // 過去のデータ
        for (let i = 14; i >= 0; i--) {
            const date = new Date(now);
            date.setDate(now.getDate() - i);
            dates.push(date.toLocaleDateString());
            
            // 実際の値（トランザクション数など）
            const actual = Math.floor(Math.random() * 50) + 100;
            actualValues.push(actual);
            
            // 予測値（過去のデータでは実際の値と同じ）
            if (i > 0) {
                predictedValues.push(actual);
            }
        }
        
        // 未来の予測
        for (let i = 1; i <= 7; i++) {
            const date = new Date(now);
            date.setDate(now.getDate() + i);
            dates.push(date.toLocaleDateString());
            
            // 未来の実際の値はnull
            actualValues.push(null);
            
            // 予測値
            const lastActual = actualValues[actualValues.length - 2];
            const predicted = lastActual + Math.floor(Math.random() * 20) - 10;
            predictedValues.push(predicted);
        }
        
        return {
            dates,
            actualValues,
            predictedValues
        };
    } catch (error) {
        console.error('Error fetching prediction data:', error);
        return { dates: [], actualValues: [], predictedValues: [] };
    }
}

// 異常検出チャートを描画
function drawAnomalyChart(container, data) {
    if (anomalyChart) {
        anomalyChart.destroy();
    }
    
    const ctx = container.getContext('2d');
    
    // 閾値を超えるポイントを特定
    const anomalies = data.anomalyScores.map((score, index) => {
        return score >= data.threshold ? score : null;
    });
    
    anomalyChart = new Chart(ctx, {
        type: 'line',
        data: {
            labels: data.dates,
            datasets: [
                {
                    label: 'Anomaly Score',
                    data: data.anomalyScores,
                    backgroundColor: 'rgba(54, 162, 235, 0.5)',
                    borderColor: 'rgba(54, 162, 235, 1)',
                    borderWidth: 2,
                    tension: 0.1
                },
                {
                    label: 'Threshold',
                    data: Array(data.dates.length).fill(data.threshold),
                    backgroundColor: 'rgba(255, 99, 132, 0.5)',
                    borderColor: 'rgba(255, 99, 132, 1)',
                    borderWidth: 2,
                    borderDash: [5, 5]
                },
                {
                    label: 'Anomalies',
                    data: anomalies,
                    backgroundColor: 'rgba(255, 0, 0, 1)',
                    borderColor: 'rgba(255, 0, 0, 1)',
                    borderWidth: 0,
                    pointRadius: 6,
                    pointHoverRadius: 8,
                    pointStyle: 'circle',
                    showLine: false
                }
            ]
        },
        options: {
            responsive: true,
            scales: {
                y: {
                    beginAtZero: true,
                    max: 1,
                    title: {
                        display: true,
                        text: 'Anomaly Score'
                    }
                }
            },
            plugins: {
                tooltip: {
                    callbacks: {
                        label: function(context) {
                            const label = context.dataset.label || '';
                            const value = context.parsed.y;
                            return label + ': ' + value.toFixed(3);
                        }
                    }
                }
            }
        }
    });
    
    return anomalyChart;
}

// 予測チャートを描画
function drawPredictionChart(container, data) {
    if (predictionChart) {
        predictionChart.destroy();
    }
    
    const ctx = container.getContext('2d');
    
    // 現在の日付のインデックスを特定
    const todayIndex = 14; // 0-indexedで14日目が「今日」
    
    predictionChart = new Chart(ctx, {
        type: 'line',
        data: {
            labels: data.dates,
            datasets: [
                {
                    label: 'Actual',
                    data: data.actualValues,
                    backgroundColor: 'rgba(54, 162, 235, 0.5)',
                    borderColor: 'rgba(54, 162, 235, 1)',
                    borderWidth: 2,
                    tension: 0.1
                },
                {
                    label: 'Predicted',
                    data: data.predictedValues,
                    backgroundColor: 'rgba(255, 159, 64, 0.5)',
                    borderColor: 'rgba(255, 159, 64, 1)',
                    borderWidth: 2,
                    borderDash: [5, 5],
                    tension: 0.1
                }
            ]
        },
        options: {
            responsive: true,
            scales: {
                y: {
                    beginAtZero: true,
                    title: {
                        display: true,
                        text: 'Transaction Count'
                    }
                },
                x: {
                    grid: {
                        color: (context) => {
                            return context.index === todayIndex ? 'rgba(255, 0, 0, 0.3)' : null;
                        },
                        lineWidth: (context) => {
                            return context.index === todayIndex ? 2 : 1;
                        }
                    }
                }
            },
            plugins: {
                annotation: {
                    annotations: {
                        line1: {
                            type: 'line',
                            xMin: todayIndex,
                            xMax: todayIndex,
                            borderColor: 'rgba(255, 0, 0, 0.5)',
                            borderWidth: 2,
                            label: {
                                content: 'Today',
                                enabled: true,
                                position: 'top'
                            }
                        }
                    }
                }
            }
        }
    });
    
    return predictionChart;
}

// 異常トランザクションリストを取得
async function fetchAnomalousTransactions() {
    try {
        // 実際のAPIが実装されるまでダミーデータを使用
        return [
            {
                id: '0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890',
                sender: '0x1234567890abcdef1234567890abcdef12345678',
                recipient: '0x9876543210fedcba9876543210fedcba98765432',
                amount: 50000,
                fee: 10,
                timestamp: Date.now() / 1000 - 3600,
                anomaly_score: 0.92,
                anomaly_reason: 'Unusually large transaction amount'
            },
            {
                id: '0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef',
                sender: '0xabcdef1234567890abcdef1234567890abcdef12',
                recipient: '0x1234567890abcdef1234567890abcdef12345678',
                amount: 1000,
                fee: 100,
                timestamp: Date.now() / 1000 - 7200,
                anomaly_score: 0.85,
                anomaly_reason: 'Unusually high fee'
            },
            {
                id: '0xfedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210',
                sender: '0x9876543210fedcba9876543210fedcba98765432',
                recipient: '0xabcdef1234567890abcdef1234567890abcdef12',
                amount: 5000,
                fee: 5,
                timestamp: Date.now() / 1000 - 10800,
                anomaly_score: 0.78,
                anomaly_reason: 'Unusual transaction pattern'
            }
        ];
    } catch (error) {
        console.error('Error fetching anomalous transactions:', error);
        return [];
    }
}

// 異常トランザクションリストを表示
function displayAnomalousTransactions(transactions) {
    const container = document.getElementById('anomalous-transactions');
    if (!container) return;
    
    if (transactions.length === 0) {
        container.innerHTML = `
            <div class="alert alert-info" role="alert">
                <i class="bi bi-info-circle me-2"></i>
                No anomalous transactions detected.
            </div>
        `;
        return;
    }
    
    let html = '<div class="list-group">';
    
    for (const tx of transactions) {
        html += `
            <div class="list-group-item">
                <div class="d-flex justify-content-between align-items-center">
                    <div>
                        <div class="transaction-id">
                            <a href="#" class="text-decoration-none tx-details" data-tx-id="${tx.id}">
                                ${formatTxId(tx.id)}
                            </a>
                        </div>
                        <div class="d-flex mt-1">
                            <div class="me-2">
                                <small class="text-muted">From:</small>
                                <span class="address ms-1">${formatAddress(tx.sender)}</span>
                            </div>
                            <div class="me-2">
                                <i class="bi bi-arrow-right text-muted"></i>
                            </div>
                            <div>
                                <small class="text-muted">To:</small>
                                <span class="address ms-1">${formatAddress(tx.recipient)}</span>
                            </div>
                        </div>
                    </div>
                    <div class="text-end">
                        <div class="amount">${formatAmount(tx.amount)}</div>
                        <div class="timestamp">${formatTimestamp(tx.timestamp)}</div>
                    </div>
                </div>
                <div class="mt-2">
                    <div class="d-flex justify-content-between align-items-center">
                        <span class="badge bg-danger">
                            Anomaly Score: ${tx.anomaly_score.toFixed(2)}
                        </span>
                        <small class="text-danger">${tx.anomaly_reason}</small>
                    </div>
                </div>
            </div>
        `;
    }
    
    html += '</div>';
    container.innerHTML = html;
    
    // トランザクション詳細リンクのイベントリスナーを追加
    container.querySelectorAll('.tx-details').forEach(link => {
        link.addEventListener('click', function(e) {
            e.preventDefault();
            const txId = this.getAttribute('data-tx-id');
            showTransactionDetails(txId);
        });
    });
}

// AI分析ページを表示
function showAiPage() {
    contentArea.innerHTML = `
        <h1 class="mb-4">AI Analytics</h1>
        
        <div class="row mb-4">
            <div class="col-md-6">
                <div class="card">
                    <div class="card-header">
                        <h5 class="mb-0">Anomaly Detection</h5>
                    </div>
                    <div class="card-body">
                        <canvas id="anomaly-chart" height="250"></canvas>
                    </div>
                </div>
            </div>
            <div class="col-md-6">
                <div class="card">
                    <div class="card-header">
                        <h5 class="mb-0">Transaction Prediction</h5>
                    </div>
                    <div class="card-body">
                        <canvas id="prediction-chart" height="250"></canvas>
                    </div>
                </div>
            </div>
        </div>
        
        <div class="card">
            <div class="card-header">
                <h5 class="mb-0">Anomalous Transactions</h5>
            </div>
            <div class="card-body">
                <div id="anomalous-transactions">
                    <div class="d-flex justify-content-center my-5">
                        <div class="spinner-border text-primary" role="status">
                            <span class="visually-hidden">Loading...</span>
                        </div>
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
            // Chart.js Annotationプラグインを読み込む
            const annotationScript = document.createElement('script');
            annotationScript.src = 'https://cdn.jsdelivr.net/npm/chartjs-plugin-annotation';
            annotationScript.onload = async () => {
                await loadAiData();
            };
            document.head.appendChild(annotationScript);
        };
        document.head.appendChild(script);
    } else {
        // すでに読み込まれている場合は直接実行
        loadAiData();
    }
}

// AI分析データを読み込む
async function loadAiData() {
    // 異常検出チャートを描画
    const anomalyData = await fetchAnomalyData();
    const anomalyChartContainer = document.getElementById('anomaly-chart');
    if (anomalyChartContainer) {
        drawAnomalyChart(anomalyChartContainer, anomalyData);
    }
    
    // 予測チャートを描画
    const predictionData = await fetchPredictionData();
    const predictionChartContainer = document.getElementById('prediction-chart');
    if (predictionChartContainer) {
        drawPredictionChart(predictionChartContainer, predictionData);
    }
    
    // 異常トランザクションリストを表示
    const anomalousTransactions = await fetchAnomalousTransactions();
    displayAnomalousTransactions(anomalousTransactions);
}