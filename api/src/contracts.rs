use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use uuid::Uuid;

/// コントラクトの種類
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ContractType {
    /// 標準的なコントラクト
    Standard,
    /// トークンコントラクト（ERC-20相当）
    Token,
    /// NFTコントラクト（ERC-721相当）
    NFT,
    /// 委任可能なコントラクト
    Delegatable,
    /// アップグレード可能なコントラクト
    Upgradeable,
}

/// コントラクトの状態変数の型
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StateValueType {
    String(String),
    Number(i64),
    Float(f64),
    Boolean(bool),
    Address(String),
    Bytes(Vec<u8>),
    Array(Vec<StateValue>),
    Map(HashMap<String, StateValue>),
}

/// コントラクトの状態変数
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StateValue {
    pub value_type: StateValueType,
    pub is_public: bool,
    pub is_constant: bool,
}

/// スマートコントラクトの構造体
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Contract {
    /// コントラクトのアドレス
    pub address: String,
    /// コントラクトの名前
    pub name: Option<String>,
    /// コントラクトの作成者
    pub creator: String,
    /// コントラクトのバイトコード
    pub bytecode: String,
    /// コントラクトのABI（Application Binary Interface）
    pub abi: Option<String>,
    /// コントラクトの種類
    pub contract_type: ContractType,
    /// コントラクトの作成時のトランザクションID
    pub creation_transaction: String,
    /// コントラクトの作成時のブロック番号
    pub creation_block: Option<u64>,
    /// コントラクトの状態変数（シンプルな文字列マップ）
    pub state: HashMap<String, String>,
    /// コントラクトの拡張状態変数（型付き）
    pub typed_state: HashMap<String, StateValue>,
    /// コントラクトのイベントログ
    pub events: Vec<ContractEvent>,
    /// コントラクトの作成日時
    pub created_at: chrono::DateTime<Utc>,
    /// 最後のアクティビティ日時
    pub last_activity: chrono::DateTime<Utc>,
    /// コントラクトのバージョン
    pub version: String,
    /// コントラクトのソースコード（検証済みの場合）
    pub source_code: Option<String>,
    /// コントラクトが検証済みかどうか
    pub is_verified: bool,
    /// コントラクトのメタデータ
    pub metadata: Option<String>,
}

/// コントラクトイベント
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContractEvent {
    /// イベント名
    pub name: String,
    /// イベントの引数
    pub args: HashMap<String, String>,
    /// イベントが発生したブロック番号
    pub block_number: u64,
    /// イベントが発生したトランザクションID
    pub transaction_id: String,
    /// イベントのインデックス
    pub log_index: u32,
    /// イベントの発生日時
    pub timestamp: chrono::DateTime<Utc>,
}

impl Contract {
    /// 新しいコントラクトを作成
    pub fn new(
        creator: String,
        bytecode: String,
        abi: Option<String>,
        creation_transaction: String,
        creation_block: Option<u64>,
    ) -> Self {
        // コントラクトアドレスの生成（実際の実装ではより複雑なアルゴリズムを使用）
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}", creator, bytecode, Uuid::new_v4()));
        let address = format!("0x{}", hex::encode(hasher.finalize())[..40].to_string());

