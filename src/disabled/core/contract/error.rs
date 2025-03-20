use thiserror::Error;

/// コントラクトのエラー
#[derive(Debug, Error)]
pub enum ContractError {
    /// 初期化エラー
    #[error("Failed to initialize contract: {0}")]
    InitializationError(String),

    /// 実行エラー
    #[error("Failed to execute contract: {0}")]
    ExecutionError(String),

    /// 権限エラー
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// 残高不足
    #[error("Insufficient balance: {0}")]
    InsufficientBalance(String),

    /// ガス不足
    #[error("Out of gas: {0}")]
    OutOfGas(String),

    /// 無効なパラメータ
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// 無効な状態
    #[error("Invalid state: {0}")]
    InvalidState(String),

    /// 無効なアドレス
    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    /// 無効なトークン
    #[error("Invalid token: {0}")]
    InvalidToken(String),

    /// 無効な関数
    #[error("Invalid function: {0}")]
    InvalidFunction(String),

    /// タイムアウト
    #[error("Timeout: {0}")]
    Timeout(String),

    /// 内部エラー
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<anyhow::Error> for ContractError {
    fn from(err: anyhow::Error) -> Self {
        ContractError::InternalError(err.to_string())
    }
}

impl From<bincode::Error> for ContractError {
    fn from(err: bincode::Error) -> Self {
        ContractError::InternalError(err.to_string())
    }
}

impl From<std::io::Error> for ContractError {
    fn from(err: std::io::Error) -> Self {
        ContractError::InternalError(err.to_string())
    }
}