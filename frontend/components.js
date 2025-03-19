// Rustorium UI コンポーネント

// トースト通知を表示
function showToast(message, type = 'info', duration = 3000) {
  // 既存のトーストコンテナを取得または作成
  let toastContainer = document.querySelector('.toast-container');
  if (!toastContainer) {
    toastContainer = document.createElement('div');
    toastContainer.className = 'toast-container position-fixed bottom-0 end-0 p-3';
    document.body.appendChild(toastContainer);
  }
  
  // トーストID生成
  const toastId = 'toast-' + Date.now();
  
  // トーストタイプに基づくアイコンとクラス
  let icon, bgClass;
  switch (type) {
    case 'success':
      icon = 'bi-check-circle-fill';
      bgClass = 'bg-success';
      break;
    case 'warning':
      icon = 'bi-exclamation-triangle-fill';
      bgClass = 'bg-warning';
      break;
    case 'error':
      icon = 'bi-x-circle-fill';
      bgClass = 'bg-danger';
      break;
    default:
      icon = 'bi-info-circle-fill';
      bgClass = 'bg-primary';
  }
  
  // トーストHTML作成
  const toastHtml = `
    <div id="${toastId}" class="toast" role="alert" aria-live="assertive" aria-atomic="true">
      <div class="toast-header ${bgClass} text-white">
        <i class="bi ${icon} me-2"></i>
        <strong class="me-auto">${type.charAt(0).toUpperCase() + type.slice(1)}</strong>
        <small>Just now</small>
        <button type="button" class="btn-close btn-close-white" data-bs-dismiss="toast" aria-label="Close"></button>
      </div>
      <div class="toast-body">
        ${message}
      </div>
    </div>
  `;
  
  // トーストをDOMに追加
  toastContainer.insertAdjacentHTML('beforeend', toastHtml);
  
  // Bootstrapトーストを初期化して表示
  const toastElement = document.getElementById(toastId);
  const toast = new bootstrap.Toast(toastElement, { autohide: true, delay: duration });
  toast.show();
  
  // 指定時間後に削除
  setTimeout(() => {
    toastElement.remove();
  }, duration + 500);
}

// モーダルダイアログを表示
function showModal(options) {
  const {
    title = 'Modal Title',
    body = '',
    size = '', // '', 'modal-sm', 'modal-lg', 'modal-xl'
    buttons = [
      {
        text: 'Close',
        type: 'secondary',
        dismiss: true
      }
    ],
    onShow = null,
    onHide = null,
    staticBackdrop = false
  } = options;
  
  // モーダルID生成
  const modalId = 'modal-' + Date.now();
  
  // ボタンHTML生成
  const buttonsHtml = buttons.map(button => {
    const { text, type = 'primary', dismiss = false, id = '', onClick = null } = button;
    return `
      <button type="button" class="btn btn-${type}" ${dismiss ? 'data-bs-dismiss="modal"' : ''} ${id ? `id="${id}"` : ''}>
        ${text}
      </button>
    `;
  }).join('');
  
  // モーダルHTML作成
  const modalHtml = `
    <div class="modal fade" id="${modalId}" tabindex="-1" aria-labelledby="${modalId}-label" aria-hidden="true" ${staticBackdrop ? 'data-bs-backdrop="static" data-bs-keyboard="false"' : ''}>
      <div class="modal-dialog ${size}">
        <div class="modal-content">
          <div class="modal-header">
            <h5 class="modal-title" id="${modalId}-label">${title}</h5>
            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
          </div>
          <div class="modal-body">
            ${body}
          </div>
          <div class="modal-footer">
            ${buttonsHtml}
          </div>
        </div>
      </div>
    </div>
  `;
  
  // モーダルをDOMに追加
  document.body.insertAdjacentHTML('beforeend', modalHtml);
  
  // モーダル要素を取得
  const modalElement = document.getElementById(modalId);
  
  // イベントリスナーを設定
  if (onShow) {
    modalElement.addEventListener('shown.bs.modal', onShow);
  }
  
  if (onHide) {
    modalElement.addEventListener('hidden.bs.modal', onHide);
  }
  
  // モーダルが閉じられたときにDOMから削除
  modalElement.addEventListener('hidden.bs.modal', () => {
    modalElement.remove();
  });
  
  // ボタンのイベントリスナーを設定
  buttons.forEach((button, index) => {
    if (button.onClick && !button.id) {
      const buttonElement = modalElement.querySelectorAll('.modal-footer .btn')[index];
      buttonElement.addEventListener('click', (e) => {
        button.onClick(e, modalElement);
      });
    } else if (button.onClick && button.id) {
      const buttonElement = document.getElementById(button.id);
      if (buttonElement) {
        buttonElement.addEventListener('click', (e) => {
          button.onClick(e, modalElement);
        });
      }
    }
  });
  
  // Bootstrapモーダルを初期化して表示
  const modal = new bootstrap.Modal(modalElement);
  modal.show();
  
  // モーダルインスタンスを返す
  return {
    element: modalElement,
    instance: modal,
    close: () => modal.hide()
  };
}

