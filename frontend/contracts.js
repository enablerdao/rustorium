// スマートコントラクト管理用のスクリプト
let currentContract = null;

// コントラクトリストを取得
async function fetchContracts() {
    try {
        // 実際のAPIが実装されるまでダミーデータを使用
        return [
            {
                address: '0x7c4d9c5e30e47f5c52f6c057b5c48e9a7e6c7c1e',
                name: 'Token Contract',
                creator: '0x1234567890abcdef1234567890abcdef12345678',
                created_at: 1677721600,
                vm_type: 'EVM',
                verified: true
            },
            {
                address: '0x3a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b',
                name: 'NFT Marketplace',
                creator: '0x1234567890abcdef1234567890abcdef12345678',
                created_at: 1677635200,
                vm_type: 'EVM',
                verified: true
            },
            {
                address: '0x9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d3e2f1a0b',
                name: 'Staking Contract',
                creator: '0xabcdef1234567890abcdef1234567890abcdef12',
                created_at: 1677548800,
                vm_type: 'WASM',
                verified: false
            }
        ];
    } catch (error) {
        console.error('Error fetching contracts:', error);
        return [];
    }
}

// コントラクト詳細を取得
async function fetchContractDetails(address) {
    try {
        // 実際のAPIが実装されるまでダミーデータを使用
        const contracts = await fetchContracts();
        const contract = contracts.find(c => c.address === address);
        
        if (!contract) {
            throw new Error('Contract not found');
        }
        
        // コントラクト詳細を追加
        return {
            ...contract,
            balance: 25000,
            transaction_count: 156,
            abi: [
                {
                    "inputs": [],
                    "name": "name",
                    "outputs": [{"type": "string"}],
                    "stateMutability": "view",
                    "type": "function"
                },
                {
                    "inputs": [],
                    "name": "symbol",
                    "outputs": [{"type": "string"}],
                    "stateMutability": "view",
                    "type": "function"
                },
                {
                    "inputs": [],
                    "name": "totalSupply",
                    "outputs": [{"type": "uint256"}],
                    "stateMutability": "view",
                    "type": "function"
                },
                {
                    "inputs": [{"name": "account", "type": "address"}],
                    "name": "balanceOf",
                    "outputs": [{"type": "uint256"}],
                    "stateMutability": "view",
                    "type": "function"
                },
                {
                    "inputs": [
                        {"name": "recipient", "type": "address"},
                        {"name": "amount", "type": "uint256"}
                    ],
                    "name": "transfer",
                    "outputs": [{"type": "bool"}],
                    "stateMutability": "nonpayable",
                    "type": "function"
                }
            ],
            source_code: contract.verified ? `// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Token {
    string public name = "Rustorium Token";
    string public symbol = "RLT";
    uint8 public decimals = 18;
    uint256 public totalSupply = 1000000 * 10**18;
    
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    
    constructor() {
        balanceOf[msg.sender] = totalSupply;
    }
    
    function transfer(address to, uint256 value) public returns (bool success) {
        require(balanceOf[msg.sender] >= value, "Insufficient balance");
        
        balanceOf[msg.sender] -= value;
        balanceOf[to] += value;
        
        emit Transfer(msg.sender, to, value);
        return true;
    }
    
    function approve(address spender, uint256 value) public returns (bool success) {
        allowance[msg.sender][spender] = value;
        emit Approval(msg.sender, spender, value);
        return true;
    }
    
    function transferFrom(address from, address to, uint256 value) public returns (bool success) {
        require(balanceOf[from] >= value, "Insufficient balance");
        require(allowance[from][msg.sender] >= value, "Insufficient allowance");
        
        balanceOf[from] -= value;
        balanceOf[to] += value;
        allowance[from][msg.sender] -= value;
        
        emit Transfer(from, to, value);
        return true;
    }
}` : 'Source code not verified'
        };
    } catch (error) {
        console.error('Error fetching contract details:', error);
        throw error;
    }
}

