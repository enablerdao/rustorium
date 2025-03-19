// Rustorium テーマ管理とUI拡張

// テーマ設定
// ユーザーの環境設定を確認
function getPreferredTheme() {
  // ローカルストレージに保存されている場合はそれを使用
  const savedTheme = localStorage.getItem('theme');
  if (savedTheme) {
    return savedTheme;
  }
  
  // ユーザーのシステム設定を確認
  if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
    return 'dark';
  }
  
  // デフォルトはライトモード
  return 'light';
}

let currentTheme = getPreferredTheme();

// テーマ切り替え関数
function toggleTheme() {
  currentTheme = currentTheme === 'light' ? 'dark' : 'light';
  applyTheme();
  localStorage.setItem('theme', currentTheme);
}

// テーマ適用関数
function applyTheme() {
  document.documentElement.setAttribute('data-theme', currentTheme);
  
  // テーマアイコンの切り替え
  const themeIcon = document.getElementById('theme-toggle-icon');
  if (themeIcon) {
    themeIcon.className = currentTheme === 'light' ? 'bi bi-moon-fill' : 'bi bi-sun-fill';
  }
  
  // テーマ切り替えボタンのツールチップを更新
  const themeToggle = document.getElementById('theme-toggle');
  if (themeToggle) {
    themeToggle.setAttribute('title', currentTheme === 'light' ? 'ダークモードに切り替え' : 'ライトモードに切り替え');
    
    // Bootstrapのツールチップが初期化されている場合は更新
    if (typeof bootstrap !== 'undefined' && bootstrap.Tooltip) {
      const tooltip = bootstrap.Tooltip.getInstance(themeToggle);
      if (tooltip) {
        tooltip.dispose();
      }
      new bootstrap.Tooltip(themeToggle);
    }
  }
}

// サイドバー折りたたみ状態
let sidebarCollapsed = localStorage.getItem('sidebar-collapsed') === 'true';

// サイドバー折りたたみ切り替え関数
function toggleSidebar() {
  sidebarCollapsed = !sidebarCollapsed;
  applySidebarState();
  localStorage.setItem('sidebar-collapsed', sidebarCollapsed);
}

// サイドバー状態適用関数
function applySidebarState() {
  const sidebar = document.querySelector('.sidebar');
  const mainContent = document.querySelector('.main-content');
  const toggleIcon = document.getElementById('sidebar-toggle-icon');
  
  if (sidebar && mainContent) {
    if (sidebarCollapsed) {
      sidebar.classList.add('sidebar-collapsed');
      mainContent.classList.add('main-content-expanded');
      if (toggleIcon) toggleIcon.className = 'bi bi-chevron-right';
    } else {
      sidebar.classList.remove('sidebar-collapsed');
      mainContent.classList.remove('main-content-expanded');
      if (toggleIcon) toggleIcon.className = 'bi bi-chevron-left';
    }
  }
}

// ページ読み込み時の初期化
document.addEventListener('DOMContentLoaded', function() {
  // 拡張UIの追加
  enhanceUI();
  
  // テーマの適用
  applyTheme();
  
  // サイドバー状態の適用
  applySidebarState();
  
  // システムのカラースキーム変更を監視
  if (window.matchMedia) {
    const colorSchemeQuery = window.matchMedia('(prefers-color-scheme: dark)');
    
    // 変更イベントのリスナーを追加
    try {
      // Chrome & Firefox
      colorSchemeQuery.addEventListener('change', (e) => {
        // ユーザーが手動で設定していない場合のみ自動的に変更
        if (!localStorage.getItem('theme')) {
          currentTheme = e.matches ? 'dark' : 'light';
          applyTheme();
        }
      });
    } catch (e1) {
      try {
        // Safari
        colorSchemeQuery.addListener((e) => {
          // ユーザーが手動で設定していない場合のみ自動的に変更
          if (!localStorage.getItem('theme')) {
            currentTheme = e.matches ? 'dark' : 'light';
            applyTheme();
          }
        });
      } catch (e2) {
        console.error('Could not add color scheme change listener:', e2);
      }
    }
  }
});

