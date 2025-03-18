"""
Rustorium ブロックチェーンコアモジュール
"""

import hashlib
import json
import time
import uuid
from typing import Dict, List, Optional, Any, Union
import threading
import logging

# ロギングの設定
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(name)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

class Block:
    """ブロックチェーンのブロックを表すクラス"""
    
    def __init__(self, index: int, transactions: List[Dict], timestamp: float, previous_hash: str, 
                 validator: str = None, difficulty: int = 4):
        """
        ブロックの初期化
        
        Args:
            index: ブロック番号
            transactions: トランザクションのリスト
            timestamp: タイムスタンプ
            previous_hash: 前のブロックのハッシュ
            validator: バリデータのアドレス
            difficulty: マイニング難易度
        """
        self.index = index
        self.transactions = transactions
        self.timestamp = timestamp
        self.previous_hash = previous_hash
        self.validator = validator
        self.difficulty = difficulty
        self.nonce = 0
        self.hash = self.calculate_hash()
        self.size = len(json.dumps(self.__dict__).encode('utf-8'))
        self.gas_used = sum(tx.get('gas_used', 0) for tx in transactions) if transactions else 0
        self.gas_limit = 10000000  # 固定値（実際には動的に計算される）
        
    def calculate_hash(self) -> str:
        """ブロックのハッシュを計算"""
        block_string = json.dumps({
            "index": self.index,
            "transactions": self.transactions,
            "timestamp": self.timestamp,
            "previous_hash": self.previous_hash,
            "validator": self.validator,
            "nonce": self.nonce
        }, sort_keys=True).encode()
        
        return hashlib.sha256(block_string).hexdigest()
    
    def mine_block(self, difficulty: int = None) -> None:
        """
        ブロックをマイニング（Proof of Work）
        
        Args:
            difficulty: マイニング難易度（指定がなければself.difficultyを使用）
        """
        if difficulty is not None:
            self.difficulty = difficulty
            
        target = "0" * self.difficulty
        
        while self.hash[:self.difficulty] != target:
            self.nonce += 1
            self.hash = self.calculate_hash()
            
        logger.info(f"Block #{self.index} mined: {self.hash}")
    
    def to_dict(self) -> Dict:
        """ブロックを辞書形式に変換"""
        return {
            "number": self.index,
            "hash": self.hash,
            "parent_hash": self.previous_hash,
            "timestamp": self.timestamp,
            "transactions": [tx.get('id', '') for tx in self.transactions],
            "size": self.size,
            "gas_used": self.gas_used,
            "gas_limit": self.gas_limit,
            "difficulty": self.difficulty,
            "validator": self.validator,
            "nonce": self.nonce
        }


class Transaction:
    """トランザクションを表すクラス"""
    
    def __init__(self, sender: str, recipient: str, amount: float, fee: float = 0.0, 
                 data: str = "", nonce: int = 0, gas_price: int = 5, gas_limit: int = 21000):
        """
        トランザクションの初期化
        
        Args:
            sender: 送信者のアドレス
            recipient: 受信者のアドレス
            amount: 送金額
            fee: 手数料
            data: トランザクションデータ
            nonce: ノンス値
            gas_price: ガス価格（Gwei）
            gas_limit: ガスリミット
        """
        self.id = f"0x{uuid.uuid4().hex}"
        self.sender = sender
        self.recipient = recipient
        self.amount = amount
        self.fee = fee
        self.data = data
        self.nonce = nonce
        self.timestamp = time.time()
        self.status = "Pending"  # Pending, Confirmed, Failed
        self.block_number = None
        self.gas_price = gas_price
        self.gas_limit = gas_limit
        self.gas_used = 21000  # 基本的なトランザクションのガス使用量
        
        # データフィールドがある場合、追加のガスを使用
        if data:
            # データの長さに応じてガス使用量を増加
            self.gas_used += len(data) * 68
            
        # 実際のガス使用量はガスリミットを超えない
        self.gas_used = min(self.gas_used, gas_limit)
        
        # 手数料を計算（gas_used * gas_price）
        self.fee = (self.gas_used * self.gas_price) / 1e9  # GweiからETHに変換
    
    def sign(self, private_key: str) -> None:
        """
        トランザクションに署名
        
        Args:
            private_key: 秘密鍵
        """
        # 実際の実装では、秘密鍵を使用して署名を生成
        # ここではシンプルな実装として、署名されたことを示すフラグを設定
        self.is_signed = True
    
    def verify(self) -> bool:
        """トランザクションの検証"""
        # 実際の実装では、署名の検証などを行う
        return hasattr(self, 'is_signed') and self.is_signed
    
    def to_dict(self) -> Dict:
        """トランザクションを辞書形式に変換"""
        return {
            "id": self.id,
            "sender": self.sender,
            "recipient": self.recipient,
            "amount": self.amount,
            "fee": self.fee,
            "nonce": self.nonce,
            "timestamp": self.timestamp,
            "data": self.data,
            "status": self.status,
            "block_number": self.block_number,
            "gas_used": self.gas_used,
            "gas_price": self.gas_price,
            "gas_limit": self.gas_limit
        }