// コントラクトページを表示
function showContractsPage() {
    contentArea.innerHTML = `
        <h1 class="mb-4">Smart Contracts</h1>
        
        <div class="row mb-4">
            <div class="col-md-8">
                <div class="card">
                    <div class="card-header d-flex justify-content-between align-items-center">
                        <h5 class="mb-0">Deployed Contracts</h5>
                        <button class="btn btn-sm btn-primary" id="deploy-contract-btn">
                            <i class="bi bi-cloud-upload me-1"></i>
                            Deploy New Contract
                        </button>
                    </div>
                    <div class="card-body">
                        <div id="contract-list">
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
                        <h5 class="mb-0">Verify Contract</h5>
                    </div>
                    <div class="card-body">
                        <form id="verify-contract-form">
                            <div class="mb-3">
                                <label for="contract-address" class="form-label">Contract Address</label>
                                <input type="text" class="form-control" id="contract-address" placeholder="0x..." required>
                            </div>
                            <div class="mb-3">
                                <label for="contract-name" class="form-label">Contract Name</label>
                                <input type="text" class="form-control" id="contract-name" required>
                            </div>
                            <div class="mb-3">
                                <label for="compiler-version" class="form-label">Compiler Version</label>
                                <select class="form-select" id="compiler-version" required>
                                    <option value="0.8.17">Solidity 0.8.17</option>
                                    <option value="0.8.16">Solidity 0.8.16</option>
                                    <option value="0.8.15">Solidity 0.8.15</option>
                                    <option value="0.8.14">Solidity 0.8.14</option>
                                </select>
                            </div>
                            <div class="mb-3">
                                <label for="contract-code" class="form-label">Source Code</label>
                                <textarea class="form-control" id="contract-code" rows="5" required></textarea>
                            </div>
                            <button type="submit" class="btn btn-primary">
                                <i class="bi bi-check-circle me-1"></i>
                                Verify
                            </button>
                        </form>
                    </div>
                </div>
            </div>
        </div>
        
        <div id="contract-details"></div>
    `;

    // コントラクトリストを読み込む
    loadContractList();
    
    // デプロイボタンのイベントリスナーを追加
    document.getElementById('deploy-contract-btn').addEventListener('click', showDeployContractForm);
    
    // 検証フォームのイベントリスナーを追加
    document.getElementById('verify-contract-form').addEventListener('submit', function(e) {
        e.preventDefault();
        handleVerifyContract();
    });
}

// コントラクトリストを読み込む
async function loadContractList() {
    const contractListEl = document.getElementById('contract-list');
    
    try {
        const contracts = await fetchContracts();
        
        if (contracts.length === 0) {
            contractListEl.innerHTML = `
                <div class="alert alert-info" role="alert">
                    <i class="bi bi-info-circle me-2"></i>
                    No contracts found. Deploy a new contract to get started.
                </div>
            `;
            return;
        }
        
        let html = '<div class="list-group">';
        
        for (const contract of contracts) {
            html += `
                <a href="#" class="list-group-item list-group-item-action contract-item" data-address="${contract.address}">
                    <div class="d-flex justify-content-between align-items-center">
                        <div>
                            <div class="fw-bold">${contract.name}</div>
                            <div class="font-monospace small">${formatAddress(contract.address)}</div>
                            <div class="small text-muted mt-1">
                                <span class="me-2">Created: ${formatTimestamp(contract.created_at)}</span>
                                <span class="me-2">VM: ${contract.vm_type}</span>
                                ${contract.verified ? 
                                    '<span class="badge bg-success"><i class="bi bi-check-circle me-1"></i>Verified</span>' : 
                                    '<span class="badge bg-secondary"><i class="bi bi-x-circle me-1"></i>Unverified</span>'
                                }
                            </div>
                        </div>
                        <div>
                            <span class="badge bg-primary rounded-pill">
                                <i class="bi bi-box-arrow-in-right me-1"></i>
                                Interact
                            </span>
                        </div>
                    </div>
                </a>
            `;
        }
        
        html += '</div>';
        contractListEl.innerHTML = html;
        
        // コントラクト項目のイベントリスナーを追加
        document.querySelectorAll('.contract-item').forEach(item => {
            item.addEventListener('click', function(e) {
                e.preventDefault();
                const address = this.getAttribute('data-address');
                showContractDetails(address);
            });
        });
        
    } catch (error) {
        console.error('Error loading contract list:', error);
        contractListEl.innerHTML = `
            <div class="alert alert-danger" role="alert">
                <i class="bi bi-exclamation-triangle me-2"></i>
                Failed to load contracts: ${error.message}
            </div>
        `;
    }
}

