# Rustorium プロジェクト構造

このドキュメントでは、Rustoriumプロジェクトのディレクトリ構造と各コンポーネントの役割について説明します。

## ディレクトリ構造

```
rustorium/
├── benches/                  # ベンチマークテスト
├── docs/                     # ドキュメント
│   ├── api/                  # APIドキュメント
│   ├── architecture/         # アーキテクチャドキュメント
│   ├── features/             # 機能詳細ドキュメント
│   └── guides/               # ユーザーガイド
├── src/                      # ソースコード
│   ├── ai/                   # AI処理層
│   ├── api/                  # APIサーバー
│   ├── common/               # 共通ユーティリティ
│   ├── consensus/            # コンセンサスアルゴリズム
│   ├── dag/                  # DAGベース並列処理
│   ├── gossip/               # ゴシッププロトコル
│   ├── sharding/             # シャーディング実装
│   ├── storage/              # ストレージエンジン
│   ├── vm/                   # 仮想マシン実装
│   ├── lib.rs                # ライブラリエントリポイント
│   ├── main.rs               # メインエントリポイント
│   └── node.rs               # ノード実装
├── standalone_api/           # スタンドアロンAPIサーバー
│   ├── src/                  # APIサーバーソースコード
│   └── Cargo.toml            # APIサーバー設定
├── tests/                    # 統合テスト
├── web/                      # Yewベースのフロントエンド（開発中）
│   ├── src/                  # Webフロントエンドソースコード
│   └── Cargo.toml            # Webフロントエンド設定
├── web_ui/                   # 現行WebUI実装
│   ├── src/                  # WebUIサーバーソースコード
│   ├── static/               # 静的ファイル（HTML, CSS, JS）
│   └── Cargo.toml            # WebUI設定
├── .gitignore                # Gitの除外ファイル設定
├── build_web.sh              # Webビルドスクリプト
├── Cargo.lock                # 依存関係ロックファイル
├── Cargo.toml                # プロジェクト設定
├── config.toml               # アプリケーション設定
├── README.md                 # プロジェクト概要
└── run_all.sh                # 全サービス起動スクリプト
```

## 主要コンポーネント

### src/ - コアライブラリ

Rustoriumのコア機能を実装するライブラリコードが含まれています。

#### src/ai/ - AI処理層

- **detector.rs**: 異常検出エンジン
- **predictor.rs**: 予測モデル
- **mod.rs**: AIモジュールのエントリポイント

#### src/api/ - API実装

- **handlers.rs**: APIリクエストハンドラ
- **models.rs**: APIリクエスト/レスポンスモデル
- **server.rs**: APIサーバー実装
- **standalone.rs**: スタンドアロンAPIサーバー
- **web.rs**: WebUI API連携
- **mod.rs**: APIモジュールのエントリポイント

#### src/common/ - 共通ユーティリティ

- **config.rs**: 設定管理
- **errors.rs**: エラー定義
- **types.rs**: 共通型定義
- **utils.rs**: ユーティリティ関数
- **mod.rs**: 共通モジュールのエントリポイント

#### src/consensus/ - コンセンサス実装

- **avalanche.rs**: Avalancheコンセンサスプロトコル
- **validator.rs**: バリデータ管理
- **mod.rs**: コンセンサスモジュールのエントリポイント

#### src/dag/ - DAG実装

- **executor.rs**: DAGベーストランザクション実行
- **graph.rs**: 依存関係グラフ
- **mod.rs**: DAGモジュールのエントリポイント

#### src/gossip/ - ゴシッププロトコル

- **message.rs**: メッセージ定義
- **network.rs**: ネットワーク通信
- **peer.rs**: ピア管理
- **protocol.rs**: プロトコル実装
- **mod.rs**: ゴシッププロトコルモジュールのエントリポイント

#### src/sharding/ - シャーディング

- **cross_shard.rs**: クロスシャードトランザクション
- **manager.rs**: シャード管理
- **rebalancer.rs**: シャードリバランシング
- **ring.rs**: 一貫性ハッシュリング
- **mod.rs**: シャーディングモジュールのエントリポイント

#### src/storage/ - ストレージ

- **cache.rs**: メモリキャッシュ
- **db.rs**: RocksDBバックエンド
- **init.rs**: ストレージ初期化
- **state.rs**: 状態管理
- **mod.rs**: ストレージモジュールのエントリポイント

#### src/vm/ - 仮想マシン

- **evm.rs**: Ethereum Virtual Machine実装
- **wasm.rs**: WebAssembly実装
- **executor.rs**: VM実行エンジン
- **mod.rs**: VMモジュールのエントリポイント

### standalone_api/ - スタンドアロンAPIサーバー

APIサーバーを単独で実行するためのコードが含まれています。

- **src/main.rs**: APIサーバーのエントリポイント

### web_ui/ - WebUI

ウェブインターフェースを提供するコードが含まれています。

- **src/main.rs**: WebUIサーバーのエントリポイント
- **static/**: 静的ファイル（HTML, CSS, JavaScript）
  - **index.html**: メインHTMLファイル
  - **style.css**: スタイルシート
  - **theme.css**: テーマ定義
  - **animations.css**: アニメーション定義
  - **app.js**: メインアプリケーションロジック
  - **network.js**: ネットワーク可視化
  - **analytics.js**: 分析ダッシュボード
  - **wallet.js**: ウォレット機能
  - **contracts.js**: スマートコントラクト管理
  - **ai.js**: AI分析機能
  - **components.js**: UIコンポーネント
  - **theme.js**: テーマ管理
  - **logo.svg**: ロゴ
  - **favicon.svg**: ファビコン

### web/ - Yewベースのフロントエンド（開発中）

Rust/Wasm（Yew）を使用した新しいフロントエンドの開発コードが含まれています。

### benches/ - ベンチマーク

パフォーマンス測定用のベンチマークコードが含まれています。

- **consensus_bench.rs**: コンセンサスアルゴリズムのベンチマーク
- **sharding_bench.rs**: シャーディングのベンチマーク
- **storage_bench.rs**: ストレージエンジンのベンチマーク

### tests/ - テスト

統合テストコードが含まれています。

### docs/ - ドキュメント

プロジェクトのドキュメントが含まれています。

## 実装状況

### 現在実装されている機能

- ✅ WebUI基本インターフェース
- ✅ APIサーバー基本構造
- ✅ テーマ切り替え機能
- ✅ レスポンシブデザイン
- ✅ ダッシュボードUI

### 開発中の機能

- 🔄 スタンドアロンAPIサーバーの完全実装
- 🔄 トランザクション送信機能
- 🔄 アカウント管理機能
- 🔄 ネットワーク可視化
- 🔄 スマートコントラクト管理

### 今後実装予定の機能

- ⏳ シャーディング実装
- ⏳ Avalancheコンセンサス
- ⏳ マルチVM実行環境
- ⏳ DAGベース並列処理
- ⏳ AI処理層
- ⏳ 分散ストレージ

## ビルドスクリプト

### run_all.sh

すべてのサービス（APIサーバーとWebUI）を一度に起動するスクリプトです。

### build_web.sh

WebUIをビルドするスクリプトです。