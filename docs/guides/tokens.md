# 🪙 トークン規格ガイド

Rustoriumは、ブロックチェーン上でのトークン作成と管理のための包括的な規格を提供します。このガイドでは、Rustoriumのトークン規格の概要と使用方法について説明します。

## 📋 対応トークン規格

Rustoriumは以下のトークン規格をサポートしています：

| 規格 | 種類 | 互換性 | ステータス |
|------|------|--------|----------|
| RTS-20 | 代替可能トークン | ERC-20互換 | ✅ 実装済み |
| RTS-721 | 非代替トークン(NFT) | ERC-721互換 | ✅ 実装済み |
| RTS-1155 | マルチトークン | ERC-1155互換 | 🔄 開発中 |

## 🚀 トークンの作成

### RTS-20トークンの作成

RTS-20トークンは、通貨やポイントなどの代替可能なトークンに適しています。

#### APIを使用した作成

```bash
curl -X POST http://localhost:50128/contracts/token/create \
  -H "Content-Type: application/json" \
  -d '{
    "from": "0xYOUR_ACCOUNT_ADDRESS",
    "bytecode": "0x...",
    "gas_limit": 1000000,
    "gas_price": 10,
    "token_name": "My Token",
    "token_symbol": "MTK",
    "token_decimals": 18,
    "token_total_supply": 1000000
  }'
```

#### Solidity実装例

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract RTS20Token {
    string public name;
    string public symbol;
    uint8 public decimals;
    uint256 public totalSupply;
    
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    
    constructor(
        string memory _name,
        string memory _symbol,
        uint8 _decimals,
        uint256 _initialSupply
    ) {
        name = _name;
        symbol = _symbol;
        decimals = _decimals;
        totalSupply = _initialSupply * 10**uint256(_decimals);
        balanceOf[msg.sender] = totalSupply;
        emit Transfer(address(0), msg.sender, totalSupply);
    }
    
    function transfer(address _to, uint256 _value) public returns (bool success) {
        require(balanceOf[msg.sender] >= _value, "Insufficient balance");
        balanceOf[msg.sender] -= _value;
        balanceOf[_to] += _value;
        emit Transfer(msg.sender, _to, _value);
        return true;
    }
    
    function approve(address _spender, uint256 _value) public returns (bool success) {
        allowance[msg.sender][_spender] = _value;
        emit Approval(msg.sender, _spender, _value);
        return true;
    }
    
    function transferFrom(address _from, address _to, uint256 _value) public returns (bool success) {
        require(balanceOf[_from] >= _value, "Insufficient balance");
        require(allowance[_from][msg.sender] >= _value, "Insufficient allowance");
        balanceOf[_from] -= _value;
        balanceOf[_to] += _value;
        allowance[_from][msg.sender] -= _value;
        emit Transfer(_from, _to, _value);
        return true;
    }
}
```

### RTS-721トークン（NFT）の作成

RTS-721トークンは、アート、コレクティブル、ゲーム内アイテムなどの非代替トークンに適しています。

#### APIを使用した作成

```bash
curl -X POST http://localhost:50128/contracts/nft/create \
  -H "Content-Type: application/json" \
  -d '{
    "from": "0xYOUR_ACCOUNT_ADDRESS",
    "bytecode": "0x...",
    "gas_limit": 1000000,
    "gas_price": 10,
    "token_name": "My NFT Collection",
    "token_symbol": "MNFT"
  }'
```

#### Solidity実装例

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract RTS721Token {
    string public name;
    string public symbol;
    
    // トークンID => 所有者アドレス
    mapping(uint256 => address) public ownerOf;
    // 所有者アドレス => 所有トークン数
    mapping(address => uint256) public balanceOf;
    // トークンID => 承認されたアドレス
    mapping(uint256 => address) public getApproved;
    // 所有者 => オペレーター => 承認状態
    mapping(address => mapping(address => bool)) public isApprovedForAll;
    
    // イベント
    event Transfer(address indexed from, address indexed to, uint256 indexed tokenId);
    event Approval(address indexed owner, address indexed approved, uint256 indexed tokenId);
    event ApprovalForAll(address indexed owner, address indexed operator, bool approved);
    
    constructor(string memory _name, string memory _symbol) {
        name = _name;
        symbol = _symbol;
    }
    
    // トークンの発行
    function mint(address _to, uint256 _tokenId) public {
        require(_to != address(0), "Invalid recipient");
        require(ownerOf[_tokenId] == address(0), "Token already exists");
        
        ownerOf[_tokenId] = _to;
        balanceOf[_to]++;
        
        emit Transfer(address(0), _to, _tokenId);
    }
    
    // トークンの転送
    function transferFrom(address _from, address _to, uint256 _tokenId) public {
        require(_isApprovedOrOwner(msg.sender, _tokenId), "Not approved or owner");
        require(ownerOf[_tokenId] == _from, "Not the owner");
        require(_to != address(0), "Invalid recipient");
        
        ownerOf[_tokenId] = _to;
        balanceOf[_from]--;
        balanceOf[_to]++;
        
        if (getApproved[_tokenId] != address(0)) {
            delete getApproved[_tokenId];
        }
        
        emit Transfer(_from, _to, _tokenId);
    }
    
    // トークンの承認
    function approve(address _approved, uint256 _tokenId) public {
        address owner = ownerOf[_tokenId];
        require(msg.sender == owner || isApprovedForAll[owner][msg.sender], "Not owner or approved operator");
        
        getApproved[_tokenId] = _approved;
        emit Approval(owner, _approved, _tokenId);
    }
    
    // オペレーターの承認
    function setApprovalForAll(address _operator, bool _approved) public {
        isApprovedForAll[msg.sender][_operator] = _approved;
        emit ApprovalForAll(msg.sender, _operator, _approved);
    }
    
    // 承認またはオーナーかどうかを確認
    function _isApprovedOrOwner(address _spender, uint256 _tokenId) internal view returns (bool) {
        address owner = ownerOf[_tokenId];
        return (_spender == owner || getApproved[_tokenId] == _spender || isApprovedForAll[owner][_spender]);
    }
}
```

