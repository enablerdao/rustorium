# Running a Node

This guide explains how to run and maintain a Rustorium node in production.

## Deployment Options

### Systemd Service

1. Create service file:
```bash
sudo cat > /etc/systemd/system/rustorium.service << 'EOL'
[Unit]
Description=Rustorium Node
After=network.target
StartLimitIntervalSec=0

[Service]
Type=simple
User=rustorium
Group=rustorium
ExecStart=/usr/local/bin/rustorium --config /etc/rustorium/config.toml
Restart=always
RestartSec=1
LimitNOFILE=65535

[Install]
WantedBy=multi-user.target
EOL
```

2. Create user and directories:
```bash
# Create user
sudo useradd -r -s /bin/false rustorium

# Create directories
sudo mkdir -p /etc/rustorium /var/lib/rustorium
sudo chown -R rustorium:rustorium /etc/rustorium /var/lib/rustorium
sudo chmod 700 /var/lib/rustorium
```

3. Start service:
```bash
sudo systemctl daemon-reload
sudo systemctl enable rustorium
sudo systemctl start rustorium
```

### Docker Container

1. Create Dockerfile:
```dockerfile
FROM rust:1.75-slim as builder
WORKDIR /usr/src/rustorium
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y openssl ca-certificates
COPY --from=builder /usr/src/rustorium/target/release/rustorium /usr/local/bin/
VOLUME /etc/rustorium /var/lib/rustorium
EXPOSE 9070 9071 9072
CMD ["rustorium", "--config", "/etc/rustorium/config.toml"]
```

2. Build and run:
```bash
# Build image
docker build -t rustorium:latest .

# Run container
docker run -d \
  --name rustorium \
  -v /etc/rustorium:/etc/rustorium \
  -v /var/lib/rustorium:/var/lib/rustorium \
  -p 9070:9070 \
  -p 9071:9071 \
  -p 9072:9072 \
  rustorium:latest
```

### Kubernetes

1. Create ConfigMap:
```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: rustorium-config
data:
  config.toml: |
    [node]
    name = "k8s-node"
    role = "validator"
    data_dir = "/var/lib/rustorium"
    
    [network]
    enabled = true
    port = 9070
    
    [validator]
    stake = 1000000
    commission = 0.1
```

2. Create StatefulSet:
```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: rustorium
spec:
  serviceName: rustorium
  replicas: 1
  selector:
    matchLabels:
      app: rustorium
  template:
    metadata:
      labels:
        app: rustorium
    spec:
      containers:
      - name: rustorium
        image: rustorium:latest
        ports:
        - containerPort: 9070
        - containerPort: 9071
        - containerPort: 9072
        volumeMounts:
        - name: config
          mountPath: /etc/rustorium
        - name: data
          mountPath: /var/lib/rustorium
      volumes:
      - name: config
        configMap:
          name: rustorium-config
  volumeClaimTemplates:
  - metadata:
      name: data
    spec:
      accessModes: [ "ReadWriteOnce" ]
      resources:
        requests:
          storage: 100Gi
```

3. Create Service:
```yaml
apiVersion: v1
kind: Service
metadata:
  name: rustorium
spec:
  selector:
    app: rustorium
  ports:
  - name: dashboard
    port: 9070
  - name: api
    port: 9071
  - name: websocket
    port: 9072
```

## Monitoring

### Prometheus Metrics

Metrics are available at `http://localhost:9071/metrics`:

```bash
# HELP rustorium_blocks_total Total number of blocks
# TYPE rustorium_blocks_total counter
rustorium_blocks_total 12345

# HELP rustorium_transactions_total Total number of transactions
# TYPE rustorium_transactions_total counter
rustorium_transactions_total 67890

# HELP rustorium_peers_connected Number of connected peers
# TYPE rustorium_peers_connected gauge
rustorium_peers_connected 10

# HELP rustorium_memory_usage_bytes Memory usage in bytes
# TYPE rustorium_memory_usage_bytes gauge
rustorium_memory_usage_bytes 1073741824
```

### Grafana Dashboard