// 確認ダイアログを表示
function showConfirmDialog(message, onConfirm, onCancel = null) {
  return showModal({
    title: 'Confirmation',
    body: `<p>${message}</p>`,
    buttons: [
      {
        text: 'Cancel',
        type: 'secondary',
        dismiss: true,
        onClick: onCancel
      },
      {
        text: 'Confirm',
        type: 'primary',
        onClick: (e, modalElement) => {
          const modal = bootstrap.Modal.getInstance(modalElement);
          modal.hide();
          if (onConfirm) onConfirm();
        }
      }
    ]
  });
}

// アラートダイアログを表示
function showAlert(message, type = 'info', onClose = null) {
  let icon, title;
  
  switch (type) {
    case 'success':
      icon = 'bi-check-circle-fill text-success';
      title = 'Success';
      break;
    case 'warning':
      icon = 'bi-exclamation-triangle-fill text-warning';
      title = 'Warning';
      break;
    case 'error':
      icon = 'bi-x-circle-fill text-danger';
      title = 'Error';
      break;
    default:
      icon = 'bi-info-circle-fill text-primary';
      title = 'Information';
  }
  
  return showModal({
    title: `<i class="bi ${icon} me-2"></i> ${title}`,
    body: `<p>${message}</p>`,
    buttons: [
      {
        text: 'OK',
        type: type === 'error' ? 'danger' : (type === 'warning' ? 'warning' : 'primary'),
        dismiss: true,
        onClick: onClose
      }
    ]
  });
}

// ローディングスピナーを表示
function showSpinner(container, size = 'medium', message = 'Loading...') {
  const spinnerSizeClass = size === 'small' ? 'spinner-border-sm' : (size === 'large' ? '' : '');
  
  const spinnerHtml = `
    <div class="d-flex flex-column align-items-center justify-content-center my-5">
      <div class="spinner-border text-primary ${spinnerSizeClass}" role="status">
        <span class="visually-hidden">Loading...</span>
      </div>
      ${message ? `<div class="mt-3">${message}</div>` : ''}
    </div>
  `;
  
  if (typeof container === 'string') {
    container = document.getElementById(container);
  }
  
  if (container) {
    container.innerHTML = spinnerHtml;
  }
  
  return {
    remove: () => {
      if (container) {
        container.innerHTML = '';
      }
    },
    update: (newMessage) => {
      if (container) {
        const messageElement = container.querySelector('.mt-3');
        if (messageElement) {
          messageElement.textContent = newMessage;
        }
      }
    }
  };
}

