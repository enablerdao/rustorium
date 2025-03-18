# Rustorium

Rustoriumは、Rustで実装された高性能なブロックチェーンプラットフォームです。シャーディング、Avalanche型コンセンサス、マルチVM実行環境などの先進的な機能を備えた次世代分散台帳システムです。

![Rustorium Dashboard](docs/images/dashboard.png)

## 主な特徴

- **モダンなWebインターフェース**: ダークモード対応の直感的なUI
- **高性能APIサーバー**: 外部アプリケーションとの連携
- **拡張性の高いアーキテクチャ**: 将来の機能追加に対応

## 現在の機能

- ✅ WebUI基本インターフェース
- ✅ APIサーバー基本構造
- ✅ テーマ切り替え機能
- ✅ レスポンシブデザイン
- ✅ ダッシュボードUI

## 開発中の機能

- 🔄 スタンドアロンAPIサーバーの完全実装
- 🔄 トランザクション送信機能
- 🔄 アカウント管理機能
- 🔄 ネットワーク可視化
- 🔄 スマートコントラクト管理

## 今後実装予定の機能

- ⏳ シャーディング実装
- ⏳ Avalancheコンセンサス
- ⏳ マルチVM実行環境
- ⏳ DAGベース並列処理
- ⏳ AI処理層
- ⏳ 分散ストレージ

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
./run_all.sh
```

これにより、以下のサービスが起動します：
- APIサーバー: http://localhost:51055
- WebUI: http://localhost:57620

## ドキュメント

詳細なドキュメントは以下のリンクから参照できます：

- [プロジェクト構造](docs/project-structure.md)
- [アーキテクチャ概要](docs/architecture/overview.md)
- [APIリファレンス](docs/api/reference.md)
- [インストールガイド](docs/guides/installation.md)
- [基本的な使い方](docs/guides/basic-usage.md)

## 機能詳細

- [シャーディング実装](docs/features/sharding.md)
- [Avalancheコンセンサス](docs/features/consensus.md)
- [マルチVM実行環境](docs/features/multi-vm.md)
- [DAGベース並列処理](docs/features/dag-execution.md)
- [AI処理層](docs/features/ai-layer.md)

## ライセンス

MIT