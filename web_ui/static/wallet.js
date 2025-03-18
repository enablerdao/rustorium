// ウォレット機能用のスクリプト
let currentAccount = null;

// アカウントリストを取得
async function fetchAccounts() {
    try {
        // 実際のAPIが実装されるまでダミーデータを使用
        return [
            {
                address: '0x1234567890abcdef1234567890abcdef12345678',
                balance: 1000000,
                nonce: 5,
                is_contract: false
            },
            {
                address: '0xabcdef1234567890abcdef1234567890abcdef12',
                balance: 500000,
                nonce: 3,
                is_contract: false
            },
            {
                address: '0x9876543210fedcba9876543210fedcba98765432',
                balance: 750000,
                nonce: 8,
                is_contract: false
            }
        ];
    } catch (error) {
        console.error('Error fetching accounts:', error);
        return [];
    }
}

// アカウントのトランザクション履歴を取得
async function fetchAccountTransactions(address) {
    try {
        // 実際のAPIが実装されるまでダミーデータを使用
        return [
            {
                id: '0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef',
                sender: address,
                recipient: '0xabcdef1234567890abcdef1234567890abcdef12',
                amount: 1000,
                fee: 10,
                nonce: 5,
                timestamp: 1677721600,
                data: '',
                status: 'Confirmed'
            },
            {
                id: '0xfedcba0987654321fedcba0987654321fedcba0987654321fedcba0987654321',
                sender: '0x9876543210fedcba9876543210fedcba98765432',
                recipient: address,
                amount: 2500,
                fee: 15,
                nonce: 3,
                timestamp: 1677721300,
                data: '',
                status: 'Confirmed'
            },
            {
                id: '0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890',
                sender: address,
                recipient: '0x9876543210fedcba9876543210fedcba98765432',
                amount: 500,
                fee: 5,
                nonce: 4,
                timestamp: 1677720000,
                data: '',
                status: 'Confirmed'
            }
        ];
    } catch (error) {
        console.error('Error fetching account transactions:', error);
        return [];
    }
}

// 新しいアカウントを作成
async function createNewAccount() {
    try {
        // 実際のAPIが実装されるまでダミーデータを使用
        const randomHex = Array.from({length: 40}, () => 
            Math.floor(Math.random() * 16).toString(16)
        ).join('');
        
        return {
            address: '0x' + randomHex,
            balance: 0,
            nonce: 0,
            is_contract: false,
            privateKey: '0x' + Array.from({length: 64}, () => 
                Math.floor(Math.random() * 16).toString(16)
            ).join('')
        };
    } catch (error) {
        console.error('Error creating new account:', error);
        throw error;
    }
}

// ウォレットページを表示
function showWalletPage() {
    contentArea.innerHTML = `
        <h1 class="mb-4">Wallet</h1>
        
        <div class="row mb-4">
            <div class="col-md-8">
                <div class="card">
                    <div class="card-header d-flex justify-content-between align-items-center">
                        <h5 class="mb-0">My Accounts</h5>
                        <button class="btn btn-sm btn-primary" id="create-account-btn">
                            <i class="bi bi-plus-circle me-1"></i>
                            Create New Account
                        </button>
                    </div>
                    <div class="card-body">
                        <div id="account-list">
                            <div class="d-flex justify-content-center my-5">
                                <div class="spinner-border text-primary" role="status">
                                    <span class="visually-hidden">Loading...</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="col-md-4">
                <div class="card">
                    <div class="card-header">
                        <h5 class="mb-0">Import Account</h5>
                    </div>
                    <div class="card-body">
                        <form id="import-account-form">
                            <div class="mb-3">
                                <label for="private-key" class="form-label">Private Key</label>
                                <input type="password" class="form-control" id="private-key" placeholder="0x..." required>
                                <div class="form-text">Enter your private key to import an existing account.</div>
                            </div>
                            <button type="submit" class="btn btn-primary">
                                <i class="bi bi-upload me-1"></i>
                                Import
                            </button>
                        </form>
                    </div>
                </div>
            </div>
        </div>
        
        <div id="account-details"></div>
    `;

    // アカウントリストを読み込む
    loadAccountList();
    
    // 新規アカウント作成ボタンのイベントリスナーを追加
    document.getElementById('create-account-btn').addEventListener('click', handleCreateAccount);
    
    // アカウントインポートフォームのイベントリスナーを追加
    document.getElementById('import-account-form').addEventListener('submit', function(e) {
        e.preventDefault();
        handleImportAccount();
    });
}