## 🔍 トークンの操作

### トークン情報の取得

```bash
# トークン名の取得
curl -X POST http://localhost:50128/contracts/YOUR_TOKEN_ADDRESS/call \
  -H "Content-Type: application/json" \
  -d '{
    "from": "0xYOUR_ACCOUNT_ADDRESS",
    "method": "name",
    "gas_limit": 100000,
    "gas_price": 10,
    "value": 0
  }'

# トークンシンボルの取得
curl -X POST http://localhost:50128/contracts/YOUR_TOKEN_ADDRESS/call \
  -H "Content-Type: application/json" \
  -d '{
    "from": "0xYOUR_ACCOUNT_ADDRESS",
    "method": "symbol",
    "gas_limit": 100000,
    "gas_price": 10,
    "value": 0
  }'

# 残高の取得
curl -X POST http://localhost:50128/contracts/YOUR_TOKEN_ADDRESS/call \
  -H "Content-Type: application/json" \
  -d '{
    "from": "0xYOUR_ACCOUNT_ADDRESS",
    "method": "balanceOf",
    "args": "0xTARGET_ADDRESS",
    "gas_limit": 100000,
    "gas_price": 10,
    "value": 0
  }'
```

### トークンの転送

```bash
# RTS-20トークンの転送
curl -X POST http://localhost:50128/contracts/YOUR_TOKEN_ADDRESS/call \
  -H "Content-Type: application/json" \
  -d '{
    "from": "0xYOUR_ACCOUNT_ADDRESS",
    "method": "transfer",
    "args": "0xRECIPIENT_ADDRESS,1000",
    "gas_limit": 100000,
    "gas_price": 10,
    "value": 0
  }'

# NFTの転送
curl -X POST http://localhost:50128/contracts/YOUR_NFT_ADDRESS/call \
  -H "Content-Type: application/json" \
  -d '{
    "from": "0xYOUR_ACCOUNT_ADDRESS",
    "method": "transferFrom",
    "args": "0xYOUR_ACCOUNT_ADDRESS,0xRECIPIENT_ADDRESS,1",
    "gas_limit": 100000,
    "gas_price": 10,
    "value": 0
  }'
```

## 🧩 トークン規格の拡張

Rustoriumのトークン規格は拡張可能で、以下のような追加機能を実装できます：

### メタデータ拡張

NFTにメタデータを追加する例：

```solidity
// トークンIDからメタデータURIへのマッピング
mapping(uint256 => string) private _tokenURIs;

// メタデータURIの設定
function setTokenURI(uint256 tokenId, string memory uri) public {
    require(ownerOf[tokenId] == msg.sender, "Not the owner");
    _tokenURIs[tokenId] = uri;
}

// メタデータURIの取得
function tokenURI(uint256 tokenId) public view returns (string memory) {
    require(ownerOf[tokenId] != address(0), "Token does not exist");
    return _tokenURIs[tokenId];
}
```

### バーナブルトークン

トークンを破棄する機能：

```solidity
// トークンの破棄
function burn(uint256 amount) public {
    require(balanceOf[msg.sender] >= amount, "Insufficient balance");
    balanceOf[msg.sender] -= amount;
    totalSupply -= amount;
    emit Transfer(msg.sender, address(0), amount);
}
```

## 📊 トークンエコノミクス設計

効果的なトークンエコノミクスを設計するためのヒント：

1. **明確な用途**: トークンの用途と価値提案を明確にする
2. **供給メカニズム**: 固定供給、インフレーション、デフレーションなど
3. **分配方法**: 初期配布、マイニング、ステーキングなど
4. **インセンティブ設計**: ネットワーク参加者へのインセンティブ
5. **ガバナンス**: トークン保有者の意思決定への参加方法

## 🔒 セキュリティのベストプラクティス

トークン実装におけるセキュリティのベストプラクティス：

1. **整数オーバーフロー対策**: SafeMathライブラリの使用
2. **リエントランシー対策**: 状態変更を先に行う
3. **アクセス制御**: 適切な権限チェック
4. **イベント発行**: すべての重要な操作に対するイベント発行
5. **コード監査**: デプロイ前の専門家によるレビュー

## 📚 関連リソース

- [ERC-20仕様](https://eips.ethereum.org/EIPS/eip-20)
- [ERC-721仕様](https://eips.ethereum.org/EIPS/eip-721)
- [ERC-1155仕様](https://eips.ethereum.org/EIPS/eip-1155)
- [トークンエコノミクス設計ガイド](https://docs.rustorium.example.com/tokenomics)

---

質問やフィードバックがありましたら、[GitHub Issues](https://github.com/enablerdao/rustorium/issues)でお気軽にお問い合わせください。