// UI拡張関数
function enhanceUI() {
  // ヘッダーの追加
  addHeader();
  
  // サイドバーの拡張
  enhanceSidebar();
  
  // フォントの読み込み
  loadFonts();
  
  // ツールチップの初期化
  initTooltips();
  
  // アニメーションの追加
  addAnimations();
}

// ヘッダーの追加
function addHeader() {
  const mainContent = document.querySelector('.main-content');
  if (!mainContent) return;
  
  const header = document.createElement('header');
  header.className = 'header mb-4';
  header.innerHTML = `
    <div class="d-flex justify-content-between align-items-center">
      <div class="d-flex align-items-center">
        <button id="sidebar-toggle" class="btn btn-icon me-3" aria-label="Toggle Sidebar">
          <i id="sidebar-toggle-icon" class="bi bi-chevron-left"></i>
        </button>
        <h1 id="page-title" class="mb-0">Dashboard</h1>
      </div>
      <div class="d-flex align-items-center">
        <div class="dropdown me-3">
          <button class="btn btn-outline-secondary dropdown-toggle" type="button" id="networkDropdown" data-bs-toggle="dropdown" aria-expanded="false">
            <i class="bi bi-globe me-1"></i> Testnet
          </button>
          <ul class="dropdown-menu" aria-labelledby="networkDropdown">
            <li><a class="dropdown-item active" href="#"><i class="bi bi-check me-2"></i>Testnet</a></li>
            <li><a class="dropdown-item" href="#">Mainnet</a></li>
            <li><a class="dropdown-item" href="#">Local Network</a></li>
            <li><hr class="dropdown-divider"></li>
            <li><a class="dropdown-item" href="#"><i class="bi bi-plus me-2"></i>Add Custom Network</a></li>
          </ul>
        </div>
        <div class="dropdown me-3">
          <button class="btn btn-outline-secondary dropdown-toggle" type="button" id="notificationsDropdown" data-bs-toggle="dropdown" aria-expanded="false">
            <i class="bi bi-bell"></i>
            <span class="position-absolute top-0 start-100 translate-middle badge rounded-pill bg-danger">
              3
            </span>
          </button>
          <div class="dropdown-menu dropdown-menu-end" aria-labelledby="notificationsDropdown" style="width: 320px;">
            <div class="d-flex justify-content-between align-items-center px-3 py-2 border-bottom">
              <h6 class="mb-0">Notifications</h6>
              <a href="#" class="text-decoration-none small">Mark all as read</a>
            </div>
            <div class="notification-item p-3 border-bottom">
              <div class="d-flex">
                <div class="me-3">
                  <i class="bi bi-box text-primary fs-4"></i>
                </div>
                <div>
                  <p class="mb-1">New block #10245 has been mined</p>
                  <p class="text-muted small mb-0">2 minutes ago</p>
                </div>
              </div>
            </div>
            <div class="notification-item p-3 border-bottom">
              <div class="d-flex">
                <div class="me-3">
                  <i class="bi bi-arrow-left-right text-success fs-4"></i>
                </div>
                <div>
                  <p class="mb-1">Your transaction has been confirmed</p>
                  <p class="text-muted small mb-0">15 minutes ago</p>
                </div>
              </div>
            </div>
            <div class="notification-item p-3">
              <div class="d-flex">
                <div class="me-3">
                  <i class="bi bi-cpu text-danger fs-4"></i>
                </div>
                <div>
                  <p class="mb-1">Anomalous transaction detected</p>
                  <p class="text-muted small mb-0">1 hour ago</p>
                </div>
              </div>
            </div>
            <div class="text-center p-2 border-top">
              <a href="#" class="text-decoration-none">View all notifications</a>
            </div>
          </div>
        </div>
        <button id="theme-toggle" class="btn btn-icon me-3" aria-label="Toggle Theme">
          <i id="theme-toggle-icon" class="bi bi-moon-fill"></i>
        </button>
        <div class="dropdown">
          <button class="btn btn-primary dropdown-toggle" type="button" id="userDropdown" data-bs-toggle="dropdown" aria-expanded="false">
            <i class="bi bi-person-circle me-1"></i> Admin
          </button>
          <ul class="dropdown-menu dropdown-menu-end" aria-labelledby="userDropdown">
            <li><a class="dropdown-item" href="#"><i class="bi bi-gear me-2"></i>Settings</a></li>
            <li><a class="dropdown-item" href="#"><i class="bi bi-shield-lock me-2"></i>Security</a></li>
            <li><hr class="dropdown-divider"></li>
            <li><a class="dropdown-item" href="#"><i class="bi bi-box-arrow-right me-2"></i>Logout</a></li>
          </ul>
        </div>
      </div>
    </div>
  `;
  
  mainContent.prepend(header);
  
  // サイドバー切り替えボタンのイベントリスナー
  const sidebarToggle = document.getElementById('sidebar-toggle');
  if (sidebarToggle) {
    sidebarToggle.addEventListener('click', toggleSidebar);
  }
  
  // テーマ切り替えボタンのイベントリスナー
  const themeToggle = document.getElementById('theme-toggle');
  if (themeToggle) {
    themeToggle.addEventListener('click', function() {
      toggleTheme();
      // ユーザーが手動で設定したことを記録
      localStorage.setItem('theme', currentTheme);
    });
    
    // ツールチップを初期化
    themeToggle.setAttribute('title', currentTheme === 'light' ? 'ダークモードに切り替え' : 'ライトモードに切り替え');
    themeToggle.setAttribute('data-bs-toggle', 'tooltip');
    themeToggle.setAttribute('data-bs-placement', 'bottom');
  }
}

