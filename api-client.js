// Rustorium API クライアント

// API基本URL
const API_BASE_URL = 'http://localhost:57620';

// ネットワーク設定
const NETWORKS = {
    mainnet: {
        name: 'Mainnet',
        url: 'http://localhost:57620',
        icon: 'bi-globe',
        color: '#ff6b4a',
        badge: 'danger'
    },
    testnet: {
        name: 'Testnet',
        url: 'http://localhost:57620',
        icon: 'bi-globe-americas',
        color: '#5468ff',
        badge: 'primary'
    },
    local: {
        name: 'Local Network',
        url: 'http://localhost:57620',
        icon: 'bi-laptop',
        color: '#00c9a7',
        badge: 'success'
    }
};

// 現在のネットワーク（デフォルトはメインネット）
let currentNetwork = localStorage.getItem('network') || 'mainnet';

// APIクライアントクラス
class RustoriumApiClient {
    constructor(network = currentNetwork) {
        this.network = network;
        this.baseUrl = NETWORKS[network].url;
    }
    
    // ネットワークを切り替える
    switchNetwork(network) {
        if (NETWORKS[network]) {
            this.network = network;
            this.baseUrl = NETWORKS[network].url;
            localStorage.setItem('network', network);
            return true;
        }
        return false;
    }
    
    // 現在のネットワーク情報を取得
    getCurrentNetworkInfo() {
        return NETWORKS[this.network];
    }

    // 共通のフェッチメソッド
    async _fetch(endpoint, options = {}) {
        try {
            const url = `${this.baseUrl}${endpoint}`;
            const response = await fetch(url, {
                ...options,
                headers: {
                    'Content-Type': 'application/json',
                    ...options.headers
                }
            });

            if (!response.ok) {
                const errorData = await response.json().catch(() => ({}));
                throw new Error(errorData.message || `API request failed with status ${response.status}`);
            }

            return await response.json();
        } catch (error) {
            console.error(`API request error: ${error.message}`);
            throw error;
        }
    }

    // ブロック関連 API

    // 最新のブロックリストを取得
    async getBlocks(limit = 10, page = 1) {
        return this._fetch(`/blocks?limit=${limit}&page=${page}`);
    }

    // ブロック詳細を取得
    async getBlockByNumberOrHash(blockId) {
        return this._fetch(`/blocks/${blockId}`);
    }

    // ブロック内のトランザクションを取得
    async getBlockTransactions(blockId, limit = 10, page = 1) {
        return this._fetch(`/blocks/${blockId}/transactions?limit=${limit}&page=${page}`);
    }

    // トランザクション関連 API

    // トランザクションリストを取得
    async getTransactions(limit = 10, page = 1) {
        return this._fetch(`/transactions?limit=${limit}&page=${page}`);
    }

    // トランザクション詳細を取得
    async getTransaction(txHash) {
        return this._fetch(`/transactions/${txHash}`);
    }

    // トランザクションを送信
    async sendTransaction(txData) {
        return this._fetch('/transactions', {
            method: 'POST',
            body: JSON.stringify(txData)
        });
    }

    // アカウント関連 API

    // アカウントリストを取得
    async getAccounts(limit = 10, page = 1) {
        return this._fetch(`/accounts?limit=${limit}&page=${page}`);
    }

    // アカウント詳細を取得
    async getAccount(address) {
        return this._fetch(`/accounts/${address}`);
    }

    // アカウントのトランザクション履歴を取得
    async getAccountTransactions(address, limit = 10, page = 1) {
        return this._fetch(`/accounts/${address}/transactions?limit=${limit}&page=${page}`);
    }

    // アカウントの残高を取得
    async getAccountBalance(address) {
        return this._fetch(`/accounts/${address}/balance`);
    }

    // アカウントのトークン残高を取得
    async getAccountTokens(address) {
        return this._fetch(`/accounts/${address}/tokens`);
    }

    // 新しいアカウントを作成
    async createAccount() {
        return this._fetch('/accounts', {
            method: 'POST'
        });
    }

    // スマートコントラクト関連 API

    // コントラクトをデプロイ
    async deployContract(contractData) {
        return this._fetch('/contracts', {
            method: 'POST',
            body: JSON.stringify(contractData)
        });
    }

    // コントラクト詳細を取得
    async getContract(address) {
        return this._fetch(`/contracts/${address}`);
    }

    // コントラクト関数を呼び出し
    async callContract(address, functionData) {
        return this._fetch(`/contracts/${address}/call`, {
            method: 'POST',
            body: JSON.stringify(functionData)
        });
    }

    // ネットワーク関連 API

    // ネットワークステータスを取得
    async getNetworkStatus() {
        return this._fetch('/network/status');
    }

    // ピア情報を取得
    async getPeers() {
        return this._fetch('/network/peers');
    }

    // シャード情報を取得
    async getShards() {
        return this._fetch('/network/shards');
    }

    // 統計情報を取得
    async getStats() {
        return this._fetch('/stats');
    }

    // ウォレット関連 API

    // ウォレット情報を取得
    async getWalletInfo() {
        return this._fetch('/wallet');
    }

    // ウォレットからトランザクションを送信
    async sendFromWallet(txData) {
        return this._fetch('/wallet/send', {
            method: 'POST',
            body: JSON.stringify(txData)
        });
    }

    // AI分析関連 API

    // トランザクション分析を取得
    async getTransactionAnalysis(txHash) {
        return this._fetch(`/ai/transactions/${txHash}/analysis`);
    }

    // アカウント分析を取得
    async getAccountAnalysis(address) {
        return this._fetch(`/ai/accounts/${address}/analysis`);
    }

    // 異常検知結果を取得
    async getAnomalyDetection() {
        return this._fetch('/ai/anomalies');
    }
}

// APIクライアントのインスタンスを作成
const apiClient = new RustoriumApiClient();

// エラーハンドリング用のラッパー関数
async function apiRequest(apiCall, fallbackData = null) {
    try {
        return await apiCall();
    } catch (error) {
        console.error('API request failed:', error);
        // 開発中はダミーデータを返す
        if (fallbackData) {
            console.warn('Using fallback data instead of API response');
            return fallbackData;
        }
        throw error;
    }
}

// APIリクエストのモック（開発中のみ使用）
function mockApiResponse(data, delay = 500) {
    return new Promise(resolve => {
        setTimeout(() => resolve(data), delay);
    });
}