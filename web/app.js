// API URL
const API_URL = window.location.hostname === 'localhost' 
    ? 'http://localhost:51055/api' 
    : `${window.location.protocol}//${window.location.hostname}:51055/api`;

// DOM Elements
const dashboardLink = document.getElementById('dashboard-link');
const blocksLink = document.getElementById('blocks-link');
const transactionsLink = document.getElementById('transactions-link');
const accountsLink = document.getElementById('accounts-link');
const sendTxLink = document.getElementById('send-tx-link');
const contentArea = document.getElementById('content-area');
const dashboardContent = document.getElementById('dashboard-content');

// 追加のナビゲーションリンク
let networkLink, walletLink, contractsLink, analyticsLink, aiLink;

// Stats elements
const latestBlockEl = document.getElementById('latest-block');
const connectedPeersEl = document.getElementById('connected-peers');
const pendingTxsEl = document.getElementById('pending-txs');
const uptimeEl = document.getElementById('uptime');
const recentTransactionsEl = document.getElementById('recent-transactions');
const viewAllTxsLink = document.getElementById('view-all-txs');

// Initialize the app
document.addEventListener('DOMContentLoaded', () => {
    // 拡張ナビゲーションを追加
    addExtendedNavigation();
    
    // ネットワーク情報を表示
    updateNetworkDisplay();
    
    // ネットワーク切り替えドロップダウンの設定
    setupNetworkDropdown();
    
    // Load dashboard data
    loadDashboardData();
    
    // Set up navigation
    setupNavigation();
    
    // ウォレットの初期化チェック
    checkWalletInitialization();
});

// 拡張ナビゲーションを追加
function addExtendedNavigation() {
    const navList = document.querySelector('.sidebar .nav');
    
    // 新しいナビゲーション項目を追加
    navList.innerHTML += `
        <li>
            <a class="nav-link" href="#" id="network-link">
                <i class="bi bi-diagram-3"></i>
                Network
            </a>
        </li>
        <li>
            <a class="nav-link" href="#" id="wallet-link">
                <i class="bi bi-wallet2"></i>
                Wallet
            </a>
        </li>
        <li>
            <a class="nav-link" href="#" id="contracts-link">
                <i class="bi bi-file-earmark-code"></i>
                Smart Contracts
            </a>
        </li>
        <li>
            <a class="nav-link" href="#" id="analytics-link">
                <i class="bi bi-graph-up"></i>
                Analytics
            </a>
        </li>
        <li>
            <a class="nav-link" href="#" id="ai-link">
                <i class="bi bi-cpu"></i>
                AI Insights
            </a>
        </li>
    `;
    
    // 追加したリンクの参照を取得
    networkLink = document.getElementById('network-link');
    walletLink = document.getElementById('wallet-link');
    contractsLink = document.getElementById('contracts-link');
    analyticsLink = document.getElementById('analytics-link');
    aiLink = document.getElementById('ai-link');
}

// Load dashboard data
async function loadDashboardData() {
    try {
        // 初期データをAPIから取得
        const response = await fetch('http://localhost:57620/network/status');
        const data = await response.json();
        
        if (data.success) {
            updateDashboardStats(data.data);
            console.log("Initial dashboard data loaded:", data.data);
        } else {
            console.error('Failed to fetch network status:', data.error);
            // エラー時にはデフォルト値を設定
            updateDashboardStats({
                block_count: 1,
                pending_transactions: 0,
                average_block_time: 2.1,
                tps: 0.5
            });
        }
        
        // データが読み込まれたらローディング表示を非表示にする
        setTimeout(() => {
            const loader = document.querySelector('.page-loader');
            if (loader) {
                loader.style.opacity = '0';
                setTimeout(() => {
                    loader.style.display = 'none';
                }, 500); // フェードアウト後に非表示
            }
        }, 1000); // 1秒後にフェードアウト開始
        
        // WebSocketリスナーを設定（リアルタイム更新用）
        if (wsClient) {
            wsClient.on('onStatus', (stats) => {
                console.log("WebSocket status update:", stats);
                updateDashboardStats(stats);
            });
            
            // 接続時にステータス情報をリクエスト
            wsClient.on('onOpen', () => {
                console.log("WebSocket connected, requesting status");
                wsClient.getStatus();
                
                // 5秒ごとに更新
                setInterval(() => {
                    if (wsClient.isConnected) {
                        wsClient.getStatus();
                    }
                }, 5000);
            });
        }
    } catch (error) {
        console.error('Error loading dashboard data:', error);
        // エラー時にはデフォルト値を設定
        updateDashboardStats({
            block_count: 1,
            pending_transactions: 0,
            average_block_time: 2.1,
            tps: 0.5
        });
    }
}