// コントラクト詳細を表示
async function showContractDetails(address) {
    currentContract = address;
    const contractDetailsEl = document.getElementById('contract-details');
    
    contractDetailsEl.innerHTML = `
        <div class="d-flex justify-content-center my-5">
            <div class="spinner-border text-primary" role="status">
                <span class="visually-hidden">Loading...</span>
            </div>
        </div>
    `;
    
    try {
        const contract = await fetchContractDetails(address);
        
        contractDetailsEl.innerHTML = `
            <div class="card mb-4">
                <div class="card-header">
                    <ul class="nav nav-tabs card-header-tabs" id="contract-tabs" role="tablist">
                        <li class="nav-item" role="presentation">
                            <button class="nav-link active" id="info-tab" data-bs-toggle="tab" data-bs-target="#info-tab-pane" type="button" role="tab" aria-controls="info-tab-pane" aria-selected="true">
                                Contract Info
                            </button>
                        </li>
                        <li class="nav-item" role="presentation">
                            <button class="nav-link" id="code-tab" data-bs-toggle="tab" data-bs-target="#code-tab-pane" type="button" role="tab" aria-controls="code-tab-pane" aria-selected="false">
                                Source Code
                            </button>
                        </li>
                        <li class="nav-item" role="presentation">
                            <button class="nav-link" id="interact-tab" data-bs-toggle="tab" data-bs-target="#interact-tab-pane" type="button" role="tab" aria-controls="interact-tab-pane" aria-selected="false">
                                Interact
                            </button>
                        </li>
                    </ul>
                </div>
                <div class="card-body">
                    <div class="tab-content" id="contract-tab-content">
                        <div class="tab-pane fade show active" id="info-tab-pane" role="tabpanel" aria-labelledby="info-tab" tabindex="0">
                            <div class="row mb-3">
                                <div class="col-md-3 fw-bold">Name:</div>
                                <div class="col-md-9">${contract.name}</div>
                            </div>
                            <div class="row mb-3">
                                <div class="col-md-3 fw-bold">Address:</div>
                                <div class="col-md-9 font-monospace">${contract.address}</div>
                            </div>
                            <div class="row mb-3">
                                <div class="col-md-3 fw-bold">Creator:</div>
                                <div class="col-md-9 font-monospace">${contract.creator}</div>
                            </div>
                            <div class="row mb-3">
                                <div class="col-md-3 fw-bold">Created:</div>
                                <div class="col-md-9">${formatTimestamp(contract.created_at)}</div>
                            </div>
                            <div class="row mb-3">
                                <div class="col-md-3 fw-bold">VM Type:</div>
                                <div class="col-md-9">${contract.vm_type}</div>
                            </div>
                            <div class="row mb-3">
                                <div class="col-md-3 fw-bold">Balance:</div>
                                <div class="col-md-9">${formatAmount(contract.balance)}</div>
                            </div>
                            <div class="row mb-3">
                                <div class="col-md-3 fw-bold">Transactions:</div>
                                <div class="col-md-9">${contract.transaction_count}</div>
                            </div>
                            <div class="row">
                                <div class="col-md-3 fw-bold">Verified:</div>
                                <div class="col-md-9">
                                    ${contract.verified ? 
                                        '<span class="badge bg-success"><i class="bi bi-check-circle me-1"></i>Yes</span>' : 
                                        '<span class="badge bg-secondary"><i class="bi bi-x-circle me-1"></i>No</span>'
                                    }
                                </div>
                            </div>
                        </div>
                        <div class="tab-pane fade" id="code-tab-pane" role="tabpanel" aria-labelledby="code-tab" tabindex="0">
                            ${contract.verified ? `
                                <pre class="bg-light p-3 rounded"><code class="language-solidity">${contract.source_code}</code></pre>
                            ` : `
                                <div class="alert alert-warning" role="alert">
                                    <i class="bi bi-exclamation-triangle me-2"></i>
                                    This contract is not verified. Please verify the contract to view the source code.
                                </div>
                                <button class="btn btn-primary" id="verify-this-contract-btn">
                                    <i class="bi bi-check-circle me-1"></i>
                                    Verify This Contract
                                </button>
                            `}
                        </div>
                        <div class="tab-pane fade" id="interact-tab-pane" role="tabpanel" aria-labelledby="interact-tab" tabindex="0">
                            <div class="mb-4">
                                <h5>Read Contract</h5>
                                <div class="list-group" id="read-functions">
                                    ${generateFunctionList(contract.abi, 'read')}
                                </div>
                            </div>
                            
                            <div>
                                <h5>Write Contract</h5>
                                <div class="list-group" id="write-functions">
                                    ${generateFunctionList(contract.abi, 'write')}
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        `;
        
        // シンタックスハイライトを適用
        applySyntaxHighlighting();
        
        // 関数呼び出しフォームのイベントリスナーを追加
        setupFunctionCallListeners();
        
        // 検証ボタンのイベントリスナーを追加
        const verifyBtn = document.getElementById('verify-this-contract-btn');
        if (verifyBtn) {
            verifyBtn.addEventListener('click', function() {
                document.getElementById('contract-address').value = address;
                document.getElementById('contract-tabs').querySelector('a[href="#verify-tab-pane"]').click();
            });
        }
        
    } catch (error) {
        console.error('Error showing contract details:', error);
        contractDetailsEl.innerHTML = `
            <div class="alert alert-danger" role="alert">
                <i class="bi bi-exclamation-triangle me-2"></i>
                Failed to load contract details: ${error.message}
            </div>
        `;
    }
}

