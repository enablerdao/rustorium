# マルチVM実行環境

Rustoriumは、複数の仮想マシン（VM）環境をサポートする柔軟な実行環境を提供します。この文書では、マルチVM実行環境の実装詳細について説明します。

## 概要

マルチVM実行環境は、異なるスマートコントラクト言語やプラットフォームをサポートするために設計されています。現在、以下のVMをサポートしています：

- **EVM**: Ethereum Virtual Machine互換の実行環境
- **WASM**: WebAssemblyベースの実行環境
- **Solana VM**: Solanaプログラム実行環境（開発中）

## アーキテクチャ

マルチVM実行環境は、共通インターフェースを通じて異なるVMを統一的に扱います。

```
+----------------------------------+
|          VM Executor             |
+----------------------------------+
              |
    +---------+---------+---------+
    |         |         |         |
+-------+ +-------+ +-------+ +-------+
|  EVM  | | WASM  | |Solana | | Future|
+-------+ +-------+ +-------+ +-------+
```

## 主要コンポーネント

### VM実行インターフェース

すべてのVMは共通のインターフェースを実装します。

```rust
pub trait Executor: Send + Sync {
    fn execute_transaction(
        &self,
        tx: &Transaction,
        state: &mut dyn StateAccess,
    ) -> Result<ExecutionResult, VmError>;
    
    fn deploy_contract(
        &self,
        code: Vec<u8>,
        constructor_args: Vec<u8>,
        sender: Address,
        state: &mut dyn StateAccess,
    ) -> Result<Address, VmError>;
    
    fn call_contract(
        &self,
        contract_address: Address,
        method: &str,
        args: Vec<u8>,
        sender: Address,
        state: &mut dyn StateAccess,
    ) -> Result<Vec<u8>, VmError>;
}
```

### VM実行ルーター

トランザクションのVMタイプに基づいて、適切なVM実行環境にルーティングします。

```rust
pub struct VmExecutor {
    evm: EvmExecutor,
    wasm: WasmExecutor,
    solana: Option<SolanaExecutor>,
}

impl VmExecutor {
    pub fn execute_transaction(
        &self,
        tx: &Transaction,
        state: &mut dyn StateAccess,
    ) -> Result<ExecutionResult, VmError> {
        match tx.vm_type {
            VmType::Evm => self.evm.execute_transaction(tx, state),
            VmType::Wasm => self.wasm.execute_transaction(tx, state),
            VmType::Solana => {
                if let Some(solana_vm) = &self.solana {
                    solana_vm.execute_transaction(tx, state)
                } else {
                    Err(VmError::UnsupportedVm("Solana VM not enabled".to_string()))
                }
            }
        }
    }
    
    // 他のメソッド...
}
```

### EVM実装

Ethereum互換の実行環境を提供します。

```rust
pub struct EvmExecutor {
    evm: revm::EVM<StateDB>,
}

impl Executor for EvmExecutor {
    fn execute_transaction(
        &self,
        tx: &Transaction,
        state: &mut dyn StateAccess,
    ) -> Result<ExecutionResult, VmError> {
        // トランザクションをEVMフォーマットに変換
        let evm_tx = self.convert_to_evm_tx(tx)?;
        
        // 送信者アカウントを取得
        let sender_account = self
            .get_account(tx.from, state)
            .ok_or(VmError::AccountNotFound)?;
        
        // EVMトランザクションを実行
        let result = self.evm.transact(evm_tx)?;
        
        // 結果を変換して返す
        self.convert_evm_result(result)
    }
    
    fn deploy_contract(
        &self,
        code: Vec<u8>,
        constructor_args: Vec<u8>,
        sender: Address,
        state: &mut dyn StateAccess,
    ) -> Result<Address, VmError> {
        // コントラクトデプロイトランザクションを作成
        let deploy_tx = self.create_deploy_tx(code, constructor_args, sender)?;
        
        // トランザクションを実行
        let result = self.execute_transaction(&deploy_tx, state)?;
        
        // デプロイされたコントラクトのアドレスを返す
        match result {
            ExecutionResult::ContractCreated { address, .. } => Ok(address),
            _ => Err(VmError::ContractDeploymentFailed(
                "Contract deployment did not return an address".to_string(),
            )),
        }
    }
    
    // 他のメソッド...
}
```

### WASM実装

WebAssemblyベースの実行環境を提供します。