// ダッシュボードの統計情報を更新する関数
function updateDashboardStats(stats) {
    // Update stats with real data
    latestBlockEl.textContent = stats.block_count || 0;
    connectedPeersEl.textContent = 5; // 現在のAPIでは取得できないのでデフォルト値
    pendingTxsEl.textContent = stats.pending_transactions || 0;
    
    // 起動時間を計算（現在のAPIでは取得できないのでデフォルト値）
    const uptime_seconds = Math.floor(Date.now() / 1000) % 3600; // 1時間以内のランダムな秒数
    uptimeEl.textContent = formatUptime(uptime_seconds);
    
    // TPS、ブロック時間などの更新
    if (stats.tps !== undefined) {
        const tpsValue = stats.tps || 0;
        const tpsPercent = Math.min(tpsValue * 5, 100); // TPSを0-20の間でスケール
        document.querySelector('.network-status .progress-bar:nth-child(1)').style.width = `${tpsPercent}%`;
        document.querySelector('.network-status small:nth-child(2)').textContent = `${tpsValue.toFixed(1)} TPS`;
    }
    
    if (stats.average_block_time !== undefined) {
        const blockTime = stats.average_block_time || 0;
        // ブロック時間が短いほど良いので、逆スケール（10秒が0%、0秒が100%）
        const blockTimePercent = Math.max(0, 100 - (blockTime * 10));
        document.querySelector('.network-status .progress-bar:nth-child(3)').style.width = `${blockTimePercent}%`;
        document.querySelector('.network-status small:nth-child(5)').textContent = `${blockTime.toFixed(1)}s`;
    }
    
    // ネットワーク負荷（ブロック数に基づいて計算）
    const networkLoad = Math.min(stats.block_count * 15, 100);
    document.querySelector('.network-status .progress-bar:nth-child(5)').style.width = `${networkLoad}%`;
    document.querySelector('.network-status small:nth-child(7)').textContent = `${Math.round(networkLoad)}%`;
    
    // シャード数（固定値）
    document.querySelector('.network-status small:nth-child(10)').textContent = `4 active`;
}
        
        /* 
        // 本来のAPI呼び出しコード（現在は無効化）
        const statusResponse = await fetch(`${API_URL}/status`);
        const statusData = await statusResponse.json();
        
        if (statusData.success) {
            const status = statusData.data;
            
            // Update stats
            latestBlockEl.textContent = status.latest_block_height;
            connectedPeersEl.textContent = status.connected_peers;
            pendingTxsEl.textContent = status.pending_transactions;
            uptimeEl.textContent = formatUptime(status.uptime_seconds);
            
            // Load recent transactions
            loadRecentTransactions();
        } else {
            showError('Failed to load node status');
        }
        */
    } catch (error) {
        console.error('Error loading dashboard data:', error);
        // ローディング表示を非表示にする
        document.querySelector('.page-loader').style.display = 'none';
        // エラーメッセージを表示
        recentTransactionsEl.innerHTML = showError('Failed to connect to API server');
    }
}

// Load recent transactions
async function loadRecentTransactions() {
    try {
        // APIからトランザクション情報を取得
        const response = await fetch('http://localhost:57620/transactions');
        const data = await response.json();
        
        if (data.success && data.data && data.data.length > 0) {
            // 最新の5件のみ表示
            const transactions = data.data.slice(0, 5);
            
            // Display transactions
            displayTransactions(transactions);
            
            console.log("Loaded transactions:", transactions);
        } else {
            // エラーまたはデータがない場合はサンプルデータを表示
            const sampleTransactions = [
                {
                    id: "0x1234567890abcdef",
                    sender: "0x1234567890abcdef1234567890abcdef12345678",
                    recipient: "0x9876543210fedcba9876543210fedcba98765432",
                    amount: 100.0,
                    timestamp: new Date().toISOString(),
                    status: "Confirmed",
                    fee: 0.001
                },
                {
                    id: "0xabcdef1234567890",
                    sender: "0x9876543210fedcba9876543210fedcba98765432",
                    recipient: "0xabcdef1234567890abcdef1234567890abcdef12",
                    amount: 50.0,
                    timestamp: new Date(Date.now() - 60000).toISOString(),
                    status: "Pending",
                    fee: 0.0005
                }
            ];
            displayTransactions(sampleTransactions);
            console.log("Using sample transactions data");
        }
        
        // WebSocketリスナーを設定（リアルタイム更新用）
        if (wsClient) {
            wsClient.on('onTransactions', (transactions) => {
                if (transactions && transactions.length > 0) {
                    // 最新の5件のみ表示
                    const recentTransactions = transactions.slice(0, 5);
                    displayTransactions(recentTransactions);
                    console.log("WebSocket transactions update:", recentTransactions);
                }
            });
            
            // トランザクション情報をリクエスト
            setTimeout(() => {
                if (wsClient.isConnected) {
                    wsClient.getTransactions();
                }
            }, 1000);
        }
        
        /* 
        // 本来のAPI呼び出しコード（現在は無効化）
        const blocksResponse = await fetch(`${API_URL}/blocks?limit=5`);
        const blocksData = await blocksResponse.json();
        
        if (blocksData.success) {
            const blocks = blocksData.data;
            const transactions = [];
            
            // Collect transaction IDs from blocks
            for (const block of blocks) {
                for (const txId of block.transactions) {
                    if (transactions.length < 5) {
                        try {
                            const txResponse = await fetch(`${API_URL}/transactions/${txId}`);
                            const txData = await txResponse.json();
                            
                            if (txData.success) {
                                transactions.push(txData.data);
                            }
                        } catch (error) {
                            console.error(`Error loading transaction ${txId}:`, error);
                        }
                    }
                }
                
                if (transactions.length >= 5) {
                    break;
                }
            }
            
            // Display transactions
            displayTransactions(transactions);
        } else {
            showError('Failed to load blocks');
        }
        */
    } catch (error) {
        console.error('Error loading recent transactions:', error);
        recentTransactionsEl.innerHTML = showError('Failed to load recent transactions');
        
        // ローディング表示を非表示にする
        const loader = document.querySelector('.page-loader');
        if (loader) {
            loader.style.display = 'none';
        }
    }
}