        Contract {
            address,
            name: None,
            creator,
            bytecode,
            abi,
            contract_type: ContractType::Standard,
            creation_transaction,
            creation_block,
            state: HashMap::new(),
            typed_state: HashMap::new(),
            events: Vec::new(),
            created_at: Utc::now(),
            last_activity: Utc::now(),
            version: "1.0.0".to_string(),
            source_code: None,
            is_verified: false,
            metadata: None,
        }
    }

    /// コントラクトの状態変数を設定（文字列）
    pub fn set_state(&mut self, key: String, value: String) {
        self.state.insert(key, value);
        self.last_activity = Utc::now();
    }

    /// コントラクトの状態変数を取得（文字列）
    pub fn get_state(&self, key: &str) -> Option<&String> {
        self.state.get(key)
    }
    
    /// 型付き状態変数を設定
    pub fn set_typed_state(&mut self, key: String, value: StateValue) {
        self.typed_state.insert(key, value);
        self.last_activity = Utc::now();
    }
    
    /// 型付き状態変数を取得
    pub fn get_typed_state(&self, key: &str) -> Option<&StateValue> {
        self.typed_state.get(key)
    }
    
    /// イベントを発行
    pub fn emit_event(
        &mut self,
        name: String,
        args: HashMap<String, String>,
        block_number: u64,
        transaction_id: String,
        log_index: u32,
    ) {
        let event = ContractEvent {
            name,
            args,
            block_number,
            transaction_id,
            log_index,
            timestamp: Utc::now(),
        };
        
        self.events.push(event);
        self.last_activity = Utc::now();
    }
    
    /// コントラクトを検証
    pub fn verify(&mut self, source_code: String) {
        self.source_code = Some(source_code);
        self.is_verified = true;
        self.last_activity = Utc::now();
    }
    
    /// コントラクトの名前を設定
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }
    
    /// コントラクトのメタデータを設定
    pub fn set_metadata(&mut self, metadata: String) {
        self.metadata = Some(metadata);
    }
    
    /// コントラクトの種類を設定
    pub fn set_contract_type(&mut self, contract_type: ContractType) {
        self.contract_type = contract_type;
    }
}

/// トークンコントラクト用のヘルパーメソッド
impl Contract {
    /// トークンコントラクトを作成
    pub fn new_token_contract(
        creator: String,
        bytecode: String,
        abi: Option<String>,
        creation_transaction: String,
        creation_block: Option<u64>,
        name: String,
        symbol: String,
        decimals: u8,
        total_supply: u64,
    ) -> Self {
        let mut contract = Self::new(
            creator.clone(),
            bytecode,
            abi,
            creation_transaction,
            creation_block,
        );
        
        contract.set_contract_type(ContractType::Token);
        contract.set_name(name.clone());
        
        // トークン情報を状態に設定
        contract.set_state("name".to_string(), name);
        contract.set_state("symbol".to_string(), symbol);
        contract.set_state("decimals".to_string(), decimals.to_string());
        contract.set_state("totalSupply".to_string(), total_supply.to_string());
        
        // 作成者に全供給量を付与
        contract.set_state(format!("balanceOf:{}", creator), total_supply.to_string());
        
        contract
    }
    
    /// NFTコントラクトを作成
    pub fn new_nft_contract(
        creator: String,
        bytecode: String,
        abi: Option<String>,
        creation_transaction: String,
        creation_block: Option<u64>,
        name: String,
        symbol: String,
    ) -> Self {
        let mut contract = Self::new(
            creator,
            bytecode,
            abi,
            creation_transaction,
            creation_block,
        );
        
        contract.set_contract_type(ContractType::NFT);
        contract.set_name(name.clone());
        
        // NFT情報を状態に設定
        contract.set_state("name".to_string(), name);
        contract.set_state("symbol".to_string(), symbol);
        contract.set_state("totalSupply".to_string(), "0".to_string());
        
        contract
    }
}

/// コントラクトデプロイリクエスト
#[derive(Debug, Serialize, Deserialize)]
pub struct DeployContractRequest {
    pub from: String,
    pub bytecode: String,
    pub abi: Option<String>,
    pub constructor_args: Option<String>,
    pub gas_limit: u64,
    pub gas_price: u64,
    pub contract_type: Option<ContractType>,
    pub name: Option<String>,
    pub metadata: Option<String>,
    // トークン関連のパラメータ
    pub token_name: Option<String>,
    pub token_symbol: Option<String>,
    pub token_decimals: Option<u8>,
    pub token_total_supply: Option<u64>,
}

/// コントラクト呼び出しリクエスト
#[derive(Debug, Serialize, Deserialize)]
pub struct CallContractRequest {
    pub from: String,
    pub method: String,
    pub args: Option<String>,
    pub gas_limit: u64,
    pub gas_price: u64,
    pub value: f64,
    pub debug_mode: Option<bool>,
}

/// コントラクト呼び出し結果
#[derive(Debug, Serialize, Deserialize)]
pub struct CallContractResult {
    pub transaction_id: String,
    pub result: Option<String>,
    pub gas_used: u64,
    pub events: Option<Vec<ContractEvent>>,
    pub debug_info: Option<DebugInfo>,
}

