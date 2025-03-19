#!/bin/bash

# 高速起動用スクリプト
echo "Rustoriumを高速モードで起動しています..."

# fast-devプロファイルを使用してビルド
cargo build --profile fast-dev

# 直接バイナリを実行（cargo runを使わない）
./target/fast-dev/rustorium