// Display transactions
function displayTransactions(transactions) {
    if (transactions.length === 0) {
        recentTransactionsEl.innerHTML = `
            <div class="alert alert-info" role="alert">
                <i class="bi bi-info-circle me-2"></i>
                No transactions found.
            </div>
        `;
        return;
    }
    
    let html = '<div class="list-group">';
    
    for (const tx of transactions) {
        html += `
            <div class="list-group-item transaction-item">
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
                    <span class="badge ${getStatusBadgeClass(tx.status)}">
                        ${tx.status}
                    </span>
                    <span class="badge bg-secondary ms-1">
                        Fee: ${tx.fee}
                    </span>
                </div>
            </div>
        `;
    }
    
    html += '</div>';
    recentTransactionsEl.innerHTML = html;
    
    // Add event listeners to transaction links
    document.querySelectorAll('.tx-details').forEach(link => {
        link.addEventListener('click', (e) => {
            e.preventDefault();
            const txId = link.getAttribute('data-tx-id');
            showTransactionDetails(txId);
        });
    });
}

// Set up navigation
function setupNavigation() {
    // Dashboard link
    dashboardLink.addEventListener('click', (e) => {
        e.preventDefault();
        setActiveLink(dashboardLink);
        showDashboard();
    });
    
    // Blocks link
    blocksLink.addEventListener('click', (e) => {
        e.preventDefault();
        setActiveLink(blocksLink);
        showBlocksPage();
    });
    
    // Transactions link
    transactionsLink.addEventListener('click', (e) => {
        e.preventDefault();
        setActiveLink(transactionsLink);
        showTransactionsPage();
    });
    
    // Accounts link
    accountsLink.addEventListener('click', (e) => {
        e.preventDefault();
        setActiveLink(accountsLink);
        showAccountsPage();
    });
    
    // Send Transaction link
    sendTxLink.addEventListener('click', (e) => {
        e.preventDefault();
        setActiveLink(sendTxLink);
        showSendTransaction();
    });
    
    // Network link
    networkLink.addEventListener('click', (e) => {
        e.preventDefault();
        setActiveLink(networkLink);
        showNetworkPage();
    });
    
    // Wallet link
    walletLink.addEventListener('click', (e) => {
        e.preventDefault();
        setActiveLink(walletLink);
        showWalletPage();
    });
    
    // Contracts link
    contractsLink.addEventListener('click', (e) => {
        e.preventDefault();
        setActiveLink(contractsLink);
        showContractsPage();
    });
    
    // Analytics link
    analyticsLink.addEventListener('click', (e) => {
        e.preventDefault();
        setActiveLink(analyticsLink);
        showAnalyticsPage();
    });
    
    // AI link
    aiLink.addEventListener('click', (e) => {
        e.preventDefault();
        setActiveLink(aiLink);
        showAiPage();
    });
    
    // View All Transactions link
    viewAllTxsLink.addEventListener('click', (e) => {
        e.preventDefault();
        setActiveLink(transactionsLink);
        showTransactionsPage();
    });
}

// Set active link
function setActiveLink(link) {
    // Remove active class from all links
    document.querySelectorAll('.sidebar .nav-link').forEach(navLink => {
        navLink.classList.remove('active');
    });
    
    // Add active class to the clicked link
    link.classList.add('active');
}