// ABIから関数リストを生成
function generateFunctionList(abi, type) {
    if (!abi || !Array.isArray(abi)) {
        return '<div class="alert alert-warning">No ABI available</div>';
    }
    
    const functions = abi.filter(item => {
        if (item.type !== 'function') return false;
        return type === 'read' ? 
            (item.stateMutability === 'view' || item.stateMutability === 'pure') : 
            (item.stateMutability !== 'view' && item.stateMutability !== 'pure');
    });
    
    if (functions.length === 0) {
        return `<div class="alert alert-info">No ${type} functions found</div>`;
    }
    
    let html = '';
    
    for (const func of functions) {
        const functionId = `function-${func.name}`;
        
        html += `
            <div class="list-group-item">
                <div class="d-flex justify-content-between align-items-center mb-2">
                    <h6 class="mb-0">${func.name}</h6>
                    <span class="badge ${type === 'read' ? 'bg-info' : 'bg-warning'}">
                        ${func.stateMutability}
                    </span>
                </div>
                
                <form class="function-call-form" data-function-name="${func.name}" data-function-type="${type}">
                    ${func.inputs && func.inputs.length > 0 ? `
                        <div class="mb-3">
                            ${func.inputs.map((input, index) => `
                                <div class="mb-2">
                                    <label class="form-label small">${input.name || `param${index}`} (${input.type})</label>
                                    <input type="text" class="form-control form-control-sm" name="${input.name || `param${index}`}" placeholder="${input.type}">
                                </div>
                            `).join('')}
                        </div>
                    ` : ''}
                    
                    <div class="d-flex justify-content-between align-items-center">
                        <button type="submit" class="btn btn-sm ${type === 'read' ? 'btn-info' : 'btn-warning'}">
                            ${type === 'read' ? 'Query' : 'Execute'}
                        </button>
                        
                        <div class="function-result" id="${functionId}-result"></div>
                    </div>
                </form>
            </div>
        `;
    }
    
    return html;
}

