#!/bin/bash

CLIPROXYAPI_DIR="/workspace/jashan/cliproxyapi"
CLIPROXYAPI_BIN="$CLIPROXYAPI_DIR/cli-proxy-api"

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║              CLIProxyAPI Starter                             ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# Check if CLIProxyAPI exists
if [ ! -f "$CLIPROXYAPI_BIN" ]; then
    echo "❌ Error: CLIProxyAPI not found at $CLIPROXYAPI_BIN"
    echo ""
    echo "Please ensure CLIProxyAPI is installed at:"
    echo "  $CLIPROXYAPI_DIR"
    exit 1
fi

# Check if already running
if pgrep -f "cli-proxy-api" > /dev/null; then
    echo "⚠️  CLIProxyAPI is already running!"
    echo ""
    ps aux | grep cli-proxy-api | grep -v grep
    echo ""
    echo "To restart, run: pkill cli-proxy-api && $0"
    exit 0
fi

# Start CLIProxyAPI
echo "🚀 Starting CLIProxyAPI on localhost:8317..."
cd "$CLIPROXYAPI_DIR"
nohup ./cli-proxy-api > cliproxyapi.log 2>&1 &

sleep 2

# Verify it started
if pgrep -f "cli-proxy-api" > /dev/null; then
    PID=$(pgrep -f "cli-proxy-api")
    echo "✅ CLIProxyAPI started successfully (PID: $PID)"
    echo ""
    echo "Listening on: http://localhost:8317"
    echo "Log file: $CLIPROXYAPI_DIR/cliproxyapi.log"
    echo ""
    echo "To view logs: tail -f $CLIPROXYAPI_DIR/cliproxyapi.log"
    echo "To stop: pkill cli-proxy-api"
    echo ""
    echo "Now you can run: agent"
else
    echo "❌ Failed to start CLIProxyAPI"
    echo ""
    echo "Check logs at: $CLIPROXYAPI_DIR/cliproxyapi.log"
    exit 1
fi