// サイドバーの拡張
function enhanceSidebar() {
  const sidebar = document.querySelector('.sidebar');
  if (!sidebar) return;
  
  // サイドバーブランド部分の更新
  const sidebarBrand = sidebar.querySelector('.d-flex.align-items-center.mb-3');
  if (sidebarBrand) {
    sidebarBrand.className = 'sidebar-brand';
    sidebarBrand.innerHTML = `
      <img src="logo.svg" alt="Rustorium Logo" class="sidebar-brand-icon">
      <span class="sidebar-brand-text">Rustorium</span>
    `;
  }
  
  // サイドバーフッターの追加
  const sidebarFooter = document.createElement('div');
  sidebarFooter.className = 'sidebar-footer';
  sidebarFooter.innerHTML = `
    <div class="d-flex justify-content-between align-items-center">
      <div>
        <span class="badge bg-primary">v0.1.0</span>
      </div>
      <div>
        <a href="#" class="text-white-50 me-2" title="Documentation"><i class="bi bi-book"></i></a>
        <a href="#" class="text-white-50 me-2" title="GitHub"><i class="bi bi-github"></i></a>
        <a href="#" class="text-white-50" title="Support"><i class="bi bi-question-circle"></i></a>
      </div>
    </div>
  `;
  
  sidebar.appendChild(sidebarFooter);
}

// フォントの読み込み
function loadFonts() {
  // Google Fontsからフォントを読み込む
  const fontLink = document.createElement('link');
  fontLink.rel = 'stylesheet';
  fontLink.href = 'https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap';
  document.head.appendChild(fontLink);
}

// ツールチップの初期化
function initTooltips() {
  // Bootstrap 5のツールチップを初期化
  if (typeof bootstrap !== 'undefined' && bootstrap.Tooltip) {
    const tooltipTriggerList = [].slice.call(document.querySelectorAll('[data-bs-toggle="tooltip"]'));
    tooltipTriggerList.map(function (tooltipTriggerEl) {
      return new bootstrap.Tooltip(tooltipTriggerEl);
    });
  }
}

// アニメーションの追加
function addAnimations() {
  // カードにアニメーションクラスを追加
  const cards = document.querySelectorAll('.card');
  cards.forEach((card, index) => {
    card.classList.add('fade-in');
    card.style.animationDelay = `${index * 0.1}s`;
  });
  
  // 統計カードにアニメーションクラスを追加
  const statsCards = document.querySelectorAll('.stats-card');
  statsCards.forEach((card, index) => {
    card.classList.add('slide-in-up');
    card.style.animationDelay = `${index * 0.1}s`;
  });
}

