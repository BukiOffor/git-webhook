#!/bin/bash
set -e

export PATH="$HOME/.cargo/bin:$PATH"

PROJECT_DIR="/root/ils-development"
SERVICE_NAME="root_server.service"
BINARY_NAME="root_server"

echo "[$(date)] === Deploy started ==="

# Give webhook server time to respond before it dies
sleep 2

echo "[$(date)] Stopping service..."
systemctl stop $SERVICE_NAME

echo "[$(date)] Pulling latest code..."
cd $PROJECT_DIR
#git pull origin main
git pull

cd $PROJECT_DIR/root/client && npm install && npm run build
cd $PROJECT_DIR

echo "[$(date)] Building..."
cargo build -p root_server --release 

echo "[$(date)] Copying binary..."
cp target/release/$BINARY_NAME /opt/gateway/$BINARY_NAME

echo "[$(date)] Starting service..."
systemctl start $SERVICE_NAME

echo "[$(date)] === Deploy complete ==="