// スケルトンローディングを表示
function showSkeleton(container, type = 'card', count = 1) {
  let skeletonHtml = '';
  
  switch (type) {
    case 'card':
      for (let i = 0; i < count; i++) {
        skeletonHtml += `
          <div class="card mb-3">
            <div class="card-body">
              <div class="skeleton" style="height: 24px; width: 60%; margin-bottom: 15px;"></div>
              <div class="skeleton" style="height: 16px; width: 90%; margin-bottom: 10px;"></div>
              <div class="skeleton" style="height: 16px; width: 80%; margin-bottom: 10px;"></div>
              <div class="skeleton" style="height: 16px; width: 70%;"></div>
            </div>
          </div>
        `;
      }
      break;
      
    case 'list':
      skeletonHtml = `<div class="list-group">`;
      for (let i = 0; i < count; i++) {
        skeletonHtml += `
          <div class="list-group-item">
            <div class="d-flex justify-content-between">
              <div style="width: 60%;">
                <div class="skeleton" style="height: 20px; width: 80%; margin-bottom: 10px;"></div>
                <div class="skeleton" style="height: 16px; width: 60%;"></div>
              </div>
              <div style="width: 30%;">
                <div class="skeleton" style="height: 20px; width: 100%;"></div>
              </div>
            </div>
          </div>
        `;
      }
      skeletonHtml += `</div>`;
      break;
      
    case 'table':
      skeletonHtml = `
        <div class="table-responsive">
          <table class="table">
            <thead>
              <tr>
                ${Array(5).fill('<th><div class="skeleton" style="height: 20px;"></div></th>').join('')}
              </tr>
            </thead>
            <tbody>
      `;
      
      for (let i = 0; i < count; i++) {
        skeletonHtml += `
          <tr>
            ${Array(5).fill('<td><div class="skeleton" style="height: 16px;"></div></td>').join('')}
          </tr>
        `;
      }
      
      skeletonHtml += `
            </tbody>
          </table>
        </div>
      `;
      break;
      
    case 'stats':
      skeletonHtml = `<div class="row">`;
      for (let i = 0; i < count; i++) {
        skeletonHtml += `
          <div class="col-md-${12 / Math.min(count, 4)}">
            <div class="stats-card">
              <div class="skeleton" style="height: 40px; width: 40px; border-radius: 50%; margin: 0 auto 15px;"></div>
              <div class="skeleton" style="height: 30px; width: 60%; margin: 0 auto 10px;"></div>
              <div class="skeleton" style="height: 16px; width: 80%; margin: 0 auto;"></div>
            </div>
          </div>
        `;
      }
      skeletonHtml += `</div>`;
      break;
  }
  
  if (typeof container === 'string') {
    container = document.getElementById(container);
  }
  
  if (container) {
    container.innerHTML = skeletonHtml;
  }
  
  return {
    remove: () => {
      if (container) {
        container.innerHTML = '';
      }
    }
  };
}

// ツールチップを作成
function createTooltip(element, text, position = 'top') {
  if (typeof element === 'string') {
    element = document.getElementById(element);
  }
  
  if (element) {
    element.setAttribute('data-bs-toggle', 'tooltip');
    element.setAttribute('data-bs-placement', position);
    element.setAttribute('title', text);
    
    // Bootstrapツールチップを初期化
    new bootstrap.Tooltip(element);
  }
}

// ドロップダウンメニューを作成
function createDropdown(options) {
  const {
    container,
    buttonText = 'Dropdown',
    buttonIcon = '',
    buttonClass = 'btn-primary',
    items = [],
    position = 'bottom-end'
  } = options;
  
  const dropdownId = 'dropdown-' + Date.now();
  
  // ドロップダウンアイテムHTML生成
  const itemsHtml = items.map(item => {
    if (item.divider) {
      return `<li><hr class="dropdown-divider"></li>`;
    }
    
    const { text, icon = '', href = '#', onClick = null, active = false } = item;
    
    return `
      <li>
        <a class="dropdown-item ${active ? 'active' : ''}" href="${href}" ${onClick ? `data-action="${dropdownId}-item-${items.indexOf(item)}"` : ''}>
          ${icon ? `<i class="bi ${icon} me-2"></i>` : ''}${text}
        </a>
      </li>
    `;
  }).join('');
  
  // ドロップダウンHTML作成
  const dropdownHtml = `
    <div class="dropdown">
      <button class="btn ${buttonClass} dropdown-toggle" type="button" id="${dropdownId}" data-bs-toggle="dropdown" aria-expanded="false">
        ${buttonIcon ? `<i class="bi ${buttonIcon} me-1"></i>` : ''}${buttonText}
      </button>
      <ul class="dropdown-menu ${position.includes('end') ? 'dropdown-menu-end' : ''}" aria-labelledby="${dropdownId}">
        ${itemsHtml}
      </ul>
    </div>
  `;
  
  // コンテナにドロップダウンを追加
  if (typeof container === 'string') {
    container = document.getElementById(container);
  }
  
  if (container) {
    container.innerHTML = dropdownHtml;
    
    // クリックイベントを設定
    items.forEach((item, index) => {
      if (item.onClick) {
        const actionSelector = `[data-action="${dropdownId}-item-${index}"]`;
        const actionElement = container.querySelector(actionSelector);
        
        if (actionElement) {
          actionElement.addEventListener('click', (e) => {
            e.preventDefault();
            item.onClick(e);
          });
        }
      }
    });
  }
}

