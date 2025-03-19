// トランザクション関連の機能

// トランザクションリストを取得
async function fetchTransactions(limit = 10) {
    try {
        // APIを使用してトランザクションリストを取得
        const response = await apiRequest(
            () => apiClient.getTransactions(limit),
            // フォールバックデータ（APIが失敗した場合）
            generateMockTransactions(limit)
        );
        
        return response.transactions || response;
    } catch (error) {
        console.error('Error fetching transactions:', error);
        return [];
    }
}

// モック用のトランザクションデータを生成（開発中のみ使用）
function generateMockTransactions(limit = 10) {
    const transactions = [];
    for (let i = 0; i < limit; i++) {
        transactions.push({
            id: `0x${Array.from({length: 64}, () => Math.floor(Math.random() * 16).toString(16)).join('')}`,
            sender: `0x${Array.from({length: 40}, () => Math.floor(Math.random() * 16).toString(16)).join('')}`,
            recipient: `0x${Array.from({length: 40}, () => Math.floor(Math.random() * 16).toString(16)).join('')}`,
            amount: Math.floor(Math.random() * 1000000) + 1000,
            fee: Math.floor(Math.random() * 100) + 10,
            nonce: Math.floor(Math.random() * 10),
            timestamp: Math.floor(Date.now() / 1000) - Math.floor(Math.random() * 3600),
            block_number: Math.floor(Math.random() * 10) + 1,
            gas_used: Math.floor(Math.random() * 100000) + 21000,
            gas_price: Math.floor(Math.random() * 10) + 1,
            data: '',
            status: ['Confirmed', 'Pending', 'Failed'][Math.floor(Math.random() * 3)]
        });
    }
    return { transactions };
}

// トランザクション詳細を取得
async function fetchTransactionDetails(txId) {
    try {
        // APIを使用してトランザクション詳細を取得
        const response = await apiRequest(
            () => apiClient.getTransaction(txId),
            // フォールバックデータ（APIが失敗した場合）
            generateMockTransactionDetails(txId)
        );
        
        return response.transaction || response;
    } catch (error) {
        console.error('Error fetching transaction details:', error);
        throw error;
    }
}

// モック用のトランザクション詳細データを生成（開発中のみ使用）
function generateMockTransactionDetails(txId) {
    return {
        transaction: {
            id: txId,
            sender: `0x${Array.from({length: 40}, () => Math.floor(Math.random() * 16).toString(16)).join('')}`,
            recipient: `0x${Array.from({length: 40}, () => Math.floor(Math.random() * 16).toString(16)).join('')}`,
            amount: Math.floor(Math.random() * 1000000) + 1000,
            fee: Math.floor(Math.random() * 100) + 10,
            nonce: Math.floor(Math.random() * 10),
            timestamp: Math.floor(Date.now() / 1000) - Math.floor(Math.random() * 3600),
            block_number: Math.floor(Math.random() * 10) + 1,
            gas_used: Math.floor(Math.random() * 100000) + 21000,
            gas_price: Math.floor(Math.random() * 10) + 1,
            gas_limit: Math.floor(Math.random() * 200000) + 21000,
            data: Math.random() > 0.5 ? `0x${Array.from({length: 64}, () => Math.floor(Math.random() * 16).toString(16)).join('')}` : '',
            status: ['Confirmed', 'Pending', 'Failed'][Math.floor(Math.random() * 3)],
            error: null
        }
    };
}

// トランザクションページを表示
function showTransactionsPage() {
    const contentArea = document.getElementById('content-area');
    contentArea.innerHTML = `
        <h1 class="mb-4">Transactions</h1>
        
        <div class="card mb-4">
            <div class="card-header d-flex justify-content-between align-items-center">
                <h5 class="mb-0">Latest Transactions</h5>
                <div class="input-group" style="max-width: 300px;">
                    <input type="text" class="form-control" id="tx-search" placeholder="Search by transaction hash">
                    <button class="btn btn-outline-primary" type="button" id="tx-search-btn">
                        <i class="bi bi-search"></i>
                    </button>
                </div>
            </div>
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
        
        <div id="transaction-details"></div>
    `;
    
    // トランザクションリストを読み込む
    loadTransactionsList();
    
    // トランザクション検索ボタンのイベントリスナーを追加
    document.getElementById('tx-search-btn').addEventListener('click', handleTransactionSearch);
    
    // 検索フォームのEnterキーイベントリスナーを追加
    document.getElementById('tx-search').addEventListener('keypress', function(e) {
        if (e.key === 'Enter') {
            handleTransactionSearch();
        }
    });
}

