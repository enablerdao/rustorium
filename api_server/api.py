"""
Rustorium API サーバー
"""

from fastapi import FastAPI, HTTPException, Query, Path, Body, Depends
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel, Field
from typing import List, Dict, Optional, Any
import uvicorn
import time
import logging
from blockchain import blockchain, Transaction

# ロギングの設定
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(name)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

# FastAPIアプリケーションの作成
app = FastAPI(
    title="Rustorium API",
    description="Rustorium ブロックチェーンのREST API",
    version="0.1.0"
)

# CORSミドルウェアの追加
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # 本番環境では適切に制限すべき
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# リクエスト/レスポンスモデル
class TransactionRequest(BaseModel):
    from_address: str = Field(..., alias="from")
    to_address: str = Field(..., alias="to")
    amount: float
    gas_price: int = 5
    gas_limit: int = 21000
    data: Optional[str] = None

class TransactionResponse(BaseModel):
    success: bool
    transaction_hash: Optional[str] = None
    error: Optional[str] = None

class AccountResponse(BaseModel):
    address: str
    balance: float
    nonce: int
    is_contract: bool
    transaction_count: int
    last_activity: float
    tokens: Dict[str, float] = {}

class BlockResponse(BaseModel):
    number: int
    hash: str
    parent_hash: str
    timestamp: float
    transactions: List[str]
    size: int
    gas_used: int
    gas_limit: int
    difficulty: int
    validator: str
    nonce: int

class NetworkStatsResponse(BaseModel):
    block_count: int
    latest_block: Optional[Dict[str, Any]] = None
    pending_transactions: int
    average_block_time: float
    tps: float
    account_count: int
    difficulty: int

# APIエンドポイント

@app.get("/")
async def root():
    """APIルートエンドポイント"""
    return {
        "name": "Rustorium API",
        "version": "0.1.0",
        "status": "running"
    }

# ブロック関連エンドポイント

@app.get("/blocks", response_model=Dict[str, Any])
async def get_blocks(limit: int = Query(10, ge=1, le=100), page: int = Query(1, ge=1)):
    """
    ブロックリストを取得
    
    Args:
        limit: 取得するブロック数
        page: ページ番号
    """
    start = (page - 1) * limit
    end = start + limit
    
    blocks = [block.to_dict() for block in blockchain.chain[start:end]]
    blocks.reverse()  # 最新のブロックを先頭に
    
    return {
        "blocks": blocks,
        "total": len(blockchain.chain),
        "page": page,
        "limit": limit
    }

@app.get("/blocks/{block_id}", response_model=Dict[str, Any])
async def get_block(block_id: str = Path(..., description="ブロック番号またはハッシュ")):
    """
    ブロック詳細を取得
    
    Args:
        block_id: ブロック番号またはハッシュ
    """
    # ブロック番号の場合
    if block_id.isdigit():
        block = blockchain.get_block_by_number(int(block_id))
    else:
        # ハッシュの場合
        block = blockchain.get_block_by_hash(block_id)
    
    if not block:
        raise HTTPException(status_code=404, detail=f"Block {block_id} not found")
    
    return {"block": block.to_dict()}

@app.get("/blocks/{block_id}/transactions", response_model=Dict[str, Any])
async def get_block_transactions(
    block_id: str = Path(..., description="ブロック番号またはハッシュ"),
    limit: int = Query(10, ge=1, le=100),
    page: int = Query(1, ge=1)
):
    """
    ブロック内のトランザクションを取得
    
    Args:
        block_id: ブロック番号またはハッシュ
        limit: 取得するトランザクション数
        page: ページ番号
    """
    # ブロック番号の場合
    if block_id.isdigit():
        block = blockchain.get_block_by_number(int(block_id))
    else:
        # ハッシュの場合
        block = blockchain.get_block_by_hash(block_id)
    
    if not block:
        raise HTTPException(status_code=404, detail=f"Block {block_id} not found")
    
    start = (page - 1) * limit
    end = start + limit
    
    transactions = block.transactions[start:end]
    
    return {
        "transactions": transactions,
        "total": len(block.transactions),
        "page": page,
        "limit": limit
    }

# トランザクション関連エンドポイント

@app.get("/transactions", response_model=Dict[str, Any])
async def get_transactions(limit: int = Query(10, ge=1, le=100), page: int = Query(1, ge=1)):
    """
    トランザクションリストを取得
    
    Args:
        limit: 取得するトランザクション数
        page: ページ番号
    """
    # すべてのトランザクションを収集
    all_transactions = []
    
    # ペンディングトランザクション
    for tx in blockchain.pending_transactions:
        all_transactions.append(tx.to_dict())
    
    # 確認済みトランザクション（最新のブロックから順に）
    for block in reversed(blockchain.chain):
        for tx in block.transactions:
            tx_copy = tx.copy()
            tx_copy["block_number"] = block.index
            all_transactions.append(tx_copy)
    
    start = (page - 1) * limit
    end = start + limit
    
    return {
        "transactions": all_transactions[start:end],
        "total": len(all_transactions),
        "page": page,
        "limit": limit
    }

