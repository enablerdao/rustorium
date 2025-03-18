use crate::models::{
    Account, ApiResponse, Block, CreateTransactionRequest, NodeStatus, Transaction,
};
use anyhow::Result;
use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

const API_BASE_URL: &str = "/api";

/// API client for Rustorium
pub struct ApiClient;

impl ApiClient {
    /// Get node status
    pub async fn get_status() -> Result<NodeStatus> {
        Self::get::<NodeStatus>("/status").await
    }
    
    /// Get block by height
    pub async fn get_block(height: u64) -> Result<Block> {
        Self::get::<Block>(&format!("/blocks/{}", height)).await
    }
    
    /// Get blocks list
    pub async fn get_blocks(start: Option<u64>, limit: Option<u64>) -> Result<Vec<Block>> {
        let mut url = "/blocks".to_string();
        
        if start.is_some() || limit.is_some() {
            url.push('?');
            
            if let Some(start) = start {
                url.push_str(&format!("start={}", start));
                
                if limit.is_some() {
                    url.push('&');
                }
            }
            
            if let Some(limit) = limit {
                url.push_str(&format!("limit={}", limit));
            }
        }
        
        Self::get::<Vec<Block>>(&url).await
    }
    
    /// Get transaction by ID
    pub async fn get_transaction(tx_id: &str) -> Result<Transaction> {
        Self::get::<Transaction>(&format!("/transactions/{}", tx_id)).await
    }
    
    /// Get account by address
    pub async fn get_account(address: &str) -> Result<Account> {
        Self::get::<Account>(&format!("/accounts/{}", address)).await
    }
    
    /// Create a new transaction
    pub async fn create_transaction(request: CreateTransactionRequest) -> Result<Transaction> {
        Self::post::<CreateTransactionRequest, Transaction>("/transactions", &request).await
    }
    
    /// Generic GET request
    async fn get<T: DeserializeOwned>(endpoint: &str) -> Result<T> {
        let url = format!("{}{}", API_BASE_URL, endpoint);
        
        let response = Request::get(&url)
            .send()
            .await?;
        
        if !response.ok() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("API error: {}", error_text));
        }
        
        let api_response: ApiResponse<T> = response.json().await?;
        
        if !api_response.success {
            return Err(anyhow::anyhow!(
                "API returned error: {}",
                api_response.error.unwrap_or_else(|| "Unknown error".to_string())
            ));
        }
        
        api_response
            .data
            .ok_or_else(|| anyhow::anyhow!("API returned no data"))
    }
    
    /// Generic POST request
    async fn post<Req, Res>(endpoint: &str, data: &Req) -> Result<Res>
    where
        Req: serde::Serialize,
        Res: DeserializeOwned,
    {
        let url = format!("{}{}", API_BASE_URL, endpoint);
        
        let response = Request::post(&url)
            .json(data)?
            .send()
            .await?;
        
        if !response.ok() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("API error: {}", error_text));
        }
        
        let api_response: ApiResponse<Res> = response.json().await?;
        
        if !api_response.success {
            return Err(anyhow::anyhow!(
                "API returned error: {}",
                api_response.error.unwrap_or_else(|| "Unknown error".to_string())
            ));
        }
        
        api_response
            .data
            .ok_or_else(|| anyhow::anyhow!("API returned no data"))
    }
}