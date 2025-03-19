use serde::{Deserialize, Serialize};
use std::fmt;

/// アドレス
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Address(pub [u8; 32]);

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

impl Address {
    /// 新しいアドレスを作成
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// 文字列からアドレスを作成
    pub fn from_hex(s: &str) -> Result<Self, hex::FromHexError> {
        let s = s.trim_start_matches("0x");
        let mut bytes = [0u8; 32];
        hex::decode_to_slice(s, &mut bytes)?;
        Ok(Self(bytes))
    }

    /// アドレスをバイト列として取得
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// アドレスを16進数文字列として取得
    pub fn to_hex(&self) -> String {
        format!("0x{}", hex::encode(self.0))
    }
}