# WebSocket API Reference

Rustorium provides a WebSocket API for real-time updates and bidirectional communication. This document describes the available events and message formats.

## Connection

### Endpoint

The WebSocket endpoint is available at:
```
ws://localhost:9072/ws
```

### Authentication

Authentication is performed during the connection handshake using a query parameter:

```
ws://localhost:9072/ws?token=<API_KEY>
```

### Connection Status

The server sends heartbeat messages every 30 seconds:

```json
{
  "type": "heartbeat",
  "data": {
    "timestamp": 1706011200
  }
}
```

## Message Format

### Client to Server

```json
{
  "type": "string",
  "id": "string",
  "data": {}
}
```

### Server to Client

```json
{
  "type": "string",
  "id": "string",
  "data": {},
  "error": null
}
```

## Subscriptions

### Subscribe to Events

Request:
```json
{
  "type": "subscribe",
  "id": "sub1",
  "data": {
    "channel": "blocks",
    "filter": {
      "confirmations": 1
    }
  }
}
```

Response:
```json
{
  "type": "subscribe_success",
  "id": "sub1",
  "data": {
    "subscription_id": "abc123"
  }
}
```

### Unsubscribe from Events

Request:
```json
{
  "type": "unsubscribe",
  "id": "sub1",
  "data": {
    "subscription_id": "abc123"
  }
}
```

Response:
```json
{
  "type": "unsubscribe_success",
  "id": "sub1",
  "data": {}
}
```

## Events

### Block Events

New block notification:
```json
{
  "type": "new_block",
  "id": "block1",
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

### Transaction Events

Transaction confirmation:
```json
{
  "type": "tx_confirmed",
  "id": "tx1",
  "data": {
    "tx_hash": "0x5678...",
    "block_number": 12345,
    "timestamp": "2024-01-23T12:34:56Z",
    "confirmations": 1
  }
}
```

### State Events

State update:
```json
{
  "type": "state_update",
  "id": "state1",
  "data": {
    "key": "0x1234...",
    "value": "base64_encoded_value",
    "block_number": 12345,
    "timestamp": "2024-01-23T12:34:56Z"
  }
}
```

## Error Handling

### Connection Errors

| Code | Description | Action |
|------|-------------|--------|
| 4000 | Invalid token | Check API key |
| 4001 | Rate limited | Reduce connection attempts |
| 4002 | Invalid message | Check message format |
| 4003 | Subscription error | Check subscription parameters |

### Error Response

```json
{
  "type": "error",
  "id": "sub1",
  "data": null,
  "error": {
    "code": 4002,
    "message": "Invalid message format"
  }
}
```

## Rate Limiting

- Maximum 10 concurrent connections per API key
- Maximum 100 subscriptions per connection
- Maximum 1000 messages per minute per connection

## Examples

### JavaScript

```javascript
const ws = new WebSocket('ws://localhost:9072/ws?token=API_KEY');

ws.onopen = () => {
  console.log('Connected to Rustorium');
  
  // Subscribe to blocks
  ws.send(JSON.stringify({
    type: 'subscribe',
    id: 'sub1',
    data: {
      channel: 'blocks',
      filter: {
        confirmations: 1
      }
    }
  }));
};

ws.onmessage = (event) => {
  const message = JSON.parse(event.data);
  
  switch (message.type) {
    case 'new_block':
      console.log('New block:', message.data);
      break;
      
    case 'tx_confirmed':
      console.log('Transaction confirmed:', message.data);
      break;
      
    case 'state_update':
      console.log('State updated:', message.data);
      break;
      
    case 'error':
      console.error('Error:', message.error);
      break;
  }
};

ws.onerror = (error) => {
  console.error('WebSocket error:', error);
};

ws.onclose = (event) => {
  console.log('Connection closed:', event.code, event.reason);
  // Implement reconnection logic
};
```

### Rust

```rust
use rustorium_sdk::{WebSocketClient, Event};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let client = WebSocketClient::new()
        .with_endpoint("ws://localhost:9072/ws")
        .with_api_key(env::var("RUSTORIUM_API_KEY")?)
        .build()?;
        
    // Subscribe to events
    let mut events = client.subscribe_events(&["blocks", "transactions"]).await?;
    
    while let Some(event) = events.next().await {
        match event {
            Event::NewBlock(block) => {
                println!("New block: {}", block.number);
            }
            Event::TransactionConfirmed(tx) => {
                println!("Transaction confirmed: {}", tx.hash);
            }
            Event::StateUpdate(update) => {
                println!("State updated: {:?}", update);
            }
            Event::Error(error) => {
                eprintln!("Error: {}", error);
            }
        }
    }
    
    Ok(())
}
```

## Best Practices

1. **Connection Management**
   - Implement exponential backoff for reconnection
   - Monitor connection health with heartbeats
   - Clean up resources on disconnect

2. **Error Handling**
   - Handle all error types appropriately
   - Log errors for debugging
   - Implement retry logic where appropriate

3. **Performance**
   - Limit subscription count
   - Process messages asynchronously
   - Buffer messages when necessary

4. **Security**
   - Use secure WebSocket (wss://) in production
   - Rotate API keys regularly
   - Validate all incoming messages

## Support

If you need help with the WebSocket API:

1. Check the [FAQ](../appendix/faq.md)
2. Join our [Discord](https://discord.gg/rustorium)
3. Open an issue on [GitHub](https://github.com/enablerdao/rustorium/issues)