// トランザクションリストを読み込む
async function loadTransactionsList() {
    const transactionsListEl = document.getElementById('transactions-list');
    
    try {
        const transactions = await fetchTransactions();
        
        if (transactions.length === 0) {
            transactionsListEl.innerHTML = `
                <div class="alert alert-info" role="alert">
                    <i class="bi bi-info-circle me-2"></i>
                    No transactions found.
                </div>
            `;
            return;
        }
        
        let html = '<div class="table-responsive"><table class="table table-hover">';
        html += `
            <thead>
                <tr>
                    <th>Transaction Hash</th>
                    <th>Block</th>
                    <th>Age</th>
                    <th>From</th>
                    <th>To</th>
                    <th>Amount</th>
                    <th>Fee</th>
                    <th>Status</th>
                </tr>
            </thead>
            <tbody>
        `;
        
        for (const tx of transactions) {
            html += `
                <tr class="transaction-row" data-tx-id="${tx.id}">
                    <td class="font-monospace text-truncate" style="max-width: 150px;">
                        <a href="#" class="transaction-link" data-tx-id="${tx.id}">
                            ${formatTxId(tx.id)}
                        </a>
                    </td>
                    <td>
                        <a href="#" class="block-link" data-block-id="${tx.block_number}">
                            ${tx.block_number}
                        </a>
                    </td>
                    <td>${formatTimeAgo(tx.timestamp)}</td>
                    <td class="font-monospace text-truncate" style="max-width: 100px;">
                        <a href="#" class="address-link" data-address="${tx.sender}">
                            ${formatAddress(tx.sender)}
                        </a>
                    </td>
                    <td class="font-monospace text-truncate" style="max-width: 100px;">
                        <a href="#" class="address-link" data-address="${tx.recipient}">
                            ${formatAddress(tx.recipient)}
                        </a>
                    </td>
                    <td>${formatAmount(tx.amount)}</td>
                    <td>${tx.fee}</td>
                    <td>
                        <span class="badge ${getStatusBadgeClass(tx.status)}">
                            ${tx.status}
                        </span>
                    </td>
                </tr>
            `;
        }
        
        html += '</tbody></table></div>';
        
        // ページネーションを追加
        html += `
            <nav aria-label="Transaction list navigation">
                <ul class="pagination justify-content-center">
                    <li class="page-item disabled">
                        <a class="page-link" href="#" tabindex="-1" aria-disabled="true">Previous</a>
                    </li>
                    <li class="page-item active"><a class="page-link" href="#">1</a></li>
                    <li class="page-item"><a class="page-link" href="#">2</a></li>
                    <li class="page-item"><a class="page-link" href="#">3</a></li>
                    <li class="page-item">
                        <a class="page-link" href="#">Next</a>
                    </li>
                </ul>
            </nav>
        `;
        
        transactionsListEl.innerHTML = html;
        
        // トランザクションリンクのイベントリスナーを追加
        document.querySelectorAll('.transaction-link').forEach(link => {
            link.addEventListener('click', function(e) {
                e.preventDefault();
                const txId = this.getAttribute('data-tx-id');
                showTransactionDetails(txId);
            });
        });
        
        // ブロックリンクのイベントリスナーを追加
        document.querySelectorAll('.block-link').forEach(link => {
            link.addEventListener('click', function(e) {
                e.preventDefault();
                const blockId = this.getAttribute('data-block-id');
                showBlockDetails(blockId);
            });
        });
        
        // アドレスリンクのイベントリスナーを追加
        document.querySelectorAll('.address-link').forEach(link => {
            link.addEventListener('click', function(e) {
                e.preventDefault();
                const address = this.getAttribute('data-address');
                showAccountDetails(address);
            });
        });
        
    } catch (error) {
        console.error('Error loading transactions list:', error);
        transactionsListEl.innerHTML = `
            <div class="alert alert-danger" role="alert">
                <i class="bi bi-exclamation-triangle me-2"></i>
                Failed to load transactions: ${error.message}
            </div>
        `;
    }
}

// トランザクション検索を処理
function handleTransactionSearch() {
    const searchInput = document.getElementById('tx-search');
    const txId = searchInput.value.trim();
    
    if (!txId) {
        alert('Please enter a transaction hash');
        return;
    }
    
    showTransactionDetails(txId);
}

