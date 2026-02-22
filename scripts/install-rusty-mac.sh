#!/bin/bash

# ============================================================================
# Rusty TUI Installation Script for macOS
# ============================================================================

set -e

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Progress indicator
show_progress() {
    local pid=$1
    local delay=0.1
    local spinstr='|/-\'
    while [ "$(ps a | awk '{print $1}' | grep $pid)" ]; do
        local temp=${spinstr#?}
        printf " [%c]  " "$spinstr"
        local spinstr=$temp${spinstr%"$temp"}
        sleep $delay
        printf "\b\b\b\b\b\b"
    done
    printf "    \b\b\b\b"
}

echo ""
echo -e "${CYAN}========================================"
echo " Rusty TUI Installer for macOS"
echo -e "========================================${NC}"
echo ""

# ============================================================================
# CLEANUP PHASE
# ============================================================================

echo -e "${BLUE}[1/4] CLEANUP PHASE${NC}"
echo "----------------------------------------"

# Check for old agent installation
OLD_AGENT="$HOME/.local/bin/agent"
if [ -f "$OLD_AGENT" ]; then
    echo -e "${YELLOW}[INFO]${NC} Found old agent installation at $OLD_AGENT"
    rm -f "$OLD_AGENT" 2>/dev/null || true
    if [ -f "$OLD_AGENT" ]; then
        echo -e "${YELLOW}[WARNING]${NC} Could not delete old agent binary"
    else
        echo -e "${GREEN}[SUCCESS]${NC} Removed old agent binary"
    fi
fi

# Remove old agent data directory
OLD_DATA="$HOME/.agent"
if [ -d "$OLD_DATA" ]; then
    echo -e "${YELLOW}[INFO]${NC} Found old agent data directory at $OLD_DATA"
    read -p "Do you want to remove old agent data? (y/N): " -n 1 -r CONFIRM
    echo
    if [[ $CONFIRM =~ ^[Yy]$ ]]; then
        rm -rf "$OLD_DATA" 2>/dev/null || true
        if [ -d "$OLD_DATA" ]; then
            echo -e "${YELLOW}[WARNING]${NC} Could not delete old agent data"
        else
            echo -e "${GREEN}[SUCCESS]${NC} Removed old agent data directory"
        fi
    else
        echo -e "${YELLOW}[INFO]${NC} Keeping old agent data"
    fi
fi

# Clean up old PATH entries
echo -e "${YELLOW}[INFO]${NC} Cleaning up old PATH entries..."
SHELL_CONFIG=""
if [ -n "$ZSH_VERSION" ]; then
    SHELL_CONFIG="$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ]; then
    if [ -f "$HOME/.bash_profile" ]; then
        SHELL_CONFIG="$HOME/.bash_profile"
    else
        SHELL_CONFIG="$HOME/.bashrc"
    fi
fi

if [ -n "$SHELL_CONFIG" ] && [ -f "$SHELL_CONFIG" ]; then
    # Remove old agent PATH entries
    sed -i.bak '/\.local\/bin\/agent/d' "$SHELL_CONFIG" 2>/dev/null || true
    echo -e "${GREEN}[SUCCESS]${NC} Cleaned up old PATH entries"
fi

echo ""

# ============================================================================
# INSTALLATION PHASE
# ============================================================================

echo -e "${BLUE}[2/4] INSTALLATION PHASE${NC}"
echo "----------------------------------------"

# Get the script's directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
# Point to Rusty TUI directory
PROJECT_DIR="/workspace/jashan/rust_agent/rusty_ide_v2"

# Check if Cargo.toml exists
if [ ! -f "$PROJECT_DIR/src-tauri/Cargo.toml" ]; then
    echo -e "${RED}[ERROR]${NC} Rusty TUI not found at $PROJECT_DIR/src-tauri/"
    exit 1
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}[ERROR]${NC} Cargo/Rust is not installed."
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

# Build in release mode
echo -e "${YELLOW}[INFO]${NC} Building Rusty TUI in release mode..."
cd "$PROJECT_DIR/src-tauri"
cargo build --release > /tmp/rusty_build.log 2>&1 &
BUILD_PID=$!
show_progress $BUILD_PID
wait $BUILD_PID
BUILD_STATUS=$?

if [ $BUILD_STATUS -ne 0 ]; then
    echo -e "${RED}[ERROR]${NC} Build failed. Check /tmp/rusty_build.log for details."
    tail -n 20 /tmp/rusty_build.log
    exit 1
fi
echo -e "${GREEN}[SUCCESS]${NC} Build completed successfully"

# Create installation directory
INSTALL_DIR="/usr/local/bin"
if [ ! -d "$INSTALL_DIR" ]; then
    echo -e "${YELLOW}[INFO]${NC} Creating installation directory..."
    sudo mkdir -p "$INSTALL_DIR"
fi

