# 🚀 Rustorium

<div align="center">
  <img src="docs/images/rustorium_logo.png" alt="Rustorium Logo" width="300"/>
  <br/>
  <strong>次世代ブロックチェーンプラットフォーム</strong>
</div>

<br/>

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![API Docs](https://img.shields.io/badge/API-Docs-blue.svg)](docs/api/reference.md)

Rustoriumは、**Rust言語**で実装された高性能なブロックチェーンプラットフォームです。シンプルなブロックチェーン機能から始まり、将来的には**シャーディング**、**Avalanche型コンセンサス**、**マルチVM実行環境**などの先進的な機能を備えた次世代分散台帳システムを目指しています。

> 💡 **Rustorium** = Rust + Emporium（宝物庫）の造語で、Rustの力を活かした価値の集積地を意味します。

![Rustorium Dashboard](docs/images/dashboard.png)

## ✨ 主な特徴

- 🖥️ **[モダンなWebインターフェース](docs/guides/basic-usage.md#webui-の使用)**: ダークモード対応の直感的なUI
- 🔌 **[高性能APIサーバー](docs/api/reference.md)**: RESTful APIによる外部アプリケーションとの連携
- 🧩 **[拡張性の高いアーキテクチャ](docs/architecture/overview.md)**: モジュール設計による将来の機能追加に対応
- 🔒 **[堅牢なセキュリティ](docs/guides/security.md)**: 最新の暗号技術を採用

## 🚀 クイックスタート

### 📋 必要条件

- **Rust** 1.70以上
- **Cargo** (Rustのパッケージマネージャー)
- **Git** (ソースコード管理)

### 🔧 インストール

```bash
# リポジトリをクローン
git clone https://github.com/enablerdao/rustorium.git
cd rustorium

# 依存関係をインストール
cargo build
```

### 🏃‍♂️ 起動方法

基本的な起動:

```bash
cargo run
```

これにより、以下のサービスが自動的に起動します:

- 🌐 **APIサーバー**: [http://localhost:50128](http://localhost:50128)
- 🖥️ **フロントエンド**: [http://localhost:55560](http://localhost:55560)

### ⚙️ 起動オプション

Rustoriumは様々な起動オプションをサポートしています:

| オプション | 説明 | コマンド例 |
|------------|------|------------|
| `--api-only` | APIサーバーのみを起動 | `cargo run -- --api-only` |
| `--frontend-only` | フロントエンドのみを起動 | `cargo run -- --frontend-only` |
| `--fast` | 高速起動モード（最適化レベル低） | `cargo run -- --fast` |
| `--release` | リリースモード（最適化レベル高） | `cargo run -- --release` |
| `--help` | ヘルプを表示 | `cargo run -- --help` |

> 💡 **ヒント**: これらのオプションは組み合わせて使用できます。例えば:
> ```bash
> # APIサーバーのみをリリースモードで起動
> cargo run -- --api-only --release
> ```

## 💎 機能一覧

### 🧱 コア機能
| 機能 | 説明 | ステータス | ドキュメント |
|------|------|------------|--------------|
| ブロックチェーン基盤 | ブロック生成、トランザクション処理、簡易的なコンセンサス | ✅ 完了 | [詳細](docs/architecture/overview.md) |
| アカウント管理 | アカウント作成、秘密鍵管理、残高管理 | ✅ 完了 | [詳細](docs/guides/accounts.md) |
| トランザクション処理 | 送金処理、手数料計算、署名検証 | ✅ 完了 | [詳細](docs/guides/transactions.md) |
| ブロックエクスプローラー | ブロック情報の閲覧、トランザクション履歴 | ✅ 完了 | [詳細](docs/guides/blocks-explorer.md) |
| スマートコントラクト | コントラクトのデプロイと実行、トークン規格 | ✅ 完了 | [詳細](docs/guides/smart-contracts.md) |

### 🔌 API機能
| 機能 | 説明 | ステータス | ドキュメント |
|------|------|------------|--------------|
| RESTful API | JSON形式のレスポンス、CORS対応 | ✅ 完了 | [詳細](docs/api/reference.md) |
| ブロック関連API | ブロック一覧取得、ブロック詳細取得 | ✅ 完了 | [詳細](docs/api/reference.md#ブロック関連) |
| トランザクション関連API | トランザクション送信、詳細取得 | ✅ 完了 | [詳細](docs/api/reference.md#トランザクション関連) |
| アカウント関連API | アカウント情報取得、トランザクション履歴 | ✅ 完了 | [詳細](docs/api/reference.md#アカウント関連) |
| コントラクト関連API | コントラクトデプロイ、呼び出し、検証 | ✅ 完了 | [詳細](docs/api/reference.md#コントラクト関連) |
| ネットワークステータスAPI | ブロックチェーンの状態取得 | ✅ 完了 | [詳細](docs/api/reference.md#ネットワーク関連) |

### 🖥️ フロントエンド機能
| 機能 | 説明 | ステータス | ドキュメント |
|------|------|------------|--------------|
| ダッシュボードUI | ブロックチェーンの概要表示、リアルタイム更新 | ✅ 完了 | [詳細](docs/guides/basic-usage.md#ダッシュボード) |
| レスポンシブデザイン | モバイル対応レイアウト、タブレット対応 | ✅ 完了 | [詳細](docs/guides/basic-usage.md#レスポンシブデザイン) |
| テーマ切り替え | ライト/ダークモード、カスタムテーマ | ✅ 完了 | [詳細](docs/guides/basic-usage.md#テーマ切り替え) |
| ウォレット連携 | ウォレット接続、トランザクション署名 | 🔄 開発中 | [詳細](docs/guides/wallet.md) |

## 🔮 ロードマップ

### 🔄 開発中の機能
| 機能 | 説明 | 完了予定 | 詳細 |
|------|------|----------|------|
| 改良版コンセンサス | Proof of Stakeベースの効率的なブロック生成 | 2025年Q2 | [詳細](docs/features/consensus.md) |
| トークン規格 | ERC-20/ERC-721互換のトークン規格 | 2025年Q1 | [詳細](docs/guides/tokens.md) |
| ウォレット機能強化 | 秘密鍵管理、トランザクション署名、マルチシグ | 2025年Q1 | [詳細](docs/guides/wallet.md) |
| ネットワーク可視化 | ブロックチェーンネットワークの視覚的表現 | 2025年Q2 | [詳細](docs/guides/basic-usage.md#ネットワーク可視化) |
| 分析ダッシュボード | リアルタイム統計、トレンド分析 | 2025年Q2 | [詳細](docs/guides/basic-usage.md#分析ダッシュボード) |

### 🔍 研究開発中の先進機能
| 機能 | 説明 | 研究段階 | 詳細 |
|------|------|----------|------|
| シャーディング | 水平スケーリングによるスループット向上 | 設計段階 | [詳細](docs/features/sharding.md) |
| Avalancheコンセンサス | 高速なファイナリティを実現する確率的合意形成 | プロトタイプ | [詳細](docs/features/consensus.md#avalanche) |
| マルチVM実行環境 | EVM、WASM、Move VMなど複数の仮想マシン対応 | 研究段階 | [詳細](docs/features/multi-vm.md) |
| DAGベース並列処理 | 依存関係グラフによるトランザクション並列実行 | 設計段階 | [詳細](docs/features/dag-execution.md) |

### 🚀 将来のビジョン
| 機能 | 説明 | 予定 | 詳細 |
|------|------|------|------|
| AI処理層 | 異常検出、予測分析、最適化 | 2026年~ | [詳細](docs/features/ai-layer.md) |
| 分散ストレージ | IPFS連携、効率的なデータ保存 | 2025年Q4 | [詳細](docs/architecture/overview.md#7-ストレージ層) |
| クロスチェーン連携 | IBC対応、他のブロックチェーンとの相互運用性 | 2026年~ | [詳細](docs/features/cross-chain.md) |
| ゼロ知識証明 | プライバシー保護、スケーラビリティ向上 | 研究段階 | [詳細](docs/features/zero-knowledge.md) |

> 📅 **注意**: ロードマップは開発の進捗や優先順位の変更により調整される場合があります。

## 📚 ドキュメント

Rustoriumは包括的なドキュメントを提供しています。以下のリンクから詳細情報にアクセスできます：

### 📘 開発者ガイド

| カテゴリ | 説明 | リンク |
|---------|------|--------|
| プロジェクト構造 | ディレクトリ構成、モジュール関係 | [詳細](docs/project-structure.md) |
| アーキテクチャ | システム設計、コンポーネント構成 | [詳細](docs/architecture/overview.md) |
| API仕様 | エンドポイント、リクエスト/レスポンス形式 | [詳細](docs/api/reference.md) |
| 開発環境構築 | 環境設定、依存関係 | [詳細](docs/guides/installation.md) |
| コントリビューション | コーディング規約、PR手順 | [詳細](docs/contributing.md) |

### 🛠️ 機能ガイド

| 機能 | 説明 | リンク |
|------|------|--------|
| ブロックエクスプローラー | ブロック・トランザクション検索 | [詳細](docs/guides/blocks-explorer.md) |
| トランザクション | 送金、手数料、署名 | [詳細](docs/guides/transactions.md) |
| アカウント管理 | アドレス生成、残高管理 | [詳細](docs/guides/accounts.md) |
| スマートコントラクト | デプロイ、実行、デバッグ | [詳細](docs/guides/smart-contracts.md) |
| ウォレット連携 | ウォレット接続、署名 | [詳細](docs/guides/wallet.md) |
| トークン規格 | ERC-20/ERC-721互換実装 | [詳細](docs/guides/tokens.md) |

### 🔬 先進機能解説

| 機能 | 説明 | リンク |
|------|------|--------|
| シャーディング | 水平スケーリング技術 | [詳細](docs/features/sharding.md) |
| Avalancheコンセンサス | 確率的合意形成アルゴリズム | [詳細](docs/features/consensus.md) |
| マルチVM | 複数実行環境のサポート | [詳細](docs/features/multi-vm.md) |
| DAG並列処理 | 依存関係グラフによる並列実行 | [詳細](docs/features/dag-execution.md) |
| AI処理層 | 機械学習による最適化 | [詳細](docs/features/ai-layer.md) |
| ゼロ知識証明 | プライバシー保護技術 | [詳細](docs/features/zero-knowledge.md) |

## 🧪 テスト

Rustoriumは包括的なテストスイートを提供しています：

```bash
# 単体テストを実行
cargo test

# 統合テストを実行
cargo test --test '*'

# 特定のテストを実行
cargo test --package api --test blockchain_test
```

## 🔧 開発者向け情報

### コードスタイル

```bash
# コードフォーマットを適用
cargo fmt

# リントチェックを実行
cargo clippy
```

### パフォーマンス最適化

```bash
# ベンチマークを実行
cargo bench

# フレームグラフを生成（flamegraphツールが必要）
cargo flamegraph
```

## 📊 パフォーマンス指標

| 指標 | 値 | 備考 |
|------|-----|------|
| トランザクション処理速度 | ~1,000 tx/秒 | 単一ノード環境 |
| ブロック生成時間 | 5秒 | 標準設定 |
| ストレージ効率 | ~500 tx/MB | 圧縮後 |
| P2P通信レイテンシ | <100ms | 地理的に近接したノード間 |

## 📜 ライセンス

Rustoriumは[MIT](LICENSE)ライセンスの下で公開されています。

---

<div align="center">
  <p>🌟 Rustoriumにスターを付けて応援してください！ 🌟</p>
  <p>質問やフィードバックは<a href="https://github.com/enablerdao/rustorium/issues">GitHub Issues</a>までお気軽に。</p>
</div>