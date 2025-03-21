# トランザクション管理ガイド

Rustoriumでのトランザクションの表示、送信、管理方法について説明します。

## トランザクションの表示

### トランザクションリストの表示

トランザクションリストを表示するには、サイドバーの「Transactions」をクリックします。

![トランザクションリスト](../images/transactions-list.png)

トランザクションリストには以下の情報が表示されます：

- **トランザクションID**: トランザクションの一意の識別子
- **送信元**: トランザクションの送信者アドレス
- **送信先**: トランザクションの受信者アドレス
- **金額**: 送金された金額
- **ガス使用量**: トランザクションの実行に使用されたガス
- **ステータス**: トランザクションの状態（保留中、確定、失敗）
- **タイムスタンプ**: トランザクションがブロックチェーンに含まれた日時

### トランザクション詳細の表示

特定のトランザクションの詳細を表示するには、トランザクションリストからトランザクションIDをクリックします。

![トランザクション詳細](../images/transaction-details.png)

トランザクション詳細ページには以下の情報が表示されます：

#### 基本情報

- **トランザクションID**: トランザクションの一意の識別子
- **ブロック**: トランザクションが含まれるブロック番号
- **タイムスタンプ**: トランザクションがブロックチェーンに含まれた日時
- **ステータス**: トランザクションの状態（確定、失敗など）

#### 送信情報

- **送信元**: トランザクションの送信者アドレス
- **送信先**: トランザクションの受信者アドレス
- **金額**: 送金された金額
- **手数料**: トランザクション手数料

#### 技術情報

- **ガス使用量**: トランザクションの実行に使用されたガス
- **ガス価格**: ガス単価
- **ガス上限**: トランザクションのガス上限
- **ノンス**: トランザクションのノンス値
- **データ**: トランザクションに含まれるデータ（16進数形式）

#### 実行結果

- **ステータスコード**: トランザクション実行のステータスコード
- **ログ**: トランザクション実行中に生成されたイベントログ
- **エラーメッセージ**: 失敗した場合のエラーメッセージ

## トランザクションの送信

新しいトランザクションを送信するには、サイドバーの「Send Transaction」をクリックします。

![トランザクション送信](../images/send-transaction.png)

### 基本的なトランザクション送信

1. **送信元アドレス**を選択します（ウォレットに接続している場合は自動的に選択されます）
2. **送信先アドレス**を入力します
3. 送金**金額**を入力します
4. 必要に応じて**ガス価格**と**ガス上限**を調整します
5. 「Send」ボタンをクリックします

### 高度なオプション

「Advanced Options」をクリックすると、以下の追加オプションが表示されます：

- **データ**: トランザクションに含めるデータ（16進数形式）
- **ノンス**: カスタムノンス値（通常は自動的に設定されます）
- **メモ**: トランザクションに関するメモ（ブロックチェーンには保存されません）

### スマートコントラクトの呼び出し

スマートコントラクトを呼び出すトランザクションを送信するには：

1. **送信先アドレス**にコントラクトアドレスを入力します
2. 「Contract Interaction」タブを選択します
3. **関数名**を選択します
4. 関数の**パラメータ**を入力します
5. 必要に応じて**送金金額**を入力します（payable関数の場合）
6. 「Send」ボタンをクリックします

## トランザクションの検索

特定のトランザクションを検索するには、画面上部の検索ボックスを使用します。トランザクションIDで検索できます。

![トランザクション検索](../images/transaction-search.png)

## フィルタリングとソート

トランザクションリストは以下の条件でフィルタリングおよびソートできます：

- **時間範囲**: 特定の期間内のトランザクションのみを表示
- **ステータス**: 特定のステータス（保留中、確定、失敗）のトランザクションのみを表示
- **アドレス**: 特定のアドレスに関連するトランザクションのみを表示
- **金額**: 金額の範囲に基づいてフィルタリング
- **タイプ**: トランザクションタイプ（送金、コントラクト作成、コントラクト呼び出し）に基づいてフィルタリング

## トランザクションの監視

特定のトランザクションを監視するには、トランザクション詳細ページの「Watch」ボタンをクリックします。トランザクションのステータスが変更されると通知が表示されます。

## トランザクションの再送信

失敗したトランザクションを再送信するには、トランザクション詳細ページの「Resend」ボタンをクリックします。ガス価格やガス上限を調整して再送信できます。

## トランザクションのエクスポート

トランザクションデータをCSV、JSON、またはPDFフォーマットでエクスポートできます。エクスポートするには、トランザクションリストまたはトランザクション詳細ページの「Export」ボタンをクリックします。

## トラブルシューティング

### トランザクションが保留中のまま

1. **ガス価格が低すぎる**: ガス価格を上げて再送信してください
2. **ノンスの問題**: 同じアドレスからの前のトランザクションが保留中の場合、そのトランザクションが確定するまで待つか、適切なノンスで再送信してください
3. **ネットワーク混雑**: ネットワークが混雑している場合は、トランザクションの処理に時間がかかることがあります

### トランザクションが失敗する

1. **ガス不足**: ガス上限を上げて再送信してください
2. **残高不足**: 送金金額とガス代をカバーするのに十分な残高があることを確認してください
3. **コントラクトエラー**: スマートコントラクトの呼び出しが失敗した場合は、パラメータが正しいか確認してください