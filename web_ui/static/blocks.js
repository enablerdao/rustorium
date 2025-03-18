// ブロック関連の機能

// ブロックリストを取得
async function fetchBlocks(limit = 10) {
    try {
        // 実際のAPIが実装されるまでダミーデータを使用
        const blocks = [];
        for (let i = 10; i > 0; i--) {
            blocks.push({
                number: i,
                hash: `0x${Array.from({length: 64}, () => Math.floor(Math.random() * 16).toString(16)).join('')}`,
                timestamp: Math.floor(Date.now() / 1000) - i * 12,
                transactions: Array.from({length: Math.floor(Math.random() * 5) + 1}, () => 
                    `0x${Array.from({length: 64}, () => Math.floor(Math.random() * 16).toString(16)).join('')}`
                ),
                size: Math.floor(Math.random() * 5000) + 1000,
                gas_used: Math.floor(Math.random() * 8000000) + 2000000,
                gas_limit: 10000000,
                validator: `0x${Array.from({length: 40}, () => Math.floor(Math.random() * 16).toString(16)).join('')}`
            });
        }
        return blocks;
    } catch (error) {
        console.error('Error fetching blocks:', error);
        return [];
    }
}

// ブロック詳細を取得
async function fetchBlockDetails(blockId) {
    try {
        // 実際のAPIが実装されるまでダミーデータを使用
        const isHash = blockId.startsWith('0x');
        const blockNumber = isHash ? Math.floor(Math.random() * 10) + 1 : parseInt(blockId);
        
        return {
            number: blockNumber,
            hash: isHash ? blockId : `0x${Array.from({length: 64}, () => Math.floor(Math.random() * 16).toString(16)).join('')}`,
            parent_hash: `0x${Array.from({length: 64}, () => Math.floor(Math.random() * 16).toString(16)).join('')}`,
            timestamp: Math.floor(Date.now() / 1000) - blockNumber * 12,
            transactions: Array.from({length: Math.floor(Math.random() * 5) + 1}, () => 
                `0x${Array.from({length: 64}, () => Math.floor(Math.random() * 16).toString(16)).join('')}`
            ),
            size: Math.floor(Math.random() * 5000) + 1000,
            gas_used: Math.floor(Math.random() * 8000000) + 2000000,
            gas_limit: 10000000,
            difficulty: Math.floor(Math.random() * 1000) + 100,
            validator: `0x${Array.from({length: 40}, () => Math.floor(Math.random() * 16).toString(16)).join('')}`
        };
    } catch (error) {
        console.error('Error fetching block details:', error);
        throw error;
    }
}

// ブロックのトランザクションを取得
async function fetchBlockTransactions(blockId) {
    try {
        // 実際のAPIが実装されるまでダミーデータを使用
        const transactions = [];
        const count = Math.floor(Math.random() * 5) + 1;
        
        for (let i = 0; i < count; i++) {
            const txId = `0x${Array.from({length: 64}, () => Math.floor(Math.random() * 16).toString(16)).join('')}`;
            transactions.push({
                id: txId,
                sender: `0x${Array.from({length: 40}, () => Math.floor(Math.random() * 16).toString(16)).join('')}`,
                recipient: `0x${Array.from({length: 40}, () => Math.floor(Math.random() * 16).toString(16)).join('')}`,
                amount: Math.floor(Math.random() * 1000000) + 1000,
                fee: Math.floor(Math.random() * 100) + 10,
                nonce: Math.floor(Math.random() * 10),
                timestamp: Math.floor(Date.now() / 1000) - Math.floor(Math.random() * 3600),
                data: '',
                status: 'Confirmed'
            });
        }
        
        return transactions;
    } catch (error) {
        console.error('Error fetching block transactions:', error);
        return [];
    }
}

// ブロックページを表示
function showBlocksPage() {
    const contentArea = document.getElementById('content-area');
    contentArea.innerHTML = `
        <h1 class="mb-4">Blocks</h1>
        
        <div class="card mb-4">
            <div class="card-header d-flex justify-content-between align-items-center">
                <h5 class="mb-0">Latest Blocks</h5>
                <div class="input-group" style="max-width: 300px;">
                    <input type="text" class="form-control" id="block-search" placeholder="Search by block number or hash">
                    <button class="btn btn-outline-primary" type="button" id="block-search-btn">
                        <i class="bi bi-search"></i>
                    </button>
                </div>
            </div>
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
        
        <div id="block-details"></div>
    `;
    
    // ブロックリストを読み込む
    loadBlocksList();
    
    // ブロック検索ボタンのイベントリスナーを追加
    document.getElementById('block-search-btn').addEventListener('click', handleBlockSearch);
    
    // 検索フォームのEnterキーイベントリスナーを追加
    document.getElementById('block-search').addEventListener('keypress', function(e) {
        if (e.key === 'Enter') {
            handleBlockSearch();
        }
    });
}

