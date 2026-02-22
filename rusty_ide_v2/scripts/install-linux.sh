#!/bin/bash

# Rusty IDE v2 - Linux Installation Script
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

# Check for old agent startup scripts in systemd
SYSTEMD_USER_DIR="$HOME/.config/systemd/user"
if [ -f "$SYSTEMD_USER_DIR/agent.service" ]; then
    print_info "Removing old agent systemd service..."
    systemctl --user stop agent.service 2>/dev/null || true
    systemctl --user disable agent.service 2>/dev/null || true
    rm -f "$SYSTEMD_USER_DIR/agent.service"
    systemctl --user daemon-reload 2>/dev/null || true
    print_ok "Removed old systemd service"
fi

# Check for old autostart entries
AUTOSTART_DIR="$HOME/.config/autostart"
if [ -f "$AUTOSTART_DIR/agent.desktop" ]; then
    print_info "Removing old autostart entry..."
    rm -f "$AUTOSTART_DIR/agent.desktop"
    print_ok "Removed old autostart entry"
fi

# Check PATH cleanup in shell configs
for config in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile"; do
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
INSTALL_DIR="$HOME/.local/bin"

# Create installation directory if it doesn't exist
if [ ! -d "$INSTALL_DIR" ]; then
    print_info "Creating installation directory: $INSTALL_DIR"
    mkdir -p "$INSTALL_DIR"
fi

# Find the built binary
BINARY_PATH="$TAURI_DIR/target/release/rusty_ide_v2"
if [ ! -f "$BINARY_PATH" ]; then
    print_error "Built binary not found at $BINARY_PATH"
    exit 1
fi

# Copy binary
print_info "Copying rusty to $INSTALL_DIR"
cp "$BINARY_PATH" "$INSTALL_DIR/rusty"
print_ok "Binary installed to $INSTALL_DIR/rusty"

# Make executable
print_info "Making binary executable..."
chmod +x "$INSTALL_DIR/rusty"
print_ok "Set executable permissions"

# Add to PATH if not already present
print_info "Checking PATH configuration..."
PATH_ADDED=false

# Check if already in PATH
if [[ ":$PATH:" == *":$INSTALL_DIR:"* ]]; then
    print_ok "Already in PATH"
else
    # Determine which shell config to update
    SHELL_CONFIG=""
    if [ -n "$BASH_VERSION" ]; then
        if [ -f "$HOME/.bashrc" ]; then
            SHELL_CONFIG="$HOME/.bashrc"
        elif [ -f "$HOME/.bash_profile" ]; then
            SHELL_CONFIG="$HOME/.bash_profile"
        fi
    elif [ -n "$ZSH_VERSION" ]; then
        SHELL_CONFIG="$HOME/.zshrc"
    elif [ -f "$HOME/.profile" ]; then
        SHELL_CONFIG="$HOME/.profile"
    fi

    if [ -n "$SHELL_CONFIG" ]; then
        # Check if PATH export already exists in config
        if ! grep -q "export PATH=.*$INSTALL_DIR" "$SHELL_CONFIG" 2>/dev/null; then
            print_info "Adding to PATH in $SHELL_CONFIG"
            echo "" >> "$SHELL_CONFIG"
            echo "# Added by Rusty IDE installer" >> "$SHELL_CONFIG"
            echo "export PATH=\"\$HOME/.local/bin:\$PATH\"" >> "$SHELL_CONFIG"
            print_ok "Added to PATH in $SHELL_CONFIG"
            PATH_ADDED=true
        else
            print_ok "PATH already configured in $SHELL_CONFIG"
        fi
    else
        print_warning "Could not determine shell config file"
        echo "    Please add this to your shell config: export PATH=\"\$HOME/.local/bin:\$PATH\""
    fi
fi

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

# Add to current PATH for testing
export PATH="$INSTALL_DIR:$PATH"

# Test the rusty command
if command -v rusty &> /dev/null; then
    print_ok "Rusty IDE is in PATH and ready to use!"
else
    print_warning "Rusty command not found in PATH"
    echo "    You may need to restart your terminal or run: source $SHELL_CONFIG"
fi

# Check if binary works
if "$INSTALL_DIR/rusty" --version &> /dev/null 2>&1; then
    print_ok "Rusty IDE binary is working correctly!"
else
    print_warning "Could not verify rusty binary execution"
fi

echo

# Create optional desktop entry
print_info "Creating desktop entry..."
DESKTOP_DIR="$HOME/.local/share/applications"
mkdir -p "$DESKTOP_DIR"

cat > "$DESKTOP_DIR/rusty-ide.desktop" << DESKTOP_EOF
[Desktop Entry]
Version=2.0
Type=Application
Name=Rusty IDE
Comment=Rust-focused Integrated Development Environment
Exec=$INSTALL_DIR/rusty %F
Icon=application-x-executable
Terminal=false
Categories=Development;IDE;
MimeType=inode/directory;
DESKTOP_EOF

chmod +x "$DESKTOP_DIR/rusty-ide.desktop"
print_ok "Created desktop entry at $DESKTOP_DIR/rusty-ide.desktop"

# Update desktop database if available
if command -v update-desktop-database &> /dev/null; then
    update-desktop-database "$DESKTOP_DIR" 2>/dev/null || true
fi

echo

print_header "Installation Complete!"

cat << USAGE_EOF
Rusty IDE v2 has been installed successfully!

Installation Location: $INSTALL_DIR/rusty
Data Directory: $DATA_DIR

Usage:
  rusty [folder]     - Open folder in Rusty IDE
  rusty .            - Open current directory
  rusty --help       - Show help

USAGE_EOF

if [ "$PATH_ADDED" = true ]; then
    echo -e "${YELLOW}NOTE:${NC} PATH has been updated. Please restart your terminal or run:"
    echo "      source $SHELL_CONFIG"
    echo
fi

echo "Thank you for installing Rusty IDE v2!"
echo