// タブを作成
function createTabs(options) {
  const {
    container,
    tabs = [],
    activeTab = 0,
    contentContainer = null,
    onTabChange = null
  } = options;
  
  const tabsId = 'tabs-' + Date.now();
  
  // タブHTML生成
  const tabsHtml = `
    <ul class="nav nav-tabs" id="${tabsId}" role="tablist">
      ${tabs.map((tab, index) => `
        <li class="nav-item" role="presentation">
          <button class="nav-link ${index === activeTab ? 'active' : ''}" 
                  id="${tabsId}-tab-${index}" 
                  data-bs-toggle="tab" 
                  data-bs-target="#${tabsId}-content-${index}" 
                  type="button" 
                  role="tab" 
                  aria-controls="${tabsId}-content-${index}" 
                  aria-selected="${index === activeTab}">
            ${tab.icon ? `<i class="bi ${tab.icon} me-1"></i>` : ''}${tab.title}
          </button>
        </li>
      `).join('')}
    </ul>
  `;
  
  // タブコンテンツHTML生成
  const contentHtml = `
    <div class="tab-content" id="${tabsId}-content">
      ${tabs.map((tab, index) => `
        <div class="tab-pane fade ${index === activeTab ? 'show active' : ''}" 
             id="${tabsId}-content-${index}" 
             role="tabpanel" 
             aria-labelledby="${tabsId}-tab-${index}">
          ${tab.content || ''}
        </div>
      `).join('')}
    </div>
  `;
  
  // コンテナにタブを追加
  if (typeof container === 'string') {
    container = document.getElementById(container);
  }
  
  if (typeof contentContainer === 'string') {
    contentContainer = document.getElementById(contentContainer);
  }
  
  if (container) {
    container.innerHTML = tabsHtml;
    
    if (contentContainer) {
      contentContainer.innerHTML = contentHtml;
    } else {
      container.insertAdjacentHTML('afterend', contentHtml);
    }
    
    // タブ変更イベントを設定
    if (onTabChange) {
      tabs.forEach((tab, index) => {
        const tabElement = document.getElementById(`${tabsId}-tab-${index}`);
        if (tabElement) {
          tabElement.addEventListener('shown.bs.tab', (e) => {
            onTabChange(index, tab);
          });
        }
      });
    }
  }
  
  return {
    setActiveTab: (index) => {
      const tabElement = document.getElementById(`${tabsId}-tab-${index}`);
      if (tabElement) {
        const tabInstance = new bootstrap.Tab(tabElement);
        tabInstance.show();
      }
    },
    updateTabContent: (index, content) => {
      const contentElement = document.getElementById(`${tabsId}-content-${index}`);
      if (contentElement) {
        contentElement.innerHTML = content;
      }
    }
  };
}

// カードを作成
function createCard(options) {
  const {
    title = '',
    content = '',
    footer = '',
    headerActions = '',
    className = '',
    animation = ''
  } = options;
  
  return `
    <div class="card ${className} ${animation}">
      ${title ? `
        <div class="card-header d-flex justify-content-between align-items-center">
          <h5 class="mb-0">${title}</h5>
          ${headerActions}
        </div>
      ` : ''}
      <div class="card-body">
        ${content}
      </div>
      ${footer ? `
        <div class="card-footer">
          ${footer}
        </div>
      ` : ''}
    </div>
  `;
}

// 統計カードを作成
function createStatsCard(options) {
  const {
    icon = '',
    iconClass = 'text-primary',
    value = '',
    label = '',
    className = '',
    animation = ''
  } = options;
  
  return `
    <div class="stats-card ${className} ${animation}">
      <div class="icon ${iconClass}">
        <i class="bi ${icon}"></i>
      </div>
      <div class="value">${value}</div>
      <div class="label">${label}</div>
    </div>
  `;
}

// バッジを作成
function createBadge(text, type = 'primary', icon = '') {
  return `
    <span class="badge bg-${type}">
      ${icon ? `<i class="bi ${icon} me-1"></i>` : ''}${text}
    </span>
  `;
}

// アラートを作成
function createAlert(message, type = 'info', dismissible = false, icon = true) {
  let iconClass = '';
  
  switch (type) {
    case 'success':
      iconClass = 'bi-check-circle-fill';
      break;
    case 'warning':
      iconClass = 'bi-exclamation-triangle-fill';
      break;
    case 'danger':
      iconClass = 'bi-x-circle-fill';
      break;
    default:
      iconClass = 'bi-info-circle-fill';
  }
  
  return `
    <div class="alert alert-${type} ${dismissible ? 'alert-dismissible fade show' : ''}" role="alert">
      ${icon ? `<i class="bi ${iconClass} me-2"></i>` : ''}
      ${message}
      ${dismissible ? `
        <button type="button" class="btn-close" data-bs-dismiss="alert" aria-label="Close"></button>
      ` : ''}
    </div>
  `;
}