// ページタイトルの更新
function updatePageTitle(title) {
  const pageTitle = document.getElementById('page-title');
  if (pageTitle) {
    pageTitle.textContent = title;
  }
}

// スケルトンローディングの生成
function createSkeletonLoader(type, count = 1) {
  let html = '';
  
  switch (type) {
    case 'card':
      for (let i = 0; i < count; i++) {
        html += `
          <div class="card">
            <div class="card-body">
              <div class="skeleton skeleton-text" style="width: 60%;"></div>
              <div class="skeleton skeleton-text" style="width: 80%;"></div>
              <div class="skeleton skeleton-text" style="width: 40%;"></div>
            </div>
          </div>
        `;
      }
      break;
      
    case 'table-row':
      for (let i = 0; i < count; i++) {
        html += `
          <tr>
            <td><div class="skeleton skeleton-text"></div></td>
            <td><div class="skeleton skeleton-text"></div></td>
            <td><div class="skeleton skeleton-text"></div></td>
            <td><div class="skeleton skeleton-text"></div></td>
            <td><div class="skeleton skeleton-text"></div></td>
          </tr>
        `;
      }
      break;
      
    case 'list-item':
      for (let i = 0; i < count; i++) {
        html += `
          <div class="list-group-item">
            <div class="d-flex justify-content-between">
              <div style="width: 60%;">
                <div class="skeleton skeleton-text" style="width: 80%;"></div>
                <div class="skeleton skeleton-text" style="width: 60%;"></div>
              </div>
              <div style="width: 30%;">
                <div class="skeleton skeleton-text"></div>
              </div>
            </div>
          </div>
        `;
      }
      break;
  }
  
  return html;
}

// グローバル検索機能
function setupGlobalSearch() {
  const searchInput = document.getElementById('global-search');
  if (!searchInput) return;
  
  searchInput.addEventListener('keyup', function(e) {
    if (e.key === 'Enter') {
      const query = this.value.trim();
      if (query) {
        performSearch(query);
      }
    }
  });
}