class Account:
    """アカウントを表すクラス"""
    
    def __init__(self, address: str, private_key: str = None, balance: float = 0.0, 
                 is_contract: bool = False, nonce: int = 0):
        """
        アカウントの初期化
        
        Args:
            address: アカウントのアドレス
            private_key: 秘密鍵（オプション）
            balance: 残高
            is_contract: コントラクトアカウントかどうか
            nonce: ノンス値
        """
        self.address = address
        self.private_key = private_key
        self.balance = balance
        self.is_contract = is_contract
        self.nonce = nonce
        self.transaction_count = 0
        self.last_activity = time.time()
        self.tokens = {}  # トークン残高
    
    def to_dict(self, include_private_key: bool = False) -> Dict:
        """
        アカウントを辞書形式に変換
        
        Args:
            include_private_key: 秘密鍵を含めるかどうか
        """
        result = {
            "address": self.address,
            "balance": self.balance,
            "nonce": self.nonce,
            "is_contract": self.is_contract,
            "transaction_count": self.transaction_count,
            "last_activity": self.last_activity,
            "tokens": self.tokens
        }
        
        if include_private_key and self.private_key:
            result["private_key"] = self.private_key
            
        return result


class Blockchain:
    """ブロックチェーンを表すクラス"""
    
    def __init__(self):
        """ブロックチェーンの初期化"""
        self.chain = []
        self.pending_transactions = []
        self.accounts = {}  # アドレスをキーとするアカウントの辞書
        self.lock = threading.RLock()  # スレッドセーフな操作のためのロック
        
        # ジェネシスブロックの作成
        self.create_genesis_block()
        
        # 初期アカウントの作成
        self._create_initial_accounts()
    
    def create_genesis_block(self) -> None:
        """ジェネシスブロック（最初のブロック）を作成"""
        genesis_block = Block(0, [], time.time(), "0", "0x0000000000000000000000000000000000000000")
        genesis_block.hash = genesis_block.calculate_hash()
        self.chain.append(genesis_block)
        logger.info("Genesis block created")
    
    def _create_initial_accounts(self) -> None:
        """初期アカウントを作成（開発用）"""
        # 開発用の初期アカウント
        initial_accounts = [
            {
                "address": "0x1234567890abcdef1234567890abcdef12345678",
                "private_key": "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
                "balance": 1000000
            },
            {
                "address": "0xabcdef1234567890abcdef1234567890abcdef12",
                "private_key": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
                "balance": 500000
            },
            {
                "address": "0x9876543210fedcba9876543210fedcba98765432",
                "private_key": "0xfedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210",
                "balance": 750000
            }
        ]
        
        for acc_data in initial_accounts:
            account = Account(
                address=acc_data["address"],
                private_key=acc_data["private_key"],
                balance=acc_data["balance"]
            )
            self.accounts[account.address] = account
    
    def get_latest_block(self) -> Block:
        """最新のブロックを取得"""
        return self.chain[-1]
    
    def add_transaction(self, transaction: Union[Transaction, Dict]) -> str:
        """
        トランザクションをペンディングリストに追加
        
        Args:
            transaction: トランザクションオブジェクトまたは辞書
            
        Returns:
            トランザクションID
        """
        with self.lock:
            # 辞書の場合はTransactionオブジェクトに変換
            if isinstance(transaction, dict):
                tx = Transaction(
                    sender=transaction.get("from", ""),
                    recipient=transaction.get("to", ""),
                    amount=float(transaction.get("amount", 0)),
                    data=transaction.get("data", ""),
                    gas_price=int(transaction.get("gas_price", 5)),
                    gas_limit=int(transaction.get("gas_limit", 21000))
                )
            else:
                tx = transaction
            
            # 送信者アカウントの存在確認
            if tx.sender not in self.accounts:
                raise ValueError(f"Sender account {tx.sender} does not exist")
            
            # 受信者アカウントの存在確認（存在しない場合は作成）
            if tx.recipient not in self.accounts:
                self.accounts[tx.recipient] = Account(tx.recipient, balance=0)
            
            sender_account = self.accounts[tx.sender]
            
            # 残高チェック
            if sender_account.balance < (tx.amount + tx.fee):
                raise ValueError(f"Insufficient balance: {sender_account.balance} < {tx.amount + tx.fee}")
            
            # ノンス値の設定
            tx.nonce = sender_account.nonce
            
            # ペンディングトランザクションに追加
            self.pending_transactions.append(tx)
            
            # 送信者のノンス値を増加
            sender_account.nonce += 1
            
            logger.info(f"Transaction added: {tx.id}")
            return tx.id
    
    def mine_pending_transactions(self, miner_address: str) -> Optional[Block]:
        """
        ペンディング中のトランザクションをマイニング
        
        Args:
            miner_address: マイナーのアドレス（報酬の受け取り先）
            
        Returns:
            マイニングされたブロック
        """
        with self.lock:
            if not self.pending_transactions:
                logger.info("No transactions to mine")
                return None
            
            # マイニング報酬トランザクションを追加
            reward_tx = Transaction(
                sender="0x0000000000000000000000000000000000000000",  # システムアドレス
                recipient=miner_address,
                amount=5.0,  # マイニング報酬
                fee=0
            )
            reward_tx.status = "Confirmed"
            self.pending_transactions.append(reward_tx)
            
            # 新しいブロックを作成
            new_block = Block(
                index=len(self.chain),
                transactions=[tx.to_dict() for tx in self.pending_transactions],
                timestamp=time.time(),
                previous_hash=self.get_latest_block().hash,
                validator=miner_address
            )
            
            # ブロックをマイニング
            new_block.mine_block()
            
            # ブロックをチェーンに追加
            self.chain.append(new_block)
            
            # トランザクションの処理（残高の更新など）
            for tx in self.pending_transactions:
                self._process_transaction(tx, new_block.index)
            
            # ペンディングトランザクションをクリア
            self.pending_transactions = []
            
            logger.info(f"Block #{new_block.index} mined and added to the chain")
            return new_block
    
    def _process_transaction(self, transaction: Transaction, block_number: int) -> None:
        """
        トランザクションを処理（残高の更新など）
        
        Args:
            transaction: 処理するトランザクション
            block_number: ブロック番号
        """
        # トランザクションのステータスを更新
        transaction.status = "Confirmed"
        transaction.block_number = block_number
        
        # システムアドレスからの送金（マイニング報酬など）の場合は残高チェックをスキップ
        if transaction.sender != "0x0000000000000000000000000000000000000000":
            # 送信者の残高を減少
            sender = self.accounts.get(transaction.sender)
            if sender:
                sender.balance -= (transaction.amount + transaction.fee)
                sender.transaction_count += 1
                sender.last_activity = transaction.timestamp
        
        # 受信者の残高を増加
        recipient = self.accounts.get(transaction.recipient)
        if recipient:
            recipient.balance += transaction.amount
            recipient.last_activity = transaction.timestamp
        else:
            # 受信者アカウントが存在しない場合は作成
            self.accounts[transaction.recipient] = Account(
                transaction.recipient, 
                balance=transaction.amount,
                last_activity=transaction.timestamp
            )
    
    def create_account(self) -> Account:
        """
        新しいアカウントを作成
        
        Returns:
            作成されたアカウント
        """
        with self.lock:
            # アドレスと秘密鍵を生成（実際の実装ではより安全な方法を使用）
            private_key = f"0x{uuid.uuid4().hex}{uuid.uuid4().hex}"
            address = f"0x{hashlib.sha256(private_key.encode()).hexdigest()[:40]}"
            
            # アカウントを作成
            account = Account(address, private_key)
            self.accounts[address] = account
            
            logger.info(f"New account created: {address}")
            return account
    
    def get_account(self, address: str) -> Optional[Account]:
        """
        アドレスからアカウントを取得
        
        Args:
            address: アカウントのアドレス
            
        Returns:
            アカウントオブジェクト（存在しない場合はNone）
        """
        return self.accounts.get(address)
    
    def get_account_transactions(self, address: str) -> List[Dict]:
        """
        アカウントのトランザクション履歴を取得
        
        Args:
            address: アカウントのアドレス
            
        Returns:
            トランザクションのリスト
        """
        transactions = []
        
        # チェーン内のすべてのブロックを検索
        for block in self.chain:
            for tx_dict in block.transactions:
                # 送信者または受信者がアドレスと一致するトランザクションを抽出
                if tx_dict.get('sender') == address or tx_dict.get('recipient') == address:
                    tx_dict['block_number'] = block.index
                    transactions.append(tx_dict)
        
        # ペンディングトランザクションも検索
        for tx in self.pending_transactions:
            if tx.sender == address or tx.recipient == address:
                tx_dict = tx.to_dict()
                transactions.append(tx_dict)
        
        # タイムスタンプの降順でソート
        transactions.sort(key=lambda x: x.get('timestamp', 0), reverse=True)
        
        return transactions
    
    def get_block_by_number(self, number: int) -> Optional[Block]:
        """
        ブロック番号からブロックを取得
        
        Args:
            number: ブロック番号
            
        Returns:
            ブロックオブジェクト（存在しない場合はNone）
        """
        if 0 <= number < len(self.chain):
            return self.chain[number]
        return None
    
    def get_block_by_hash(self, block_hash: str) -> Optional[Block]:
        """
        ハッシュからブロックを取得
        
        Args:
            block_hash: ブロックのハッシュ
            
        Returns:
            ブロックオブジェクト（存在しない場合はNone）
        """
        for block in self.chain:
            if block.hash == block_hash:
                return block
        return None
    
    def get_transaction(self, tx_id: str) -> Optional[Dict]:
        """
        トランザクションIDからトランザクションを取得
        
        Args:
            tx_id: トランザクションID
            
        Returns:
            トランザクション辞書（存在しない場合はNone）
        """
        # ペンディングトランザクションを検索
        for tx in self.pending_transactions:
            if tx.id == tx_id:
                return tx.to_dict()
        
        # チェーン内のすべてのブロックを検索
        for block in self.chain:
            for tx_dict in block.transactions:
                if tx_dict.get('id') == tx_id:
                    tx_dict['block_number'] = block.index
                    return tx_dict
        
        return None
    
    def is_chain_valid(self) -> bool:
        """ブロックチェーンの有効性を検証"""
        for i in range(1, len(self.chain)):
            current_block = self.chain[i]
            previous_block = self.chain[i-1]
            
            # 現在のブロックのハッシュが正しいか
            if current_block.hash != current_block.calculate_hash():
                return False
            
            # 前のブロックへの参照が正しいか
            if current_block.previous_hash != previous_block.hash:
                return False
        
        return True
    
    def get_network_stats(self) -> Dict:
        """ネットワーク統計情報を取得"""
        # 最新10ブロックの平均ブロック時間を計算
        block_times = []
        for i in range(1, min(11, len(self.chain))):
            block_times.append(self.chain[i].timestamp - self.chain[i-1].timestamp)
        
        avg_block_time = sum(block_times) / len(block_times) if block_times else 0
        
        # 最新ブロックのトランザクション数
        latest_tx_count = len(self.get_latest_block().transactions) if self.chain else 0
        
        # TPS（1秒あたりのトランザクション数）を計算
        tps = latest_tx_count / avg_block_time if avg_block_time > 0 else 0
        
        return {
            "block_count": len(self.chain),
            "latest_block": self.get_latest_block().to_dict() if self.chain else None,
            "pending_transactions": len(self.pending_transactions),
            "average_block_time": avg_block_time,
            "tps": tps,
            "account_count": len(self.accounts),
            "difficulty": self.get_latest_block().difficulty if self.chain else 0
        }


# シングルトンインスタンス
blockchain = Blockchain()