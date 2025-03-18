// ネットワーク関連のユーティリティ関数

// ネットワーク情報の表示を更新
function updateNetworkDisplay() {
    const networkInfo = apiClient.getCurrentNetworkInfo();
    const networkDropdown = document.getElementById('networkDropdown');
    
    if (networkDropdown) {
        // ドロップダウンボタンのテキストとアイコンを更新
        networkDropdown.innerHTML = `
            <i class="bi ${networkInfo.icon} me-1"></i> ${networkInfo.name}
        `;
        
        // ネットワークに応じてボタンの色を変更
        networkDropdown.classList.remove('btn-outline-secondary', 'btn-outline-danger', 'btn-outline-primary', 'btn-outline-success');
        
        if (apiClient.network === 'mainnet') {
            networkDropdown.classList.add('btn-outline-danger');
            // メインネットの場合は警告バッジを表示
            const badge = document.createElement('span');
            badge.className = 'position-absolute top-0 start-100 translate-middle p-1 bg-danger border border-light rounded-circle';
            badge.setAttribute('title', 'メインネット接続中');
            networkDropdown.parentNode.appendChild(badge);
        } else if (apiClient.network === 'testnet') {
            networkDropdown.classList.add('btn-outline-primary');
        } else {
            networkDropdown.classList.add('btn-outline-success');
        }
    }
    
    // ネットワーク名をページタイトルに追加
    const pageTitle = document.getElementById('page-title');
    if (pageTitle) {
        const currentTitle = pageTitle.textContent.split(' - ')[0];
        pageTitle.textContent = `${currentTitle} - ${networkInfo.name}`;
    }
    
    // ネットワークバッジをヘッダーに追加
    const headerRight = document.querySelector('.header .d-flex.align-items-center');
    if (headerRight) {
        const existingBadge = document.getElementById('network-badge');
        if (!existingBadge) {
            const badge = document.createElement('span');
            badge.id = 'network-badge';
            badge.className = `badge bg-${networkInfo.badge} me-3`;
            badge.innerHTML = `<i class="bi ${networkInfo.icon} me-1"></i> ${networkInfo.name}`;
            headerRight.insertBefore(badge, headerRight.firstChild);
        } else {
            existingBadge.className = `badge bg-${networkInfo.badge} me-3`;
            existingBadge.innerHTML = `<i class="bi ${networkInfo.icon} me-1"></i> ${networkInfo.name}`;
        }
    }
}

// ネットワーク切り替えドロップダウンの設定
function setupNetworkDropdown() {
    const networkDropdownMenu = document.querySelector('#networkDropdown + .dropdown-menu');
    
    if (networkDropdownMenu) {
        // ドロップダウンメニューの内容を更新
        networkDropdownMenu.innerHTML = '';
        
        // 各ネットワークのメニュー項目を追加
        Object.keys(NETWORKS).forEach(network => {
            const networkInfo = NETWORKS[network];
            const item = document.createElement('li');
            const link = document.createElement('a');
            link.className = 'dropdown-item' + (network === apiClient.network ? ' active' : '');
            link.href = '#';
            link.innerHTML = `
                <i class="bi ${networkInfo.icon} me-2"></i>
                ${networkInfo.name}
                ${network === apiClient.network ? '<i class="bi bi-check me-2"></i>' : ''}
            `;
            
            // ネットワーク切り替えイベントを設定
            link.addEventListener('click', function(e) {
                e.preventDefault();
                if (network === 'mainnet') {
                    // メインネットに切り替える前に確認
                    if (confirm('メインネットに接続しますか？実際の資産が影響を受ける可能性があります。')) {
                        switchNetwork(network);
                    }
                } else {
                    switchNetwork(network);
                }
            });
            
            item.appendChild(link);
            networkDropdownMenu.appendChild(item);
        });
        
        // 区切り線を追加
        const divider = document.createElement('li');
        divider.innerHTML = '<hr class="dropdown-divider">';
        networkDropdownMenu.appendChild(divider);
        
        // カスタムネットワーク追加オプション
        const customItem = document.createElement('li');
        const customLink = document.createElement('a');
        customLink.className = 'dropdown-item';
        customLink.href = '#';
        customLink.innerHTML = '<i class="bi bi-plus me-2"></i>Add Custom Network';
        customLink.addEventListener('click', function(e) {
            e.preventDefault();
            alert('カスタムネットワーク機能は開発中です');
        });
        customItem.appendChild(customLink);
        networkDropdownMenu.appendChild(customItem);
    }
}

// ネットワークを切り替え
function switchNetwork(network) {
    if (apiClient.switchNetwork(network)) {
        updateNetworkDisplay();
        setupNetworkDropdown();
        
        // 現在のページを再読み込み
        const hash = window.location.hash || '#';
        window.location.hash = '';
        setTimeout(() => {
            window.location.hash = hash;
        }, 100);
        
        return true;
    }
    return false;
}

// 確認ダイアログ付きのネットワーク切り替え
function switchNetworkWithConfirmation(network) {
    if (network === apiClient.network) {
        alert(`Already connected to ${NETWORKS[network].name}`);
        return;
    }
    
    let confirmMessage = `Switch to ${NETWORKS[network].name}?`;
    
    if (network === 'mainnet') {
        confirmMessage = 'WARNING: You are about to connect to the Mainnet. All transactions will affect real assets. Continue?';
    }
    
    if (confirm(confirmMessage)) {
        if (switchNetwork(network)) {
            // 成功メッセージを表示
            alert(`Successfully connected to ${NETWORKS[network].name}`);
        } else {
            alert(`Failed to connect to ${NETWORKS[network].name}`);
        }
    }
}