// プログレスバーを作成
function createProgressBar(value, max = 100, type = 'primary', height = null, striped = false, animated = false, label = false) {
  const percentage = Math.round((value / max) * 100);
  
  return `
    <div class="progress ${height ? `" style="height: ${height}px;` : ''}">
      <div class="progress-bar bg-${type} ${striped ? 'progress-bar-striped' : ''} ${animated ? 'progress-bar-animated' : ''}" 
           role="progressbar" 
           style="width: ${percentage}%;" 
           aria-valuenow="${value}" 
           aria-valuemin="0" 
           aria-valuemax="${max}">
        ${label ? `${percentage}%` : ''}
      </div>
    </div>
  `;
}

// ボタングループを作成
function createButtonGroup(buttons, vertical = false) {
  const groupHtml = `
    <div class="btn-group ${vertical ? 'btn-group-vertical' : ''}" role="group" aria-label="Button group">
      ${buttons.map(button => {
        const { text, type = 'primary', icon = '', onClick = null, id = '', disabled = false } = button;
        return `
          <button type="button" class="btn btn-${type}" ${id ? `id="${id}"` : ''} ${disabled ? 'disabled' : ''}>
            ${icon ? `<i class="bi ${icon} me-1"></i>` : ''}${text}
          </button>
        `;
      }).join('')}
    </div>
  `;
  
  return groupHtml;
}

// ページネーションを作成
function createPagination(currentPage, totalPages, onPageChange) {
  const paginationHtml = `
    <nav aria-label="Page navigation">
      <ul class="pagination">
        <li class="page-item ${currentPage === 1 ? 'disabled' : ''}">
          <a class="page-link" href="#" aria-label="Previous" data-page="${currentPage - 1}">
            <span aria-hidden="true">&laquo;</span>
          </a>
        </li>
        
        ${Array.from({ length: totalPages }, (_, i) => i + 1).map(page => `
          <li class="page-item ${page === currentPage ? 'active' : ''}">
            <a class="page-link" href="#" data-page="${page}">${page}</a>
          </li>
        `).join('')}
        
        <li class="page-item ${currentPage === totalPages ? 'disabled' : ''}">
          <a class="page-link" href="#" aria-label="Next" data-page="${currentPage + 1}">
            <span aria-hidden="true">&raquo;</span>
          </a>
        </li>
      </ul>
    </nav>
  `;
  
  const paginationElement = document.createElement('div');
  paginationElement.innerHTML = paginationHtml;
  
  // ページ変更イベントを設定
  const pageLinks = paginationElement.querySelectorAll('.page-link');
  pageLinks.forEach(link => {
    link.addEventListener('click', (e) => {
      e.preventDefault();
      const page = parseInt(link.getAttribute('data-page'));
      if (page >= 1 && page <= totalPages && page !== currentPage) {
        onPageChange(page);
      }
    });
  });
  
  return paginationElement.firstChild;
}

