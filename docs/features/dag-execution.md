# DAGベース並列処理

Rustoriumは、トランザクションの依存関係を考慮した効率的な並列処理を実現するために、有向非巡回グラフ（DAG）ベースの実行エンジンを実装しています。この文書では、その詳細について説明します。

## 概要

従来のブロックチェーンでは、トランザクションは順次処理されることが多く、並列処理の可能性が十分に活用されていませんでした。Rustoriumは、トランザクション間の依存関係を分析し、独立したトランザクションを並列に実行することで、処理スループットを大幅に向上させます。

## 主要コンポーネント

### DAGエグゼキュータ

DAGエグゼキュータは、トランザクションの依存関係グラフを構築し、並列実行を管理します。

```rust
pub struct DagExecutor {
    dag: SharedDag,
    executor_pool: ThreadPool,
    vm_executor: Arc<VmExecutor>,
    state_manager: Arc<StateManager>,
    execution_queue: mpsc::Sender<TransactionId>,
    result_queue: mpsc::Receiver<ExecutionResult>,
}

impl DagExecutor {
    pub async fn execute_batch(
        &self,
        transactions: Vec<Transaction>,
    ) -> Result<Vec<ExecutionResult>, DagError> {
        // トランザクションの依存関係グラフを構築
        self.build_dependency_graph(&transactions).await?;
        
        // 実行可能なトランザクションを特定
        let executable_txs = self.find_executable_transactions().await?;
        
        // 実行可能なトランザクションを実行キューに送信
        for tx_id in executable_txs {
            self.execution_queue.send(tx_id).await?;
        }
        
        // 実行結果を収集
        let mut results = Vec::with_capacity(transactions.len());
        let mut completed = 0;
        
        while completed < transactions.len() {
            tokio::select! {
                res = self.execution_loop() => {
                    // 新しい実行可能トランザクションを特定して実行キューに送信
                    // ...
                },
                res = self.result_loop() => {
                    // 実行結果を処理
                    results.push(res?);
                    completed += 1;
                }
            }
        }
        
        Ok(results)
    }
    
    // 他のメソッド...
}
```

### 依存関係グラフ

トランザクション間の依存関係を表現するグラフ構造です。

```rust
pub struct DependencyGraph {
    graph: petgraph::Graph<TransactionId, ()>,
    tx_to_node: HashMap<TransactionId, NodeIndex>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            graph: petgraph::Graph::new(),
            tx_to_node: HashMap::new(),
        }
    }
    
    pub fn add_transaction(&mut self, tx_id: TransactionId) -> NodeIndex {
        let node = self.graph.add_node(tx_id.clone());
        self.tx_to_node.insert(tx_id, node);
        node
    }
    
    pub fn add_dependency(
        &mut self,
        dependent: &TransactionId,
        dependency: &TransactionId,
    ) -> Result<(), DagError> {
        let dependent_node = self.tx_to_node.get(dependent)
            .ok_or(DagError::NodeNotFound)?;
        let dependency_node = self.tx_to_node.get(dependency)
            .ok_or(DagError::NodeNotFound)?;
        
        self.graph.add_edge(*dependent_node, *dependency_node, ());
        
        // 循環依存関係をチェック
        if petgraph::algo::is_cyclic_directed(&self.graph) {
            return Err(DagError::CyclicDependency);
        }
        
        Ok(())
    }
    
    pub fn get_roots(&self) -> Vec<TransactionId> {
        self.graph
            .externals(petgraph::Direction::Incoming)
            .map(|node| self.graph[node].clone())
            .collect()
    }
    
    // 他のメソッド...
}
```

### 依存関係分析

トランザクション間の依存関係を分析するロジックです。

```rust
pub fn analyze_dependencies(
    transactions: &[Transaction],
) -> Result<DependencyGraph, DagError> {
    let mut graph = DependencyGraph::new();
    
    // すべてのトランザクションをグラフに追加
    for tx in transactions {
        graph.add_transaction(tx.id.clone());
    }
    
    // 依存関係を分析して追加
    for (i, tx) in transactions.iter().enumerate() {
        for (j, other_tx) in transactions.iter().enumerate() {
            if i == j {
                continue;
            }
            
            // 同じアカウントに対するトランザクションは依存関係がある
            if tx.from == other_tx.from || tx.to == other_tx.to || tx.to == other_tx.from {
                // トランザクションの順序に基づいて依存関係を追加
                if i > j {
                    graph.add_dependency(&tx.id, &other_tx.id)?;
                }
            }
        }
    }
    
    Ok(graph)
}
```

### 並列実行エンジン

依存関係グラフに基づいて、トランザクションを並列に実行するエンジンです。

