# REST API Reference

Rustorium provides a comprehensive REST API for interacting with the node. This document describes all available endpoints and their usage.

## API Basics

### Base URL

The API is available at:
```
http://localhost:9071/api/v1
```

### Authentication

Most endpoints require authentication using an API key:

```http
Authorization: Bearer <API_KEY>
```

### Response Format

All responses are in JSON format and follow this structure:

```json
{
  "success": true,
  "data": { ... },
  "error": null
}
```

Or in case of an error:

```json
{
  "success": false,
  "data": null,
  "error": {
    "code": "ERROR_CODE",
    "message": "Error description"
  }
}
```

## Endpoints

### Node Information

#### Get Node Status
```http
GET /node/status
```

Response:
```json
{
  "success": true,
  "data": {
    "version": "0.1.0",
    "network": "mainnet",
    "peers": 10,
    "synced": true,
    "latest_block": 12345,
    "uptime": 3600
  }
}
```

#### Get Node Metrics
```http
GET /node/metrics
```

Response:
```json
{
  "success": true,
  "data": {
    "tps": 1000,
    "latency": 50,
    "memory_usage": 1024,
    "cpu_usage": 50,
    "disk_usage": 10240
  }
}
```

### Transactions

#### Submit Transaction
```http
POST /transactions
Content-Type: application/json

{
  "to": "0x1234...",
  "value": 100,
  "data": "base64_encoded_data"
}
```

Response:
```json
{
  "success": true,
  "data": {
    "tx_hash": "0x5678...",
    "status": "pending",
    "timestamp": "2024-01-23T12:34:56Z"
  }
}
```

#### Get Transaction Status
```http
GET /transactions/{tx_hash}
```

Response:
```json
{
  "success": true,
  "data": {
    "tx_hash": "0x5678...",
    "status": "confirmed",
    "block_number": 12345,
    "timestamp": "2024-01-23T12:34:56Z",
    "confirmations": 10
  }
}
```

#### List Transactions
```http
GET /transactions?limit=10&offset=0&status=confirmed
```

Response:
```json
{
  "success": true,
  "data": {
    "transactions": [
      {
        "tx_hash": "0x5678...",
        "status": "confirmed",
        "block_number": 12345,
        "timestamp": "2024-01-23T12:34:56Z"
      }
    ],
    "total": 100,
    "limit": 10,
    "offset": 0
  }
}
```

### Blocks

#### Get Latest Block
```http
GET /blocks/latest
```

Response:
```json
{
  "success": true,
  "data": {
    "number": 12345,
    "hash": "0x9abc...",
    "parent_hash": "0xdef0...",
    "timestamp": "2024-01-23T12:34:56Z",
    "transactions": [
      "0x5678..."
    ]
  }
}
```

#### Get Block by Number
```http
GET /blocks/{block_number}
```

Response:
```json
{
  "success": true,
  "data": {
    "number": 12345,
    "hash": "0x9abc...",
    "parent_hash": "0xdef0...",
    "timestamp": "2024-01-23T12:34:56Z",
    "transactions": [
      "0x5678..."
    ]
  }
}
```

### State

#### Get State
```http
GET /state/{key}
```

Response:
```json
{
  "success": true,
  "data": {
    "key": "0x1234...",
    "value": "base64_encoded_value",
    "proof": {
      "root": "0x5678...",
      "proof": [
        "0x9abc...",
        "0xdef0..."
      ]
    }
  }
}
```

#### Update State
```http
PUT /state/{key}
Content-Type: application/json

{
  "value": "base64_encoded_value"
}
```

Response:
```json
{
  "success": true,
  "data": {
    "tx_hash": "0x5678...",
    "status": "pending"
  }
}
```

## Error Codes

| Code | Description | Solution |
|------|-------------|----------|
| 1000 | Invalid request | Check request parameters |
| 1001 | Authentication failed | Verify API key |
| 1002 | Permission denied | Check permissions |
| 2000 | Transaction error | Verify transaction data |
| 2001 | Block not found | Check block number |
| 3000 | Node error | Check node status |
| 5000 | Internal error | Contact support |

## Rate Limiting

The API implements rate limiting:

- 1000 requests per minute for GET endpoints
- 100 requests per minute for POST/PUT endpoints
- 10 requests per minute for heavy operations

Rate limit headers are included in responses:

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1706011200
```

## SDKs

Official SDKs are available for:

- [Rust](https://github.com/enablerdao/rustorium-rs)
- [TypeScript](https://github.com/enablerdao/rustorium-ts)
- [Python](https://github.com/enablerdao/rustorium-py)
- [Go](https://github.com/enablerdao/rustorium-go)

## Examples

### Rust
```rust
use rustorium_sdk::{Client, Transaction};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new()
        .with_endpoint("http://localhost:9071")
        .with_api_key(env::var("RUSTORIUM_API_KEY")?)
        .build()?;

    let tx = Transaction::new()
        .with_data(data)
        .with_location(location)
        .build()?;

    let receipt = client.submit_transaction(tx).await?;
    println!("Transaction submitted: {}", receipt.hash);
}
```

### TypeScript
```typescript
import { Client, Transaction } from '@rustorium/sdk';

async function main() {
    const client = new Client({
        endpoint: 'http://localhost:9071',
        apiKey: process.env.RUSTORIUM_API_KEY,
    });

    const tx = new Transaction()
        .withData(data)
        .withLocation(location)
        .build();

    const receipt = await client.submitTransaction(tx);
    console.log('Transaction submitted:', receipt.hash);
}
```

## Support

If you need help with the API:

1. Check the [FAQ](../appendix/faq.md)
2. Join our [Discord](https://discord.gg/rustorium)
3. Open an issue on [GitHub](https://github.com/enablerdao/rustorium/issues)