// 検索フォームを作成
function createSearchForm(options) {
  const {
    placeholder = 'Search...',
    buttonText = '',
    buttonIcon = 'bi-search',
    onSearch = null,
    advanced = false,
    filters = []
  } = options;
  
  const searchFormId = 'search-form-' + Date.now();
  
  let formHtml = `
    <form id="${searchFormId}" class="search-form">
      <div class="input-group">
        <input type="text" class="form-control" placeholder="${placeholder}" aria-label="${placeholder}">
        <button class="btn btn-primary" type="submit">
          <i class="bi ${buttonIcon}"></i> ${buttonText}
        </button>
        ${advanced ? `
          <button class="btn btn-outline-secondary" type="button" data-bs-toggle="collapse" data-bs-target="#${searchFormId}-advanced" aria-expanded="false" aria-controls="${searchFormId}-advanced">
            <i class="bi bi-sliders"></i>
          </button>
        ` : ''}
      </div>
      
      ${advanced ? `
        <div class="collapse mt-3" id="${searchFormId}-advanced">
          <div class="card card-body">
            <div class="row">
              ${filters.map(filter => {
                const { type, name, label, options = [], placeholder = '' } = filter;
                
                if (type === 'select') {
                  return `
                    <div class="col-md-4 mb-3">
                      <label for="${searchFormId}-${name}" class="form-label">${label}</label>
                      <select class="form-select" id="${searchFormId}-${name}" name="${name}">
                        ${options.map(opt => `
                          <option value="${opt.value}" ${opt.selected ? 'selected' : ''}>${opt.text}</option>
                        `).join('')}
                      </select>
                    </div>
                  `;
                } else if (type === 'date') {
                  return `
                    <div class="col-md-4 mb-3">
                      <label for="${searchFormId}-${name}" class="form-label">${label}</label>
                      <input type="date" class="form-control" id="${searchFormId}-${name}" name="${name}">
                    </div>
                  `;
                } else {
                  return `
                    <div class="col-md-4 mb-3">
                      <label for="${searchFormId}-${name}" class="form-label">${label}</label>
                      <input type="${type}" class="form-control" id="${searchFormId}-${name}" name="${name}" placeholder="${placeholder}">
                    </div>
                  `;
                }
              }).join('')}
            </div>
            <div class="d-flex justify-content-end">
              <button type="button" class="btn btn-secondary me-2" id="${searchFormId}-reset">Reset</button>
              <button type="submit" class="btn btn-primary">Apply Filters</button>
            </div>
          </div>
        </div>
      ` : ''}
    </form>
  `;
  
  const formElement = document.createElement('div');
  formElement.innerHTML = formHtml;
  
  // 検索イベントを設定
  const form = formElement.querySelector(`#${searchFormId}`);
  if (form && onSearch) {
    form.addEventListener('submit', (e) => {
      e.preventDefault();
      
      const formData = new FormData(form);
      const searchData = {};
      
      for (const [key, value] of formData.entries()) {
        searchData[key] = value;
      }
      
      // 検索入力値を取得
      const searchInput = form.querySelector('input[type="text"]');
      if (searchInput) {
        searchData.query = searchInput.value;
      }
      
      onSearch(searchData);
    });
    
    // リセットボタンのイベント設定
    const resetButton = formElement.querySelector(`#${searchFormId}-reset`);
    if (resetButton) {
      resetButton.addEventListener('click', () => {
        form.reset();
      });
    }
  }
  
  return formElement.firstChild;
}

// コピーボタンを作成
function createCopyButton(text, buttonText = '', buttonIcon = 'bi-clipboard', successMessage = 'Copied!') {
  const buttonId = 'copy-btn-' + Date.now();
  
  const buttonHtml = `
    <button id="${buttonId}" class="btn btn-sm btn-outline-secondary" type="button" data-bs-toggle="tooltip" title="Copy to clipboard">
      <i class="bi ${buttonIcon}"></i> ${buttonText}
    </button>
  `;
  
  const buttonElement = document.createElement('div');
  buttonElement.innerHTML = buttonHtml;
  
  // コピーイベントを設定
  const button = buttonElement.querySelector(`#${buttonId}`);
  if (button) {
    button.addEventListener('click', () => {
      navigator.clipboard.writeText(text).then(() => {
        // ツールチップを更新
        const tooltip = bootstrap.Tooltip.getInstance(button);
        if (tooltip) {
          button.setAttribute('data-bs-original-title', successMessage);
          tooltip.show();
          
          // アイコンを一時的に変更
          const icon = button.querySelector('i');
          if (icon) {
            const originalClass = icon.className;
            icon.className = 'bi bi-check2';
            
            // 元に戻す
            setTimeout(() => {
              icon.className = originalClass;
              button.setAttribute('data-bs-original-title', 'Copy to clipboard');
            }, 2000);
          }
        }
      });
    });
    
    // ツールチップを初期化
    new bootstrap.Tooltip(button);
  }
  
  return buttonElement.firstChild;
}

// QRコードを生成
function createQRCode(container, data, options = {}) {
  const {
    width = 128,
    height = 128,
    colorDark = '#000000',
    colorLight = '#ffffff',
    correctLevel = 'H'
  } = options;
  
  if (typeof container === 'string') {
    container = document.getElementById(container);
  }
  
  if (!container) return null;
  
  // QRコードライブラリが読み込まれているか確認
  if (typeof QRCode === 'undefined') {
    // QRコードライブラリを動的に読み込む
    const script = document.createElement('script');
    script.src = 'https://cdn.jsdelivr.net/npm/qrcode@1.5.1/build/qrcode.min.js';
    script.onload = () => {
      // ライブラリ読み込み後にQRコードを生成
      const qrCodeCorrectLevel = {
        'L': QRCode.CorrectLevel.L,
        'M': QRCode.CorrectLevel.M,
        'Q': QRCode.CorrectLevel.Q,
        'H': QRCode.CorrectLevel.H
      };
      
      new QRCode(container, {
        text: data,
        width: width,
        height: height,
        colorDark: colorDark,
        colorLight: colorLight,
        correctLevel: qrCodeCorrectLevel[correctLevel] || QRCode.CorrectLevel.H
      });
    };
    document.head.appendChild(script);
  } else {
    // すでに読み込まれている場合は直接実行
    const qrCodeCorrectLevel = {
      'L': QRCode.CorrectLevel.L,
      'M': QRCode.CorrectLevel.M,
      'Q': QRCode.CorrectLevel.Q,
      'H': QRCode.CorrectLevel.H
    };
    
    new QRCode(container, {
      text: data,
      width: width,
      height: height,
      colorDark: colorDark,
      colorLight: colorLight,
      correctLevel: qrCodeCorrectLevel[correctLevel] || QRCode.CorrectLevel.H
    });
  }
}