// 検索実行
function performSearch(query) {
  // 検索結果ページを表示
  contentArea.innerHTML = `
    <h1 class="mb-4">Search Results for "${query}"</h1>
    
    <div class="card mb-4">
      <div class="card-body">
        <div class="input-group">
          <input type="text" class="form-control" id="search-input" value="${query}" placeholder="Search for blocks, transactions, accounts...">
          <button class="btn btn-primary" type="button" id="search-button">
            <i class="bi bi-search"></i>
          </button>
        </div>
      </div>
    </div>
    
    <div class="row">
      <div class="col-md-12">
        <div class="card">
          <div class="card-header">
            <ul class="nav nav-tabs card-header-tabs" id="search-tabs" role="tablist">
              <li class="nav-item" role="presentation">
                <button class="nav-link active" id="all-tab" data-bs-toggle="tab" data-bs-target="#all-tab-pane" type="button" role="tab" aria-controls="all-tab-pane" aria-selected="true">
                  All Results
                </button>
              </li>
              <li class="nav-item" role="presentation">
                <button class="nav-link" id="blocks-tab" data-bs-toggle="tab" data-bs-target="#blocks-tab-pane" type="button" role="tab" aria-controls="blocks-tab-pane" aria-selected="false">
                  Blocks
                </button>
              </li>
              <li class="nav-item" role="presentation">
                <button class="nav-link" id="txs-tab" data-bs-toggle="tab" data-bs-target="#txs-tab-pane" type="button" role="tab" aria-controls="txs-tab-pane" aria-selected="false">
                  Transactions
                </button>
              </li>
              <li class="nav-item" role="presentation">
                <button class="nav-link" id="accounts-tab" data-bs-toggle="tab" data-bs-target="#accounts-tab-pane" type="button" role="tab" aria-controls="accounts-tab-pane" aria-selected="false">
                  Accounts
                </button>
              </li>
            </ul>
          </div>
          <div class="card-body">
            <div class="tab-content" id="search-tab-content">
              <div class="tab-pane fade show active" id="all-tab-pane" role="tabpanel" aria-labelledby="all-tab" tabindex="0">
                <div id="all-results">
                  ${createSkeletonLoader('list-item', 5)}
                </div>
              </div>
              <div class="tab-pane fade" id="blocks-tab-pane" role="tabpanel" aria-labelledby="blocks-tab" tabindex="0">
                <div id="block-results">
                  ${createSkeletonLoader('list-item', 3)}
                </div>
              </div>
              <div class="tab-pane fade" id="txs-tab-pane" role="tabpanel" aria-labelledby="txs-tab" tabindex="0">
                <div id="tx-results">
                  ${createSkeletonLoader('list-item', 3)}
                </div>
              </div>
              <div class="tab-pane fade" id="accounts-tab-pane" role="tabpanel" aria-labelledby="accounts-tab" tabindex="0">
                <div id="account-results">
                  ${createSkeletonLoader('list-item', 3)}
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  `;
  
  // 検索ボタンのイベントリスナー
  const searchButton = document.getElementById('search-button');
  const searchInput = document.getElementById('search-input');
  
  if (searchButton && searchInput) {
    searchButton.addEventListener('click', function() {
      const newQuery = searchInput.value.trim();
      if (newQuery) {
        performSearch(newQuery);
      }
    });
  }
  
  // 模擬的な検索結果を表示（実際のAPIが実装されるまで）
  setTimeout(() => {
    displaySearchResults(query);
  }, 1500);
  
  // ページタイトルを更新
  updatePageTitle('Search Results');
}