/// コントラクト検証リクエスト
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyContractRequest {
    pub address: String,
    pub source_code: String,
    pub compiler_version: String,
    pub optimization: bool,
    pub constructor_args: Option<String>,
}

/// コントラクトデバッグ情報
#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct DebugInfo {
    pub execution_trace: Vec<ExecutionStep>,
    pub state_changes: HashMap<String, StateChange>,
    pub gas_profile: HashMap<String, u64>,
    pub error: Option<String>,
}

/// 実行ステップ
#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct ExecutionStep {
    pub step: u32,
    pub op_code: String,
    pub stack: Vec<String>,
    pub memory: Vec<String>,
    pub gas_used: u64,
    pub gas_remaining: u64,
}

/// 状態変更
#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct StateChange {
    pub key: String,
    pub old_value: Option<String>,
    pub new_value: String,
}

/// コントラクト管理モジュール
#[derive(Clone)]
pub struct ContractManager {
    pub contracts: HashMap<String, Contract>,
    pub verified_contracts: HashMap<String, String>, // アドレス -> ソースコード
    pub debug_info: HashMap<String, DebugInfo>,      // トランザクションID -> デバッグ情報
}

impl ContractManager {
    /// 新しいコントラクトマネージャーを作成
    pub fn new() -> Self {
        ContractManager {
            contracts: HashMap::new(),
            verified_contracts: HashMap::new(),
            debug_info: HashMap::new(),
        }
    }

    /// コントラクトをデプロイ
    pub fn deploy_contract(
        &mut self,
        creator: String,
        bytecode: String,
        abi: Option<String>,
        creation_transaction: String,
        creation_block: Option<u64>,
    ) -> String {
        let contract = Contract::new(
            creator,
            bytecode,
            abi,
            creation_transaction,
            creation_block,
        );
        
        let address = contract.address.clone();
        self.contracts.insert(address.clone(), contract);
        
        address
    }
    
    /// 特殊なコントラクトをデプロイ
    pub fn deploy_special_contract(
        &mut self,
        request: &DeployContractRequest,
        creation_transaction: String,
        creation_block: Option<u64>,
    ) -> String {
        let contract = match request.contract_type {
            Some(ContractType::Token) => {
                // トークンコントラクトの場合
                if let (Some(name), Some(symbol), Some(decimals), Some(total_supply)) = (
                    request.token_name.clone(),
                    request.token_symbol.clone(),
                    request.token_decimals,
                    request.token_total_supply,
                ) {
                    Contract::new_token_contract(
                        request.from.clone(),
                        request.bytecode.clone(),
                        request.abi.clone(),
                        creation_transaction,
                        creation_block,
                        name,
                        symbol,
                        decimals,
                        total_supply,
                    )
                } else {
                    // 必要なパラメータが不足している場合は標準コントラクトとしてデプロイ
                    Contract::new(
                        request.from.clone(),
                        request.bytecode.clone(),
                        request.abi.clone(),
                        creation_transaction,
                        creation_block,
                    )
                }
            },
            Some(ContractType::NFT) => {
                // NFTコントラクトの場合
                if let (Some(name), Some(symbol)) = (
                    request.token_name.clone(),
                    request.token_symbol.clone(),
                ) {
                    Contract::new_nft_contract(
                        request.from.clone(),
                        request.bytecode.clone(),
                        request.abi.clone(),
                        creation_transaction,
                        creation_block,
                        name,
                        symbol,
                    )
                } else {
                    // 必要なパラメータが不足している場合は標準コントラクトとしてデプロイ
                    Contract::new(
                        request.from.clone(),
                        request.bytecode.clone(),
                        request.abi.clone(),
                        creation_transaction,
                        creation_block,
                    )
                }
            },
            _ => {
                // その他のコントラクトタイプまたは指定なしの場合
                let mut contract = Contract::new(
                    request.from.clone(),
                    request.bytecode.clone(),
                    request.abi.clone(),
                    creation_transaction,
                    creation_block,
                );
                
                // 名前が指定されている場合は設定
                if let Some(name) = &request.name {
                    contract.set_name(name.clone());
                }
                
                // メタデータが指定されている場合は設定
                if let Some(metadata) = &request.metadata {
                    contract.set_metadata(metadata.clone());
                }
                
                // コントラクトタイプが指定されている場合は設定
                if let Some(contract_type) = &request.contract_type {
                    contract.set_contract_type(contract_type.clone());
                }
                
                contract
            }
        };
        
        let address = contract.address.clone();
        self.contracts.insert(address.clone(), contract);
        
        address
    }

