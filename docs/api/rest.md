# RESTful API

## Base URL

```
https://api.rustorium.com/v1
```

## Authentication

All API requests require an API key to be included in the `Authorization` header:

```http
Authorization: Bearer YOUR_API_KEY
```

## Endpoints

### Token Management

#### Create Token

```http
POST /tokens
```

Request body:
```json
{
    "name": "My Token",
    "symbol": "MTK",
    "decimals": 18,
    "total_supply": 1000000,
    "token_type": "standard",
    "economics": {
        "initial_price": 1.0,
        "inflation_rate": 0.02,
        "max_supply": 2000000,
        "fee": 100
    }
}
```

Response:
```json
{
    "token_id": "0x...",
    "status": "created",
    "transaction_hash": "0x..."
}
```

#### Get Token

```http
GET /tokens/{token_id}
```

Response:
```json
{
    "token_id": "0x...",
    "name": "My Token",
    "symbol": "MTK",
    "decimals": 18,
    "total_supply": 1000000,
    "current_supply": 1000000,
    "price": 1.0,
    "holders": 100
}
```

#### List Tokens

```http
GET /tokens
```

Query parameters:
- `page`: Page number (default: 1)
- `limit`: Items per page (default: 10)
- `type`: Token type filter
- `sort`: Sort field (name, symbol, supply)
- `order`: Sort order (asc, desc)

Response:
```json
{
    "tokens": [
        {
            "token_id": "0x...",
            "name": "My Token",
            "symbol": "MTK",
            "total_supply": 1000000
        }
    ],
    "pagination": {
        "current_page": 1,
        "total_pages": 10,
        "total_items": 100
    }
}
```

### Transactions

#### Submit Transaction

```http
POST /transactions
```

Request body:
```json
{
    "from": "0x...",
    "to": "0x...",
    "amount": "1000",
    "token_id": "0x...",
    "fee": "10"
}
```

Response:
```json
{
    "transaction_id": "0x...",
    "status": "pending",
    "timestamp": "2024-01-01T12:00:00Z"
}
```

#### Get Transaction

```http
GET /transactions/{transaction_id}
```

Response:
```json
{
    "transaction_id": "0x...",
    "from": "0x...",
    "to": "0x...",
    "amount": "1000",
    "token_id": "0x...",
    "fee": "10",
    "status": "confirmed",
    "timestamp": "2024-01-01T12:00:00Z",
    "block_number": 12345
}
```

### Account Management

#### Get Balance

```http
GET /accounts/{address}/balance
```

Query parameters:
- `token_id`: Token ID (optional)

Response:
```json
{
    "address": "0x...",
    "balances": [
        {
            "token_id": "0x...",
            "symbol": "MTK",
            "balance": "1000",
            "value_usd": "1000.00"
        }
    ]
}
```

#### Get Transaction History

```http
GET /accounts/{address}/transactions
```

Query parameters:
- `page`: Page number
- `limit`: Items per page
- `token_id`: Filter by token
- `type`: Transaction type filter

Response:
```json
{
    "transactions": [
        {
            "transaction_id": "0x...",
            "type": "transfer",
            "amount": "1000",
            "token_id": "0x...",
            "timestamp": "2024-01-01T12:00:00Z"
        }
    ],
    "pagination": {
        "current_page": 1,
        "total_pages": 10,
        "total_items": 100
    }
}
```

## Error Handling

All errors follow this format:

```json
{
    "error": {
        "code": "invalid_request",
        "message": "Detailed error message",
        "details": {
            "field": "specific_field",
            "reason": "validation_failed"
        }
    }
}
```

Common error codes:
- `400`: Invalid request
- `401`: Unauthorized
- `403`: Forbidden
- `404`: Not found
- `429`: Too many requests
- `500`: Internal server error

## Rate Limiting

Rate limits are applied per API key and are included in response headers:

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1640995200
```

## Pagination

All list endpoints support pagination with these parameters:
- `page`: Page number (1-based)
- `limit`: Items per page (max 100)

Response includes pagination metadata:
```json
{
    "data": [...],
    "pagination": {
        "current_page": 1,
        "total_pages": 10,
        "total_items": 100,
        "has_next": true,
        "has_previous": false
    }
}
```

## Versioning

The API is versioned through the URL path. Breaking changes will result in a new version number.

Example versions:
- `/v1/`: Current stable version
- `/v2/`: Next major version (when available)
- `/beta/`: Preview of upcoming features