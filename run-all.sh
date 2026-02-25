#!/bin/bash
# run-all.sh - Install and setup Rusty Agent
# This script installs everything needed except rusty itself (which gets installed to PATH)

set -e  # Exit on error

echo "🦀 Rusty Agent - Installation & Setup Script"
echo "============================================="
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_info() {
    echo -e "${YELLOW}ℹ${NC} $1"
}

# 1. Check for Rust installation
echo "Step 1: Checking Rust installation..."
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    print_success "Rust is installed: $RUST_VERSION"
else
    print_error "Rust is not installed"
    echo "Please install Rust from https://rustup.rs/"
    echo "Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# 2. Check for cargo
echo
echo "Step 2: Checking cargo..."
if command -v cargo &> /dev/null; then
    CARGO_VERSION=$(cargo --version)
    print_success "Cargo is installed: $CARGO_VERSION"
else
    print_error "Cargo is not installed (should come with Rust)"
    exit 1
fi

# 3. Build rusty binary
echo
echo "Step 3: Building rusty binary..."
print_info "This may take a few minutes on first build..."

# Set CARGO_HOME to project's .cargo directory to avoid permission issues
export CARGO_HOME="$(pwd)/.cargo"

cd rusty_tui
if cargo build --release 2>&1 | tee /tmp/rusty_build.log | grep -E "(Compiling|Finished)"; then
    print_success "Built rusty binary successfully"
else
    print_error "Build failed - check /tmp/rusty_build.log for details"
    exit 1
fi
cd ..

# 4. Install to PATH
echo
echo "Step 4: Installing rusty to PATH..."
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

if cp rusty_tui/target/release/rusty "$INSTALL_DIR/rusty"; then
    chmod +x "$INSTALL_DIR/rusty"
    print_success "Installed rusty to $INSTALL_DIR/rusty"
else
    print_error "Failed to install rusty"
    exit 1
fi

# Check if ~/.local/bin is in PATH
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    print_info "Note: $HOME/.local/bin is not in your PATH"
    echo "Add this line to your ~/.bashrc or ~/.zshrc:"
    echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo
fi

# 5. Check for ClaudeProxyAPI
echo
echo "Step 5: Checking ClaudeProxyAPI..."
if curl -s http://localhost:8317/health &> /dev/null; then
    print_success "ClaudeProxyAPI is already running on localhost:8317"
else
    print_info "ClaudeProxyAPI is not running"
    
    if [ -f "start_cliproxyapi.sh" ]; then
        echo "Starting ClaudeProxyAPI in background..."
        ./start_cliproxyapi.sh &
        CLAUDE_PID=$!
        
        # Wait for it to start
        sleep 3
        
        if curl -s http://localhost:8317/health &> /dev/null; then
            print_success "ClaudeProxyAPI started successfully (PID: $CLAUDE_PID)"
        else
            print_error "Failed to start ClaudeProxyAPI"
            echo "Try starting it manually: ./start_cliproxyapi.sh"
        fi
    else
        print_error "start_cliproxyapi.sh not found"
        echo "Please ensure ClaudeProxyAPI is running on localhost:8317"
    fi
fi

# 6. Initialize knowledge database
echo
echo "Step 6: Checking knowledge database..."
DB_PATH="$HOME/.agent/data/knowledge.db"
if [ -f "$DB_PATH" ]; then
    print_success "Knowledge database already exists at $DB_PATH"
else
    print_info "Knowledge database will be initialized on first run of 'rusty'"
    print_info "This will take ~1-2 seconds"
fi

# 7. Verify knowledge JSON files
echo
echo "Step 7: Verifying knowledge files..."
KNOWLEDGE_DIR="knowledge"
if [ -d "$KNOWLEDGE_DIR" ]; then
    JSON_COUNT=$(find "$KNOWLEDGE_DIR" -name "*.json" | wc -l)
    if [ "$JSON_COUNT" -gt 0 ]; then
        print_success "Found $JSON_COUNT knowledge JSON files"
    else
        print_error "No JSON files found in $KNOWLEDGE_DIR"
    fi
else
    print_error "Knowledge directory not found: $KNOWLEDGE_DIR"
fi

# 8. Summary
echo
echo "============================================="
echo "🎉 Installation Complete!"
echo "============================================="
echo
print_success "Rusty agent is installed to: $INSTALL_DIR/rusty"
print_success "Knowledge database location: $HOME/.agent/data/"
echo
echo "To start using Rusty:"
echo "  1. Make sure ClaudeProxyAPI is running (localhost:8317)"
echo "  2. Run: rusty"
echo
echo "Commands in Rusty:"
echo "  /help      - Show available commands"
echo "  /search    - Search knowledge database"
echo "  /stats     - Show database statistics"
echo "  /quit      - Exit"
echo
print_info "If 'rusty' command not found, add ~/.local/bin to PATH:"
echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
echo

# Check if user wants to run rusty now
read -p "Do you want to run 'rusty' now? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    if [[ ":$PATH:" == *":$HOME/.local/bin:"* ]]; then
        exec rusty
    else
        exec "$INSTALL_DIR/rusty"
    fi
fi

print_success "Setup complete! Run 'rusty' to start."