    /// コントラクトを呼び出し（拡張実装）
    pub fn call_contract(
        &mut self,
        address: &str,
        method: &str,
        args: Option<&str>,
        caller: &str,
    ) -> Result<String, String> {
        // コントラクトの存在確認とクローン
        let mut contract = match self.contracts.get(address) {
            Some(c) => c.clone(),
            None => return Err(format!("Contract not found: {}", address)),
        };

        // コントラクトタイプに基づいて処理を分岐
        let result = match contract.contract_type {
            ContractType::Token => self.call_token_contract(&mut contract, method, args, caller),
            ContractType::NFT => self.call_nft_contract(&mut contract, method, args, caller),
            _ => {
                // 標準コントラクトの場合
                match method {
                    "store" => {
                        if let Some(value) = args {
                            contract.set_state(method.to_string(), value.to_string());
                            Ok("success".to_string())
                        } else {
                            Err("Missing arguments for store method".to_string())
                        }
                    }
                    "retrieve" => {
                        if let Some(value) = contract.get_state("store") {
                            Ok(value.clone())
                        } else {
                            Ok("0".to_string())
                        }
                    }
                    _ => Err(format!("Unknown method: {}", method)),
                }
            }
        };
        
        // 最終アクティビティ時間を更新
        contract.last_activity = Utc::now();
        
        result
    }
    
    /// コントラクトを呼び出し（デバッグモード）
    pub fn call_contract_with_debug(
        &mut self,
        address: &str,
        method: &str,
        args: Option<&str>,
        caller: &str,
        transaction_id: &str,
    ) -> Result<(String, DebugInfo), String> {
        // コントラクトの存在確認とクローン
        let mut contract = match self.contracts.get(address) {
            Some(c) => c.clone(),
            None => return Err(format!("Contract not found: {}", address)),
        };
        
        // 実行前の状態をコピー
        let pre_state = contract.state.clone();
        
        // 実行トレースの初期化
        let mut execution_trace = Vec::new();
        let mut gas_profile = HashMap::new();
        
        // 実行ステップの記録（シミュレーション）
        execution_trace.push(ExecutionStep {
            step: 0,
            op_code: "CALL".to_string(),
            stack: vec![method.to_string()],
            memory: vec![args.unwrap_or("").to_string()],
            gas_used: 0,
            gas_remaining: 1000000,
        });
        
        // メソッド実行
        let result = self.call_contract(address, method, args, caller)?;
        
        // 実行後の状態を取得
        let post_state = contract.state.clone();
        
        // 状態変更を計算
        let mut state_changes = HashMap::new();
        for (key, new_value) in &post_state {
            let old_value = pre_state.get(key).cloned();
            if old_value != Some(new_value.clone()) {
                state_changes.insert(key.clone(), StateChange {
                    key: key.clone(),
                    old_value,
                    new_value: new_value.clone(),
                });
            }
        }
        
        // 最終ステップの記録
        execution_trace.push(ExecutionStep {
            step: 1,
            op_code: "RETURN".to_string(),
            stack: vec![result.clone()],
            memory: vec![],
            gas_used: 21000,
            gas_remaining: 979000,
        });
        
        // ガスプロファイルの記録
        gas_profile.insert(method.to_string(), 21000);
        
        // デバッグ情報の作成
        let debug_info = DebugInfo {
            execution_trace,
            state_changes,
            gas_profile,
            error: None,
        };
        
        // デバッグ情報を保存
        self.debug_info.insert(transaction_id.to_string(), debug_info.clone());
        
        Ok((result, debug_info))
    }
    