// 検索結果の表示
function displaySearchResults(query) {
  // 全結果タブ
  const allResults = document.getElementById('all-results');
  if (allResults) {
    allResults.innerHTML = `
      <div class="mb-3">
        <h5>Blocks</h5>
        <div class="list-group mb-3">
          <a href="#" class="list-group-item list-group-item-action">
            <div class="d-flex justify-content-between align-items-center">
              <div>
                <div class="fw-bold">Block #10245</div>
                <div class="small text-muted">Hash: 0x1234...5678</div>
              </div>
              <span class="badge bg-primary rounded-pill">3 transactions</span>
            </div>
          </a>
        </div>
        
        <h5>Transactions</h5>
        <div class="list-group mb-3">
          <a href="#" class="list-group-item list-group-item-action">
            <div class="d-flex justify-content-between align-items-center">
              <div>
                <div class="fw-bold font-monospace">0xabcd...1234</div>
                <div class="small text-muted">From: 0x1234...5678 To: 0xabcd...efgh</div>
              </div>
              <div class="text-end">
                <div>1,000 tokens</div>
                <div class="small text-muted">2 hours ago</div>
              </div>
            </div>
          </a>
          <a href="#" class="list-group-item list-group-item-action">
            <div class="d-flex justify-content-between align-items-center">
              <div>
                <div class="fw-bold font-monospace">0xefgh...5678</div>
                <div class="small text-muted">From: 0xabcd...efgh To: 0x9876...5432</div>
              </div>
              <div class="text-end">
                <div>500 tokens</div>
                <div class="small text-muted">3 hours ago</div>
              </div>
            </div>
          </a>
        </div>
        
        <h5>Accounts</h5>
        <div class="list-group">
          <a href="#" class="list-group-item list-group-item-action">
            <div class="d-flex justify-content-between align-items-center">
              <div>
                <div class="fw-bold font-monospace">0x1234...5678</div>
                <div class="small text-muted">Balance: 10,000 tokens</div>
              </div>
              <span class="badge bg-success">Active</span>
            </div>
          </a>
        </div>
      </div>
    `;
  }
  
  // ブロック結果タブ
  const blockResults = document.getElementById('block-results');
  if (blockResults) {
    blockResults.innerHTML = `
      <div class="list-group">
        <a href="#" class="list-group-item list-group-item-action">
          <div class="d-flex justify-content-between align-items-center">
            <div>
              <div class="fw-bold">Block #10245</div>
              <div class="small text-muted">Hash: 0x1234...5678</div>
              <div class="small text-muted">Timestamp: 2023-03-15 14:30:45</div>
            </div>
            <span class="badge bg-primary rounded-pill">3 transactions</span>
          </div>
        </a>
        <a href="#" class="list-group-item list-group-item-action">
          <div class="d-flex justify-content-between align-items-center">
            <div>
              <div class="fw-bold">Block #10244</div>
              <div class="small text-muted">Hash: 0xabcd...efgh</div>
              <div class="small text-muted">Timestamp: 2023-03-15 14:29:30</div>
            </div>
            <span class="badge bg-primary rounded-pill">1 transaction</span>
          </div>
        </a>
      </div>
    `;
  }
  
  // トランザクション結果タブ
  const txResults = document.getElementById('tx-results');
  if (txResults) {
    txResults.innerHTML = `
      <div class="list-group">
        <a href="#" class="list-group-item list-group-item-action">
          <div class="d-flex justify-content-between align-items-center">
            <div>
              <div class="fw-bold font-monospace">0xabcd...1234</div>
              <div class="small text-muted">From: 0x1234...5678 To: 0xabcd...efgh</div>
              <div class="small text-muted">Block: #10245</div>
            </div>
            <div class="text-end">
              <div>1,000 tokens</div>
              <div class="small text-muted">2 hours ago</div>
            </div>
          </div>
        </a>
        <a href="#" class="list-group-item list-group-item-action">
          <div class="d-flex justify-content-between align-items-center">
            <div>
              <div class="fw-bold font-monospace">0xefgh...5678</div>
              <div class="small text-muted">From: 0xabcd...efgh To: 0x9876...5432</div>
              <div class="small text-muted">Block: #10244</div>
            </div>
            <div class="text-end">
              <div>500 tokens</div>
              <div class="small text-muted">3 hours ago</div>
            </div>
          </div>
        </a>
      </div>
    `;
  }
  
  // アカウント結果タブ
  const accountResults = document.getElementById('account-results');
  if (accountResults) {
    accountResults.innerHTML = `
      <div class="list-group">
        <a href="#" class="list-group-item list-group-item-action">
          <div class="d-flex justify-content-between align-items-center">
            <div>
              <div class="fw-bold font-monospace">0x1234...5678</div>
              <div class="small text-muted">Balance: 10,000 tokens</div>
              <div class="small text-muted">Transactions: 25</div>
            </div>
            <span class="badge bg-success">Active</span>
          </div>
        </a>
      </div>
    `;
  }
}

// ページ遷移時のアニメーション
function pageTransition(callback) {
  const contentArea = document.getElementById('content-area');
  if (!contentArea) return;
  
  // フェードアウト
  contentArea.style.opacity = '0';
  contentArea.style.transition = 'opacity 0.3s ease';
  
  // アニメーション完了後にコールバックを実行
  setTimeout(() => {
    callback();
    
    // フェードイン
    setTimeout(() => {
      contentArea.style.opacity = '1';
    }, 50);
  }, 300);
}

// ナビゲーションリンクのクリックイベントをオーバーライド
function overrideNavigation() {
  // ダッシュボードリンク
  const dashboardLink = document.getElementById('dashboard-link');
  if (dashboardLink) {
    dashboardLink.addEventListener('click', function(e) {
      e.preventDefault();
      pageTransition(() => {
        setActiveLink(dashboardLink);
        showDashboard();
        updatePageTitle('Dashboard');
      });
    });
  }
  
  // その他のリンクも同様に処理
  // ...
}

// ウィンドウサイズ変更時の処理
window.addEventListener('resize', function() {
  // モバイル表示時にサイドバーを自動的に折りたたむ
  if (window.innerWidth < 992 && !sidebarCollapsed) {
    sidebarCollapsed = true;
    applySidebarState();
  }
});