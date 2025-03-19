// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract SimpleStorage {
    uint256 private value;
    
    // 値を設定する関数
    function store(uint256 newValue) public {
        value = newValue;
    }
    
    // 値を取得する関数
    function retrieve() public view returns (uint256) {
        return value;
    }
}