// ブロックリストを読み込む
async function loadBlocksList() {
    const blocksListEl = document.getElementById('blocks-list');
    
    try {
        const blocks = await fetchBlocks();
        
        if (blocks.length === 0) {
            blocksListEl.innerHTML = `
                <div class="alert alert-info" role="alert">
                    <i class="bi bi-info-circle me-2"></i>
                    No blocks found.
                </div>
            `;
            return;
        }
        
        let html = '<div class="table-responsive"><table class="table table-hover">';
        html += `
            <thead>
                <tr>
                    <th>Block</th>
                    <th>Hash</th>
                    <th>Age</th>
                    <th>Txns</th>
                    <th>Size</th>
                    <th>Gas Used</th>
                    <th>Validator</th>
                </tr>
            </thead>
            <tbody>
        `;
        
        for (const block of blocks) {
            html += `
                <tr class="block-row" data-block-id="${block.number}">
                    <td>
                        <a href="#" class="block-link" data-block-id="${block.number}">
                            ${block.number}
                        </a>
                    </td>
                    <td class="font-monospace text-truncate" style="max-width: 150px;">
                        <a href="#" class="block-link" data-block-id="${block.hash}">
                            ${block.hash}
                        </a>
                    </td>
                    <td>${formatTimeAgo(block.timestamp)}</td>
                    <td>${block.transactions.length}</td>
                    <td>${formatSize(block.size)}</td>
                    <td>${formatNumber(block.gas_used)} (${Math.floor(block.gas_used / block.gas_limit * 100)}%)</td>
                    <td class="font-monospace text-truncate" style="max-width: 100px;">
                        <a href="#" class="address-link" data-address="${block.validator}">
                            ${formatAddress(block.validator)}
                        </a>
                    </td>
                </tr>
            `;
        }
        
        html += '</tbody></table></div>';
        
        // ページネーションを追加
        html += `
            <nav aria-label="Block list navigation">
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
        
        blocksListEl.innerHTML = html;
        
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
        console.error('Error loading blocks list:', error);
        blocksListEl.innerHTML = `
            <div class="alert alert-danger" role="alert">
                <i class="bi bi-exclamation-triangle me-2"></i>
                Failed to load blocks: ${error.message}
            </div>
        `;
    }
}

// ブロック検索を処理
function handleBlockSearch() {
    const searchInput = document.getElementById('block-search');
    const blockId = searchInput.value.trim();
    
    if (!blockId) {
        alert('Please enter a block number or hash');
        return;
    }
    
    showBlockDetails(blockId);
}

// ブロック詳細を表示
async function showBlockDetails(blockId) {
    const blockDetailsEl = document.getElementById('block-details');
    
    blockDetailsEl.innerHTML = `
        <div class="d-flex justify-content-center my-5">
            <div class="spinner-border text-primary" role="status">
                <span class="visually-hidden">Loading...</span>
            </div>
        </div>
    `;
    
    try {
        // ブロック詳細を取得
        const block = await fetchBlockDetails(blockId);
        
        // ブロックのトランザクションを取得
        const transactions = await fetchBlockTransactions(blockId);
        
        blockDetailsEl.innerHTML = `
            <div class="card mb-4">
                <div class="card-header d-flex justify-content-between align-items-center">
                    <h5 class="mb-0">Block #${block.number}</h5>
                    <div>
                        <button class="btn btn-sm btn-outline-primary" id="prev-block-btn" ${block.number <= 1 ? 'disabled' : ''}>
                            <i class="bi bi-chevron-left me-1"></i>
                            Previous Block
                        </button>
                        <button class="btn btn-sm btn-outline-primary" id="next-block-btn">
                            Next Block
                            <i class="bi bi-chevron-right ms-1"></i>
                        </button>
                    </div>
                </div>
                <div class="card-body">
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Block Number:</div>
                        <div class="col-md-9">${block.number}</div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Block Hash:</div>
                        <div class="col-md-9 font-monospace text-break">${block.hash}</div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Parent Hash:</div>
                        <div class="col-md-9 font-monospace text-break">
                            <a href="#" class="block-link" data-block-id="${block.parent_hash}">
                                ${block.parent_hash}
                            </a>
                        </div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Timestamp:</div>
                        <div class="col-md-9">${formatTimestamp(block.timestamp)} (${formatTimeAgo(block.timestamp)})</div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Transactions:</div>
                        <div class="col-md-9">${block.transactions.length}</div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Size:</div>
                        <div class="col-md-9">${formatSize(block.size)}</div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Gas Used:</div>
                        <div class="col-md-9">
                            ${formatNumber(block.gas_used)} (${Math.floor(block.gas_used / block.gas_limit * 100)}%)
                            <div class="progress mt-1" style="height: 6px;">
                                <div class="progress-bar" role="progressbar" style="width: ${Math.floor(block.gas_used / block.gas_limit * 100)}%;" aria-valuenow="${Math.floor(block.gas_used / block.gas_limit * 100)}" aria-valuemin="0" aria-valuemax="100"></div>
                            </div>
                        </div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Gas Limit:</div>
                        <div class="col-md-9">${formatNumber(block.gas_limit)}</div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Difficulty:</div>
                        <div class="col-md-9">${formatNumber(block.difficulty)}</div>
                    </div>
                    <div class="row">
                        <div class="col-md-3 fw-bold">Validator:</div>
                        <div class="col-md-9 font-monospace">
                            <a href="#" class="address-link" data-address="${block.validator}">
                                ${formatAddress(block.validator)}
                            </a>
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="card">
                <div class="card-header">
                    <h5 class="mb-0">Transactions (${transactions.length})</h5>
                </div>
                <div class="card-body">
                    <div id="block-transactions">
                        ${transactions.length === 0 ? `
                            <div class="alert alert-info" role="alert">
                                <i class="bi bi-info-circle me-2"></i>
                                No transactions in this block.
                            </div>
                        ` : ''}
                    </div>
                </div>
            </div>
        `;
        
        // トランザクションリストを表示
        if (transactions.length > 0) {
            displayBlockTransactions('block-transactions', transactions);
        }
        
        // 前のブロックボタンのイベントリスナーを追加
        if (block.number > 1) {
            document.getElementById('prev-block-btn').addEventListener('click', function() {
                showBlockDetails(block.number - 1);
            });
        }
        
        // 次のブロックボタンのイベントリスナーを追加
        document.getElementById('next-block-btn').addEventListener('click', function() {
            showBlockDetails(block.number + 1);
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
        console.error('Error showing block details:', error);
        blockDetailsEl.innerHTML = `
            <div class="alert alert-danger" role="alert">
                <i class="bi bi-exclamation-triangle me-2"></i>
                Failed to load block details: ${error.message}
            </div>
        `;
    }
}

// ブロックのトランザクションを表示
function displayBlockTransactions(containerId, transactions) {
    const container = document.getElementById(containerId);
    
    let html = '<div class="table-responsive"><table class="table table-hover">';
    html += `
        <thead>
            <tr>
                <th>Transaction Hash</th>
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
    container.innerHTML = html;
    
    // トランザクションリンクのイベントリスナーを追加
    document.querySelectorAll('.transaction-link').forEach(link => {
        link.addEventListener('click', function(e) {
            e.preventDefault();
            const txId = this.getAttribute('data-tx-id');
            showTransactionDetails(txId);
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
}

// ステータスに応じたバッジクラスを取得
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

// サイズをフォーマット
function formatSize(bytes) {
    if (bytes < 1024) {
        return `${bytes} B`;
    } else if (bytes < 1024 * 1024) {
        return `${(bytes / 1024).toFixed(2)} KB`;
    } else {
        return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
    }
}

// 数値をフォーマット
function formatNumber(num) {
    return num.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ',');
}

// 経過時間をフォーマット
function formatTimeAgo(timestamp) {
    const now = Math.floor(Date.now() / 1000);
    const diff = now - timestamp;
    
    if (diff < 60) {
        return `${diff} sec ago`;
    } else if (diff < 3600) {
        return `${Math.floor(diff / 60)} min ago`;
    } else if (diff < 86400) {
        return `${Math.floor(diff / 3600)} hr ago`;
    } else {
        return `${Math.floor(diff / 86400)} days ago`;
    }
}

// トランザクションIDをフォーマット
function formatTxId(txId) {
    if (txId.length <= 20) return txId;
    return `${txId.substring(0, 10)}...${txId.substring(txId.length - 8)}`;
}

// アドレスをフォーマット
function formatAddress(address) {
    if (address.length <= 20) return address;
    return `${address.substring(0, 6)}...${address.substring(address.length - 6)}`;
}