```rust
pub struct WasmExecutor {
    store: Store<()>,
    engine: Engine,
    linker: Linker<()>,
}

impl Executor for WasmExecutor {
    fn execute_transaction(
        &self,
        tx: &Transaction,
        state: &mut dyn StateAccess,
    ) -> Result<ExecutionResult, VmError> {
        // コントラクトコードを取得
        let contract = state.get_contract(&tx.to)?;
        
        // WASMモジュールをコンパイル
        let module = Module::new(&self.engine, &contract.code)?;
        
        // インポートオブジェクトを作成
        let import_object = self.create_import_object(state, tx.from)?;
        
        // インスタンスを作成して実行
        let instance = Instance::new(&mut self.store.clone(), &module, &import_object)?;
        
        // エントリポイント関数を取得して呼び出し
        let entry_point = instance.exports.get_function(tx.method.as_str())?;
        let result = entry_point.call(&mut self.store, &[Value::I32(0), Value::I32(0)])?;
        
        // 結果を変換して返す
        self.convert_wasm_result(result)
    }
    
    // 他のメソッド...
}
```

### 状態アクセスインターフェース

各VMは共通の状態アクセスインターフェースを通じてブロックチェーン状態にアクセスします。

```rust
pub trait StateAccess {
    fn get_account(&self, address: Address) -> Option<Account>;
    fn set_account(&mut self, address: Address, account: Account);
    
    fn get_storage(&self, address: Address, key: H256) -> H256;
    fn set_storage(&mut self, address: Address, key: H256, value: H256);
    
    fn get_code(&self, address: Address) -> Option<Vec<u8>>;
    fn set_code(&mut self, address: Address, code: Vec<u8>);
    
    fn get_contract(&self, address: &Address) -> Result<Contract, VmError>;
    fn create_contract(&mut self, code: Vec<u8>) -> Result<Address, VmError>;
    
    fn transfer(
        &mut self,
        from: Address,
        to: Address,
        amount: U256,
    ) -> Result<(), VmError>;
}
```

## JITコンパイル最適化

頻繁に使用されるコントラクトのパフォーマンスを向上させるために、JITコンパイル最適化を実装しています。

```rust
pub struct JitCache {
    compiled_contracts: DashMap<Address, Arc<CompiledContract>>,
}

impl JitCache {
    pub fn get_or_compile(
        &self,
        address: Address,
        code: &[u8],
        vm_type: VmType,
    ) -> Result<Arc<CompiledContract>, VmError> {
        // キャッシュをチェック
        if let Some(compiled) = self.compiled_contracts.get(&address) {
            return Ok(compiled.clone());
        }
        
        // コントラクトをコンパイル
        let compiled = match vm_type {
            VmType::Evm => self.compile_evm(code)?,
            VmType::Wasm => self.compile_wasm(code)?,
            VmType::Solana => self.compile_solana(code)?,
        };
        
        // コンパイル結果をキャッシュ
        let compiled_arc = Arc::new(compiled);
        self.compiled_contracts.insert(address, compiled_arc.clone());
        
        Ok(compiled_arc)
    }
    
    // 他のメソッド...
}
```

## VM間通信

異なるVM間での通信をサポートするためのブリッジ機能を提供します。

```rust
pub struct VmBridge {
    executors: HashMap<VmType, Box<dyn Executor>>,
}

impl VmBridge {
    pub fn call_cross_vm(
        &self,
        from_vm: VmType,
        to_vm: VmType,
        contract_address: Address,
        method: &str,
        args: Vec<u8>,
        caller: Address,
        state: &mut dyn StateAccess,
    ) -> Result<Vec<u8>, VmError> {
        // 呼び出し元VMのコンテキストを保存
        let context = self.save_context(from_vm, state)?;
        
        // 対象VMでコントラクトを呼び出し
        let result = self.executors
            .get(&to_vm)
            .ok_or(VmError::UnsupportedVm(format!("{:?} not supported", to_vm)))?
            .call_contract(contract_address, method, args, caller, state);
        
        // 呼び出し元VMのコンテキストを復元
        self.restore_context(from_vm, context, state)?;
        
        result
    }
    
    // 他のメソッド...
}
```

## 設定例

```toml
[vm]
# 有効にするVM
enabled_vms = ["evm", "wasm"]

# JITコンパイル最適化を有効にする
enable_jit = true

# VM間通信を有効にする
enable_cross_vm_calls = true

[vm.evm]
# EVMのバージョン
version = "london"

# ガス制限
gas_limit = 10000000

[vm.wasm]
# メモリ制限（バイト）
memory_limit = 67108864

# 実行時間制限（ミリ秒）
execution_time_limit = 1000
```

## 今後の改善点

1. より多くのVM環境のサポート: Move VMなど
2. VM間通信の最適化: より効率的なデータ変換とコンテキスト切り替え
3. 並列実行: 独立したコントラクト呼び出しの並列実行
4. セキュリティ強化: より厳格なリソース制限と分離