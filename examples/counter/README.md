# Counter Example

This is a simple counter application built with Rustorium. It demonstrates basic smart contract development, frontend integration, and testing.

## Features

- Smart contract with state management
- React frontend with TypeScript
- Real-time updates via WebSocket
- Comprehensive test suite
- Deployment scripts

## Project Structure

```
counter/
├── contracts/
│   └── counter.rs       # Smart contract
├── frontend/
│   ├── src/
│   │   ├── App.tsx     # Main component
│   │   └── main.tsx    # Entry point
│   ├── package.json
│   └── vite.config.ts
├── tests/
│   ├── contract.rs     # Contract tests
│   └── e2e.rs         # E2E tests
├── Cargo.toml
└── README.md
```

## Quick Start

1. Install dependencies:
```bash
# Install Rustorium
curl -sSf https://raw.githubusercontent.com/enablerdao/rustorium/main/scripts/install.sh | bash

# Install frontend dependencies
cd frontend
npm install
```

2. Start development node:
```bash
cargo run -- --dev
```

3. Deploy contract:
```bash
rustorium contract deploy \
  --contract Counter \
  --args '[]'
```

4. Start frontend:
```bash
cd frontend
npm run dev
```

5. Open application:
- Frontend: http://localhost:5173
- Node API: http://localhost:9071
- WebSocket: ws://localhost:9072

## Development

### Smart Contract

The counter contract is defined in `contracts/counter.rs`:

```rust
use rustorium_sdk::{Contract, State};

#[derive(Contract)]
pub struct Counter {
    count: State<i32>,
}

#[contract]
impl Counter {
    pub fn new() -> Self {
        Self {
            count: State::new(0),
        }
    }

    pub fn increment(&mut self) {
        *self.count += 1;
    }

    pub fn decrement(&mut self) {
        *self.count -= 1;
    }

    pub fn get_count(&self) -> i32 {
        *self.count
    }
}
```

### Frontend

The frontend is built with React and TypeScript:

```tsx
import { useState, useEffect } from 'react';
import { useRustorium, useContract } from '@rustorium/react';
import { Counter } from '../contracts/counter';

function App() {
  const { isConnected, connect } = useRustorium();
  const [count, setCount] = useState<number>(0);
  const counter = useContract<Counter>(COUNTER_ADDRESS);

  useEffect(() => {
    if (counter) {
      updateCount();
    }
  }, [counter]);

  async function updateCount() {
    const value = await counter.get_count();
    setCount(value);
  }

  async function handleIncrement() {
    await counter.increment();
    await updateCount();
  }

  async function handleDecrement() {
    await counter.decrement();
    await updateCount();
  }

  if (!isConnected) {
    return (
      <button onClick={connect}>
        Connect to Rustorium
      </button>
    );
  }

  return (
    <div>
      <h1>Counter: {count}</h1>
      <button onClick={handleIncrement}>+</button>
      <button onClick={handleDecrement}>-</button>
    </div>
  );
}

export default App;
```

### Testing

Run contract tests:
```bash
cargo test
```

Run frontend tests:
```bash
cd frontend
npm test
```

Run E2E tests:
```bash
cargo test --test e2e
```

## Deployment

### Contract Deployment

1. Configure network:
```bash
export RUSTORIUM_NETWORK=mainnet
export RUSTORIUM_PRIVATE_KEY=0x1234...
```

2. Deploy contract:
```bash
rustorium contract deploy \
  --network mainnet \
  --contract Counter \
  --args '[]'
```

### Frontend Deployment

1. Build frontend:
```bash
cd frontend
npm run build
```

2. Deploy to Vercel:
```bash
vercel deploy dist
```

## Configuration

### Environment Variables

```env
# Node
RUSTORIUM_NETWORK=mainnet
RUSTORIUM_PRIVATE_KEY=0x1234...

# Frontend
VITE_RUSTORIUM_NODE=http://localhost:9070
VITE_COUNTER_ADDRESS=0x5678...
```

### Network Configuration

```toml
# config.toml
[node]
name = "counter-node"
role = "validator"

[network]
port = 9070

[validator]
stake = 1000000
```

## Monitoring

### Metrics

The following metrics are available at `http://localhost:9071/metrics`:

```
# HELP rustorium_counter_total Total number of counter operations
# TYPE rustorium_counter_total counter
rustorium_counter_total{operation="increment"} 42
rustorium_counter_total{operation="decrement"} 21

# HELP rustorium_counter_value Current counter value
# TYPE rustorium_counter_value gauge
rustorium_counter_value 21
```

### Logging

```bash
# View logs
journalctl -u rustorium -f

# Filter errors
journalctl -u rustorium -p err
```

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

If you need help:

1. Check the [FAQ](../docs/faq.md)
2. Join our [Discord](https://discord.gg/rustorium)
3. Open an issue on [GitHub](https://github.com/enablerdao/rustorium/issues)