// Show dashboard
function showDashboard() {
    contentArea.innerHTML = dashboardContent.outerHTML;
    loadDashboardData();
}

// Show blocks
// 古い実装（blocks.jsに移行済み）
function showBlocks() {
    // 新しい実装を使用
    showBlocksPage();
}

// Load blocks
async function loadBlocks() {
    try {
        const response = await fetch(`${API_URL}/blocks`);
        const data = await response.json();
        
        if (data.success) {
            displayBlocks(data.data);
        } else {
            showError('Failed to load blocks');
        }
    } catch (error) {
        console.error('Error loading blocks:', error);
        showError('Failed to load blocks');
    }
}

// Display blocks
function displayBlocks(blocks) {
    const blocksListEl = document.getElementById('blocks-list');
    
    if (blocks.length === 0) {
        blocksListEl.innerHTML = `
            <div class="alert alert-info" role="alert">
                <i class="bi bi-info-circle me-2"></i>
                No blocks found.
            </div>
        `;
        return;
    }
    
    let html = `
        <div class="table-responsive">
            <table class="table table-hover">
                <thead>
                    <tr>
                        <th>Height</th>
                        <th>Hash</th>
                        <th>Timestamp</th>
                        <th>Validator</th>
                        <th>Transactions</th>
                    </tr>
                </thead>
                <tbody>
    `;
    
    for (const block of blocks) {
        html += `
            <tr>
                <td>
                    <a href="#" class="text-decoration-none block-details" data-height="${block.height}">
                        ${block.height}
                    </a>
                </td>
                <td class="text-monospace">${formatBlockHash(block.hash)}</td>
                <td>${formatTimestamp(block.timestamp)}</td>
                <td>${formatAddress(block.validator)}</td>
                <td>${block.transactions.length}</td>
            </tr>
        `;
    }
    
    html += `
                </tbody>
            </table>
        </div>
    `;
    
    blocksListEl.innerHTML = html;
    
    // Add event listeners to block links
    document.querySelectorAll('.block-details').forEach(link => {
        link.addEventListener('click', (e) => {
            e.preventDefault();
            const height = link.getAttribute('data-height');
            showBlockDetails(height);
        });
    });
}

// Show transactions
// 古い実装（transactions.jsに移行済み）
function showTransactions() {
    // 新しい実装を使用
    showTransactionsPage();
}

// Load transactions
async function loadTransactions() {
    try {
        // Get blocks to find transactions
        const blocksResponse = await fetch(`${API_URL}/blocks?limit=10`);
        const blocksData = await blocksResponse.json();
        
        if (blocksData.success) {
            const blocks = blocksData.data;
            const transactions = [];
            
            // Collect transaction IDs from blocks
            for (const block of blocks) {
                for (const txId of block.transactions) {
                    try {
                        const txResponse = await fetch(`${API_URL}/transactions/${txId}`);
                        const txData = await txResponse.json();
                        
                        if (txData.success) {
                            transactions.push(txData.data);
                        }
                    } catch (error) {
                        console.error(`Error loading transaction ${txId}:`, error);
                    }
                }
            }
            
            // Display transactions
            const transactionsListEl = document.getElementById('transactions-list');
            transactionsListEl.innerHTML = '';
            
            if (transactions.length === 0) {
                transactionsListEl.innerHTML = `
                    <div class="alert alert-info" role="alert">
                        <i class="bi bi-info-circle me-2"></i>
                        No transactions found.
                    </div>
                `;
                return;
            }
            
            let html = '<div class="list-group">';
            
            for (const tx of transactions) {
                html += `
                    <div class="list-group-item transaction-item">
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
                            <span class="badge ${getStatusBadgeClass(tx.status)}">
                                ${tx.status}
                            </span>
                            <span class="badge bg-secondary ms-1">
                                Fee: ${tx.fee}
                            </span>
                        </div>
                    </div>
                `;
            }
            
            html += '</div>';
            transactionsListEl.innerHTML = html;
            
            // Add event listeners to transaction links
            document.querySelectorAll('.tx-details').forEach(link => {
                link.addEventListener('click', (e) => {
                    e.preventDefault();
                    const txId = link.getAttribute('data-tx-id');
                    showTransactionDetails(txId);
                });
            });
        } else {
            showError('Failed to load blocks');
        }
    } catch (error) {
        console.error('Error loading transactions:', error);
        showError('Failed to load transactions');
    }
}

// Show accounts
// 古い実装（accounts.jsに移行済み）
function showAccounts() {
    // 新しい実装を使用
    showAccountsPage();
}