// 関数呼び出しフォームのイベントリスナーを設定
function setupFunctionCallListeners() {
    document.querySelectorAll('.function-call-form').forEach(form => {
        form.addEventListener('submit', function(e) {
            e.preventDefault();
            
            const functionName = this.getAttribute('data-function-name');
            const functionType = this.getAttribute('data-function-type');
            const resultEl = document.getElementById(`function-${functionName}-result`);
            
            // フォームデータを収集
            const formData = new FormData(this);
            const params = {};
            for (const [key, value] of formData.entries()) {
                params[key] = value;
            }
            
            // 関数呼び出しを処理
            handleFunctionCall(functionName, functionType, params, resultEl);
        });
    });
}

// 関数呼び出しを処理
async function handleFunctionCall(functionName, functionType, params, resultEl) {
    try {
        // 実際のAPIが実装されるまでダミー処理
        resultEl.innerHTML = `
            <div class="spinner-border spinner-border-sm text-primary" role="status">
                <span class="visually-hidden">Loading...</span>
            </div>
        `;
        
        // 処理をシミュレート
        await new Promise(resolve => setTimeout(resolve, 1000));
        
        if (functionType === 'read') {
            // 読み取り関数の結果をシミュレート
            let result;
            
            switch (functionName) {
                case 'name':
                    result = 'Rustorium Token';
                    break;
                case 'symbol':
                    result = 'RLT';
                    break;
                case 'totalSupply':
                    result = '1000000000000000000000000';
                    break;
                case 'balanceOf':
                    result = '500000000000000000000';
                    break;
                default:
                    result = 'Success';
            }
            
            resultEl.innerHTML = `<span class="badge bg-success">${result}</span>`;
        } else {
            // 書き込み関数の結果をシミュレート
            resultEl.innerHTML = `<span class="badge bg-success">Success</span>`;
        }
    } catch (error) {
        console.error(`Error calling function ${functionName}:`, error);
        resultEl.innerHTML = `<span class="badge bg-danger">Error: ${error.message}</span>`;
    }
}

// シンタックスハイライトを適用
function applySyntaxHighlighting() {
    // highlight.jsライブラリが読み込まれているか確認
    if (typeof hljs === 'undefined') {
        // highlight.jsを動的に読み込む
        const link = document.createElement('link');
        link.rel = 'stylesheet';
        link.href = 'https://cdn.jsdelivr.net/npm/highlight.js@11.7.0/styles/github.min.css';
        document.head.appendChild(link);
        
        const script = document.createElement('script');
        script.src = 'https://cdn.jsdelivr.net/npm/highlight.js@11.7.0/highlight.min.js';
        script.onload = () => {
            // Solidityの言語定義を読み込む
            const solidityScript = document.createElement('script');
            solidityScript.src = 'https://cdn.jsdelivr.net/npm/highlightjs-solidity@2.0.5/dist/solidity.min.js';
            solidityScript.onload = () => {
                hljs.registerLanguage('solidity', window.hljsDefineSolidity);
                hljs.highlightAll();
            };
            document.head.appendChild(solidityScript);
        };
        document.head.appendChild(script);
    } else {
        // すでに読み込まれている場合は直接実行
        hljs.highlightAll();
    }
}