// アカウントリストを読み込む
async function loadAccountList() {
    const accountListEl = document.getElementById('account-list');
    
    try {
        const accounts = await fetchAccounts();
        
        if (accounts.length === 0) {
            accountListEl.innerHTML = `
                <div class="alert alert-info" role="alert">
                    <i class="bi bi-info-circle me-2"></i>
                    No accounts found. Create a new account to get started.
                </div>
            `;
            return;
        }
        
        let html = '<div class="list-group">';
        
        for (const account of accounts) {
            html += `
                <a href="#" class="list-group-item list-group-item-action account-item" data-address="${account.address}">
                    <div class="d-flex justify-content-between align-items-center">
                        <div>
                            <div class="fw-bold font-monospace">${formatAddress(account.address)}</div>
                            <div class="small text-muted">
                                <span class="me-2">Balance: ${formatAmount(account.balance)}</span>
                                <span>Nonce: ${account.nonce}</span>
                            </div>
                        </div>
                        <div>
                            <span class="badge bg-primary rounded-pill">
                                <i class="bi bi-eye me-1"></i>
                                View
                            </span>
                        </div>
                    </div>
                </a>
            `;
        }
        
        html += '</div>';
        accountListEl.innerHTML = html;
        
        // アカウント項目のイベントリスナーを追加
        document.querySelectorAll('.account-item').forEach(item => {
            item.addEventListener('click', function(e) {
                e.preventDefault();
                const address = this.getAttribute('data-address');
                showAccountDetails(address);
            });
        });
        
    } catch (error) {
        console.error('Error loading account list:', error);
        accountListEl.innerHTML = `
            <div class="alert alert-danger" role="alert">
                <i class="bi bi-exclamation-triangle me-2"></i>
                Failed to load accounts: ${error.message}
            </div>
        `;
    }
}