# Copy binary
echo -e "${YELLOW}[INFO]${NC} Installing Rusty TUI to $INSTALL_DIR..."
sudo cp "$PROJECT_DIR/src-tauri/target/release/rusty-tui" "$INSTALL_DIR/rusty"
sudo chmod +x "$INSTALL_DIR/rusty"
echo -e "${GREEN}[SUCCESS]${NC} Binary installed successfully"

# /usr/local/bin is usually in PATH by default on macOS
echo -e "${YELLOW}[INFO]${NC} Checking PATH configuration..."
if [[ ":$PATH:" != *":/usr/local/bin:"* ]]; then
    echo -e "${YELLOW}[WARNING]${NC} /usr/local/bin is not in your PATH"
    echo -e "${YELLOW}[INFO]${NC} Adding to PATH..."
    
    if [ -n "$SHELL_CONFIG" ]; then
        echo 'export PATH="/usr/local/bin:$PATH"' >> "$SHELL_CONFIG"
        echo -e "${GREEN}[SUCCESS]${NC} Added to PATH in $SHELL_CONFIG"
        echo -e "${YELLOW}[INFO]${NC} Please restart your terminal or run: source $SHELL_CONFIG"
    fi
else
    echo -e "${GREEN}[SUCCESS]${NC} Already in PATH"
fi

# Create data directory
DATA_DIR="$HOME/.rusty"
if [ ! -d "$DATA_DIR" ]; then
    mkdir -p "$DATA_DIR"
    echo -e "${GREEN}[SUCCESS]${NC} Created data directory at $DATA_DIR"
fi

# Create desktop shortcut (optional)
read -p "Create Applications shortcut? (y/N): " -n 1 -r CREATE_SHORTCUT
echo
if [[ $CREATE_SHORTCUT =~ ^[Yy]$ ]]; then
    APP_DIR="/Applications/Rusty TUI.app"
    sudo mkdir -p "$APP_DIR/Contents/MacOS"
    sudo cat > "$APP_DIR/Contents/MacOS/Rusty TUI" << 'APPEOF'
#!/bin/bash
open -a Terminal "$INSTALL_DIR/rusty"
APPEOF
    sudo chmod +x "$APP_DIR/Contents/MacOS/Rusty TUI"
    
    sudo mkdir -p "$APP_DIR/Contents/Resources"
    sudo cat > "$APP_DIR/Contents/Info.plist" << 'PLISTEOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>Rusty TUI</string>
    <key>CFBundleName</key>
    <string>Rusty TUI</string>
    <key>CFBundleIdentifier</key>
    <string>com.rusty.ide</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
</dict>
</plist>
PLISTEOF
    echo -e "${GREEN}[SUCCESS]${NC} Applications shortcut created"
fi

echo ""

# ============================================================================
# VERIFICATION PHASE
# ============================================================================

echo -e "${BLUE}[3/4] VERIFICATION PHASE${NC}"
echo "----------------------------------------"

# Test the installation
echo -e "${YELLOW}[INFO]${NC} Testing Rusty TUI installation..."
if "$INSTALL_DIR/rusty" --version &>/dev/null; then
    echo -e "${GREEN}[SUCCESS]${NC} Rusty TUI is working correctly"
    VERSION=$("$INSTALL_DIR/rusty" --version 2>/dev/null || echo "unknown")
    echo -e "${CYAN}[INFO]${NC} Version: $VERSION"
else
    echo -e "${YELLOW}[WARNING]${NC} Command test failed. You may need to restart your terminal."
fi

echo ""

# ============================================================================
# SUCCESS MESSAGE
# ============================================================================

echo -e "${BLUE}[4/4] INSTALLATION COMPLETE${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo -e "  ${GREEN}✓${NC} Rusty TUI has been successfully installed!"
echo ""
echo -e "  Installation location: ${CYAN}$INSTALL_DIR/rusty${NC}"
echo -e "  Data directory: ${CYAN}$DATA_DIR${NC}"
echo ""
echo -e "${CYAN}========================================${NC}"
echo -e "${CYAN} USAGE INSTRUCTIONS${NC}"
echo -e "${CYAN}========================================${NC}"
echo ""
echo "  To start Rusty TUI:"
echo -e "    ${YELLOW}rusty${NC}"
echo ""
echo "  To start in interactive mode:"
echo -e "    ${YELLOW}rusty interactive${NC}"
echo ""
echo "  To start in learning mode:"
echo -e "    ${YELLOW}rusty learning${NC}"
echo ""
echo "  For help:"
echo -e "    ${YELLOW}rusty --help${NC}"
echo ""
echo -e "${YELLOW}  NOTE:${NC} If 'rusty' command is not found, please:"
echo "    1. Restart your terminal"
echo "    2. Or run: source $SHELL_CONFIG"
echo ""
echo -e "${CYAN}========================================${NC}"
echo ""
