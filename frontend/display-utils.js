// 表示用ユーティリティ関数

// トランザクションIDを短縮表示
function formatTxId(txId) {
    if (!txId) return '';
    if (txId.length <= 10) return txId;
    return `${txId.substring(0, 6)}...${txId.substring(txId.length - 4)}`;
}

// アドレスを短縮表示
function formatAddress(address) {
    if (!address) return '';
    if (address.length <= 10) return address;
    return `${address.substring(0, 6)}...${address.substring(address.length - 4)}`;
}

// 金額をフォーマット
function formatAmount(amount) {
    if (amount === undefined || amount === null) return '0';
    return parseFloat(amount).toLocaleString(undefined, {
        minimumFractionDigits: 2,
        maximumFractionDigits: 6
    });
}

// タイムスタンプをフォーマット
function formatTimestamp(timestamp) {
    if (!timestamp) return '';
    const date = new Date(timestamp);
    return date.toLocaleString();
}

// 稼働時間をフォーマット
function formatUptime(seconds) {
    if (seconds < 60) {
        return `${seconds} sec`;
    } else if (seconds < 3600) {
        const minutes = Math.floor(seconds / 60);
        const remainingSeconds = seconds % 60;
        return `${minutes}m ${remainingSeconds}s`;
    } else {
        const hours = Math.floor(seconds / 3600);
        const minutes = Math.floor((seconds % 3600) / 60);
        return `${hours}h ${minutes}m`;
    }
}

// トランザクションステータスに応じたバッジクラスを取得
function getStatusBadgeClass(status) {
    if (!status) return 'bg-secondary';
    
    switch (status.toLowerCase()) {
        case 'confirmed':
            return 'bg-success';
        case 'pending':
            return 'bg-warning';
        case 'failed':
            return 'bg-danger';
        case 'rejected':
            return 'bg-danger';
        default:
            return 'bg-secondary';
    }
}

// トランザクションステータスに応じた色を取得
function getTxStatusColor(status) {
    if (!status) return 'secondary';
    
    switch (status.toLowerCase()) {
        case 'confirmed':
            return 'success';
        case 'pending':
            return 'warning';
        case 'failed':
            return 'danger';
        case 'rejected':
            return 'danger';
        default:
            return 'secondary';
    }
}

// エラーメッセージを表示するHTMLを生成
function showError(message) {
    return `
        <div class="alert alert-danger">
            <i class="bi bi-exclamation-triangle"></i> ${message}
        </div>
    `;
}

// トランザクションカードを表示
function displayTransactions(transactions) {
    if (!recentTransactionsEl) return;
    
    console.log("Displaying transactions:", transactions);
    
    if (!transactions || transactions.length === 0) {
        recentTransactionsEl.innerHTML = `
            <div class="alert alert-info">
                <i class="bi bi-info-circle"></i> No transactions found
            </div>
        `;
        return;
    }
    
    let html = '<div class="transaction-list">';
    
    for (const tx of transactions) {
        const txId = tx.id || '';
        const sender = tx.sender || '';
        const recipient = tx.recipient || '';
        const amount = tx.amount || 0;
        const timestamp = tx.timestamp || new Date().toISOString();
        const status = tx.status || 'Unknown';
        const fee = tx.fee || 0;
        
        html += `
            <div class="card mb-3 transaction-card">
                <div class="card-body">
                    <div class="d-flex justify-content-between align-items-center">
                        <div>
                            <div class="transaction-id">
                                <a href="#" class="text-decoration-none tx-details" data-tx-id="${txId}">
                                    ${formatTxId(txId)}
                                </a>
                            </div>
                            <div class="d-flex mt-1">
                                <div class="me-2">
                                    <small class="text-muted">From:</small>
                                    <span class="address ms-1">${formatAddress(sender)}</span>
                                </div>
                                <div class="me-2">
                                    <i class="bi bi-arrow-right text-muted"></i>
                                </div>
                                <div>
                                    <small class="text-muted">To:</small>
                                    <span class="address ms-1">${formatAddress(recipient)}</span>
                                </div>
                            </div>
                        </div>
                        <div class="text-end">
                            <div class="amount">${formatAmount(amount)}</div>
                            <div class="timestamp">${formatTimestamp(timestamp)}</div>
                        </div>
                    </div>
                    <div class="mt-2">
                        <span class="badge ${getStatusBadgeClass(status)}">
                            ${status}
                        </span>
                        <span class="badge bg-secondary ms-1">
                            Fee: ${fee}
                        </span>
                    </div>
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