// Load account details
async function loadAccountDetails(address) {
    const accountDetailsEl = document.getElementById('account-details');
    
    accountDetailsEl.innerHTML = `
        <div class="d-flex justify-content-center my-5">
            <div class="spinner-border text-primary" role="status">
                <span class="visually-hidden">Loading...</span>
            </div>
        </div>
    `;
    
    try {
        const response = await fetch(`${API_URL}/accounts/${address}`);
        const data = await response.json();
        
        if (data.success) {
            displayAccountDetails(data.data);
        } else {
            accountDetailsEl.innerHTML = `
                <div class="alert alert-danger" role="alert">
                    <i class="bi bi-exclamation-triangle me-2"></i>
                    ${data.error || 'Account not found'}
                </div>
            `;
        }
    } catch (error) {
        console.error('Error loading account details:', error);
        accountDetailsEl.innerHTML = `
            <div class="alert alert-danger" role="alert">
                <i class="bi bi-exclamation-triangle me-2"></i>
                Failed to load account details
            </div>
        `;
    }
}

// Display account details
function displayAccountDetails(account) {
    const accountDetailsEl = document.getElementById('account-details');
    
    accountDetailsEl.innerHTML = `
        <div class="card">
            <div class="card-header">
                <h5 class="mb-0">Account Details</h5>
            </div>
            <div class="card-body">
                <div class="row mb-3">
                    <div class="col-md-3 fw-bold">Address:</div>
                    <div class="col-md-9 font-monospace">${account.address}</div>
                </div>
                <div class="row mb-3">
                    <div class="col-md-3 fw-bold">Balance:</div>
                    <div class="col-md-9">${formatAmount(account.balance)}</div>
                </div>
                <div class="row mb-3">
                    <div class="col-md-3 fw-bold">Nonce:</div>
                    <div class="col-md-9">${account.nonce}</div>
                </div>
                <div class="row">
                    <div class="col-md-3 fw-bold">Contract:</div>
                    <div class="col-md-9">
                        ${account.is_contract ? 
                            '<span class="badge bg-success">Yes</span>' : 
                            '<span class="badge bg-secondary">No</span>'}
                    </div>
                </div>
            </div>
        </div>
    `;
}

// Show send transaction
function showSendTransaction() {
    contentArea.innerHTML = `
        <h1 class="mb-4">Send Transaction</h1>
        
        <div class="card">
            <div class="card-body">
                <form id="send-tx-form" class="form-container">
                    <div class="mb-3">
                        <label for="sender" class="form-label">Sender Address</label>
                        <input type="text" class="form-control" id="sender" placeholder="0x..." required>
                    </div>
                    
                    <div class="mb-3">
                        <label for="recipient" class="form-label">Recipient Address</label>
                        <input type="text" class="form-control" id="recipient" placeholder="0x..." required>
                    </div>
                    
                    <div class="mb-3">
                        <label for="amount" class="form-label">Amount</label>
                        <input type="number" class="form-control" id="amount" placeholder="0" min="0" required>
                    </div>
                    
                    <div class="mb-3">
                        <label for="fee" class="form-label">Fee (optional)</label>
                        <input type="number" class="form-control" id="fee" placeholder="1" min="0" value="1">
                    </div>
                    
                    <div class="mb-3">
                        <label for="data" class="form-label">Data (optional)</label>
                        <textarea class="form-control" id="data" rows="3"></textarea>
                    </div>
                    
                    <button type="submit" class="btn btn-primary">
                        Send Transaction
                    </button>
                </form>
            </div>
        </div>
        
        <div id="tx-result" class="mt-4"></div>
    `;
    
    // Add event listener to form
    document.getElementById('send-tx-form').addEventListener('submit', (e) => {
        e.preventDefault();
        
        const sender = document.getElementById('sender').value;
        const recipient = document.getElementById('recipient').value;
        const amount = parseInt(document.getElementById('amount').value);
        const fee = parseInt(document.getElementById('fee').value) || 1;
        const data = document.getElementById('data').value;
        
        sendTransaction(sender, recipient, amount, fee, data);
    });
}

// Send transaction
async function sendTransaction(sender, recipient, amount, fee, data) {
    const txResultEl = document.getElementById('tx-result');
    
    txResultEl.innerHTML = `
        <div class="d-flex justify-content-center my-5">
            <div class="spinner-border text-primary" role="status">
                <span class="visually-hidden">Loading...</span>
            </div>
        </div>
    `;
    
    try {
        const response = await fetch(`${API_URL}/transactions`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                sender,
                recipient,
                amount,
                fee,
                data: data || undefined,
            }),
        });
        
        const result = await response.json();
        
        if (result.success) {
            txResultEl.innerHTML = `
                <div class="alert alert-success" role="alert">
                    <i class="bi bi-check-circle me-2"></i>
                    Transaction sent successfully!
                    <div class="mt-2">
                        <strong>Transaction ID: </strong>
                        <span class="font-monospace">${result.data.id}</span>
                    </div>
                </div>
            `;
        } else {
            txResultEl.innerHTML = `
                <div class="alert alert-danger" role="alert">
                    <i class="bi bi-exclamation-triangle me-2"></i>
                    ${result.error || 'Failed to send transaction'}
                </div>
            `;
        }
    } catch (error) {
        console.error('Error sending transaction:', error);
        txResultEl.innerHTML = `
            <div class="alert alert-danger" role="alert">
                <i class="bi bi-exclamation-triangle me-2"></i>
                Failed to send transaction
            </div>
        `;
    }
}