Import our [Grafana dashboard](https://grafana.com/grafana/dashboards/12345) for visualization:

1. System metrics
   - CPU usage
   - Memory usage
   - Disk I/O
   - Network I/O

2. Node metrics
   - Block height
   - Transaction count
   - Peer count
   - Latency

3. Validator metrics
   - Stake amount
   - Commission earned
   - Block proposals
   - Missed blocks

### Log Management

1. Configure logging:
```toml
[node]
log_level = "info"
log_file = "/var/log/rustorium/node.log"
```

2. Configure log rotation:
```bash
sudo cat > /etc/logrotate.d/rustorium << 'EOL'
/var/log/rustorium/*.log {
    daily
    rotate 7
    compress
    delaycompress
    missingok
    notifempty
    create 640 rustorium rustorium
}
EOL
```

3. View logs:
```bash
# View live logs
journalctl -u rustorium -f

# View error logs
journalctl -u rustorium -p err

# Search logs
journalctl -u rustorium --since "1 hour ago" | grep "error"
```

## Backup and Recovery

### Backup Data

1. Stop node:
```bash
sudo systemctl stop rustorium
```

2. Create backup:
```bash
# Create backup directory
sudo mkdir -p /var/backups/rustorium

# Backup data
sudo tar czf /var/backups/rustorium/data-$(date +%Y%m%d).tar.gz \
  /var/lib/rustorium

# Backup config
sudo cp /etc/rustorium/config.toml \
  /var/backups/rustorium/config-$(date +%Y%m%d).toml
```

3. Start node:
```bash
sudo systemctl start rustorium
```

### Restore Data

1. Stop node:
```bash
sudo systemctl stop rustorium
```

2. Restore data:
```bash
# Remove current data
sudo rm -rf /var/lib/rustorium/*

# Restore from backup
sudo tar xzf /var/backups/rustorium/data-20240123.tar.gz -C /

# Restore config
sudo cp /var/backups/rustorium/config-20240123.toml \
  /etc/rustorium/config.toml
```

3. Start node:
```bash
sudo systemctl start rustorium
```

## Security

### Firewall Configuration

1. Configure UFW:
```bash
# Allow P2P port
sudo ufw allow 9070/tcp

# Allow API port from trusted IPs
sudo ufw allow from 192.168.1.0/24 to any port 9071

# Allow WebSocket port from trusted IPs
sudo ufw allow from 192.168.1.0/24 to any port 9072
```

2. Configure iptables:
```bash
# Allow P2P port
sudo iptables -A INPUT -p tcp --dport 9070 -j ACCEPT

# Allow API port from trusted IPs
sudo iptables -A INPUT -p tcp -s 192.168.1.0/24 --dport 9071 -j ACCEPT

# Allow WebSocket port from trusted IPs
sudo iptables -A INPUT -p tcp -s 192.168.1.0/24 --dport 9072 -j ACCEPT
```

### SSL/TLS Configuration

1. Generate certificates:
```bash
# Generate self-signed certificate
openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
  -keyout /etc/rustorium/tls/node.key \
  -out /etc/rustorium/tls/node.crt
```

2. Configure TLS:
```toml
[web]
tls_cert = "/etc/rustorium/tls/node.crt"
tls_key = "/etc/rustorium/tls/node.key"

[api]
tls_cert = "/etc/rustorium/tls/node.crt"
tls_key = "/etc/rustorium/tls/node.key"

[websocket]
tls_cert = "/etc/rustorium/tls/node.crt"
tls_key = "/etc/rustorium/tls/node.key"
```

### API Key Management

1. Generate API key:
```bash
rustorium key generate --name "prod-api" --expiry 90d
```

2. Rotate API keys:
```bash
# Generate new key
rustorium key generate --name "prod-api-new" --expiry 90d

# Update applications
# ...

# Revoke old key
rustorium key revoke <OLD_KEY_ID>
```

## Maintenance

### Regular Tasks

1. Daily:
   - Check node status
   - Monitor resource usage
   - Review error logs
   - Verify peer connections

2. Weekly:
   - Create backups
   - Rotate logs
   - Check disk space
   - Update firewall rules

3. Monthly:
   - Rotate API keys
   - Update SSL certificates
   - Review security policies
   - Check for updates

### Upgrading

1. Backup data:
```bash
sudo systemctl stop rustorium
sudo tar czf /var/backups/rustorium/pre-upgrade.tar.gz \
  /var/lib/rustorium /etc/rustorium
```

2. Install update:
```bash
# Download new version
wget https://github.com/enablerdao/rustorium/releases/latest/rustorium

# Install binary
sudo cp rustorium /usr/local/bin/
sudo chmod +x /usr/local/bin/rustorium
```

3. Start node:
```bash
sudo systemctl start rustorium
```

### Troubleshooting

1. Check status:
```bash
# View service status
sudo systemctl status rustorium

# Check logs
journalctl -u rustorium -n 100

# Check resource usage
top -p $(pgrep rustorium)
```

2. Common issues:
   - Port conflicts
   - Permission errors
   - Memory issues
   - Network connectivity

3. Recovery steps:
   - Stop node
   - Check logs
   - Fix issues
   - Start node
   - Verify status

## Support

If you need help running your node:

1. Check the [FAQ](../appendix/faq.md)
2. Join our [Discord](https://discord.gg/rustorium)
3. Open an issue on [GitHub](https://github.com/enablerdao/rustorium/issues)
