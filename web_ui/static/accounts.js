// アカウント関連の機能

// アカウントリストを取得
async function fetchAccountsList(limit = 10) {
    try {
        // 実際のAPIが実装されるまでダミーデータを使用
        const accounts = [];
        for (let i = 0; i < limit; i++) {
            accounts.push({
                address: `0x${Array.from({length: 40}, () => Math.floor(Math.random() * 16).toString(16)).join('')}`,
                balance: Math.floor(Math.random() * 10000000) + 1000,
                nonce: Math.floor(Math.random() * 100),
                is_contract: Math.random() > 0.8,
                transaction_count: Math.floor(Math.random() * 1000),
                last_activity: Math.floor(Date.now() / 1000) - Math.floor(Math.random() * 86400 * 30)
            });
        }
        return accounts;
    } catch (error) {
        console.error('Error fetching accounts list:', error);
        return [];
    }
}

// アカウントページを表示
function showAccountsPage() {
    const contentArea = document.getElementById('content-area');
    contentArea.innerHTML = `
        <h1 class="mb-4">Accounts</h1>
        
        <div class="card mb-4">
            <div class="card-header d-flex justify-content-between align-items-center">
                <h5 class="mb-0">Top Accounts</h5>
                <div class="input-group" style="max-width: 300px;">
                    <input type="text" class="form-control" id="account-search" placeholder="Search by address">
                    <button class="btn btn-outline-primary" type="button" id="account-search-btn">
                        <i class="bi bi-search"></i>
                    </button>
                </div>
            </div>
            <div class="card-body">
                <div id="accounts-list">
                    <div class="d-flex justify-content-center my-5">
                        <div class="spinner-border text-primary" role="status">
                            <span class="visually-hidden">Loading...</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        
        <div id="account-details-container"></div>
    `;
    
    // アカウントリストを読み込む
    loadAccountsList();
    
    // アカウント検索ボタンのイベントリスナーを追加
    document.getElementById('account-search-btn').addEventListener('click', handleAccountSearch);
    
    // 検索フォームのEnterキーイベントリスナーを追加
    document.getElementById('account-search').addEventListener('keypress', function(e) {
        if (e.key === 'Enter') {
            handleAccountSearch();
        }
    });
}

