# Configuration Guide

This guide explains how to configure your Rustorium node for optimal performance and security.

## Configuration File

Rustorium uses TOML format for configuration. The default location is `/etc/rustorium/config.toml`.

### Basic Structure

```toml
# Node settings
[node]
name = "my-node"
role = "validator"  # auto, validator, full, light
data_dir = "/var/lib/rustorium"
log_level = "info"  # trace, debug, info, warn, error

# Network settings
[network]
enabled = true
host = "0.0.0.0"
port = 9070
external_addr = "node1.example.com:9070"
bootstrap_nodes = [
    "/ip4/mainnet.rustorium.org/tcp/4001/p2p/12D3KooWQP6ubbGrRFGSbDyiCuw2mi1LMNLFPmwgGsXfGJNRvn2v",
    "/ip4/mainnet2.rustorium.org/tcp/4001/p2p/12D3KooWBmT4c6YvhVYy3KmXMEGaxJXuTVqGtCwwS2GTncxSoje7"
]

# Web UI settings
[web]
enabled = true
port_offset = 0  # 9070 (Dashboard)
open_browser = false
cors_origins = ["*"]

# API settings
[api]
enabled = true
port_offset = 1  # 9071 (API)
rate_limit = 1000
cors_origins = ["*"]

# WebSocket settings
[websocket]
enabled = true
port_offset = 2  # 9072 (WebSocket)

# Validator settings
[validator]
stake = 0
commission = 0.1
min_stake = 100000

# Performance settings
[performance]
max_peers = 50
max_pending_tx = 10000
block_time = 2000  # milliseconds

# Storage settings
[storage]
engine = "redb"
path = "/var/lib/rustorium/data"
max_open_files = 1000
cache_size = 512  # MB

# Development settings
[dev]
nodes = 1
base_port = 8000
auto_mining = false
block_time = 2000  # milliseconds
```

## Configuration Options

### Node Settings

| Option | Description | Default | Required |
|--------|-------------|---------|----------|
| `name` | Node name | Generated from ID | No |
| `role` | Node role | `"auto"` | No |
| `data_dir` | Data directory | `"/var/lib/rustorium"` | Yes |
| `log_level` | Log level | `"info"` | No |

### Network Settings

| Option | Description | Default | Required |
|--------|-------------|---------|----------|
| `enabled` | Enable networking | `true` | No |
| `host` | Listen address | `"0.0.0.0"` | No |
| `port` | Base port | `9070` | Yes |
| `external_addr` | Public address | None | No |
| `bootstrap_nodes` | Bootstrap nodes | Mainnet nodes | No |

### Web UI Settings

| Option | Description | Default | Required |
|--------|-------------|---------|----------|
| `enabled` | Enable Web UI | `true` | No |
| `port_offset` | Port offset | `0` | No |
| `open_browser` | Open browser | `false` | No |
| `cors_origins` | CORS origins | `["*"]` | No |

### API Settings

| Option | Description | Default | Required |
|--------|-------------|---------|----------|
| `enabled` | Enable API | `true` | No |
| `port_offset` | Port offset | `1` | No |
| `rate_limit` | Rate limit | `1000` | No |
| `cors_origins` | CORS origins | `["*"]` | No |

### WebSocket Settings

| Option | Description | Default | Required |
|--------|-------------|---------|----------|
| `enabled` | Enable WebSocket | `true` | No |
| `port_offset` | Port offset | `2` | No |

### Validator Settings

| Option | Description | Default | Required |
|--------|-------------|---------|----------|
| `stake` | Stake amount | `0` | No |
| `commission` | Commission rate | `0.1` | No |
| `min_stake` | Minimum stake | `100000` | No |

### Performance Settings

| Option | Description | Default | Required |
|--------|-------------|---------|----------|
| `max_peers` | Maximum peers | `50` | No |
| `max_pending_tx` | Max pending tx | `10000` | No |
| `block_time` | Block time (ms) | `2000` | No |

### Storage Settings

| Option | Description | Default | Required |
|--------|-------------|---------|----------|
| `engine` | Storage engine | `"redb"` | No |
| `path` | Data path | `"data"` | Yes |
| `max_open_files` | Max open files | `1000` | No |
| `cache_size` | Cache size (MB) | `512` | No |

## Environment Variables

Configuration can be overridden using environment variables:

```bash
# Node settings
export RUSTORIUM_NODE_NAME="my-node"
export RUSTORIUM_NODE_ROLE="validator"
export RUSTORIUM_DATA_DIR="/var/lib/rustorium"
export RUSTORIUM_LOG_LEVEL="info"

# Network settings
export RUSTORIUM_NETWORK_PORT=9070
export RUSTORIUM_EXTERNAL_ADDR="node1.example.com:9070"

# API settings
export RUSTORIUM_API_RATE_LIMIT=1000
```

## Configuration Examples

### Development Node

```toml
[node]
name = "dev-node"
role = "auto"
data_dir = "/tmp/rustorium/data"
log_level = "debug"

[network]
enabled = true
port = 9070
bootstrap_nodes = []

[dev]
auto_mining = true
block_time = 1000
```

### Validator Node

```toml
[node]
name = "validator-1"
role = "validator"
data_dir = "/var/lib/rustorium"
log_level = "info"

[network]
enabled = true
port = 9070
external_addr = "validator1.example.com:9070"

[validator]
stake = 1000000
commission = 0.1

[performance]
max_peers = 100
block_time = 2000
```

### Full Node

```toml
[node]
name = "full-node-1"
role = "full"
data_dir = "/var/lib/rustorium"
log_level = "info"

[network]
enabled = true
port = 9070

[performance]
max_peers = 200
max_pending_tx = 20000
```

## Best Practices

1. **Security**
   - Use restrictive file permissions
   - Enable CORS only for trusted domains
   - Use secure WebSocket in production
   - Rotate API keys regularly

2. **Performance**
   - Adjust cache size based on memory
   - Set appropriate block time
   - Tune max peers and connections
   - Monitor resource usage

3. **Storage**
   - Use SSD for data directory
   - Monitor disk space
   - Regular backups
   - Proper filesystem permissions

4. **Networking**
   - Configure firewalls properly
   - Use stable public IP
   - Set appropriate rate limits
   - Monitor bandwidth usage

## Troubleshooting

### Common Issues

1. **Port Conflicts**
   ```bash
   # Check port usage
   sudo lsof -i :9070-9072
   
   # Change ports in config
   [network]
   port = 10070
   ```

2. **Permission Issues**
   ```bash
   # Fix data directory permissions
   sudo chown -R rustorium:rustorium /var/lib/rustorium
   sudo chmod 700 /var/lib/rustorium
   ```

3. **Memory Issues**
   ```toml
   # Reduce cache size
   [storage]
   cache_size = 256  # MB
   
   # Reduce pending transactions
   [performance]
   max_pending_tx = 5000
   ```

4. **Network Issues**
   ```toml
   # Enable debug logging
   [node]
   log_level = "debug"
   
   # Check connectivity
   [network]
   bootstrap_nodes = [
     "/ip4/mainnet.rustorium.org/tcp/4001/p2p/12D3KooWQP6ubbGrRFGSbDyiCuw2mi1LMNLFPmwgGsXfGJNRvn2v"
   ]
   ```

## Support

If you need help with configuration:

1. Check the [FAQ](../appendix/faq.md)
2. Join our [Discord](https://discord.gg/rustorium)
3. Open an issue on [GitHub](https://github.com/enablerdao/rustorium/issues)
