#!/bin/bash
# Install Rusty IDE Agent Wrapper as a systemd user service

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
WRAPPER_SCRIPT="$PROJECT_ROOT/src-tauri/src/agent_wrapper.sh"
SERVICE_FILE="$SCRIPT_DIR/rusty-agent.service"
USER_SERVICE_DIR="$HOME/.config/systemd/user"

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║  Rusty IDE Agent Service Installer                           ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo

# Check if wrapper script exists
if [ ! -f "$WRAPPER_SCRIPT" ]; then
    echo "Error: Wrapper script not found at: $WRAPPER_SCRIPT"
    exit 1
fi

# Make wrapper script executable
chmod +x "$WRAPPER_SCRIPT"
echo "✓ Wrapper script is executable"

# Create user systemd directory
mkdir -p "$USER_SERVICE_DIR"
echo "✓ Created systemd user directory"

# Create service file with correct path
cat > "$USER_SERVICE_DIR/rusty-agent.service" << SERVICEEOF
[Unit]
Description=Rusty IDE Agent Wrapper
Documentation=https://github.com/yourusername/rusty-ide
After=network.target

[Service]
Type=simple
ExecStart=$WRAPPER_SCRIPT --daemon
Restart=on-failure
RestartSec=5
StandardOutput=journal
StandardError=journal

Environment="RUSTY_AGENT_DIR=%h/.rusty/agent"
Environment="RUST_LOG=info"

[Install]
WantedBy=default.target
SERVICEEOF

echo "✓ Created service file"

# Reload systemd
systemctl --user daemon-reload
echo "✓ Reloaded systemd"

# Enable service
systemctl --user enable rusty-agent.service
echo "✓ Enabled service"

# Start service
systemctl --user start rusty-agent.service
echo "✓ Started service"

# Show status
echo
echo "Service status:"
systemctl --user status rusty-agent.service --no-pager

echo
echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║  Installation Complete!                                       ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo
echo "The agent wrapper is now running as a systemd service."
echo
echo "Useful commands:"
echo "  Check status:  systemctl --user status rusty-agent"
echo "  View logs:     journalctl --user -u rusty-agent -f"
echo "  Restart:       systemctl --user restart rusty-agent"
echo "  Stop:          systemctl --user stop rusty-agent"
echo "  Disable:       systemctl --user disable rusty-agent"
echo
echo "Log file: ~/.rusty/agent/wrapper.log"
echo
