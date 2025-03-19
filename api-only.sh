#!/bin/bash

# APIサーバーのみを起動するスクリプト
echo "Rustorium APIサーバーのみを起動しています..."

# APIディレクトリに移動してサーバーを起動
cd api && cargo run