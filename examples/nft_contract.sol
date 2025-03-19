// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract SimpleNFT {
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
    
    // 総供給量
    uint256 public totalSupply;
    
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
        totalSupply++;
        
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