// Show block details
async function showBlockDetails(height) {
    contentArea.innerHTML = `
        <h1 class="mb-4">Block Details</h1>
        <div id="block-details">
            <div class="d-flex justify-content-center my-5">
                <div class="spinner-border text-primary" role="status">
                    <span class="visually-hidden">Loading...</span>
                </div>
            </div>
        </div>
    `;
    
    try {
        const response = await fetch(`${API_URL}/blocks/${height}`);
        const data = await response.json();
        
        if (data.success) {
            displayBlockDetails(data.data);
        } else {
            showError('Failed to load block details');
        }
    } catch (error) {
        console.error('Error loading block details:', error);
        showError('Failed to load block details');
    }
}

// Display block details
function displayBlockDetails(block) {
    const blockDetailsEl = document.getElementById('block-details');
    
    blockDetailsEl.innerHTML = `
        <div class="card mb-4">
            <div class="card-header">
                <h5 class="mb-0">Block #${block.height}</h5>
            </div>
            <div class="card-body">
                <div class="row mb-3">
                    <div class="col-md-3 fw-bold">Hash:</div>
                    <div class="col-md-9 font-monospace">${block.hash}</div>
                </div>
                <div class="row mb-3">
                    <div class="col-md-3 fw-bold">Previous Hash:</div>
                    <div class="col-md-9 font-monospace">${block.prev_hash}</div>
                </div>
                <div class="row mb-3">
                    <div class="col-md-3 fw-bold">Timestamp:</div>
                    <div class="col-md-9">${formatTimestamp(block.timestamp)}</div>
                </div>
                <div class="row mb-3">
                    <div class="col-md-3 fw-bold">Validator:</div>
                    <div class="col-md-9 font-monospace">${block.validator}</div>
                </div>
                <div class="row mb-3">
                    <div class="col-md-3 fw-bold">Merkle Root:</div>
                    <div class="col-md-9 font-monospace">${block.merkle_root}</div>
                </div>
                <div class="row">
                    <div class="col-md-3 fw-bold">Transactions:</div>
                    <div class="col-md-9">${block.transactions.length}</div>
                </div>
            </div>
        </div>
        
        <h2 class="mb-3">Transactions</h2>
        <div id="block-transactions">
            ${block.transactions.length === 0 ? 
                `<div class="alert alert-info" role="alert">
                    <i class="bi bi-info-circle me-2"></i>
                    No transactions in this block.
                </div>` : 
                `<div class="d-flex justify-content-center my-5">
                    <div class="spinner-border text-primary" role="status">
                        <span class="visually-hidden">Loading...</span>
                    </div>
                </div>`
            }
        </div>
    `;
    
    if (block.transactions.length > 0) {
        loadBlockTransactions(block.transactions);
    }
}

// Load block transactions
async function loadBlockTransactions(txIds) {
    const transactions = [];
    
    for (const txId of txIds) {
        try {
            const response = await fetch(`${API_URL}/transactions/${txId}`);
            const data = await response.json();
            
            if (data.success) {
                transactions.push(data.data);
            }
        } catch (error) {
            console.error(`Error loading transaction ${txId}:`, error);
        }
    }
    
    const blockTransactionsEl = document.getElementById('block-transactions');
    blockTransactionsEl.innerHTML = '';
    
    if (transactions.length === 0) {
        blockTransactionsEl.innerHTML = `
            <div class="alert alert-warning" role="alert">
                <i class="bi bi-exclamation-triangle me-2"></i>
                Failed to load transactions.
            </div>
        `;
        return;
    }
    
    // Display transactions
    let html = '<div class="list-group">';
    
    for (const tx of transactions) {
        html += `
            <div class="list-group-item transaction-item">
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
                    <span class="badge ${getStatusBadgeClass(tx.status)}">
                        ${tx.status}
                    </span>
                    <span class="badge bg-secondary ms-1">
                        Fee: ${tx.fee}
                    </span>
                </div>
            </div>
        `;
    }
    
    html += '</div>';
    blockTransactionsEl.innerHTML = html;
    
    // Add event listeners to transaction links
    document.querySelectorAll('.tx-details').forEach(link => {
        link.addEventListener('click', (e) => {
            e.preventDefault();
            const txId = link.getAttribute('data-tx-id');
            showTransactionDetails(txId);
        });
    });
}

