# Rustorium

Rustoriumは、Rustで実装された高性能なブロックチェーンプラットフォームです。シンプルなブロックチェーン機能から始まり、将来的にはシャーディング、Avalanche型コンセンサス、マルチVM実行環境などの先進的な機能を備えた次世代分散台帳システムを目指しています。

![Rustorium Dashboard](docs/images/dashboard.png)

## 主な特徴

- **[モダンなWebインターフェース](docs/guides/basic-usage.md#webui%E3%81%AE%E4%BD%BF%E7%94%A8)**: ダークモード対応の直感的なUI
- **[高性能APIサーバー](docs/api/reference.md)**: 外部アプリケーションとの連携
- **[拡張性の高いアーキテクチャ](docs/architecture/overview.md)**: 将来の機能追加に対応

## 使い方

### 起動方法

基本的な起動:

```bash
cargo run
```

これにより、APIサーバーとフロントエンドサーバーが起動します。

- APIサーバー: http://localhost:50128
- フロントエンド: http://localhost:55560

### 起動オプション

Rustoriumは以下のコマンドラインオプションをサポートしています:

```bash
# APIサーバーのみを起動
cargo run -- --api-only

# フロントエンドのみを起動
cargo run -- --frontend-only

# 高速起動モード（最適化レベル低）
cargo run -- --fast

# リリースモードで起動（最適化レベル高）
cargo run -- --release

# ヘルプを表示
cargo run -- --help
```

これらのオプションを組み合わせることも可能です:

```bash
# APIサーバーのみをリリースモードで起動
cargo run -- --api-only --release
```

## 現在の機能

### コア機能
- ✅ [基本的なブロックチェーン実装](docs/architecture/overview.md): ブロック生成、トランザクション処理、簡易的なコンセンサス
- ✅ [アカウント管理](docs/guides/accounts.md): アカウント作成、残高管理
- ✅ [トランザクション処理](docs/guides/transactions.md): 送金処理、手数料計算
- ✅ [ブロック探索](docs/guides/blocks-explorer.md): ブロック情報の閲覧

### API機能
- ✅ [RESTful API](docs/api/reference.md): JSON形式のレスポンス
- ✅ [ブロック関連API](docs/api/reference.md#ブロック関連): ブロック一覧取得、ブロック詳細取得
- ✅ [トランザクション関連API](docs/api/reference.md#トランザクション関連): トランザクション送信、詳細取得
- ✅ [アカウント関連API](docs/api/reference.md#アカウント関連): アカウント情報取得、トランザクション履歴
- ✅ [ネットワークステータスAPI](docs/api/reference.md#ネットワーク関連): ブロックチェーンの状態取得

### フロントエンド機能
- ✅ [ダッシュボードUI](docs/guides/basic-usage.md#ダッシュボード): ブロックチェーンの概要表示
- ✅ [レスポンシブデザイン](docs/guides/basic-usage.md): モバイル対応レイアウト
- ✅ [テーマ切り替え機能](docs/guides/basic-usage.md): ライト/ダークモード

## 開発中の機能

### コア機能の拡張
- ✅ [スマートコントラクト実行](docs/guides/smart-contracts.md): 基本的なスマートコントラクト機能
- 🔄 [改良版コンセンサスアルゴリズム](docs/features/consensus.md): より効率的なブロック生成
- 🔄 [トークン規格](docs/guides/tokens.md): カスタムトークンの作成と管理

### UI/UX改善
- 🔄 [ウォレット機能強化](docs/guides/wallet.md): 秘密鍵管理、トランザクション署名
- 🔄 [ネットワーク可視化](docs/guides/basic-usage.md#ネットワーク可視化): ブロックチェーンネットワークの視覚的表現
- 🔄 [分析ダッシュボード](docs/guides/basic-usage.md#分析ダッシュボード): 詳細な統計情報

## 今後実装予定の機能

### 先進的なブロックチェーン機能
- ⏳ [シャーディング実装](docs/features/sharding.md): スケーラビリティの向上
- ⏳ [Avalancheコンセンサス](docs/features/consensus.md): 高速なファイナリティ
- ⏳ [マルチVM実行環境](docs/features/multi-vm.md): 複数の仮想マシン対応
- ⏳ [DAGベース並列処理](docs/features/dag-execution.md): トランザクション並列実行

### 革新的な機能
- ⏳ [AI処理層](docs/features/ai-layer.md): 異常検出、予測分析
- ⏳ [分散ストレージ](docs/architecture/overview.md#7-ストレージ層): 効率的なデータ保存
- ⏳ [クロスチェーン連携](docs/features/cross-chain.md): 他のブロックチェーンとの相互運用性

## クイックスタート

### 必要条件

- Rust 1.70以上
- Cargo

### 実行方法

```bash
# リポジトリをクローン
git clone https://github.com/enablerdao/rustorium.git
cd rustorium

# すべてのサービスを起動
cargo run
```

これにより、以下のサービスが起動します：
- APIサーバー: http://localhost:50128
- フロントエンド: http://localhost:55560

## ドキュメント

詳細なドキュメントは以下のリンクから参照できます：

### 基本ガイド
- [プロジェクト構造](docs/project-structure.md)
- [アーキテクチャ概要](docs/architecture/overview.md)
- [APIリファレンス](docs/api/reference.md)
- [インストールガイド](docs/guides/installation.md)
- [基本的な使い方](docs/guides/basic-usage.md)
- [サービス管理](docs/guides/service-management.md)
- [ネットワーク選択](docs/guides/network-selection.md)

### 機能ガイド
- [ブロックエクスプローラー](docs/guides/blocks-explorer.md)
- [トランザクション管理](docs/guides/transactions.md)
- [アカウント管理](docs/guides/accounts.md)
- [スマートコントラクト](docs/guides/smart-contracts.md)
- [ウォレット機能](docs/guides/wallet.md)
- [AI分析機能](docs/guides/ai-insights.md)

## 機能詳細

- [シャーディング実装](docs/features/sharding.md)
- [Avalancheコンセンサス](docs/features/consensus.md)
- [マルチVM実行環境](docs/features/multi-vm.md)
- [DAGベース並列処理](docs/features/dag-execution.md)
- [AI処理層](docs/features/ai-layer.md)

## ライセンス

MIT