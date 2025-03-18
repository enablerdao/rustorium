use crate::common::types::{Address, TransactionId};
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

/// Generate a random transaction ID
pub fn random_transaction_id() -> TransactionId {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill(&mut bytes);
    TransactionId::new(bytes)
}

/// Generate a random address
pub fn random_address() -> Address {
    let mut bytes = [0u8; 20];
    rand::thread_rng().fill(&mut bytes);
    Address(bytes)
}

/// Get current timestamp in milliseconds
pub fn current_time_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}

/// Get current timestamp in seconds
pub fn current_time_sec() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

/// Calculate merkle root from a list of hashes
pub fn calculate_merkle_root(hashes: &[[u8; 32]]) -> [u8; 32] {
    if hashes.is_empty() {
        return [0; 32];
    }
    
    if hashes.len() == 1 {
        return hashes[0];
    }
    
    let mut next_level = Vec::new();
    
    for chunk in hashes.chunks(2) {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&chunk[0]);
        
        if chunk.len() > 1 {
            hasher.update(&chunk[1]);
        } else {
            hasher.update(&chunk[0]); // Duplicate the last hash if odd number
        }
        
        next_level.push(hasher.finalize().into());
    }
    
    calculate_merkle_root(&next_level)
}

/// Convert bytes to hex string
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

/// Convert hex string to bytes
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(hex.trim_start_matches("0x"))
}

/// Truncate a string to a maximum length with ellipsis
pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[0..max_len - 3])
    }
}

/// Format a byte size with appropriate units (KB, MB, GB)
pub fn format_byte_size(bytes: usize) -> String {
    const KB: usize = 1024;
    const MB: usize = KB * 1024;
    const GB: usize = MB * 1024;
    
    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}