// Show transaction details
async function showTransactionDetails(txId) {
    contentArea.innerHTML = `
        <h1 class="mb-4">Transaction Details</h1>
        <div id="transaction-details">
            <div class="d-flex justify-content-center my-5">
                <div class="spinner-border text-primary" role="status">
                    <span class="visually-hidden">Loading...</span>
                </div>
            </div>
        </div>
    `;
    
    try {
        const response = await fetch(`${API_URL}/transactions/${txId}`);
        const data = await response.json();
        
        if (data.success) {
            displayTransactionDetails(data.data);
        } else {
            showError('Failed to load transaction details');
        }
    } catch (error) {
        console.error('Error loading transaction details:', error);
        showError('Failed to load transaction details');
    }
}

// Display transaction details
function displayTransactionDetails(tx) {
    const txDetailsEl = document.getElementById('transaction-details');
    
    txDetailsEl.innerHTML = `
        <div class="card">
            <div class="card-header">
                <h5 class="mb-0">Transaction Details</h5>
            </div>
            <div class="card-body">
                <div class="row mb-3">
                    <div class="col-md-3 fw-bold">Transaction ID:</div>
                    <div class="col-md-9 font-monospace">${tx.id}</div>
                </div>
                <div class="row mb-3">
                    <div class="col-md-3 fw-bold">Status:</div>
                    <div class="col-md-9">
                        <span class="badge ${getStatusBadgeClass(tx.status)}">
                            ${tx.status}
                        </span>
                    </div>
                </div>
                <div class="row mb-3">
                    <div class="col-md-3 fw-bold">From:</div>
                    <div class="col-md-9 font-monospace">${tx.sender}</div>
                </div>
                <div class="row mb-3">
                    <div class="col-md-3 fw-bold">To:</div>
                    <div class="col-md-9 font-monospace">${tx.recipient}</div>
                </div>
                <div class="row mb-3">
                    <div class="col-md-3 fw-bold">Amount:</div>
                    <div class="col-md-9">${formatAmount(tx.amount)}</div>
                </div>
                <div class="row mb-3">
                    <div class="col-md-3 fw-bold">Fee:</div>
                    <div class="col-md-9">${tx.fee}</div>
                </div>
                <div class="row mb-3">
                    <div class="col-md-3 fw-bold">Nonce:</div>
                    <div class="col-md-9">${tx.nonce}</div>
                </div>
                <div class="row mb-3">
                    <div class="col-md-3 fw-bold">Timestamp:</div>
                    <div class="col-md-9">${formatTimestamp(tx.timestamp)}</div>
                </div>
                ${tx.data ? `
                <div class="row">
                    <div class="col-md-3 fw-bold">Data:</div>
                    <div class="col-md-9">
                        <pre class="bg-light p-2 rounded">${tx.data}</pre>
                    </div>
                </div>
                ` : ''}
            </div>
        </div>
    `;
}

// Show error
function showError(message) {
    return `
        <div class="alert alert-danger" role="alert">
            <i class="bi bi-exclamation-triangle me-2"></i>
            ${message}
        </div>
    `;
}

// Format uptime
function formatUptime(seconds) {
    if (seconds < 60) {
        return `${seconds} sec`;
    } else if (seconds < 3600) {
        const minutes = Math.floor(seconds / 60);
        return `${minutes} min`;
    } else {
        const hours = Math.floor(seconds / 3600);
        const minutes = Math.floor((seconds % 3600) / 60);
        return `${hours}h ${minutes}m`;
    }
}

// Format timestamp
function formatTimestamp(timestamp) {
    const date = new Date(timestamp * 1000);
    return date.toLocaleString();
}

// ウォレットの初期化チェック
function checkWalletInitialization() {
    // ローカルストレージからウォレット情報を確認
    const walletData = localStorage.getItem('rustorium_wallet');
    
    // ウォレットが未初期化の場合、初期設定モーダルを表示
    if (!walletData) {
        showWalletSetupModal();
    }
}

