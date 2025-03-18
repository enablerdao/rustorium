use crate::common::errors::{LedgerError, VmError};
use crate::common::types::{Transaction, VmType};
use crate::storage::state::StateManager;
use crate::vm::evm::EvmExecutor;
use crate::vm::wasm::WasmExecutor;
use std::sync::Arc;
use tracing::{debug, info, warn};

/// VM実行エンジン
pub struct VmExecutor {
    /// EVM実行エンジン
    evm_executor: Option<EvmExecutor>,
    /// WASM実行エンジン
    wasm_executor: Option<WasmExecutor>,
    /// 状態マネージャー
    state_manager: Arc<StateManager>,
}

impl VmExecutor {
    /// 新しいVM実行エンジンを作成
    pub fn new(state_manager: Arc<StateManager>) -> Self {
        Self {
            evm_executor: Some(EvmExecutor::new(state_manager.clone())),
            wasm_executor: Some(WasmExecutor::new(state_manager.clone())),
            state_manager,
        }
    }
    
    /// トランザクションを実行
    pub async fn execute_transaction(&self, transaction: &Transaction) -> Result<Vec<u8>, LedgerError> {
        let tx_id = transaction.id;
        let vm_type = transaction.vm_type;
        
        debug!("Executing transaction {} with VM type {:?}", tx_id, vm_type);
        
        match vm_type {
            VmType::Evm => {
                if let Some(executor) = &self.evm_executor {
                    executor.execute(transaction).await
                } else {
                    Err(LedgerError::VmExecution(VmError::InvalidVmType(
                        "EVM executor not available".to_string(),
                    )))
                }
            }
            VmType::Wasm => {
                if let Some(executor) = &self.wasm_executor {
                    executor.execute(transaction).await
                } else {
                    Err(LedgerError::VmExecution(VmError::InvalidVmType(
                        "WASM executor not available".to_string(),
                    )))
                }
            }
            _ => Err(LedgerError::VmExecution(VmError::InvalidVmType(format!(
                "Unsupported VM type: {:?}",
                vm_type
            )))),
        }
    }
    
    /// 読み取り専用クエリを実行
    pub async fn execute_view_call(
        &self,
        vm_type: VmType,
        contract_address: &[u8; 20],
        data: &[u8],
    ) -> Result<Vec<u8>, LedgerError> {
        debug!(
            "Executing view call on contract {:?} with VM type {:?}",
            hex::encode(contract_address),
            vm_type
        );
        
        match vm_type {
            VmType::Evm => {
                if let Some(executor) = &self.evm_executor {
                    executor.execute_view_call(contract_address, data).await
                } else {
                    Err(LedgerError::VmExecution(VmError::InvalidVmType(
                        "EVM executor not available".to_string(),
                    )))
                }
            }
            VmType::Wasm => {
                if let Some(executor) = &self.wasm_executor {
                    executor.execute_view_call(contract_address, data).await
                } else {
                    Err(LedgerError::VmExecution(VmError::InvalidVmType(
                        "WASM executor not available".to_string(),
                    )))
                }
            }
            _ => Err(LedgerError::VmExecution(VmError::InvalidVmType(format!(
                "Unsupported VM type: {:?}",
                vm_type
            )))),
        }
    }
}