    /// トークンコントラクトの呼び出し
    fn call_token_contract(
        &mut self,
        contract: &mut Contract,
        method: &str,
        args: Option<&str>,
        caller: &str,
    ) -> Result<String, String> {
        match method {
            "name" => {
                if let Some(name) = contract.get_state("name") {
                    Ok(name.clone())
                } else {
                    Err("Token name not set".to_string())
                }
            },
            "symbol" => {
                if let Some(symbol) = contract.get_state("symbol") {
                    Ok(symbol.clone())
                } else {
                    Err("Token symbol not set".to_string())
                }
            },
            "decimals" => {
                if let Some(decimals) = contract.get_state("decimals") {
                    Ok(decimals.clone())
                } else {
                    Ok("18".to_string()) // デフォルト値
                }
            },
            "totalSupply" => {
                if let Some(total_supply) = contract.get_state("totalSupply") {
                    Ok(total_supply.clone())
                } else {
                    Ok("0".to_string())
                }
            },
            "balanceOf" => {
                if let Some(account) = args {
                    let balance_key = format!("balanceOf:{}", account);
                    if let Some(balance) = contract.get_state(&balance_key) {
                        Ok(balance.clone())
                    } else {
                        Ok("0".to_string())
                    }
                } else {
                    Err("Missing account argument".to_string())
                }
            },
            "transfer" => {
                if let Some(args_str) = args {
                    let args_parts: Vec<&str> = args_str.split(',').collect();
                    if args_parts.len() != 2 {
                        return Err("Invalid arguments for transfer".to_string());
                    }
                    
                    let to = args_parts[0];
                    let amount = match args_parts[1].parse::<u64>() {
                        Ok(a) => a,
                        Err(_) => return Err("Invalid amount".to_string()),
                    };
                    
                    // 送信者の残高を確認
                    let sender_key = format!("balanceOf:{}", caller);
                    let sender_balance = match contract.get_state(&sender_key) {
                        Some(balance) => match balance.parse::<u64>() {
                            Ok(b) => b,
                            Err(_) => return Err("Invalid sender balance".to_string()),
                        },
                        None => 0,
                    };
                    
                    if sender_balance < amount {
                        return Err("Insufficient balance".to_string());
                    }
                    
                    // 送信者の残高を減らす
                    contract.set_state(sender_key, (sender_balance - amount).to_string());
                    
                    // 受信者の残高を増やす
                    let receiver_key = format!("balanceOf:{}", to);
                    let receiver_balance = match contract.get_state(&receiver_key) {
                        Some(balance) => match balance.parse::<u64>() {
                            Ok(b) => b,
                            Err(_) => return Err("Invalid receiver balance".to_string()),
                        },
                        None => 0,
                    };
                    
                    contract.set_state(receiver_key, (receiver_balance + amount).to_string());
                    
                    // イベントの発行
                    let mut args = HashMap::new();
                    args.insert("from".to_string(), caller.to_string());
                    args.insert("to".to_string(), to.to_string());
                    args.insert("value".to_string(), amount.to_string());
                    
                    contract.emit_event(
                        "Transfer".to_string(),
                        args,
                        0, // ブロック番号（仮）
                        "0x0".to_string(), // トランザクションID（仮）
                        0, // ログインデックス（仮）
                    );
                    
                    Ok("true".to_string())
                } else {
                    Err("Missing arguments for transfer".to_string())
                }
            },
            _ => Err(format!("Unknown method: {}", method)),
        }
    }
    