// ドラッグ＆ドロップ機能を追加
function enableDragAndDrop(element, options = {}) {
  const {
    handle = null,
    onDragStart = null,
    onDrag = null,
    onDragEnd = null,
    containment = null
  } = options;
  
  if (typeof element === 'string') {
    element = document.getElementById(element);
  }
  
  if (!element) return;
  
  let handleElement = handle ? (typeof handle === 'string' ? element.querySelector(handle) : handle) : element;
  
  let isDragging = false;
  let startX, startY, startLeft, startTop;
  
  // 要素を絶対位置に設定
  const originalPosition = window.getComputedStyle(element).position;
  if (originalPosition !== 'absolute' && originalPosition !== 'fixed') {
    element.style.position = 'absolute';
  }
  
  // ドラッグ開始
  handleElement.addEventListener('mousedown', (e) => {
    e.preventDefault();
    
    isDragging = true;
    startX = e.clientX;
    startY = e.clientY;
    startLeft = parseInt(window.getComputedStyle(element).left) || 0;
    startTop = parseInt(window.getComputedStyle(element).top) || 0;
    
    if (onDragStart) {
      onDragStart(e, element);
    }
    
    // カーソルスタイルを変更
    document.body.style.cursor = 'move';
    
    // ドラッグ中
    const mouseMoveHandler = (e) => {
      if (!isDragging) return;
      
      const dx = e.clientX - startX;
      const dy = e.clientY - startY;
      
      let newLeft = startLeft + dx;
      let newTop = startTop + dy;
      
      // 制約がある場合は適用
      if (containment) {
        const rect = element.getBoundingClientRect();
        const container = typeof containment === 'string' ? document.querySelector(containment) : containment;
        const containerRect = container.getBoundingClientRect();
        
        newLeft = Math.max(0, Math.min(newLeft, containerRect.width - rect.width));
        newTop = Math.max(0, Math.min(newTop, containerRect.height - rect.height));
      }
      
      element.style.left = `${newLeft}px`;
      element.style.top = `${newTop}px`;
      
      if (onDrag) {
        onDrag(e, element, { left: newLeft, top: newTop });
      }
    };
    
    // ドラッグ終了
    const mouseUpHandler = (e) => {
      if (!isDragging) return;
      
      isDragging = false;
      
      // カーソルスタイルを元に戻す
      document.body.style.cursor = '';
      
      if (onDragEnd) {
        onDragEnd(e, element, {
          left: parseInt(element.style.left),
          top: parseInt(element.style.top)
        });
      }
      
      // イベントリスナーを削除
      document.removeEventListener('mousemove', mouseMoveHandler);
      document.removeEventListener('mouseup', mouseUpHandler);
    };
    
    // イベントリスナーを追加
    document.addEventListener('mousemove', mouseMoveHandler);
    document.addEventListener('mouseup', mouseUpHandler);
  });
  
  return {
    reset: () => {
      element.style.position = originalPosition;
      element.style.left = '';
      element.style.top = '';
    }
  };
}

