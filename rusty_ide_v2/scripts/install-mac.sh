#!/bin/bash

# Rusty IDE v2 - macOS Installation Script
# =========================================

set -e

# Color definitions
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Helper functions
print_header() {
    echo -e "\n${BLUE}============================================${NC}"
    echo -e "${BLUE}  $1${NC}"
    echo -e "${BLUE}============================================${NC}\n"
}

print_phase() {
    echo -e "${CYAN}[PHASE $1] $2${NC}"
    echo
}

print_info() {
    echo -e "${BLUE}[*]${NC} $1"
}

print_ok() {
    echo -e "${GREEN}[OK]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Main installation
print_header "Rusty IDE v2 - Installation Script"

# PHASE 1: CLEANUP
# ================
print_phase "1/5" "Cleaning up old agent installation..."

# Remove old agent binary
if [ -f "$HOME/.local/bin/agent" ]; then
    print_info "Found old agent binary at $HOME/.local/bin/agent"
    rm -f "$HOME/.local/bin/agent" 2>/dev/null || true
    if [ ! -f "$HOME/.local/bin/agent" ]; then
        print_ok "Deleted old agent binary"
    else
        print_warning "Could not delete old agent binary"
    fi
fi

# Check for old agent startup scripts
OLD_STARTUP_SCRIPT="$HOME/Library/LaunchAgents/com.agent.plist"
if [ -f "$OLD_STARTUP_SCRIPT" ]; then
    print_info "Removing old agent startup script..."
    launchctl unload "$OLD_STARTUP_SCRIPT" 2>/dev/null || true
    rm -f "$OLD_STARTUP_SCRIPT"
    print_ok "Removed old startup script"
fi

# Check PATH cleanup in shell configs
for config in "$HOME/.zshrc" "$HOME/.bash_profile" "$HOME/.bashrc"; do
    if [ -f "$config" ] && grep -q "\.local/bin/agent" "$config"; then
        print_info "Found old agent PATH entry in $config"
        print_warning "Please manually remove old agent PATH entries from: $config"
    fi
done

# Ask about old agent data
if [ -d "$HOME/.agent" ]; then
    echo
    echo -e "${YELLOW}[?]${NC} Old agent data directory found at $HOME/.agent"
    read -p "    Do you want to delete it? (y/N): " DELETE_OLD_DATA
    if [[ "$DELETE_OLD_DATA" =~ ^[Yy]$ ]]; then
        rm -rf "$HOME/.agent"
        print_ok "Deleted old agent data"
    else
        print_ok "Kept old agent data"
    fi
fi

echo
print_ok "Cleanup complete!"
echo

# PHASE 2: BUILD RUSTY IDE
# ========================
print_phase "2/5" "Building Rusty IDE in release mode..."

# Get the script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_DIR="$SCRIPT_DIR/.."
TAURI_DIR="$PROJECT_DIR/src-tauri"

if [ ! -d "$TAURI_DIR" ]; then
    print_error "Tauri directory not found at $TAURI_DIR"
    exit 1
fi

print_info "Building from: $TAURI_DIR"
cd "$TAURI_DIR"

print_info "Running: cargo build --release"
echo "    This may take several minutes..."
echo

if cargo build --release; then
    echo
    print_ok "Build complete!"
    echo
else
    echo
    print_error "Build failed! Please check the errors above."
    exit 1
fi

# PHASE 3: INSTALL
# ================
print_phase "3/5" "Installing Rusty IDE..."

# Installation directory
INSTALL_DIR="/usr/local/bin"

# Find the built binary
BINARY_PATH="$TAURI_DIR/target/release/rusty_ide_v2"
if [ ! -f "$BINARY_PATH" ]; then
    print_error "Built binary not found at $BINARY_PATH"
    exit 1
fi

# Copy binary
print_info "Copying rusty to $INSTALL_DIR"
if sudo cp "$BINARY_PATH" "$INSTALL_DIR/rusty"; then
    print_ok "Binary installed to $INSTALL_DIR/rusty"
else
    print_error "Failed to copy binary to installation directory"
    exit 1
fi

# Make executable
print_info "Making binary executable..."
sudo chmod +x "$INSTALL_DIR/rusty"
print_ok "Set executable permissions"

echo
print_ok "Installation complete!"
echo

# PHASE 4: SETUP DATA DIRECTORY
# =============================
print_phase "4/5" "Setting up data directories..."

DATA_DIR="$HOME/.rusty"

print_info "Creating directory structure at $DATA_DIR"
mkdir -p "$DATA_DIR/agent"
mkdir -p "$DATA_DIR/workspaces"

# Create default permissions.json
print_info "Creating default permissions.json"
cat > "$DATA_DIR/permissions.json" << 'PERMISSIONS_EOF'
{
  "version": "1.0",
  "permissions": {}
}
PERMISSIONS_EOF

print_ok "Data directories created:"
echo "    - $DATA_DIR/agent/"
echo "    - $DATA_DIR/workspaces/"
echo "    - $DATA_DIR/permissions.json"
echo

# PHASE 5: VERIFY
# ===============
print_phase "5/5" "Verifying installation..."

# Test the rusty command
if command -v rusty &> /dev/null; then
    print_ok "Rusty IDE is in PATH and ready to use!"
else
    print_warning "Rusty command not found in PATH"
    echo "    You may need to restart your terminal or add $INSTALL_DIR to PATH"
fi

# Check if binary works
if "$INSTALL_DIR/rusty" --version &> /dev/null; then
    print_ok "Rusty IDE binary is working correctly!"
else
    print_warning "Could not verify rusty binary execution"
fi

echo

# Create optional desktop application bundle
print_info "Creating macOS application bundle..."
APP_DIR="/Applications/Rusty IDE.app"
if sudo mkdir -p "$APP_DIR/Contents/MacOS" && \
   sudo mkdir -p "$APP_DIR/Contents/Resources" && \
   sudo ln -sf "$INSTALL_DIR/rusty" "$APP_DIR/Contents/MacOS/rusty" 2>/dev/null; then
    
    # Create Info.plist
    sudo tee "$APP_DIR/Contents/Info.plist" > /dev/null << 'PLIST_EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>rusty</string>
    <key>CFBundleIdentifier</key>
    <string>com.rustyide.app</string>
    <key>CFBundleName</key>
    <string>Rusty IDE</string>
    <key>CFBundleVersion</key>
    <string>2.0</string>
    <key>CFBundleShortVersionString</key>
    <string>2.0</string>
</dict>
</plist>
PLIST_EOF
    print_ok "Created application bundle at $APP_DIR"
else
    print_warning "Could not create application bundle (optional)"
fi

echo

print_header "Installation Complete!"

cat << 'USAGE_EOF'
Rusty IDE v2 has been installed successfully!

Installation Location: /usr/local/bin/rusty
Data Directory: ~/.rusty

Usage:
  rusty [folder]     - Open folder in Rusty IDE
  rusty .            - Open current directory
  rusty --help       - Show help

NOTE: If 'rusty' command is not found, please restart your terminal
      or run: export PATH="/usr/local/bin:$PATH"

Thank you for installing Rusty IDE v2!

USAGE_EOF

echo