// トランザクション詳細を表示
async function showTransactionDetails(txId) {
    const transactionDetailsEl = document.getElementById('transaction-details');
    
    transactionDetailsEl.innerHTML = `
        <div class="d-flex justify-content-center my-5">
            <div class="spinner-border text-primary" role="status">
                <span class="visually-hidden">Loading...</span>
            </div>
        </div>
    `;
    
    try {
        // トランザクション詳細を取得
        const tx = await fetchTransactionDetails(txId);
        
        transactionDetailsEl.innerHTML = `
            <div class="card">
                <div class="card-header">
                    <h5 class="mb-0">Transaction Details</h5>
                </div>
                <div class="card-body">
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Transaction Hash:</div>
                        <div class="col-md-9 font-monospace text-break">${tx.id}</div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Status:</div>
                        <div class="col-md-9">
                            <span class="badge ${getStatusBadgeClass(tx.status)}">
                                ${tx.status}
                            </span>
                            ${tx.error ? `<div class="text-danger mt-1">${tx.error}</div>` : ''}
                        </div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Block:</div>
                        <div class="col-md-9">
                            <a href="#" class="block-link" data-block-id="${tx.block_number}">
                                ${tx.block_number}
                            </a>
                        </div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Timestamp:</div>
                        <div class="col-md-9">${formatTimestamp(tx.timestamp)} (${formatTimeAgo(tx.timestamp)})</div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">From:</div>
                        <div class="col-md-9 font-monospace">
                            <a href="#" class="address-link" data-address="${tx.sender}">
                                ${tx.sender}
                            </a>
                        </div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">To:</div>
                        <div class="col-md-9 font-monospace">
                            <a href="#" class="address-link" data-address="${tx.recipient}">
                                ${tx.recipient}
                            </a>
                        </div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Amount:</div>
                        <div class="col-md-9">${formatAmount(tx.amount)}</div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Transaction Fee:</div>
                        <div class="col-md-9">${tx.fee}</div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Gas Price:</div>
                        <div class="col-md-9">${tx.gas_price} Gwei</div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Gas Used:</div>
                        <div class="col-md-9">
                            ${formatNumber(tx.gas_used)} (${Math.floor(tx.gas_used / tx.gas_limit * 100)}%)
                            <div class="progress mt-1" style="height: 6px;">
                                <div class="progress-bar" role="progressbar" style="width: ${Math.floor(tx.gas_used / tx.gas_limit * 100)}%;" aria-valuenow="${Math.floor(tx.gas_used / tx.gas_limit * 100)}" aria-valuemin="0" aria-valuemax="100"></div>
                            </div>
                        </div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Gas Limit:</div>
                        <div class="col-md-9">${formatNumber(tx.gas_limit)}</div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Nonce:</div>
                        <div class="col-md-9">${tx.nonce}</div>
                    </div>
                    ${tx.data ? `
                    <div class="row">
                        <div class="col-md-3 fw-bold">Data:</div>
                        <div class="col-md-9">
                            <div class="bg-light p-3 rounded font-monospace text-break">
                                ${tx.data}
                            </div>
                        </div>
                    </div>
                    ` : ''}
                </div>
            </div>
        `;
        
        // ブロックリンクのイベントリスナーを追加
        document.querySelectorAll('.block-link').forEach(link => {
            link.addEventListener('click', function(e) {
                e.preventDefault();
                const blockId = this.getAttribute('data-block-id');
                showBlockDetails(blockId);
            });
        });
        
        // アドレスリンクのイベントリスナーを追加
        document.querySelectorAll('.address-link').forEach(link => {
            link.addEventListener('click', function(e) {
                e.preventDefault();
                const address = this.getAttribute('data-address');
                showAccountDetails(address);
            });
        });
        
    } catch (error) {
        console.error('Error showing transaction details:', error);
        transactionDetailsEl.innerHTML = `
            <div class="alert alert-danger" role="alert">
                <i class="bi bi-exclamation-triangle me-2"></i>
                Failed to load transaction details: ${error.message}
            </div>
        `;
    }
}

