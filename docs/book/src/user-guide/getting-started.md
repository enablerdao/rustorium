# Getting Started

This guide will help you get started with Rustorium. We'll cover installation, basic configuration, and running your first node.

## Prerequisites

Before you begin, ensure you have:

- A Linux, macOS, or Windows system
- Rust 1.75.0 or later
- CMake 3.20 or later
- OpenSSL 1.1 or later

## Installation

### Using the Install Script

The easiest way to install Rustorium is using our install script:

```bash
curl -sSf https://raw.githubusercontent.com/enablerdao/rustorium/main/scripts/install.sh | bash
```

### Building from Source

If you prefer to build from source:

```bash
# Clone the repository
git clone https://github.com/enablerdao/rustorium.git
cd rustorium

# Build the project
cargo build --release

# Install the binary
sudo cp target/release/rustorium /usr/local/bin/
```

## Quick Start

### Development Mode

To start Rustorium in development mode:

```bash
rustorium --dev
```

This will:
1. Create a development data directory
2. Start a single node
3. Enable automatic block production
4. Start the Web UI, API, and WebSocket servers

### Production Mode

For production use, you'll need a configuration file:

```bash
# Create a configuration directory
sudo mkdir -p /etc/rustorium

# Copy the example config
sudo cp config/production.toml.example /etc/rustorium/config.toml

# Edit the configuration
sudo nano /etc/rustorium/config.toml

# Start the node
rustorium --config /etc/rustorium/config.toml
```

## Verifying the Installation

Once Rustorium is running, you can verify it's working by:

1. Opening the Web UI at `http://localhost:9070`
2. Checking the API at `http://localhost:9071`
3. Connecting to WebSocket at `ws://localhost:9072`

## Basic Operations

### Checking Node Status

```bash
# View node status
rustorium status

# Check connected peers
rustorium peers list

# View recent blocks
rustorium blocks list
```

### Managing Transactions

```bash
# Submit a transaction
rustorium tx send --to <ADDRESS> --value <AMOUNT>

# Query transaction status
rustorium tx status <TX_HASH>
```

### Monitoring

```bash
# View node metrics
rustorium metrics show

# Watch logs
rustorium logs tail
```

## Next Steps

Now that you have Rustorium running, you might want to:

- Learn about [Configuration](configuration.md)
- Understand [Running a Node](running-node.md)
- Set up [Monitoring](monitoring.md)
- Explore the [API Reference](../api/rest.md)

## Troubleshooting

If you encounter any issues:

1. Check the [FAQ](../appendix/faq.md)
2. Review the [Troubleshooting guide](../advanced/troubleshooting.md)
3. Join our [Discord](https://discord.gg/rustorium) for help
4. Report bugs on [GitHub](https://github.com/enablerdao/rustorium/issues)