    /// NFTコントラクトの呼び出し
    fn call_nft_contract(
        &mut self,
        contract: &mut Contract,
        method: &str,
        args: Option<&str>,
        caller: &str,
    ) -> Result<String, String> {
        match method {
            "name" => {
                if let Some(name) = contract.get_state("name") {
                    Ok(name.clone())
                } else {
                    Err("NFT name not set".to_string())
                }
            },
            "symbol" => {
                if let Some(symbol) = contract.get_state("symbol") {
                    Ok(symbol.clone())
                } else {
                    Err("NFT symbol not set".to_string())
                }
            },
            "mint" => {
                if let Some(token_id_str) = args {
                    let token_id = match token_id_str.parse::<u64>() {
                        Ok(id) => id,
                        Err(_) => return Err("Invalid token ID".to_string()),
                    };
                    
                    // トークンIDの所有者を設定
                    let owner_key = format!("ownerOf:{}", token_id);
                    if contract.get_state(&owner_key).is_some() {
                        return Err(format!("Token ID {} already exists", token_id));
                    }
                    
                    contract.set_state(owner_key, caller.to_string());
                    
                    // 所有者のトークン数を増やす
                    let balance_key = format!("balanceOf:{}", caller);
                    let balance = match contract.get_state(&balance_key) {
                        Some(balance) => match balance.parse::<u64>() {
                            Ok(b) => b,
                            Err(_) => return Err("Invalid balance".to_string()),
                        },
                        None => 0,
                    };
                    
                    contract.set_state(balance_key, (balance + 1).to_string());
                    
                    // 総供給量を増やす
                    let total_supply = match contract.get_state("totalSupply") {
                        Some(supply) => match supply.parse::<u64>() {
                            Ok(s) => s,
                            Err(_) => return Err("Invalid total supply".to_string()),
                        },
                        None => 0,
                    };
                    
                    contract.set_state("totalSupply".to_string(), (total_supply + 1).to_string());
                    
                    // イベントの発行
                    let mut args = HashMap::new();
                    args.insert("to".to_string(), caller.to_string());
                    args.insert("tokenId".to_string(), token_id.to_string());
                    
                    contract.emit_event(
                        "Transfer".to_string(),
                        args,
                        0, // ブロック番号（仮）
                        "0x0".to_string(), // トランザクションID（仮）
                        0, // ログインデックス（仮）
                    );
                    
                    Ok("true".to_string())
                } else {
                    Err("Missing token ID argument".to_string())
                }
            },
            "ownerOf" => {
                if let Some(token_id_str) = args {
                    let owner_key = format!("ownerOf:{}", token_id_str);
                    if let Some(owner) = contract.get_state(&owner_key) {
                        Ok(owner.clone())
                    } else {
                        Err(format!("Token ID {} does not exist", token_id_str))
                    }
                } else {
                    Err("Missing token ID argument".to_string())
                }
            },
            "balanceOf" => {
                if let Some(account) = args {
                    let balance_key = format!("balanceOf:{}", account);
                    if let Some(balance) = contract.get_state(&balance_key) {
                        Ok(balance.clone())
                    } else {
                        Ok("0".to_string())
                    }
                } else {
                    Err("Missing account argument".to_string())
                }
            },
            _ => Err(format!("Unknown method: {}", method)),
        }
    }

    /// コントラクト情報を取得
    pub fn get_contract(&self, address: &str) -> Option<&Contract> {
        self.contracts.get(address)
    }

    /// すべてのコントラクトを取得
    pub fn get_all_contracts(&self) -> Vec<&Contract> {
        self.contracts.values().collect()
    }
    
    /// すべてのコントラクトをクローンして取得
    pub fn get_all_contracts_cloned(&self) -> Vec<Contract> {
        self.contracts.values().cloned().collect()
    }
    
    /// コントラクトを検証
    pub fn verify_contract(
        &mut self,
        address: &str,
        source_code: String,
        _compiler_version: &str,
        _optimization: bool,
    ) -> Result<(), String> {
        let contract = match self.contracts.get_mut(address) {
            Some(c) => c,
            None => return Err(format!("Contract not found: {}", address)),
        };
        
        // 実際の実装では、ソースコードのコンパイルと検証が必要
        // ここではシンプルに検証済みとしてマーク
        contract.verify(source_code.clone());
        self.verified_contracts.insert(address.to_string(), source_code);
        
        Ok(())
    }
    
    /// デバッグ情報を取得
    pub fn get_debug_info(&self, transaction_id: &str) -> Option<&DebugInfo> {
        self.debug_info.get(transaction_id)
    }
    
    /// コントラクトのイベントを取得
    pub fn get_contract_events(&self, address: &str) -> Vec<&ContractEvent> {
        match self.contracts.get(address) {
            Some(contract) => contract.events.iter().collect(),
            None => Vec::new(),
        }
    }
}