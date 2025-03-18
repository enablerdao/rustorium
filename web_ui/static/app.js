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
    
    // Load dashboard data
    loadDashboardData();
    
    // Set up navigation
    setupNavigation();
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
        // Get node status
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
    } catch (error) {
        console.error('Error loading dashboard data:', error);
        showError('Failed to connect to API server');
    }
}

// Load recent transactions
async function loadRecentTransactions() {
    try {
        // Get blocks to find transactions
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
    } catch (error) {
        console.error('Error loading recent transactions:', error);
        showError('Failed to load recent transactions');
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
        showBlocks();
    });
    
    // Transactions link
    transactionsLink.addEventListener('click', (e) => {
        e.preventDefault();
        setActiveLink(transactionsLink);
        showTransactions();
    });
    
    // Accounts link
    accountsLink.addEventListener('click', (e) => {
        e.preventDefault();
        setActiveLink(accountsLink);
        showAccounts();
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
        showTransactions();
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
function showBlocks() {
    contentArea.innerHTML = `
        <h1 class="mb-4">Blocks</h1>
        <div class="card">
            <div class="card-body">
                <div id="blocks-list">
                    <div class="d-flex justify-content-center my-5">
                        <div class="spinner-border text-primary" role="status">
                            <span class="visually-hidden">Loading...</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    `;
    
    loadBlocks();
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
function showTransactions() {
    contentArea.innerHTML = `
        <h1 class="mb-4">Transactions</h1>
        <div class="card">
            <div class="card-body">
                <div id="transactions-list">
                    <div class="d-flex justify-content-center my-5">
                        <div class="spinner-border text-primary" role="status">
                            <span class="visually-hidden">Loading...</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    `;
    
    loadTransactions();
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
function showAccounts() {
    contentArea.innerHTML = `
        <h1 class="mb-4">Accounts</h1>
        
        <div class="card mb-4">
            <div class="card-body">
                <form id="account-search-form">
                    <div class="input-group">
                        <input type="text" class="form-control" id="account-address" placeholder="Enter account address (0x...)" required>
                        <button class="btn btn-primary" type="submit">
                            Search
                        </button>
                    </div>
                </form>
            </div>
        </div>
        
        <div id="account-details"></div>
    `;
    
    // Add event listener to form
    document.getElementById('account-search-form').addEventListener('submit', (e) => {
        e.preventDefault();
        const address = document.getElementById('account-address').value;
        loadAccountDetails(address);
    });
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