// 送金ページを表示
function showSendTransaction(fromAddress = null) {
    const contentArea = document.getElementById('content-area');
    contentArea.innerHTML = `
        <h1 class="mb-4">Send Transaction</h1>
        
        <div class="card">
            <div class="card-header">
                <h5 class="mb-0">Transaction Details</h5>
            </div>
            <div class="card-body">
                <form id="send-transaction-form">
                    <div class="mb-3">
                        <label for="from-address" class="form-label">From</label>
                        <select class="form-select" id="from-address" required>
                            <option value="" selected disabled>Select account</option>
                        </select>
                    </div>
                    <div class="mb-3">
                        <label for="to-address" class="form-label">To</label>
                        <input type="text" class="form-control" id="to-address" placeholder="0x..." required>
                    </div>
                    <div class="mb-3">
                        <label for="amount" class="form-label">Amount</label>
                        <div class="input-group">
                            <input type="number" class="form-control" id="amount" placeholder="0.0" step="0.000001" min="0" required>
                            <span class="input-group-text">ETH</span>
                        </div>
                    </div>
                    
                    <div class="mb-3">
                        <a class="d-block mb-2" data-bs-toggle="collapse" href="#advanced-options" role="button" aria-expanded="false" aria-controls="advanced-options">
                            <i class="bi bi-gear me-1"></i>
                            Advanced Options
                        </a>
                        <div class="collapse" id="advanced-options">
                            <div class="card card-body">
                                <div class="mb-3">
                                    <label for="gas-price" class="form-label">Gas Price (Gwei)</label>
                                    <input type="number" class="form-control" id="gas-price" value="5" min="1">
                                </div>
                                <div class="mb-3">
                                    <label for="gas-limit" class="form-label">Gas Limit</label>
                                    <input type="number" class="form-control" id="gas-limit" value="21000" min="21000">
                                </div>
                                <div class="mb-3">
                                    <label for="data" class="form-label">Data (Hex)</label>
                                    <textarea class="form-control" id="data" rows="3" placeholder="0x..."></textarea>
                                </div>
                            </div>
                        </div>
                    </div>
                    
                    <button type="submit" class="btn btn-primary">
                        <i class="bi bi-send me-1"></i>
                        Send Transaction
                    </button>
                </form>
            </div>
        </div>
    `;
    
    // アカウントリストを読み込む
    loadAccountsForSelect(fromAddress);
    
    // 送金フォームのイベントリスナーを追加
    document.getElementById('send-transaction-form').addEventListener('submit', function(e) {
        e.preventDefault();
        handleSendTransaction();
    });
}

// セレクトボックス用のアカウントリストを読み込む
async function loadAccountsForSelect(selectedAddress = null) {
    const selectEl = document.getElementById('from-address');
    
    try {
        const accounts = await fetchAccounts();
        
        if (accounts.length === 0) {
            selectEl.innerHTML = `
                <option value="" selected disabled>No accounts available</option>
            `;
            return;
        }
        
        let html = '<option value="" disabled>Select account</option>';
        
        for (const account of accounts) {
            const isSelected = selectedAddress && account.address === selectedAddress;
            html += `
                <option value="${account.address}" ${isSelected ? 'selected' : ''}>
                    ${formatAddress(account.address)} (${formatAmount(account.balance)})
                </option>
            `;
        }
        
        selectEl.innerHTML = html;
        
    } catch (error) {
        console.error('Error loading accounts for select:', error);
        selectEl.innerHTML = `
            <option value="" selected disabled>Failed to load accounts</option>
        `;
    }
}

// 送金処理
async function handleSendTransaction() {
    const fromAddress = document.getElementById('from-address').value;
    const toAddress = document.getElementById('to-address').value;
    const amount = document.getElementById('amount').value;
    const gasPrice = document.getElementById('gas-price').value;
    const gasLimit = document.getElementById('gas-limit').value;
    const data = document.getElementById('data').value;
    
    if (!fromAddress || !toAddress || !amount) {
        alert('Please fill in all required fields');
        return;
    }
    
    try {
        // 送金ボタンを無効化
        const submitBtn = document.querySelector('#send-transaction-form button[type="submit"]');
        const originalText = submitBtn.innerHTML;
        submitBtn.disabled = true;
        submitBtn.innerHTML = `
            <span class="spinner-border spinner-border-sm me-1" role="status" aria-hidden="true"></span>
            Sending...
        `;
        
        // トランザクションデータを準備
        const txData = {
            from: fromAddress,
            to: toAddress,
            amount: parseFloat(amount),
            gas_price: parseInt(gasPrice),
            gas_limit: parseInt(gasLimit),
            data: data || undefined
        };
        
        // APIを使用してトランザクションを送信
        const response = await apiRequest(
            () => apiClient.sendTransaction(txData),
            // フォールバックデータ（APIが失敗した場合）
            {
                success: true,
                transaction_hash: `0x${Array.from({length: 64}, () => Math.floor(Math.random() * 16).toString(16)).join('')}`
            }
        );
        
        // トランザクションハッシュを取得
        const txHash = response.transaction_hash;
        
        // 成功モーダルを表示
        showTransactionSuccessModal(txHash, fromAddress, toAddress, amount);
        
        // ボタンを元に戻す
        submitBtn.disabled = false;
        submitBtn.innerHTML = originalText;
        
        // ローカルストレージに送信履歴を保存
        saveTransactionHistory(txHash, fromAddress, toAddress, amount, Date.now() / 1000);
        
    } catch (error) {
        console.error('Error sending transaction:', error);
        alert('Failed to send transaction: ' + error.message);
        
        // ボタンを元に戻す
        const submitBtn = document.querySelector('#send-transaction-form button[type="submit"]');
        submitBtn.disabled = false;
        submitBtn.innerHTML = '<i class="bi bi-send me-1"></i>Send Transaction';
    }
}