// アカウント詳細を表示
async function showAccountDetails(address) {
    currentAccount = address;
    const accountDetailsEl = document.getElementById('account-details');
    
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
        const account = accounts.find(a => a.address === address);
        
        if (!account) {
            throw new Error('Account not found');
        }
        
        // トランザクション履歴を取得
        const transactions = await fetchAccountTransactions(address);
        
        accountDetailsEl.innerHTML = `
            <div class="card mb-4">
                <div class="card-header d-flex justify-content-between align-items-center">
                    <h5 class="mb-0">Account Details</h5>
                    <div>
                        <button class="btn btn-sm btn-outline-primary me-2" id="export-key-btn">
                            <i class="bi bi-key me-1"></i>
                            Export Private Key
                        </button>
                        <button class="btn btn-sm btn-primary" id="send-from-account-btn">
                            <i class="bi bi-send me-1"></i>
                            Send
                        </button>
                    </div>
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
                        <div class="col-md-3 fw-bold">QR Code:</div>
                        <div class="col-md-9">
                            <div id="account-qrcode"></div>
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="card">
                <div class="card-header">
                    <h5 class="mb-0">Transaction History</h5>
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
        
        // QRコードを生成
        generateQRCode('account-qrcode', account.address);
        
        // トランザクション履歴を表示
        if (transactions.length > 0) {
            displayAccountTransactions('account-transactions', transactions, address);
        }
        
        // 秘密鍵エクスポートボタンのイベントリスナーを追加
        document.getElementById('export-key-btn').addEventListener('click', function() {
            handleExportPrivateKey(address);
        });
        
        // 送金ボタンのイベントリスナーを追加
        document.getElementById('send-from-account-btn').addEventListener('click', function() {
            showSendTransaction(address);
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

// アカウントのトランザクション履歴を表示
function displayAccountTransactions(containerId, transactions, accountAddress) {
    const container = document.getElementById(containerId);
    
    let html = '<div class="list-group">';
    
    for (const tx of transactions) {
        const isOutgoing = tx.sender === accountAddress;
        
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
                                <span class="address ms-1 ${isOutgoing ? 'text-danger' : ''}">${formatAddress(tx.sender)}</span>
                            </div>
                            <div class="me-2">
                                <i class="bi bi-arrow-right text-muted"></i>
                            </div>
                            <div>
                                <small class="text-muted">To:</small>
                                <span class="address ms-1 ${!isOutgoing ? 'text-success' : ''}">${formatAddress(tx.recipient)}</span>
                            </div>
                        </div>
                    </div>
                    <div class="text-end">
                        <div class="amount ${isOutgoing ? 'text-danger' : 'text-success'}">
                            ${isOutgoing ? '-' : '+'} ${formatAmount(tx.amount)}
                        </div>
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
    container.innerHTML = html;
    
    // トランザクション詳細リンクのイベントリスナーを追加
    document.querySelectorAll('.tx-details').forEach(link => {
        link.addEventListener('click', function(e) {
            e.preventDefault();
            const txId = this.getAttribute('data-tx-id');
            showTransactionDetails(txId);
        });
    });
}

// QRコードを生成
function generateQRCode(containerId, data) {
    const container = document.getElementById(containerId);
    if (!container) return;
    
    // QRコードライブラリが読み込まれているか確認
    if (typeof QRCode === 'undefined') {
        // QRコードライブラリを動的に読み込む
        const script = document.createElement('script');
        script.src = 'https://cdn.jsdelivr.net/npm/qrcode@1.5.1/build/qrcode.min.js';
        script.onload = () => {
            new QRCode(container, {
                text: data,
                width: 128,
                height: 128,
                colorDark: '#000000',
                colorLight: '#ffffff',
                correctLevel: QRCode.CorrectLevel.H
            });
        };
        document.head.appendChild(script);
    } else {
        // すでに読み込まれている場合は直接実行
        new QRCode(container, {
            text: data,
            width: 128,
            height: 128,
            colorDark: '#000000',
            colorLight: '#ffffff',
            correctLevel: QRCode.CorrectLevel.H
        });
    }
}

// 新規アカウント作成を処理
async function handleCreateAccount() {
    try {
        const btn = document.getElementById('create-account-btn');
        const originalText = btn.innerHTML;
        
        btn.disabled = true;
        btn.innerHTML = `
            <span class="spinner-border spinner-border-sm me-1" role="status" aria-hidden="true"></span>
            Creating...
        `;
        
        // 新規アカウントを作成
        const newAccount = await createNewAccount();
        
        // モーダルを表示
        showNewAccountModal(newAccount);
        
        // ボタンを元に戻す
        btn.disabled = false;
        btn.innerHTML = originalText;
        
        // アカウントリストを再読み込み
        loadAccountList();
        
    } catch (error) {
        console.error('Error creating account:', error);
        alert('Failed to create new account: ' + error.message);
    }
}

// 新規アカウントモーダルを表示
function showNewAccountModal(account) {
    // モーダル要素を作成
    const modalEl = document.createElement('div');
    modalEl.className = 'modal fade';
    modalEl.id = 'new-account-modal';
    modalEl.tabIndex = '-1';
    modalEl.setAttribute('aria-labelledby', 'new-account-modal-label');
    modalEl.setAttribute('aria-hidden', 'true');
    
    modalEl.innerHTML = `
        <div class="modal-dialog modal-dialog-centered">
            <div class="modal-content">
                <div class="modal-header">
                    <h5 class="modal-title" id="new-account-modal-label">New Account Created</h5>
                    <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                </div>
                <div class="modal-body">
                    <div class="alert alert-warning" role="alert">
                        <i class="bi bi-exclamation-triangle me-2"></i>
                        <strong>Important:</strong> Save your private key! It will only be shown once.
                    </div>
                    
                    <div class="mb-3">
                        <label class="form-label fw-bold">Address:</label>
                        <div class="input-group">
                            <input type="text" class="form-control font-monospace" value="${account.address}" readonly>
                            <button class="btn btn-outline-secondary copy-btn" type="button" data-copy="${account.address}">
                                <i class="bi bi-clipboard"></i>
                            </button>
                        </div>
                    </div>
                    
                    <div class="mb-3">
                        <label class="form-label fw-bold">Private Key:</label>
                        <div class="input-group">
                            <input type="text" class="form-control font-monospace" value="${account.privateKey}" readonly>
                            <button class="btn btn-outline-secondary copy-btn" type="button" data-copy="${account.privateKey}">
                                <i class="bi bi-clipboard"></i>
                            </button>
                        </div>
                    </div>
                </div>
                <div class="modal-footer">
                    <button type="button" class="btn btn-primary" data-bs-dismiss="modal">I've Saved My Private Key</button>
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
}

// アカウントインポートを処理
function handleImportAccount() {
    const privateKeyInput = document.getElementById('private-key');
    const privateKey = privateKeyInput.value.trim();
    
    if (!privateKey) {
        alert('Please enter a private key');
        return;
    }
    
    // 実際のAPIが実装されるまでダミー処理
    alert('Account import functionality will be implemented in a future update.');
    privateKeyInput.value = '';
}

// 秘密鍵エクスポートを処理
function handleExportPrivateKey(address) {
    // 実際のAPIが実装されるまでダミー処理
    alert('Private key export functionality will be implemented in a future update.');
}