// コントラクトデプロイフォームを表示
function showDeployContractForm() {
    const contractDetailsEl = document.getElementById('contract-details');
    
    contractDetailsEl.innerHTML = `
        <div class="card">
            <div class="card-header">
                <h5 class="mb-0">Deploy New Contract</h5>
            </div>
            <div class="card-body">
                <form id="deploy-contract-form">
                    <div class="mb-3">
                        <label for="contract-name-input" class="form-label">Contract Name</label>
                        <input type="text" class="form-control" id="contract-name-input" required>
                    </div>
                    
                    <div class="mb-3">
                        <label for="vm-type-select" class="form-label">VM Type</label>
                        <select class="form-select" id="vm-type-select" required>
                            <option value="EVM">EVM (Ethereum Virtual Machine)</option>
                            <option value="WASM">WASM (WebAssembly)</option>
                        </select>
                    </div>
                    
                    <div class="mb-3">
                        <label for="compiler-version-select" class="form-label">Compiler Version</label>
                        <select class="form-select" id="compiler-version-select" required>
                            <option value="0.8.17">Solidity 0.8.17</option>
                            <option value="0.8.16">Solidity 0.8.16</option>
                            <option value="0.8.15">Solidity 0.8.15</option>
                            <option value="0.8.14">Solidity 0.8.14</option>
                        </select>
                    </div>
                    
                    <div class="mb-3">
                        <label for="contract-source" class="form-label">Source Code</label>
                        <textarea class="form-control font-monospace" id="contract-source" rows="15" required></textarea>
                    </div>
                    
                    <div class="mb-3">
                        <label for="constructor-args" class="form-label">Constructor Arguments (optional)</label>
                        <textarea class="form-control font-monospace" id="constructor-args" rows="3" placeholder="[arg1, arg2, ...]"></textarea>
                        <div class="form-text">Enter constructor arguments as a JSON array</div>
                    </div>
                    
                    <div class="mb-3">
                        <label for="deploy-from" class="form-label">Deploy From</label>
                        <select class="form-select" id="deploy-from" required>
                            <option value="0x1234567890abcdef1234567890abcdef12345678">0x1234...5678</option>
                            <option value="0xabcdef1234567890abcdef1234567890abcdef12">0xabcd...ef12</option>
                        </select>
                    </div>
                    
                    <div class="d-flex justify-content-between">
                        <button type="button" class="btn btn-secondary" id="cancel-deploy-btn">
                            <i class="bi bi-x-circle me-1"></i>
                            Cancel
                        </button>
                        <button type="submit" class="btn btn-primary">
                            <i class="bi bi-cloud-upload me-1"></i>
                            Deploy Contract
                        </button>
                    </div>
                </form>
            </div>
        </div>
    `;
    
    // サンプルコードを設定
    document.getElementById('contract-source').value = `// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract SimpleStorage {
    uint256 private value;
    
    event ValueChanged(uint256 newValue);
    
    function setValue(uint256 newValue) public {
        value = newValue;
        emit ValueChanged(newValue);
    }
    
    function getValue() public view returns (uint256) {
        return value;
    }
}`;
    
    // キャンセルボタンのイベントリスナーを追加
    document.getElementById('cancel-deploy-btn').addEventListener('click', function() {
        contractDetailsEl.innerHTML = '';
    });
    
    // デプロイフォームのイベントリスナーを追加
    document.getElementById('deploy-contract-form').addEventListener('submit', function(e) {
        e.preventDefault();
        handleDeployContract();
    });
}

