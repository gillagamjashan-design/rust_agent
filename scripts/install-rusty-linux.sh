#!/bin/bash

# ============================================================================
# Rusty IDE Installation Script for Linux
# ============================================================================

set -e

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m' # No Color

# Progress indicator with animation
show_progress() {
    local pid=$1
    local delay=0.1
    local spinstr='⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏'
    while kill -0 $pid 2>/dev/null; do
        local temp=${spinstr#?}
        printf " [${CYAN}%c${NC}]  " "$spinstr"
        local spinstr=$temp${spinstr%"$temp"}
        sleep $delay
        printf "\b\b\b\b\b\b"
    done
    printf "    \b\b\b\b"
}

# Print header
print_header() {
    echo ""
    echo -e "${CYAN}╔════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║${NC}  ${MAGENTA}Rusty IDE Installer for Linux${NC}     ${CYAN}║${NC}"
    echo -e "${CYAN}╔════════════════════════════════════════╗${NC}"
    echo ""
}

# Print section
print_section() {
    echo -e "${BLUE}[$1/4] $2${NC}"
    echo "----------------------------------------"
}

print_header

# ============================================================================
# CLEANUP PHASE
# ============================================================================

print_section "1" "CLEANUP PHASE"

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
    read -p "$(echo -e ${CYAN}Do you want to remove old agent data? \(y/N\):${NC} )" -n 1 -r CONFIRM
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
SHELL_CONFIGS=("$HOME/.bashrc" "$HOME/.bash_profile" "$HOME/.zshrc" "$HOME/.profile")

for config in "${SHELL_CONFIGS[@]}"; do
    if [ -f "$config" ]; then
        # Remove old agent PATH entries
        sed -i.bak '/\.local\/bin\/agent/d' "$config" 2>/dev/null || true
    fi
done
echo -e "${GREEN}[SUCCESS]${NC} Cleaned up old PATH entries"

echo ""

# ============================================================================
# INSTALLATION PHASE
# ============================================================================

print_section "2" "INSTALLATION PHASE"

# Get the script's directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
# Point to Rusty TUI directory (v2 - ratatui-based)
PROJECT_DIR="/workspace/jashan/rust_agent/rusty_ide_v2"

# Check if Cargo.toml exists
if [ ! -f "$PROJECT_DIR/src-tauri/Cargo.toml" ]; then
    echo -e "${RED}[ERROR]${NC} Rusty TUI not found at $PROJECT_DIR/src-tauri/"
    echo -e "${RED}[ERROR]${NC} Please ensure Rusty TUI is at /workspace/jashan/rust_agent/rusty_ide_v2/"
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
export CARGO_HOME="${CARGO_HOME:-/workspace/jashan/.cargo}"
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
INSTALL_DIR="$HOME/.local/bin"
if [ ! -d "$INSTALL_DIR" ]; then
    echo -e "${YELLOW}[INFO]${NC} Creating installation directory..."
    mkdir -p "$INSTALL_DIR"
fi

# Copy binary
echo -e "${YELLOW}[INFO]${NC} Installing Rusty TUI to $INSTALL_DIR..."
cp "$PROJECT_DIR/src-tauri/target/release/rusty-tui" "$INSTALL_DIR/rusty"
chmod +x "$INSTALL_DIR/rusty"
echo -e "${GREEN}[SUCCESS]${NC} Binary installed successfully"

# Add to PATH if not already present
echo -e "${YELLOW}[INFO]${NC} Checking PATH configuration..."
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${YELLOW}[WARNING]${NC} $INSTALL_DIR is not in your PATH"
    echo -e "${YELLOW}[INFO]${NC} Adding to PATH..."
    
    # Determine which shell config to use
    SHELL_CONFIG=""
    if [ -n "$ZSH_VERSION" ]; then
        SHELL_CONFIG="$HOME/.zshrc"
    elif [ -n "$BASH_VERSION" ]; then
        if [ -f "$HOME/.bashrc" ]; then
            SHELL_CONFIG="$HOME/.bashrc"
        else
            SHELL_CONFIG="$HOME/.bash_profile"
        fi
    else
        # Default to .bashrc
        SHELL_CONFIG="$HOME/.bashrc"
    fi
    
    # Add to PATH if not already there
    if ! grep -q "export PATH=\"\$HOME/.local/bin:\$PATH\"" "$SHELL_CONFIG" 2>/dev/null; then
        echo '' >> "$SHELL_CONFIG"
        echo '# Added by Rusty IDE installer' >> "$SHELL_CONFIG"
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$SHELL_CONFIG"
        echo -e "${GREEN}[SUCCESS]${NC} Added to PATH in $SHELL_CONFIG"
        echo -e "${YELLOW}[INFO]${NC} Please restart your terminal or run: source $SHELL_CONFIG"
    else
        echo -e "${GREEN}[SUCCESS]${NC} PATH entry already exists"
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
read -p "$(echo -e ${CYAN}Create desktop shortcut? \(y/N\):${NC} )" -n 1 -r CREATE_SHORTCUT
echo
if [[ $CREATE_SHORTCUT =~ ^[Yy]$ ]]; then
    DESKTOP_DIR="$HOME/Desktop"
    if [ ! -d "$DESKTOP_DIR" ]; then
        DESKTOP_DIR="$HOME/.local/share/applications"
        mkdir -p "$DESKTOP_DIR"
    fi
    
    SHORTCUT_FILE="$DESKTOP_DIR/rusty-ide.desktop"
    cat > "$SHORTCUT_FILE" << DESKTOPEOF
[Desktop Entry]
Version=1.0
Type=Application
Name=Rusty IDE
Comment=Rust Learning Environment
Exec=$INSTALL_DIR/rusty
Icon=utilities-terminal
Terminal=true
Categories=Development;IDE;
DESKTOPEOF
    
    chmod +x "$SHORTCUT_FILE"
    echo -e "${GREEN}[SUCCESS]${NC} Desktop shortcut created at $SHORTCUT_FILE"
fi

echo ""

# ============================================================================
# VERIFICATION PHASE
# ============================================================================

print_section "3" "VERIFICATION PHASE"

# Test the installation
echo -e "${YELLOW}[INFO]${NC} Testing Rusty IDE installation..."
if "$INSTALL_DIR/rusty" --version &>/dev/null; then
    echo -e "${GREEN}[SUCCESS]${NC} Rusty IDE is working correctly"
    VERSION=$("$INSTALL_DIR/rusty" --version 2>/dev/null || echo "unknown")
    echo -e "${CYAN}[INFO]${NC} Version: $VERSION"
else
    echo -e "${YELLOW}[WARNING]${NC} Command test failed. You may need to restart your terminal."
fi

# Check system info
echo -e "${YELLOW}[INFO]${NC} System information:"
echo -e "  ${CYAN}•${NC} OS: $(uname -s)"
echo -e "  ${CYAN}•${NC} Architecture: $(uname -m)"
echo -e "  ${CYAN}•${NC} Rust version: $(rustc --version 2>/dev/null || echo 'unknown')"

echo ""

# ============================================================================
# SUCCESS MESSAGE
# ============================================================================

print_section "4" "INSTALLATION COMPLETE"
echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
echo ""
echo -e "  ${GREEN}✓${NC} Rusty IDE has been successfully installed!"
echo ""
echo -e "  ${BLUE}Installation location:${NC} ${CYAN}$INSTALL_DIR/rusty${NC}"
echo -e "  ${BLUE}Data directory:${NC} ${CYAN}$DATA_DIR${NC}"
echo ""
echo -e "${CYAN}╔════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║${NC}  ${MAGENTA}USAGE INSTRUCTIONS${NC}                    ${CYAN}║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════╝${NC}"
echo ""
echo "  To start Rusty IDE:"
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
echo -e "  ${BLUE}Additional commands:${NC}"
echo -e "    ${YELLOW}rusty --version${NC}      - Show version information"
echo -e "    ${YELLOW}rusty config${NC}         - Configure settings"
echo ""
echo -e "${YELLOW}  NOTE:${NC} If 'rusty' command is not found, please:"
echo "    1. Restart your terminal"
echo "    2. Or run: ${CYAN}source $SHELL_CONFIG${NC}"
echo "    3. Or use the full path: ${CYAN}$INSTALL_DIR/rusty${NC}"
echo ""
echo -e "${CYAN}╚════════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}Happy coding with Rusty IDE!${NC}"
echo ""
