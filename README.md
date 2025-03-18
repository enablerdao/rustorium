# Rustorium

Rustoriumは、Rustで実装された高性能なブロックチェーンプラットフォームです。シャーディング、Avalanche型コンセンサス、マルチVM実行環境などの先進的な機能を備えた次世代分散台帳システムです。

![Rustorium Dashboard](docs/images/dashboard.png)

## 主な特徴

- **[モダンなWebインターフェース](docs/guides/basic-usage.md#webui%E3%81%AE%E4%BD%BF%E7%94%A8)**: ダークモード対応の直感的なUI
- **[高性能APIサーバー](docs/api/reference.md)**: 外部アプリケーションとの連携
- **[拡張性の高いアーキテクチャ](docs/architecture/overview.md)**: 将来の機能追加に対応

## 現在の機能

- ✅ [WebUI基本インターフェース](docs/guides/basic-usage.md#webui%E3%81%AE%E4%BD%BF%E7%94%A8)
- ✅ [APIサーバー基本構造](docs/api/reference.md#%E7%8F%BE%E5%9C%A8%E5%AE%9F%E8%A3%85%E3%81%95%E3%82%8C%E3%81%A6%E3%81%84%E3%82%8Bapi)
- ✅ [テーマ切り替え機能](docs/guides/basic-usage.md)
- ✅ [レスポンシブデザイン](docs/guides/basic-usage.md)
- ✅ [ダッシュボードUI](docs/guides/basic-usage.md#%E3%83%80%E3%83%83%E3%82%B7%E3%83%A5%E3%83%9C%E3%83%BC%E3%83%89)

## 開発中の機能

- 🔄 [スタンドアロンAPIサーバーの完全実装](docs/api/reference.md#%E9%96%8B%E7%99%BA%E4%B8%AD%E3%81%AEapi)
- 🔄 [トランザクション送信機能](docs/guides/basic-usage.md#%E3%83%88%E3%83%A9%E3%83%B3%E3%82%B6%E3%82%AF%E3%82%B7%E3%83%A7%E3%83%B3%E3%81%AE%E9%80%81%E4%BF%A1)
- 🔄 [アカウント管理機能](docs/guides/basic-usage.md#%E3%82%A2%E3%82%AB%E3%82%A6%E3%83%B3%E3%83%88%E7%AE%A1%E7%90%86)
- 🔄 [ネットワーク可視化](docs/guides/basic-usage.md#%E3%83%8D%E3%83%83%E3%83%88%E3%83%AF%E3%83%BC%E3%82%AF%E5%8F%AF%E8%A6%96%E5%8C%96)
- 🔄 [スマートコントラクト管理](docs/guides/basic-usage.md#%E3%82%B9%E3%83%9E%E3%83%BC%E3%83%88%E3%82%B3%E3%83%B3%E3%83%88%E3%83%A9%E3%82%AF%E3%83%88%E7%AE%A1%E7%90%86)

## 今後実装予定の機能

- ⏳ [シャーディング実装](docs/features/sharding.md)
- ⏳ [Avalancheコンセンサス](docs/features/consensus.md)
- ⏳ [マルチVM実行環境](docs/features/multi-vm.md)
- ⏳ [DAGベース並列処理](docs/features/dag-execution.md)
- ⏳ [AI処理層](docs/features/ai-layer.md)
- ⏳ [分散ストレージ](docs/architecture/overview.md#7-%E3%82%B9%E3%83%88%E3%83%AC%E3%83%BC%E3%82%B8%E5%B1%A4)

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