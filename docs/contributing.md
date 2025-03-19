# 🤝 コントリビューションガイド

Rustoriumプロジェクトへの貢献に興味をお持ちいただき、ありがとうございます！このドキュメントでは、プロジェクトに貢献するための手順とガイドラインを説明します。

## 📋 貢献の種類

Rustoriumプロジェクトには、様々な方法で貢献できます：

- 🐛 **バグ報告**: 問題を発見したら、詳細な再現手順を含めてIssueを作成してください
- 💡 **機能提案**: 新機能のアイデアがあれば、Issueで提案してください
- 📝 **ドキュメント**: ドキュメントの改善や翻訳
- 🧪 **テスト**: テストカバレッジの向上
- 💻 **コード**: 機能実装やバグ修正

## 🚀 開発環境のセットアップ

### 前提条件

- Rust 1.70以上
- Cargo
- Git

### 環境構築手順

```bash
# リポジトリをフォークしてクローン
git clone https://github.com/YOUR_USERNAME/rustorium.git
cd rustorium

# 必要なシステム依存関係をインストール
# Debian/Ubuntu
apt-get install -y clang libclang-dev librocksdb-dev

# macOS
brew install rocksdb llvm

# 依存関係をインストール
cargo build

# テストを実行して環境が正しく設定されていることを確認
cargo test
```

### 開発モード

Rustoriumには開発用のテストノード機能が組み込まれています。この機能を使用すると、複数のノードを同時に起動してテストできます。

#### テストノードの起動
```bash
# デフォルト設定（10ノード）で起動
cargo run -- --dev

# カスタム設定で起動
cargo run -- --dev --nodes 5 --base-port 50000 --data-dir /path/to/data
```

#### 開発モードの機能
- 複数のノードインスタンス
- 自動ピア発見
- 各ノードの個別データディレクトリ
- 個別のAPIとフロントエンドエンドポイント
- 自動ポート割り当て

#### ノード情報
開発モードでは、各ノードについて以下の情報が表示されます：
- API URL: `http://localhost:<port>`
- フロントエンドURL: `http://localhost:<port>`
- P2Pアドレス: `/ip4/127.0.0.1/tcp/<port>`
- Peer ID

#### ポート割り当て
ポートは各ノードに対して順番に割り当てられます：
- ノード1: base_port, base_port+1, base_port+2
- ノード2: base_port+3, base_port+4, base_port+5
- 以降同様

### アーキテクチャ

Rustoriumは以下の主要コンポーネントで構成されています：

#### 1. DAGエンジン
- トランザクショングラフ管理
- 依存関係追跡
- 並列実行スケジューラ
- トポロジカルソート

#### 2. Avalancheプロトコル
- サンプリングベースの投票システム
- コンフィデンス追跡
- メタスタビリティ検出
- 非同期ネットワーク対応

#### 3. シャーディングマネージャ
- 動的シャード割り当て
- クロスシャードトランザクション
- 2フェーズコミットプロトコル
- ステート同期

#### 4. P2Pネットワーク
- libp2pベースのネットワーキング
- Gossipsubによるメッセージ伝播
- Kademliaによるピア発見
- mDNSによるローカルピア発見

#### 5. ストレージレイヤー
- RocksDBベースの永続化
- カラムファミリーによるデータ分離
- Snappyによる圧縮
- アトミックなバッチ操作

#### ディレクトリ構造
```
rustorium/
├── src/
│   ├── core/           # コアブロックチェーンコンポーネント
│   │   ├── dag.rs      # DAG実装
│   │   ├── avalanche.rs # コンセンサスプロトコル
│   │   └── sharding.rs # シャーディング管理
│   ├── network/        # P2Pネットワーキング
│   ├── storage/        # 永続化レイヤー
│   └── dev/           # 開発ツール
├── api/               # APIサーバー
├── frontend/         # フロントエンドアプリケーション
└── docs/            # ドキュメント
```

## 🌿 ブランチ戦略

- `main`: 安定版ブランチ
- `develop`: 開発ブランチ（最新の変更を含む）
- `feature/*`: 新機能の開発用
- `bugfix/*`: バグ修正用
- `docs/*`: ドキュメント更新用

```bash
# 新機能の開発を始める場合
git checkout develop
git pull
git checkout -b feature/your-feature-name
```

## 📝 コーディング規約

### Rustスタイルガイド

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) に従ってください
- `rustfmt` と `clippy` を使用してコードの品質を確保してください

```bash
# コードフォーマット
cargo fmt

# リントチェック
cargo clippy --all-targets --all-features -- -D warnings
```

### コミットメッセージ

コミットメッセージは以下の形式に従ってください：

```
<type>(<scope>): <subject>

<body>

<footer>
```

例：
```
feat(api): トークン転送APIの実装

- 新しいエンドポイント `/api/tokens/transfer` を追加
- トランザクション署名の検証を実装
- テストケースを追加

Closes #123
```

タイプ:
- `feat`: 新機能
- `fix`: バグ修正
- `docs`: ドキュメントのみの変更
- `style`: コードの意味に影響しない変更（空白、フォーマットなど）
- `refactor`: バグ修正でも機能追加でもないコード変更
- `perf`: パフォーマンス向上のための変更
- `test`: テストの追加・修正
- `chore`: ビルドプロセスやツールの変更

## 🔍 プルリクエストのプロセス

1. 作業ブランチで変更を行い、コミットします
2. 最新の `develop` ブランチからリベースします
3. テストが通ることを確認します
4. プルリクエストを作成します
5. レビュアーからのフィードバックに対応します
6. 承認されたら、変更がマージされます

### プルリクエストのテンプレート

```markdown
## 概要
<!-- 変更の概要を簡潔に説明してください -->

## 変更内容
<!-- 具体的な変更内容を箇条書きで記載してください -->

## 関連Issue
<!-- 関連するIssueがあれば記載してください -->
Closes #XXX

## テスト
<!-- どのようにテストしたかを説明してください -->

## スクリーンショット（必要な場合）
<!-- UIの変更がある場合はスクリーンショットを添付してください -->

## チェックリスト
- [ ] テストを追加または更新しました
- [ ] ドキュメントを更新しました
- [ ] コードスタイルガイドラインに従っています
- [ ] すべてのテストが通過します
```

## 📊 テスト

すべての新機能とバグ修正には、適切なテストを含める必要があります：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_feature() {
        // テストコード
        let result = your_function(input);
        assert_eq!(result, expected);
    }
}
```

## 📄 ライセンス

Rustoriumは[MIT](../LICENSE)ライセンスの下で公開されています。プロジェクトに貢献することで、あなたの貢献物もこのライセンスの下で公開されることに同意したものとみなされます。

## 🙏 謝辞

貢献者の皆様に心から感謝いたします。皆様の協力により、Rustoriumはより良いプロジェクトになります。

---

質問やサポートが必要な場合は、[Discussions](https://github.com/enablerdao/rustorium/discussions)でお気軽にお問い合わせください。