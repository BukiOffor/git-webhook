#!/bin/bash
set -e

export PATH="$HOME/.cargo/bin:$PATH"

# Add common Ubuntu/Linux node binary paths (e.g. Snap, local binaries) to PATH
export PATH="$HOME/.local/bin:/snap/bin:/usr/local/bin:/usr/local/nodejs/bin:/opt/node/bin:$PATH"

# Load NVM if installed
export NVM_DIR="$HOME/.nvm"
if [ -s "$NVM_DIR/nvm.sh" ]; then
    . "$NVM_DIR/nvm.sh" --no-use 2>/dev/null || . "$NVM_DIR/nvm.sh" 2>/dev/null || true
fi

# If NVM node directory exists, explicitly add the latest node bin to PATH
if [ -d "$HOME/.nvm/versions/node" ]; then
    for node_dir in $(ls -r -d "$HOME/.nvm/versions/node/"*"/bin" 2>/dev/null); do
        if [ -d "$node_dir" ]; then
            export PATH="$node_dir:$PATH"
            break
        fi
    done
fi

# Load FNM if installed
if command -v fnm &> /dev/null || [ -x "$HOME/.local/share/fnm/fnm" ]; then
    export PATH="$HOME/.local/share/fnm:$PATH"
    eval "$(fnm env --use-on-cd 2>/dev/null || true)"
fi

# Load Volta if installed
if [ -d "$HOME/.volta/bin" ]; then
    export PATH="$HOME/.volta/bin:$PATH"
fi

# Verify npm is available before proceeding
if ! command -v npm &> /dev/null; then
    echo "[$(date)] ERROR: npm command not found in PATH ($PATH)" >&2
    exit 1
fi

PROJECT_DIR="/root/ils-development"
SERVICE_NAME="root_server.service"
BINARY_NAME="root_server"

echo "[$(date)] === Deploy started ==="

# Give webhook server time to respond before it dies
sleep 2

echo "[$(date)] Stopping service..."
systemctl stop $SERVICE_NAME

echo "[$(date)] Pulling latest code..."
cd "$PROJECT_DIR"
git pull || {
    echo "[$(date)] ERROR: git pull failed!" >&2
    exit 1
}

echo "[$(date)] Building frontend..."
(
    cd "$PROJECT_DIR/root/client"
    npm install
    npm run build
) || {
    echo "[$(date)] ERROR: Frontend build failed!" >&2
    exit 1
}

cd "$PROJECT_DIR"

echo "[$(date)] Building backend..."
cargo build -p root_server --release || {
    echo "[$(date)] ERROR: cargo build failed!" >&2
    exit 1
}

echo "[$(date)] Copying binary..."
cp target/release/$BINARY_NAME /opt/gateway/$BINARY_NAME

echo "[$(date)] Starting service..."
systemctl start $SERVICE_NAME

echo "[$(date)] === Deploy complete ==="