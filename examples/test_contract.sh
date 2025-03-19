#!/bin/bash

# APIサーバーのベースURL
API_URL="http://localhost:50128"

# アカウント情報の取得
echo "アカウント情報の取得中..."
ACCOUNTS=$(curl -s $API_URL/accounts)
ACCOUNT=$(echo $ACCOUNTS | jq -r '.data[0].address')
echo "使用するアカウント: $ACCOUNT"

# コントラクトのバイトコードとABIの読み込み
BYTECODE=$(cat simple_storage.bytecode.txt)
ABI=$(cat simple_storage.abi.json)

# コントラクトのデプロイ
echo "コントラクトをデプロイ中..."
DEPLOY_RESPONSE=$(curl -s -X POST $API_URL/contracts \
  -H "Content-Type: application/json" \
  -d "{
    \"from\": \"$ACCOUNT\",
    \"bytecode\": \"$BYTECODE\",
    \"abi\": \"$(jq -c . simple_storage.abi.json | sed 's/"/\\"/g')\",
    \"gas_limit\": 1000000,
    \"gas_price\": 10
  }")

echo "デプロイレスポンス: $DEPLOY_RESPONSE"
# 文字列からJSONを抽出して解析
RESPONSE_JSON=$(echo $DEPLOY_RESPONSE | jq -r '.data' | sed 's/\\n//g' | sed 's/\\//g')
echo "整形したレスポンス: $RESPONSE_JSON"
CONTRACT_ADDRESS=$(echo $RESPONSE_JSON | jq -r '.address')
echo "デプロイされたコントラクトアドレス: $CONTRACT_ADDRESS"

# 値の保存
echo "コントラクトに値を保存中..."
STORE_RESPONSE=$(curl -s -X POST $API_URL/contracts/$CONTRACT_ADDRESS/call \
  -H "Content-Type: application/json" \
  -d "{
    \"from\": \"$ACCOUNT\",
    \"method\": \"store\",
    \"args\": \"42\",
    \"gas_limit\": 100000,
    \"gas_price\": 10,
    \"value\": 0
  }")

echo "保存結果: $(echo $STORE_RESPONSE | jq .)"

# 値の取得
echo "コントラクトから値を取得中..."
RETRIEVE_RESPONSE=$(curl -s -X POST $API_URL/contracts/$CONTRACT_ADDRESS/call \
  -H "Content-Type: application/json" \
  -d "{
    \"from\": \"$ACCOUNT\",
    \"method\": \"retrieve\",
    \"gas_limit\": 100000,
    \"gas_price\": 10,
    \"value\": 0
  }")

echo "取得結果: $(echo $RETRIEVE_RESPONSE | jq .)"

# コントラクト情報の取得
echo "コントラクト情報の取得中..."
CONTRACT_INFO=$(curl -s $API_URL/contracts/$CONTRACT_ADDRESS)
echo "コントラクト情報: $(echo $CONTRACT_INFO | jq .)"