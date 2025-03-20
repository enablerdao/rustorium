pub mod models;

use anyhow::Result;
use models::{NetworkStatus, NodeStats, Block, Transaction, Account, Contract, Token};
use reqwest::{Client, StatusCode};
use serde_json::json;
use std::time::Duration;

/// API client for interacting with the Rustorium API
pub struct ApiClient {
    /// HTTP client
    client: Client,
    /// API base URL
    base_url: String,
}

impl ApiClient {
    /// Create a new API client
    pub fn new(base_url: &str) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");
        
        Self {
            client,
            base_url: base_url.to_string(),
        }
    }
    
    /// Check if the API is reachable
    pub async fn check_connection(&self) -> Result<()> {
        let url = format!("{}/network/status", self.base_url);
        let response = self.client.get(&url).send().await?;
        
        if response.status() != StatusCode::OK {
            anyhow::bail!("API returned status code: {}", response.status());
        }
        
        Ok(())
    }
    
    /// Get network status
    pub async fn get_network_status(&self) -> Result<NetworkStatus> {
        let url = format!("{}/network/status", self.base_url);
        let response = self.client.get(&url).send().await?;
        
        if response.status() != StatusCode::OK {
            anyhow::bail!("API returned status code: {}", response.status());
        }
        
        let data = response.json::<serde_json::Value>().await?;
        let status = serde_json::from_value(data["data"].clone())?;
        
        Ok(status)
    }
    
    /// Get node stats
    pub async fn get_node_stats(&self) -> Result<NodeStats> {
        let url = format!("{}/system/stats", self.base_url);
        let response = self.client.get(&url).send().await?;
        
        if response.status() != StatusCode::OK {
            anyhow::bail!("API returned status code: {}", response.status());
        }
        
        let data = response.json::<serde_json::Value>().await?;
        let stats = serde_json::from_value(data["data"].clone())?;
        
        Ok(stats)
    }
    
    /// Get block by number or hash
    pub async fn get_block(&self, id: &str) -> Result<Block> {
        let url = format!("{}/blocks/{}", self.base_url, id);
        let response = self.client.get(&url).send().await?;
        
        if response.status() != StatusCode::OK {
            anyhow::bail!("API returned status code: {}", response.status());
        }
        
        let data = response.json::<serde_json::Value>().await?;
        let block = serde_json::from_value(data["data"].clone())?;
        
        Ok(block)
    }
    
    /// Get latest block
    pub async fn get_latest_block(&self) -> Result<Block> {
        let url = format!("{}/blocks/latest", self.base_url);
        let response = self.client.get(&url).send().await?;
        
        if response.status() != StatusCode::OK {
            anyhow::bail!("API returned status code: {}", response.status());
        }
        
        let data = response.json::<serde_json::Value>().await?;
        let block = serde_json::from_value(data["data"].clone())?;
        
        Ok(block)
    }
    
    /// Get blocks
    pub async fn get_blocks(&self, limit: usize, offset: usize) -> Result<Vec<Block>> {
        let url = format!("{}/blocks?limit={}&offset={}", self.base_url, limit, offset);
        let response = self.client.get(&url).send().await?;
        
        if response.status() != StatusCode::OK {
            anyhow::bail!("API returned status code: {}", response.status());
        }
        
        let data = response.json::<serde_json::Value>().await?;
        let blocks = serde_json::from_value(data["data"].clone())?;
        
        Ok(blocks)
    }
    
    /// Get transaction by ID
    pub async fn get_transaction(&self, id: &str) -> Result<Transaction> {
        let url = format!("{}/transactions/{}", self.base_url, id);
        let response = self.client.get(&url).send().await?;
        
        if response.status() != StatusCode::OK {
            anyhow::bail!("API returned status code: {}", response.status());
        }
        
        let data = response.json::<serde_json::Value>().await?;
        let tx = serde_json::from_value(data["data"].clone())?;
        
        Ok(tx)
    }
    
    /// Get transactions
    pub async fn get_transactions(&self, limit: usize, offset: usize) -> Result<Vec<Transaction>> {
        let url = format!("{}/transactions?limit={}&offset={}", self.base_url, limit, offset);
        let response = self.client.get(&url).send().await?;
        
        if response.status() != StatusCode::OK {
            anyhow::bail!("API returned status code: {}", response.status());
        }
        
        let data = response.json::<serde_json::Value>().await?;
        let txs = serde_json::from_value(data["data"].clone())?;
        
        Ok(txs)
    }
    
    /// Create transaction
    pub async fn create_transaction(&self, from: &str, to: &str, value: f64) -> Result<Transaction> {
        let url = format!("{}/transactions", self.base_url);
        let payload = json!({
            "from": from,
            "to": to,
            "value": value
        });
        
        let response = self.client.post(&url).json(&payload).send().await?;
        
        if response.status() != StatusCode::OK && response.status() != StatusCode::CREATED {
            anyhow::bail!("API returned status code: {}", response.status());
        }
        
        let data = response.json::<serde_json::Value>().await?;
        let tx = serde_json::from_value(data["data"].clone())?;
        
        Ok(tx)
    }
    
    /// Get account by address
    pub async fn get_account(&self, address: &str) -> Result<Account> {
        let url = format!("{}/accounts/{}", self.base_url, address);
        let response = self.client.get(&url).send().await?;
        
        if response.status() != StatusCode::OK {
            anyhow::bail!("API returned status code: {}", response.status());
        }
        
        let data = response.json::<serde_json::Value>().await?;
        let account = serde_json::from_value(data["data"].clone())?;
        
        Ok(account)
    }
    
    /// Create account
    pub async fn create_account(&self) -> Result<Account> {
        let url = format!("{}/accounts", self.base_url);
        let response = self.client.post(&url).send().await?;
        
        if response.status() != StatusCode::OK && response.status() != StatusCode::CREATED {
            anyhow::bail!("API returned status code: {}", response.status());
        }
        
        let data = response.json::<serde_json::Value>().await?;
        let account = serde_json::from_value(data["data"].clone())?;
        
        Ok(account)
    }
    
    /// Get accounts
    pub async fn get_accounts(&self, limit: usize, offset: usize) -> Result<Vec<Account>> {
        let url = format!("{}/accounts?limit={}&offset={}", self.base_url, limit, offset);
        let response = self.client.get(&url).send().await?;
        
        if response.status() != StatusCode::OK {
            anyhow::bail!("API returned status code: {}", response.status());
        }
        
        let data = response.json::<serde_json::Value>().await?;
        let accounts = serde_json::from_value(data["data"].clone())?;
        
        Ok(accounts)
    }
    
    /// Get contract by address
    pub async fn get_contract(&self, address: &str) -> Result<Contract> {
        let url = format!("{}/contracts/{}", self.base_url, address);
        let response = self.client.get(&url).send().await?;
        
        if response.status() != StatusCode::OK {
            anyhow::bail!("API returned status code: {}", response.status());
        }
        
        let data = response.json::<serde_json::Value>().await?;
        let contract = serde_json::from_value(data["data"].clone())?;
        
        Ok(contract)
    }
    
    /// Deploy contract
    pub async fn deploy_contract(&self, from: &str, bytecode: &str, abi: Option<&str>) -> Result<Contract> {
        let url = format!("{}/contracts", self.base_url);
        let payload = json!({
            "from": from,
            "bytecode": bytecode,
            "abi": abi,
            "gas_limit": 3000000,
            "gas_price": 10
        });
        
        let response = self.client.post(&url).json(&payload).send().await?;
        
        if response.status() != StatusCode::OK && response.status() != StatusCode::CREATED {
            anyhow::bail!("API returned status code: {}", response.status());
        }
        
        let data = response.json::<serde_json::Value>().await?;
        let contract = serde_json::from_value(data["data"].clone())?;
        
        Ok(contract)
    }
    
    /// Call contract
    pub async fn call_contract(&self, address: &str, from: &str, method: &str, args: Option<&str>) -> Result<String> {
        let url = format!("{}/contracts/{}/call", self.base_url, address);
        let payload = json!({
            "from": from,
            "method": method,
            "args": args,
            "gas_limit": 1000000,
            "gas_price": 10,
            "value": 0
        });
        
        let response = self.client.post(&url).json(&payload).send().await?;
        
        if response.status() != StatusCode::OK {
            anyhow::bail!("API returned status code: {}", response.status());
        }
        
        let data = response.json::<serde_json::Value>().await?;
        let result = data["data"]["result"].as_str().unwrap_or("").to_string();
        
        Ok(result)
    }
    
    /// Get contracts
    pub async fn get_contracts(&self, limit: usize, offset: usize) -> Result<Vec<Contract>> {
        let url = format!("{}/contracts?limit={}&offset={}", self.base_url, limit, offset);
        let response = self.client.get(&url).send().await?;
        
        if response.status() != StatusCode::OK {
            anyhow::bail!("API returned status code: {}", response.status());
        }
        
        let data = response.json::<serde_json::Value>().await?;
        let contracts = serde_json::from_value(data["data"].clone())?;
        
        Ok(contracts)
    }
    
    /// Create token
    pub async fn create_token(&self, from: &str, name: &str, symbol: &str, token_type: &str, supply: Option<u64>) -> Result<Token> {
        let url = format!("{}/contracts/token/create", self.base_url);
        let payload = json!({
            "from": from,
            "name": name,
            "symbol": symbol,
            "token_type": token_type,
            "initial_supply": supply,
            "gas_limit": 3000000,
            "gas_price": 10
        });
        
        let response = self.client.post(&url).json(&payload).send().await?;
        
        if response.status() != StatusCode::OK && response.status() != StatusCode::CREATED {
            anyhow::bail!("API returned status code: {}", response.status());
        }
        
        let data = response.json::<serde_json::Value>().await?;
        let token = serde_json::from_value(data["data"].clone())?;
        
        Ok(token)
    }
    
    /// Get tokens
    pub async fn get_tokens(&self, limit: usize, offset: usize) -> Result<Vec<Token>> {
        let url = format!("{}/tokens?limit={}&offset={}", self.base_url, limit, offset);
        let response = self.client.get(&url).send().await?;
        
        if response.status() != StatusCode::OK {
            anyhow::bail!("API returned status code: {}", response.status());
        }
        
        let data = response.json::<serde_json::Value>().await?;
        let tokens = serde_json::from_value(data["data"].clone())?;
        
        Ok(tokens)
    }
}