```rust
pub struct ParallelExecutionEngine {
    thread_pool: ThreadPool,
    vm_executor: Arc<VmExecutor>,
    state_manager: Arc<StateManager>,
}

impl ParallelExecutionEngine {
    pub fn new(
        num_threads: usize,
        vm_executor: Arc<VmExecutor>,
        state_manager: Arc<StateManager>,
    ) -> Self {
        Self {
            thread_pool: ThreadPool::new(num_threads),
            vm_executor,
            state_manager,
        }
    }
    
    pub fn execute(
        &self,
        tx: Transaction,
        tx_result_sender: mpsc::Sender<ExecutionResult>,
    ) {
        let vm_executor = self.vm_executor.clone();
        let state_manager = self.state_manager.clone();
        let tx_id = tx.id.clone();
        
        self.thread_pool.spawn(move || {
            // トランザクション実行のためのステートスナップショットを作成
            let mut state = state_manager.create_snapshot();
            
            // トランザクションを実行
            let result = match vm_executor.execute_transaction(&tx, &mut state) {
                Ok(result) => {
                    // 成功した場合、ステートの変更をコミット
                    if let Err(e) = state_manager.commit_snapshot(state) {
                        ExecutionResult::Error {
                            tx_id,
                            error: format!("Failed to commit state: {}", e),
                        }
                    } else {
                        ExecutionResult::Success {
                            tx_id,
                            gas_used: result.gas_used,
                            return_data: result.return_data,
                        }
                    }
                },
                Err(e) => {
                    // 失敗した場合、ステートの変更を破棄
                    ExecutionResult::Error {
                        tx_id,
                        error: format!("Execution failed: {}", e),
                    }
                }
            };
            
            // 結果を送信
            let _ = tx_result_sender.blocking_send(result);
        });
    }
}
```

## 実行フロー

1. トランザクションバッチを受け取る
2. 依存関係グラフを構築
3. 実行可能なトランザクション（依存関係のないノード）を特定
4. 実行可能なトランザクションを並列に実行
5. 実行が完了したトランザクションをグラフから削除
6. 新たに実行可能になったトランザクションを特定して実行
7. すべてのトランザクションが実行されるまで繰り返す

## パフォーマンス最適化

### スレッドプール最適化

Rayonライブラリを使用して、効率的なワークスティーリングベースのスレッドプールを実装しています。

```rust
pub fn optimize_thread_pool_size() -> usize {
    let cpu_count = num_cpus::get();
    
    // CPUコア数に基づいてスレッド数を決定
    // 一般的には、CPUコア数の1.5〜2倍が最適
    let thread_count = (cpu_count as f64 * 1.5).ceil() as usize;
    
    // 最小値と最大値を設定
    thread_count.max(2).min(32)
}
```

### 状態アクセス最適化

トランザクション実行中の状態アクセスを最適化するために、読み取り/書き込みセットの追跡を実装しています。

```rust
pub struct StateAccessTracker {
    read_set: HashSet<StateKey>,
    write_set: HashSet<StateKey>,
}

impl StateAccessTracker {
    pub fn new() -> Self {
        Self {
            read_set: HashSet::new(),
            write_set: HashSet::new(),
        }
    }
    
    pub fn track_read(&mut self, key: StateKey) {
        self.read_set.insert(key);
    }
    
    pub fn track_write(&mut self, key: StateKey) {
        self.write_set.insert(key);
    }
    
    pub fn has_conflict_with(&self, other: &StateAccessTracker) -> bool {
        // 書き込みセットと他のトラッカーの読み取り/書き込みセットの交差をチェック
        for key in &self.write_set {
            if other.read_set.contains(key) || other.write_set.contains(key) {
                return true;
            }
        }
        
        // 読み取りセットと他のトラッカーの書き込みセットの交差をチェック
        for key in &self.read_set {
            if other.write_set.contains(key) {
                return true;
            }
        }
        
        false
    }
}
```

## 設定例

```toml
[dag_execution]
# 並列実行に使用するスレッド数（0は自動）
thread_count = 0

# 最大バッチサイズ
max_batch_size = 1000

# 依存関係分析の最大深さ
max_dependency_depth = 5

# 実行タイムアウト（ミリ秒）
execution_timeout_ms = 5000
```

## 今後の改善点

1. スマートな依存関係分析: 静的解析を使用してより正確な依存関係を特定
2. 投機的実行: 依存関係の可能性が低いトランザクションを投機的に実行
3. 動的負荷分散: 実行時の負荷に基づいてリソース割り当てを調整
4. 分散DAG実行: シャード間でのDAG実行の調整