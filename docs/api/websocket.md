# WebSocket API

## Connection

```
wss://ws.rustorium.com/v1
```

## Authentication

Connect with your API key in the URL:

```
wss://ws.rustorium.com/v1?api_key=YOUR_API_KEY
```

## Message Format

All messages follow this format:

```json
{
    "id": "unique_message_id",
    "type": "message_type",
    "data": {}
}
```

## Subscriptions

### Subscribe to Updates

Request:
```json
{
    "id": "1",
    "type": "subscribe",
    "data": {
        "channel": "token_updates",
        "token_id": "0x..."
    }
}
```

Response:
```json
{
    "id": "1",
    "type": "subscription_confirmation",
    "data": {
        "status": "subscribed",
        "channel": "token_updates",
        "subscription_id": "sub_123"
    }
}
```

### Available Channels

#### Token Updates

```json
{
    "type": "subscribe",
    "data": {
        "channel": "token_updates",
        "token_id": "0x..."
    }
}
```

Updates:
```json
{
    "type": "update",
    "data": {
        "token_id": "0x...",
        "price": "1.23",
        "volume": "1000000",
        "timestamp": "2024-01-01T12:00:00Z"
    }
}
```

#### Transaction Updates

```json
{
    "type": "subscribe",
    "data": {
        "channel": "transaction_updates",
        "address": "0x..."
    }
}
```

Updates:
```json
{
    "type": "update",
    "data": {
        "transaction_id": "0x...",
        "status": "confirmed",
        "timestamp": "2024-01-01T12:00:00Z"
    }
}
```

#### Market Data

```json
{
    "type": "subscribe",
    "data": {
        "channel": "market_data",
        "symbols": ["MTK/USD", "STK/USD"]
    }
}
```

Updates:
```json
{
    "type": "update",
    "data": {
        "symbol": "MTK/USD",
        "price": "1.23",
        "change_24h": "5.2",
        "volume_24h": "1000000",
        "timestamp": "2024-01-01T12:00:00Z"
    }
}
```

## Commands

### Execute Transaction

Request:
```json
{
    "id": "2",
    "type": "execute_transaction",
    "data": {
        "from": "0x...",
        "to": "0x...",
        "amount": "1000",
        "token_id": "0x..."
    }
}
```

Response:
```json
{
    "id": "2",
    "type": "transaction_response",
    "data": {
        "transaction_id": "0x...",
        "status": "pending"
    }
}
```

### Get Token Info

Request:
```json
{
    "id": "3",
    "type": "get_token_info",
    "data": {
        "token_id": "0x..."
    }
}
```

Response:
```json
{
    "id": "3",
    "type": "token_info",
    "data": {
        "token_id": "0x...",
        "name": "My Token",
        "symbol": "MTK",
        "total_supply": "1000000"
    }
}
```

## Error Handling

Errors are sent in this format:

```json
{
    "id": "request_id",
    "type": "error",
    "data": {
        "code": "invalid_request",
        "message": "Detailed error message"
    }
}
```

## Connection Management

### Heartbeat

The server sends heartbeat messages every 30 seconds:

```json
{
    "type": "heartbeat",
    "data": {
        "timestamp": "2024-01-01T12:00:00Z"
    }
}
```

Clients should respond with:

```json
{
    "type": "heartbeat_response",
    "data": {
        "timestamp": "2024-01-01T12:00:00Z"
    }
}
```

### Reconnection

If the connection is lost:
1. Wait for 1 second before first retry
2. Use exponential backoff for subsequent retries
3. Maximum retry interval: 30 seconds
4. Include last received message ID in reconnection

```json
{
    "type": "reconnect",
    "data": {
        "last_message_id": "msg_123",
        "subscriptions": ["sub_123", "sub_456"]
    }
}
```

## Rate Limiting

- Maximum 100 subscriptions per connection
- Maximum 10 messages per second
- Excess messages will trigger a rate limit error

## Examples

### Node.js Example

```javascript
const WebSocket = require('ws');

const ws = new WebSocket('wss://ws.rustorium.com/v1?api_key=YOUR_API_KEY');

ws.on('open', () => {
    // Subscribe to token updates
    ws.send(JSON.stringify({
        id: '1',
        type: 'subscribe',
        data: {
            channel: 'token_updates',
            token_id: '0x...'
        }
    }));
});

ws.on('message', (data) => {
    const message = JSON.parse(data);
    console.log('Received:', message);
});

ws.on('close', () => {
    console.log('Connection closed');
});
```

### Python Example

```python
import websockets
import json
import asyncio

async def connect():
    uri = "wss://ws.rustorium.com/v1?api_key=YOUR_API_KEY"
    async with websockets.connect(uri) as websocket:
        # Subscribe to token updates
        await websocket.send(json.dumps({
            "id": "1",
            "type": "subscribe",
            "data": {
                "channel": "token_updates",
                "token_id": "0x..."
            }
        }))

        while True:
            message = await websocket.recv()
            print(f"Received: {json.loads(message)}")

asyncio.get_event_loop().run_until_complete(connect())
```