// コントラクトデプロイを処理
async function handleDeployContract() {
    try {
        const nameInput = document.getElementById('contract-name-input');
        const vmTypeSelect = document.getElementById('vm-type-select');
        const compilerVersionSelect = document.getElementById('compiler-version-select');
        const sourceInput = document.getElementById('contract-source');
        const argsInput = document.getElementById('constructor-args');
        const deployFromSelect = document.getElementById('deploy-from');
        
        const name = nameInput.value.trim();
        const vmType = vmTypeSelect.value;
        const compilerVersion = compilerVersionSelect.value;
        const source = sourceInput.value.trim();
        const args = argsInput.value.trim();
        const deployFrom = deployFromSelect.value;
        
        if (!name || !source) {
            throw new Error('Please fill in all required fields');
        }
        
        // デプロイボタンを無効化
        const submitBtn = document.querySelector('#deploy-contract-form button[type="submit"]');
        const originalBtnText = submitBtn.innerHTML;
        submitBtn.disabled = true;
        submitBtn.innerHTML = `
            <span class="spinner-border spinner-border-sm me-1" role="status" aria-hidden="true"></span>
            Deploying...
        `;
        
        // 実際のAPIが実装されるまでタイマーでシミュレート
        await new Promise(resolve => setTimeout(resolve, 2000));
        
        // 成功メッセージを表示
        const contractDetailsEl = document.getElementById('contract-details');
        contractDetailsEl.innerHTML = `
            <div class="alert alert-success" role="alert">
                <i class="bi bi-check-circle me-2"></i>
                <strong>Success!</strong> Contract deployed successfully.
            </div>
            
            <div class="card">
                <div class="card-header">
                    <h5 class="mb-0">Deployment Details</h5>
                </div>
                <div class="card-body">
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Contract Name:</div>
                        <div class="col-md-9">${name}</div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Contract Address:</div>
                        <div class="col-md-9 font-monospace">0x${Array.from({length: 40}, () => 
                            Math.floor(Math.random() * 16).toString(16)
                        ).join('')}</div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">VM Type:</div>
                        <div class="col-md-9">${vmType}</div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Compiler:</div>
                        <div class="col-md-9">Solidity ${compilerVersion}</div>
                    </div>
                    <div class="row mb-3">
                        <div class="col-md-3 fw-bold">Deployed By:</div>
                        <div class="col-md-9 font-monospace">${deployFrom}</div>
                    </div>
                    <div class="row">
                        <div class="col-md-3 fw-bold">Status:</div>
                        <div class="col-md-9">
                            <span class="badge bg-success">Success</span>
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="d-grid gap-2 d-md-flex justify-content-md-end mt-3">
                <button class="btn btn-primary" id="view-contracts-btn">
                    <i class="bi bi-list me-1"></i>
                    View All Contracts
                </button>
            </div>
        `;
        
        // 「すべてのコントラクトを表示」ボタンのイベントリスナーを追加
        document.getElementById('view-contracts-btn').addEventListener('click', function() {
            showContractsPage();
        });
        
    } catch (error) {
        console.error('Error deploying contract:', error);
        alert('Failed to deploy contract: ' + error.message);
    }
}

// コントラクト検証を処理
function handleVerifyContract() {
    try {
        const addressInput = document.getElementById('contract-address');
        const nameInput = document.getElementById('contract-name');
        const compilerVersionSelect = document.getElementById('compiler-version');
        const codeInput = document.getElementById('contract-code');
        
        const address = addressInput.value.trim();
        const name = nameInput.value.trim();
        const compilerVersion = compilerVersionSelect.value;
        const code = codeInput.value.trim();
        
        if (!address || !name || !code) {
            throw new Error('Please fill in all required fields');
        }
        
        // 検証ボタンを無効化
        const submitBtn = document.querySelector('#verify-contract-form button[type="submit"]');
        const originalBtnText = submitBtn.innerHTML;
        submitBtn.disabled = true;
        submitBtn.innerHTML = `
            <span class="spinner-border spinner-border-sm me-1" role="status" aria-hidden="true"></span>
            Verifying...
        `;
        
        // 実際のAPIが実装されるまでタイマーでシミュレート
        setTimeout(() => {
            // ボタンを元に戻す
            submitBtn.disabled = false;
            submitBtn.innerHTML = originalBtnText;
            
            // 成功メッセージを表示
            alert('Contract verification successful!');
            
            // フォームをリセット
            addressInput.value = '';
            nameInput.value = '';
            codeInput.value = '';
            
            // コントラクトリストを再読み込み
            loadContractList();
        }, 2000);
        
    } catch (error) {
        console.error('Error verifying contract:', error);
        alert('Failed to verify contract: ' + error.message);
    }
}