// アカウントリストを読み込む
async function loadAccountsList() {
    const accountsListEl = document.getElementById('accounts-list');
    
    try {
        const accounts = await fetchAccountsList();
        
        if (accounts.length === 0) {
            accountsListEl.innerHTML = `
                <div class="alert alert-info" role="alert">
                    <i class="bi bi-info-circle me-2"></i>
                    No accounts found.
                </div>
            `;
            return;
        }
        
        let html = '<div class="table-responsive"><table class="table table-hover">';
        html += `
            <thead>
                <tr>
                    <th>Address</th>
                    <th>Balance</th>
                    <th>Txn Count</th>
                    <th>Type</th>
                    <th>Last Activity</th>
                </tr>
            </thead>
            <tbody>
        `;
        
        for (const account of accounts) {
            html += `
                <tr class="account-row" data-address="${account.address}">
                    <td class="font-monospace text-truncate" style="max-width: 150px;">
                        <a href="#" class="address-link" data-address="${account.address}">
                            ${formatAddress(account.address)}
                        </a>
                    </td>
                    <td>${formatAmount(account.balance)}</td>
                    <td>${formatNumber(account.transaction_count)}</td>
                    <td>
                        ${account.is_contract ? 
                            '<span class="badge bg-info">Contract</span>' : 
                            '<span class="badge bg-success">EOA</span>'}
                    </td>
                    <td>${formatTimeAgo(account.last_activity)}</td>
                </tr>
            `;
        }
        
        html += '</tbody></table></div>';
        
        // ページネーションを追加
        html += `
            <nav aria-label="Account list navigation">
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
        
        accountsListEl.innerHTML = html;
        
        // アドレスリンクのイベントリスナーを追加
        document.querySelectorAll('.address-link').forEach(link => {
            link.addEventListener('click', function(e) {
                e.preventDefault();
                const address = this.getAttribute('data-address');
                showAccountDetails(address);
            });
        });
        
    } catch (error) {
        console.error('Error loading accounts list:', error);
        accountsListEl.innerHTML = `
            <div class="alert alert-danger" role="alert">
                <i class="bi bi-exclamation-triangle me-2"></i>
                Failed to load accounts: ${error.message}
            </div>
        `;
    }
}

// アカウント検索を処理
function handleAccountSearch() {
    const searchInput = document.getElementById('account-search');
    const address = searchInput.value.trim();
    
    if (!address) {
        alert('Please enter an account address');
        return;
    }
    
    showAccountDetails(address);
}

// アカウント詳細を表示（アカウントページ用）
async function showAccountDetails(address) {
    // ウォレットページからの呼び出しの場合は、ウォレットページの関数を使用
    if (document.getElementById('account-details')) {
        // ウォレットページのアカウント詳細表示関数を呼び出し
        return;
    }
    
    const accountDetailsEl = document.getElementById('account-details-container');
    
    accountDetailsEl.innerHTML = `
        <div class="d-flex justify-content-center my-5">
            <div class="spinner-border text-primary" role="status">
                <span class="visually-hidden">Loading...</span>
            </div>
        </div>
    `;
    
    try {
        // アカウント情報を取得
        const accounts = await fetchAccounts();
        let account = accounts.find(a => a.address === address);
        
        if (!account) {
            // 見つからない場合はダミーデータを生成
            account = {
                address: address,
                balance: Math.floor(Math.random() * 10000000) + 1000,
                nonce: Math.floor(Math.random() * 100),
                is_contract: Math.random() > 0.8,
                transaction_count: Math.floor(Math.random() * 1000),
                last_activity: Math.floor(Date.now() / 1000) - Math.floor(Math.random() * 86400 * 30)
            };
        }
        
        // トランザクション履歴を取得
        const transactions = await fetchAccountTransactions(address);
        
        accountDetailsEl.innerHTML = `
            <div class="card mb-4">
                <div class="card-header">
                    <h5 class="mb-0">Account Details</h5>
                </div>
                <div class="card-body">
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Address:</div>
                        <div class="col-md-9">
                            <div class="input-group">
                                <input type="text" class="form-control font-monospace" value="${account.address}" readonly>
                                <button class="btn btn-outline-secondary copy-btn" type="button" data-copy="${account.address}">
                                    <i class="bi bi-clipboard"></i>
                                </button>
                            </div>
                        </div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Balance:</div>
                        <div class="col-md-9">
                            <h3 class="mb-0">${formatAmount(account.balance)}</h3>
                        </div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Nonce:</div>
                        <div class="col-md-9">${account.nonce}</div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Transaction Count:</div>
                        <div class="col-md-9">${formatNumber(account.transaction_count)}</div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Last Activity:</div>
                        <div class="col-md-9">${formatTimestamp(account.last_activity)} (${formatTimeAgo(account.last_activity)})</div>
                    </div>
                    <div class="row">
                        <div class="col-md-3 fw-bold">Account Type:</div>
                        <div class="col-md-9">
                            ${account.is_contract ? 
                                '<span class="badge bg-info">Contract</span>' : 
                                '<span class="badge bg-success">EOA (Externally Owned Account)</span>'}
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="card">
                <div class="card-header d-flex justify-content-between align-items-center">
                    <h5 class="mb-0">Transaction History</h5>
                    <div class="btn-group btn-group-sm" role="group">
                        <button type="button" class="btn btn-outline-secondary active" data-filter="all">All</button>
                        <button type="button" class="btn btn-outline-secondary" data-filter="sent">Sent</button>
                        <button type="button" class="btn btn-outline-secondary" data-filter="received">Received</button>
                    </div>
                </div>
                <div class="card-body">
                    <div id="account-transactions">
                        ${transactions.length === 0 ? `
                            <div class="alert alert-info" role="alert">
                                <i class="bi bi-info-circle me-2"></i>
                                No transactions found for this account.
                            </div>
                        ` : ''}
                    </div>
                </div>
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
        
        // トランザクション履歴を表示
        if (transactions.length > 0) {
            displayAccountTransactions('account-transactions', transactions, address);
        }
        
        // トランザクションフィルターボタンのイベントリスナーを追加
        document.querySelectorAll('[data-filter]').forEach(btn => {
            btn.addEventListener('click', function() {
                // アクティブクラスを切り替え
                document.querySelectorAll('[data-filter]').forEach(b => b.classList.remove('active'));
                this.classList.add('active');
                
                const filter = this.getAttribute('data-filter');
                
                // フィルターに基づいてトランザクションを表示
                if (filter === 'all') {
                    displayAccountTransactions('account-transactions', transactions, address);
                } else if (filter === 'sent') {
                    const sentTxs = transactions.filter(tx => tx.sender === address);
                    displayAccountTransactions('account-transactions', sentTxs, address);
                } else if (filter === 'received') {
                    const receivedTxs = transactions.filter(tx => tx.recipient === address);
                    displayAccountTransactions('account-transactions', receivedTxs, address);
                }
            });
        });
        
    } catch (error) {
        console.error('Error showing account details:', error);
        accountDetailsEl.innerHTML = `
            <div class="alert alert-danger" role="alert">
                <i class="bi bi-exclamation-triangle me-2"></i>
                Failed to load account details: ${error.message}
            </div>
        `;
    }
}