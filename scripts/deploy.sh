#!/bin/bash
set -e

# Configuration
DOCKER_IMAGE="rustorium/node"
CONFIG_DIR="/etc/rustorium"
DATA_DIR="/var/lib/rustorium"
LOG_DIR="/var/log/rustorium"
SYSTEMD_SERVICE="rustorium"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Helper functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_dependencies() {
    log_info "Checking dependencies..."
    
    # Check Docker
    if ! command -v docker &> /dev/null; then
        log_error "Docker is not installed"
        exit 1
    fi
    
    # Check systemd
    if ! command -v systemctl &> /dev/null; then
        log_error "systemd is not available"
        exit 1
    }
}

create_directories() {
    log_info "Creating directories..."
    
    sudo mkdir -p "$CONFIG_DIR" "$DATA_DIR" "$LOG_DIR"
    sudo chown -R rustorium:rustorium "$CONFIG_DIR" "$DATA_DIR" "$LOG_DIR"
    sudo chmod 700 "$DATA_DIR"
}

create_user() {
    log_info "Creating rustorium user..."
    
    if ! id -u rustorium &>/dev/null; then
        sudo useradd -r -s /bin/false rustorium
    else
        log_warn "User rustorium already exists"
    fi
}

install_systemd_service() {
    log_info "Installing systemd service..."
    
    cat > /tmp/rustorium.service << EOL
[Unit]
Description=Rustorium Node
After=network.target
StartLimitIntervalSec=0

[Service]
Type=simple
User=rustorium
Group=rustorium
Environment=RUST_LOG=info
ExecStart=/usr/local/bin/rustorium --config /etc/rustorium/config.toml
Restart=always
RestartSec=1
LimitNOFILE=65535

[Install]
WantedBy=multi-user.target
EOL

    sudo mv /tmp/rustorium.service /etc/systemd/system/
    sudo systemctl daemon-reload
    sudo systemctl enable rustorium
}

pull_docker_image() {
    log_info "Pulling Docker image..."
    
    docker pull "$DOCKER_IMAGE:latest"
}

backup_data() {
    if [ -d "$DATA_DIR" ]; then
        log_info "Backing up data..."
        
        BACKUP_DIR="/var/backups/rustorium"
        BACKUP_FILE="$BACKUP_DIR/data-$(date +%Y%m%d-%H%M%S).tar.gz"
        
        sudo mkdir -p "$BACKUP_DIR"
        sudo tar czf "$BACKUP_FILE" -C "$DATA_DIR" .
        
        log_info "Backup created: $BACKUP_FILE"
    fi
}

stop_service() {
    log_info "Stopping service..."
    
    if systemctl is-active --quiet rustorium; then
        sudo systemctl stop rustorium
    fi
}

start_service() {
    log_info "Starting service..."
    
    sudo systemctl start rustorium
    sleep 5
    
    if ! systemctl is-active --quiet rustorium; then
        log_error "Service failed to start"
        sudo journalctl -u rustorium -n 50
        exit 1
    fi
}

check_health() {
    log_info "Checking node health..."
    
    for i in {1..30}; do
        if curl -s http://localhost:9071/health > /dev/null; then
            log_info "Node is healthy"
            return 0
        fi
        sleep 1
    done
    
    log_error "Node health check failed"
    exit 1
}

deploy() {
    log_info "Starting deployment..."
    
    check_dependencies
    create_user
    create_directories
    backup_data
    pull_docker_image
    stop_service
    install_systemd_service
    start_service
    check_health
    
    log_info "Deployment completed successfully"
}

# Parse command line arguments
case "$1" in
    start)
        start_service
        ;;
    stop)
        stop_service
        ;;
    restart)
        stop_service
        start_service
        ;;
    status)
        systemctl status rustorium
        ;;
    logs)
        sudo journalctl -u rustorium -f
        ;;
    deploy)
        deploy
        ;;
    *)
        echo "Usage: $0 {start|stop|restart|status|logs|deploy}"
        exit 1
        ;;
esac

exit 0