// ショートカットキーを登録
function registerShortcut(key, callback, description = '') {
  document.addEventListener('keydown', (e) => {
    // 入力フィールドでのショートカットは無効化
    if (e.target.tagName === 'INPUT' || e.target.tagName === 'TEXTAREA') {
      return;
    }
    
    // キーの組み合わせをチェック
    const keys = key.toLowerCase().split('+');
    const mainKey = keys.pop();
    
    const hasCtrl = keys.includes('ctrl') && e.ctrlKey;
    const hasShift = keys.includes('shift') && e.shiftKey;
    const hasAlt = keys.includes('alt') && e.altKey;
    const hasModifier = (keys.includes('ctrl') || keys.includes('shift') || keys.includes('alt'));
    
    // 修飾キーの有無をチェック
    if (hasModifier) {
      if ((keys.includes('ctrl') && !e.ctrlKey) || 
          (keys.includes('shift') && !e.shiftKey) || 
          (keys.includes('alt') && !e.altKey)) {
        return;
      }
    }
    
    // メインキーをチェック
    if (e.key.toLowerCase() === mainKey) {
      e.preventDefault();
      callback(e);
    }
  });
  
  // ショートカットを登録リストに追加（ヘルプ表示用）
  if (!window.registeredShortcuts) {
    window.registeredShortcuts = [];
  }
  
  window.registeredShortcuts.push({
    key: key,
    description: description
  });
}

// ショートカットヘルプを表示
function showShortcutsHelp() {
  if (!window.registeredShortcuts || window.registeredShortcuts.length === 0) {
    showAlert('No keyboard shortcuts registered.', 'info');
    return;
  }
  
  const shortcutsHtml = window.registeredShortcuts.map(shortcut => `
    <tr>
      <td><kbd>${shortcut.key.split('+').map(k => `<kbd>${k}</kbd>`).join(' + ')}</kbd></td>
      <td>${shortcut.description}</td>
    </tr>
  `).join('');
  
  showModal({
    title: 'Keyboard Shortcuts',
    size: 'modal-lg',
    body: `
      <table class="table">
        <thead>
          <tr>
            <th>Shortcut</th>
            <th>Description</th>
          </tr>
        </thead>
        <tbody>
          ${shortcutsHtml}
        </tbody>
      </table>
    `,
    buttons: [
      {
        text: 'Close',
        type: 'secondary',
        dismiss: true
      }
    ]
  });
}

// ダークモード切り替えボタンを作成
function createDarkModeToggle(container) {
  if (typeof container === 'string') {
    container = document.getElementById(container);
  }
  
  if (!container) return;
  
  const currentTheme = document.documentElement.getAttribute('data-theme') || 'light';
  
  const toggleHtml = `
    <button id="theme-toggle" class="theme-toggle" aria-label="Toggle dark mode">
      <i class="bi ${currentTheme === 'dark' ? 'bi-sun' : 'bi-moon'}"></i>
    </button>
  `;
  
  container.innerHTML = toggleHtml;
  
  // クリックイベントを設定
  const toggleButton = container.querySelector('#theme-toggle');
  if (toggleButton) {
    toggleButton.addEventListener('click', () => {
      const theme = document.documentElement.getAttribute('data-theme');
      const newTheme = theme === 'dark' ? 'light' : 'dark';
      
      document.documentElement.setAttribute('data-theme', newTheme);
      localStorage.setItem('theme', newTheme);
      
      // アイコンを更新
      const icon = toggleButton.querySelector('i');
      if (icon) {
        icon.className = `bi ${newTheme === 'dark' ? 'bi-sun' : 'bi-moon'}`;
      }
    });
  }
}

// ページローダーを表示
function showPageLoader() {
  // 既存のローダーを確認
  let loader = document.querySelector('.page-loader');
  
  if (!loader) {
    loader = document.createElement('div');
    loader.className = 'page-loader';
    loader.innerHTML = `
      <div class="loader"></div>
    `;
    document.body.appendChild(loader);
  } else {
    loader.classList.remove('loaded');
  }
  
  return {
    hide: () => {
      loader.classList.add('loaded');
      setTimeout(() => {
        if (loader.parentNode) {
          loader.parentNode.removeChild(loader);
        }
      }, 500);
    }
  };
}

// 通知バッジを作成
function createNotificationBadge(container, count, type = 'danger') {
  if (typeof container === 'string') {
    container = document.getElementById(container);
  }
  
  if (!container) return;
  
  // 既存のバッジを削除
  const existingBadge = container.querySelector('.position-absolute.badge');
  if (existingBadge) {
    existingBadge.remove();
  }
  
  // カウントが0以下なら何も表示しない
  if (count <= 0) return;
  
  const badgeHtml = `
    <span class="position-absolute top-0 start-100 translate-middle badge rounded-pill bg-${type} badge-pulse">
      ${count > 99 ? '99+' : count}
    </span>
  `;
  
  container.style.position = 'relative';
  container.insertAdjacentHTML('beforeend', badgeHtml);
}