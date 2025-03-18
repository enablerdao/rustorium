#!/bin/bash
set -e

# Install wasm-pack if not installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the web UI
echo "Building web UI..."
cd web
wasm-pack build --target web --out-dir dist/pkg

# Copy index.html to dist
echo "Copying index.html to dist..."
cp index.html dist/

# Create a simple JavaScript loader
echo "Creating JavaScript loader..."
cat > dist/index.js << 'EOF'
import init, { run_app } from './pkg/rustledger_web.js';

async function start() {
    await init();
}

start();
EOF

echo "Web UI built successfully!"