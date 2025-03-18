use chrono::{DateTime, NaiveDateTime, Utc};

/// Format timestamp as a human-readable date
pub fn format_timestamp(timestamp: u64) -> String {
    let naive = NaiveDateTime::from_timestamp_opt(timestamp as i64, 0).unwrap_or_default();
    let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive, Utc);
    datetime.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

/// Format amount with commas
pub fn format_amount(amount: u64) -> String {
    let amount_str = amount.to_string();
    let mut result = String::new();
    let len = amount_str.len();
    
    for (i, c) in amount_str.chars().enumerate() {
        result.push(c);
        if (len - i - 1) % 3 == 0 && i < len - 1 {
            result.push(',');
        }
    }
    
    result
}

/// Truncate address or hash for display
pub fn truncate_hash(hash: &str, length: usize) -> String {
    if hash.len() <= length * 2 {
        return hash.to_string();
    }
    
    let prefix = &hash[0..length];
    let suffix = &hash[hash.len() - length..];
    format!("{}...{}", prefix, suffix)
}

/// Format address for display
pub fn format_address(address: &str) -> String {
    truncate_hash(address, 8)
}

/// Format transaction ID for display
pub fn format_tx_id(tx_id: &str) -> String {
    truncate_hash(tx_id, 10)
}

/// Format block hash for display
pub fn format_block_hash(hash: &str) -> String {
    truncate_hash(hash, 10)
}