@app.get("/transactions/{tx_hash}", response_model=Dict[str, Any])
async def get_transaction(tx_hash: str = Path(..., description="トランザクションハッシュ")):
    """
    トランザクション詳細を取得
    
    Args:
        tx_hash: トランザクションハッシュ
    """
    tx = blockchain.get_transaction(tx_hash)
    
    if not tx:
        raise HTTPException(status_code=404, detail=f"Transaction {tx_hash} not found")
    
    return {"transaction": tx}

@app.post("/transactions", response_model=TransactionResponse)
async def create_transaction(tx_data: TransactionRequest = Body(...)):
    """
    新しいトランザクションを作成
    
    Args:
        tx_data: トランザクションデータ
    """
    try:
        # トランザクションデータを辞書に変換
        tx_dict = {
            "from": tx_data.from_address,
            "to": tx_data.to_address,
            "amount": tx_data.amount,
            "gas_price": tx_data.gas_price,
            "gas_limit": tx_data.gas_limit,
            "data": tx_data.data
        }
        
        # トランザクションを追加
        tx_hash = blockchain.add_transaction(tx_dict)
        
        # 自動マイニング（開発用）
        if blockchain.pending_transactions:
            blockchain.mine_pending_transactions(tx_data.from_address)
        
        return {
            "success": True,
            "transaction_hash": tx_hash
        }
    except Exception as e:
        logger.error(f"Error creating transaction: {str(e)}")
        return {
            "success": False,
            "error": str(e)
        }

# アカウント関連エンドポイント

@app.get("/accounts", response_model=Dict[str, Any])
async def get_accounts(limit: int = Query(10, ge=1, le=100), page: int = Query(1, ge=1)):
    """
    アカウントリストを取得
    
    Args:
        limit: 取得するアカウント数
        page: ページ番号
    """
    accounts = list(blockchain.accounts.values())
    accounts.sort(key=lambda x: x.balance, reverse=True)  # 残高の降順でソート
    
    start = (page - 1) * limit
    end = start + limit
    
    return {
        "accounts": [account.to_dict() for account in accounts[start:end]],
        "total": len(accounts),
        "page": page,
        "limit": limit
    }

@app.get("/accounts/{address}", response_model=Dict[str, Any])
async def get_account(address: str = Path(..., description="アカウントアドレス")):
    """
    アカウント詳細を取得
    
    Args:
        address: アカウントアドレス
    """
    account = blockchain.get_account(address)
    
    if not account:
        raise HTTPException(status_code=404, detail=f"Account {address} not found")
    
    return {"account": account.to_dict()}

@app.get("/accounts/{address}/transactions", response_model=Dict[str, Any])
async def get_account_transactions(
    address: str = Path(..., description="アカウントアドレス"),
    limit: int = Query(10, ge=1, le=100),
    page: int = Query(1, ge=1)
):
    """
    アカウントのトランザクション履歴を取得
    
    Args:
        address: アカウントアドレス
        limit: 取得するトランザクション数
        page: ページ番号
    """
    account = blockchain.get_account(address)
    
    if not account:
        raise HTTPException(status_code=404, detail=f"Account {address} not found")
    
    transactions = blockchain.get_account_transactions(address)
    
    start = (page - 1) * limit
    end = start + limit
    
    return {
        "transactions": transactions[start:end],
        "total": len(transactions),
        "page": page,
        "limit": limit
    }

@app.get("/accounts/{address}/balance", response_model=Dict[str, Any])
async def get_account_balance(address: str = Path(..., description="アカウントアドレス")):
    """
    アカウントの残高を取得
    
    Args:
        address: アカウントアドレス
    """
    account = blockchain.get_account(address)
    
    if not account:
        raise HTTPException(status_code=404, detail=f"Account {address} not found")
    
    return {
        "address": address,
        "balance": account.balance
    }

@app.post("/accounts", response_model=Dict[str, Any])
async def create_account():
    """新しいアカウントを作成"""
    try:
        account = blockchain.create_account()
        return {
            "success": True,
            "account": account.to_dict(include_private_key=True)
        }
    except Exception as e:
        logger.error(f"Error creating account: {str(e)}")
        return {
            "success": False,
            "error": str(e)
        }

# ネットワーク関連エンドポイント

@app.get("/network/status", response_model=Dict[str, Any])
async def get_network_status():
    """ネットワークステータスを取得"""
    stats = blockchain.get_network_stats()
    return stats

# メイン関数
if __name__ == "__main__":
    # 開発用サーバーの起動
    uvicorn.run("api:app", host="0.0.0.0", port=51055, reload=True)