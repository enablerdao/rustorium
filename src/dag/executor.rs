use crate::common::errors::LedgerError;
use crate::common::types::{Transaction, TransactionId};
use crate::dag::graph::{NodeState, SharedDag};
use crate::vm::executor::VmExecutor;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{self, Duration};
use tracing::{debug, error, info, warn};

/// 実行結果
#[derive(Debug)]
pub struct ExecutionResult {
    /// トランザクションID
    pub tx_id: TransactionId,
    /// 実行結果
    pub result: Option<Vec<u8>>,
    /// 成功したか
    pub success: bool,
}

/// DAG実行エンジン
pub struct DagExecutor {
    /// DAG
    dag: SharedDag,
    /// VM実行エンジン
    vm_executor: Arc<VmExecutor>,
    /// 実行結果チャネル
    result_tx: mpsc::Sender<ExecutionResult>,
    /// 実行結果レシーバー
    result_rx: mpsc::Receiver<ExecutionResult>,
    /// 実行中のトランザクション数
    executing_count: usize,
    /// 最大並列実行数
    max_parallel: usize,
    /// 実行間隔（ミリ秒）
    execution_interval_ms: u64,
    /// 実行中フラグ
    running: bool,
}

impl DagExecutor {
    /// 新しいDAG実行エンジンを作成
    pub fn new(dag: SharedDag, vm_executor: Arc<VmExecutor>, max_parallel: usize) -> Self {
        let (result_tx, result_rx) = mpsc::channel(100);
        
        Self {
            dag,
            vm_executor,
            result_tx,
            result_rx,
            executing_count: 0,
            max_parallel,
            execution_interval_ms: 10,
            running: false,
        }
    }
    
    /// 実行エンジンを開始
    pub async fn start(&mut self) -> Result<(), LedgerError> {
        if self.running {
            return Err(LedgerError::InvalidState(
                "DAG executor is already running".to_string(),
            ));
        }
        
        self.running = true;
        info!("DAG executor started with max parallel execution: {}", self.max_parallel);
        
        // 実行ループを開始
        let execution_loop = self.execution_loop();
        let result_loop = self.result_loop();
        
        tokio::select! {
            res = execution_loop => {
                if let Err(e) = res {
                    error!("Execution loop error: {}", e);
                }
            }
            res = result_loop => {
                if let Err(e) = res {
                    error!("Result loop error: {}", e);
                }
            }
        }
        
        self.running = false;
        info!("DAG executor stopped");
        
        Ok(())
    }
    
    /// 実行ループ
    async fn execution_loop(&mut self) -> Result<(), LedgerError> {
        let mut interval = time::interval(Duration::from_millis(self.execution_interval_ms));
        
        while self.running {
            interval.tick().await;
            
            // 実行可能なトランザクションを取得
            let executable_txs = {
                let dag = self.dag.read().await;
                dag.get_executable_transactions()
            };
            
            // 並列実行数の制限内で実行
            let available_slots = self.max_parallel.saturating_sub(self.executing_count);
            
            if available_slots > 0 && !executable_txs.is_empty() {
                let to_execute = executable_txs.into_iter().take(available_slots);
                
                for tx in to_execute {
                    let tx_id = tx.id;
                    
                    // 実行開始をマーク
                    {
                        let mut dag = self.dag.write().await;
                        if let Err(e) = dag.start_execution(&tx_id) {
                            warn!("Failed to start execution of transaction {}: {}", tx_id, e);
                            continue;
                        }
                    }
                    
                    // トランザクションを実行
                    self.execute_transaction(tx).await?;
                    self.executing_count += 1;
                }
            }
        }
        
        Ok(())
    }
    
    /// 結果処理ループ
    async fn result_loop(&mut self) -> Result<(), LedgerError> {
        while self.running {
            if let Some(result) = self.result_rx.recv().await {
                // 結果を処理
                let tx_id = result.tx_id;
                let success = result.success;
                
                // DAGを更新
                {
                    let mut dag = self.dag.write().await;
                    if let Err(e) = dag.complete_execution(&tx_id, result.result, success) {
                        warn!("Failed to complete execution of transaction {}: {}", tx_id, e);
                    }
                }
                
                self.executing_count = self.executing_count.saturating_sub(1);
                
                if success {
                    debug!("Transaction {} executed successfully", tx_id);
                } else {
                    warn!("Transaction {} execution failed", tx_id);
                }
            }
        }
        
        Ok(())
    }
    
    /// トランザクションを実行
    async fn execute_transaction(&self, transaction: Transaction) -> Result<(), LedgerError> {
        let tx_id = transaction.id;
        let vm_executor = self.vm_executor.clone();
        let result_tx = self.result_tx.clone();
        
        // 別タスクで実行
        tokio::spawn(async move {
            let execution_result = match vm_executor.execute_transaction(&transaction).await {
                Ok(result) => ExecutionResult {
                    tx_id,
                    result: Some(result),
                    success: true,
                },
                Err(e) => {
                    let error_msg = format!("Execution failed: {}", e);
                    ExecutionResult {
                        tx_id,
                        result: Some(error_msg.into_bytes()),
                        success: false,
                    }
                }
            };
            
            // 結果を送信
            if let Err(e) = result_tx.send(execution_result).await {
                error!("Failed to send execution result: {}", e);
            }
        });
        
        Ok(())
    }
    
    /// 実行エンジンを停止
    pub fn stop(&mut self) {
        self.running = false;
    }
}

/// DAG実行エンジンを開始
pub async fn start_dag_executor(
    dag: SharedDag,
    vm_executor: Arc<VmExecutor>,
    max_parallel: usize,
) -> Result<(), LedgerError> {
    let mut executor = DagExecutor::new(dag, vm_executor, max_parallel);
    executor.start().await
}