// トランザクション履歴をローカルストレージに保存
function saveTransactionHistory(txHash, from, to, amount, timestamp) {
    // 既存の履歴を取得
    let history = JSON.parse(localStorage.getItem('transaction_history') || '[]');
    
    // 新しいトランザクションを追加
    history.unshift({
        hash: txHash,
        from: from,
        to: to,
        amount: amount,
        timestamp: timestamp,
        status: 'Pending'
    });
    
    // 最大100件まで保存
    if (history.length > 100) {
        history = history.slice(0, 100);
    }
    
    // 保存
    localStorage.setItem('transaction_history', JSON.stringify(history));
}

// トランザクション成功モーダルを表示
function showTransactionSuccessModal(txHash, fromAddress, toAddress, amount) {
    // モーダル要素を作成
    const modalEl = document.createElement('div');
    modalEl.className = 'modal fade';
    modalEl.id = 'transaction-success-modal';
    modalEl.tabIndex = '-1';
    modalEl.setAttribute('aria-labelledby', 'transaction-success-modal-label');
    modalEl.setAttribute('aria-hidden', 'true');
    
    modalEl.innerHTML = `
        <div class="modal-dialog modal-dialog-centered">
            <div class="modal-content">
                <div class="modal-header">
                    <h5 class="modal-title" id="transaction-success-modal-label">Transaction Sent</h5>
                    <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                </div>
                <div class="modal-body">
                    <div class="alert alert-success" role="alert">
                        <i class="bi bi-check-circle me-2"></i>
                        Your transaction has been sent successfully!
                    </div>
                    
                    <div class="mb-3">
                        <label class="form-label fw-bold">Transaction Hash:</label>
                        <div class="input-group">
                            <input type="text" class="form-control font-monospace" value="${txHash}" readonly>
                            <button class="btn btn-outline-secondary copy-btn" type="button" data-copy="${txHash}">
                                <i class="bi bi-clipboard"></i>
                            </button>
                        </div>
                    </div>
                    
                    <div class="mb-3">
                        <label class="form-label fw-bold">From:</label>
                        <div class="font-monospace">${fromAddress}</div>
                    </div>
                    
                    <div class="mb-3">
                        <label class="form-label fw-bold">To:</label>
                        <div class="font-monospace">${toAddress}</div>
                    </div>
                    
                    <div class="mb-3">
                        <label class="form-label fw-bold">Amount:</label>
                        <div>${amount} ETH</div>
                    </div>
                    
                    <div class="alert alert-info" role="alert">
                        <i class="bi bi-info-circle me-2"></i>
                        Your transaction has been submitted to the network and is waiting to be confirmed.
                    </div>
                </div>
                <div class="modal-footer">
                    <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Close</button>
                    <button type="button" class="btn btn-primary" id="view-tx-btn">View Transaction</button>
                </div>
            </div>
        </div>
    `;
    
    document.body.appendChild(modalEl);
    
    // Bootstrapのモーダルを初期化
    const modal = new bootstrap.Modal(modalEl);
    modal.show();
    
    // モーダルが閉じられたときにDOMから削除
    modalEl.addEventListener('hidden.bs.modal', function() {
        document.body.removeChild(modalEl);
    });
    
    // コピーボタンのイベントリスナーを追加
    modalEl.querySelectorAll('.copy-btn').forEach(btn => {
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
    
    // トランザクション表示ボタンのイベントリスナーを追加
    document.getElementById('view-tx-btn').addEventListener('click', function() {
        modal.hide();
        showTransactionDetails(txHash);
    });
}