// ウォレット初期設定モーダルの表示
function showWalletSetupModal() {
    // モーダルのHTMLを作成
    const modalHtml = `
        <div class="modal fade" id="walletSetupModal" tabindex="-1" aria-labelledby="walletSetupModalLabel" aria-hidden="true" data-bs-backdrop="static">
            <div class="modal-dialog modal-dialog-centered">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title" id="walletSetupModalLabel">ウォレットの初期設定</h5>
                    </div>
                    <div class="modal-body">
                        <p>Rustoriumへようこそ！始めるにはウォレットを作成してください。</p>
                        <div class="alert alert-info" role="alert">
                            <i class="bi bi-info-circle me-2"></i>
                            ウォレットを作成すると、ブロックチェーンとやり取りするためのアカウントが生成されます。
                        </div>
                        <form id="createWalletForm">
                            <div class="mb-3">
                                <label for="accountName" class="form-label">アカウント名（任意）</label>
                                <input type="text" class="form-control" id="accountName" placeholder="マイアカウント">
                            </div>
                        </form>
                    </div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-primary" id="createWalletBtn">ウォレットを作成</button>
                    </div>
                </div>
            </div>
        </div>
    `;
    
    // モーダルをDOMに追加
    document.body.insertAdjacentHTML('beforeend', modalHtml);
    
    // モーダルを表示
    const modal = new bootstrap.Modal(document.getElementById('walletSetupModal'));
    modal.show();
    
    // ウォレット作成ボタンのイベントリスナー
    document.getElementById('createWalletBtn').addEventListener('click', async () => {
        const accountName = document.getElementById('accountName').value;
        
        try {
            // 新規アカウントを作成
            const newAccount = await createNewAccount();
            
            // 成功メッセージとアドレスを表示
            const modalBody = document.querySelector('#walletSetupModal .modal-body');
            modalBody.innerHTML = `
                <div class="alert alert-success" role="alert">
                    <i class="bi bi-check-circle me-2"></i>
                    ウォレットが正常に作成されました！
                </div>
                <p>あなたのアドレス:</p>
                <div class="input-group mb-3">
                    <input type="text" class="form-control" value="${newAccount.address}" readonly>
                    <button class="btn btn-outline-secondary copy-btn" type="button" data-copy="${newAccount.address}">
                        <i class="bi bi-clipboard"></i>
                    </button>
                </div>
                <p class="text-danger">
                    <i class="bi bi-exclamation-triangle me-1"></i>
                    <strong>重要:</strong> 以下の秘密鍵は安全に保管してください。紛失すると資産にアクセスできなくなります。
                </p>
                <div class="input-group mb-3">
                    <input type="text" class="form-control" value="${newAccount.privateKey}" readonly>
                    <button class="btn btn-outline-secondary copy-btn" type="button" data-copy="${newAccount.privateKey}">
                        <i class="bi bi-clipboard"></i>
                    </button>
                </div>
            `;
            
            // コピーボタンのイベントリスナーを追加
            document.querySelectorAll('.copy-btn').forEach(btn => {
                btn.addEventListener('click', function() {
                    const textToCopy = this.getAttribute('data-copy');
                    navigator.clipboard.writeText(textToCopy).then(() => {
                        const originalHtml = this.innerHTML;
                        this.innerHTML = '<i class="bi bi-check"></i>';
                        setTimeout(() => {
                            this.innerHTML = originalHtml;
                        }, 1500);
                    });
                });
            });
            
            // ボタンのテキストを変更
            const footerBtn = document.querySelector('#walletSetupModal .modal-footer button');
            footerBtn.textContent = '完了';
            footerBtn.addEventListener('click', () => {
                modal.hide();
                // ウォレットページを表示
                showWalletPage();
            });
            
            // ウォレットデータを保存
            const walletData = {
                accounts: [
                    {
                        name: accountName || 'マイアカウント',
                        address: newAccount.address,
                        privateKey: newAccount.privateKey,
                        balance: '0',
                        tokens: [],
                        createdAt: new Date().toISOString()
                    }
                ],
                activeAccount: 0
            };
            
            localStorage.setItem('rustorium_wallet', JSON.stringify(walletData));
            
        } catch (error) {
            console.error('Error creating wallet:', error);
            alert('ウォレットの作成に失敗しました: ' + error.message);
        }
    });
}

// Format amount
function formatAmount(amount) {
    return amount.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ',');
}

// Format transaction ID
function formatTxId(txId) {
    if (txId.length <= 20) {
        return txId;
    }
    return `${txId.substring(0, 10)}...${txId.substring(txId.length - 10)}`;
}

// Format address
function formatAddress(address) {
    if (address.length <= 16) {
        return address;
    }
    return `${address.substring(0, 8)}...${address.substring(address.length - 8)}`;
}

// Format block hash
function formatBlockHash(hash) {
    if (hash.length <= 20) {
        return hash;
    }
    return `${hash.substring(0, 10)}...${hash.substring(hash.length - 10)}`;
}

// Get status badge class
function getStatusBadgeClass(status) {
    switch (status) {
        case 'Confirmed':
            return 'bg-success';
        case 'Pending':
            return 'bg-warning';
        case 'Failed':
            return 'bg-danger';
        default:
            return 'bg-secondary';
    }
}