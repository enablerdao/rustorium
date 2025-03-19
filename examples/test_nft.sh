#!/bin/bash

# APIサーバーのベースURL
API_URL="http://localhost:50128"

# アカウント情報の取得
echo "アカウント情報の取得中..."
ACCOUNTS=$(curl -s $API_URL/accounts)
ACCOUNT=$(echo $ACCOUNTS | jq -r '.data[0].address')
echo "使用するアカウント: $ACCOUNT"

# NFTコントラクトのデプロイ
echo "NFTコントラクトをデプロイ中..."
DEPLOY_RESPONSE=$(curl -s -X POST $API_URL/contracts/nft/create \
  -H "Content-Type: application/json" \
  -d "{
    \"from\": \"$ACCOUNT\",
    \"bytecode\": \"608060405234801561001057600080fd5b50610150806100206000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c80632e64cec11461003b5780636057361d14610059575b600080fd5b610043610075565b60405161005091906100a1565b60405180910390f35b610073600480360381019061006e91906100ed565b61007e565b005b60008054905090565b8060008190555050565b6000819050919050565b61009b81610088565b82525050565b60006020820190506100b66000830184610092565b92915050565b600080fd5b6100ca81610088565b81146100d557600080fd5b50565b6000813590506100e7816100c1565b92915050565b600060208284031215610103576101026100bc565b5b6000610111848285016100d8565b9150509291505056fea2646970667358221220ec5ef79ea9c3f806626466c24f736c1a5a5e3b8bc2fb7c814fa4ecc6ff3a9c4d64736f6c63430008120033\",
    \"gas_limit\": 1000000,
    \"gas_price\": 10,
    \"token_name\": \"Test NFT\",
    \"token_symbol\": \"TNFT\"
  }")

echo "デプロイレスポンス: $DEPLOY_RESPONSE"
CONTRACT_ADDRESS=$(echo $DEPLOY_RESPONSE | jq -r '.data' | jq -r 'fromjson | .address')
echo "デプロイされたNFTコントラクトアドレス: $CONTRACT_ADDRESS"

# NFT名の取得
echo "NFT名を取得中..."
NAME_RESPONSE=$(curl -s -X POST $API_URL/contracts/$CONTRACT_ADDRESS/call \
  -H "Content-Type: application/json" \
  -d "{
    \"from\": \"$ACCOUNT\",
    \"method\": \"name\",
    \"gas_limit\": 100000,
    \"gas_price\": 10,
    \"value\": 0
  }")

echo "NFT名: $(echo $NAME_RESPONSE | jq -r '.data.result')"

# NFTシンボルの取得
echo "NFTシンボルを取得中..."
SYMBOL_RESPONSE=$(curl -s -X POST $API_URL/contracts/$CONTRACT_ADDRESS/call \
  -H "Content-Type: application/json" \
  -d "{
    \"from\": \"$ACCOUNT\",
    \"method\": \"symbol\",
    \"gas_limit\": 100000,
    \"gas_price\": 10,
    \"value\": 0
  }")

echo "NFTシンボル: $(echo $SYMBOL_RESPONSE | jq -r '.data.result')"

# NFTの発行
echo "NFTを発行中..."
MINT_RESPONSE=$(curl -s -X POST $API_URL/contracts/$CONTRACT_ADDRESS/call \
  -H "Content-Type: application/json" \
  -d "{
    \"from\": \"$ACCOUNT\",
    \"method\": \"mint\",
    \"args\": \"1\",
    \"gas_limit\": 100000,
    \"gas_price\": 10,
    \"value\": 0,
    \"debug_mode\": true
  }")

echo "発行結果: $(echo $MINT_RESPONSE | jq .)"

# 所有者の確認
echo "NFTの所有者を確認中..."
OWNER_RESPONSE=$(curl -s -X POST $API_URL/contracts/$CONTRACT_ADDRESS/call \
  -H "Content-Type: application/json" \
  -d "{
    \"from\": \"$ACCOUNT\",
    \"method\": \"ownerOf\",
    \"args\": \"1\",
    \"gas_limit\": 100000,
    \"gas_price\": 10,
    \"value\": 0
  }")

echo "トークンID 1の所有者: $(echo $OWNER_RESPONSE | jq -r '.data.result')"

# 残高の確認
echo "NFT残高を確認中..."
BALANCE_RESPONSE=$(curl -s -X POST $API_URL/contracts/$CONTRACT_ADDRESS/call \
  -H "Content-Type: application/json" \
  -d "{
    \"from\": \"$ACCOUNT\",
    \"method\": \"balanceOf\",
    \"args\": \"$ACCOUNT\",
    \"gas_limit\": 100000,
    \"gas_price\": 10,
    \"value\": 0
  }")

echo "アカウントの残高: $(echo $BALANCE_RESPONSE | jq -r '.data.result')"

# 別のアカウントを作成
echo "別のアカウントを作成中..."
NEW_ACCOUNT_RESPONSE=$(curl -s -X POST $API_URL/accounts)
NEW_ACCOUNT=$(echo $NEW_ACCOUNT_RESPONSE | jq -r '.data.address')
echo "新しいアカウント: $NEW_ACCOUNT"

# NFTの転送
echo "NFTを転送中..."
TRANSFER_RESPONSE=$(curl -s -X POST $API_URL/contracts/$CONTRACT_ADDRESS/call \
  -H "Content-Type: application/json" \
  -d "{
    \"from\": \"$ACCOUNT\",
    \"method\": \"transferFrom\",
    \"args\": \"$ACCOUNT,$NEW_ACCOUNT,1\",
    \"gas_limit\": 100000,
    \"gas_price\": 10,
    \"value\": 0,
    \"debug_mode\": true
  }")

echo "転送結果: $(echo $TRANSFER_RESPONSE | jq .)"

# 転送後の所有者確認
echo "転送後のNFT所有者を確認中..."
OWNER_AFTER_RESPONSE=$(curl -s -X POST $API_URL/contracts/$CONTRACT_ADDRESS/call \
  -H "Content-Type: application/json" \
  -d "{
    \"from\": \"$ACCOUNT\",
    \"method\": \"ownerOf\",
    \"args\": \"1\",
    \"gas_limit\": 100000,
    \"gas_price\": 10,
    \"value\": 0
  }")

echo "転送後のトークンID 1の所有者: $(echo $OWNER_AFTER_RESPONSE | jq -r '.data.result')"

# コントラクトのイベントを取得
echo "コントラクトのイベントを取得中..."
EVENTS_RESPONSE=$(curl -s $API_URL/contracts/$CONTRACT_ADDRESS/events)
echo "イベント: $(echo $EVENTS_RESPONSE | jq .)"