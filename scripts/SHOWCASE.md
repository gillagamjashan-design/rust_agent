# Rusty IDE Installation Scripts - Showcase

## What Makes These Scripts Special

### 1. Professional User Experience

#### Colored Output (Unix)
The scripts use ANSI color codes for clear, professional output:

```bash
# Color palette
RED='\033[0;31m'      # Errors
GREEN='\033[0;32m'    # Success messages
YELLOW='\033[1;33m'   # Warnings and info
BLUE='\033[0;34m'     # Section headers
CYAN='\033[0;36m'     # Highlighted text
MAGENTA='\033[0;35m'  # Special headers
```

#### Beautiful Headers

**Linux:**
```
╔════════════════════════════════════════╗
║  Rusty IDE Installer for Linux         ║
╔════════════════════════════════════════╗
```

**macOS:**
```
========================================
 Rusty IDE Installer for macOS
========================================
```

#### Progress Indicators

**Windows/macOS:** Traditional spinner
```
Building... |  
Building... /  
Building... -  
Building... \  
```

**Linux:** Unicode Braille patterns
```
Building... ⠋
Building... ⠙
Building... ⠹
Building... ⠸
```

### 2. Intelligent Cleanup

The scripts intelligently handle upgrades:

```bash
# Detects old installation
OLD_AGENT="$HOME/.local/bin/agent"
if [ -f "$OLD_AGENT" ]; then
    echo "[INFO] Found old agent installation"
    rm -f "$OLD_AGENT"
    echo "[SUCCESS] Removed old agent binary"
fi

# Prompts for data directory
if [ -d "$OLD_DATA" ]; then
    read -p "Remove old agent data? (y/N): " CONFIRM
    if [[ $CONFIRM =~ ^[Yy]$ ]]; then
        rm -rf "$OLD_DATA"
    fi
fi
```

### 3. Cross-Platform Consistency

All scripts follow the same 4-phase structure:

```
Phase 1: CLEANUP
  ↓
Phase 2: INSTALLATION
  ↓
Phase 3: VERIFICATION
  ↓
Phase 4: SUCCESS MESSAGE
```

### 4. Smart PATH Management

#### Windows
```batch
:: Check if already in PATH
echo %PATH% | find /i "%INSTALL_DIR%" >nul
if %errorLevel% neq 0 (
    setx /M PATH "%PATH%;%INSTALL_DIR%"
)
```

#### Unix
```bash
# Auto-detect shell
if [ -n "$ZSH_VERSION" ]; then
    SHELL_CONFIG="$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ]; then
    SHELL_CONFIG="$HOME/.bashrc"
fi

# Add to PATH if not present
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$SHELL_CONFIG"
fi
```

### 5. Comprehensive Error Handling

Every operation is checked:

```bash
# Build with error checking
cargo build --release > /tmp/rusty_build.log 2>&1 &
BUILD_PID=$!
wait $BUILD_PID
BUILD_STATUS=$?

if [ $BUILD_STATUS -ne 0 ]; then
    echo "[ERROR] Build failed. Check /tmp/rusty_build.log"
    tail -n 20 /tmp/rusty_build.log
    exit 1
fi
```

### 6. Desktop Integration

#### Windows - PowerShell Shortcut
```batch
powershell -Command "
    $WshShell = New-Object -ComObject WScript.Shell;
    $Shortcut = $WshShell.CreateShortcut('%SHORTCUT%');
    $Shortcut.TargetPath = '%INSTALL_DIR%\rusty.exe';
    $Shortcut.Save()
"
```

#### macOS - Application Bundle
```bash
# Create full .app structure
APP_DIR="/Applications/Rusty IDE.app"
mkdir -p "$APP_DIR/Contents/MacOS"
mkdir -p "$APP_DIR/Contents/Resources"

# Info.plist with metadata
cat > "$APP_DIR/Contents/Info.plist" << 'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>Rusty IDE</string>
</dict>
</plist>
PLIST
```

#### Linux - Desktop File
```bash
# XDG-compliant .desktop file
cat > "$SHORTCUT_FILE" << 'DESKTOP'
[Desktop Entry]
Version=1.0
Type=Application
Name=Rusty IDE
Exec=~/.local/bin/rusty
Icon=utilities-terminal
Terminal=true
Categories=Development;IDE;
DESKTOP
```

### 7. Detailed Verification

```bash
# Test installation
"$INSTALL_DIR/rusty" --version &>/dev/null
if [ $? -eq 0 ]; then
    echo "[SUCCESS] Rusty IDE is working correctly"
    VERSION=$("$INSTALL_DIR/rusty" --version)
    echo "[INFO] Version: $VERSION"
else
    echo "[WARNING] Command test failed"
fi

# Linux only - System information
echo "[INFO] System information:"
echo "  • OS: $(uname -s)"
echo "  • Architecture: $(uname -m)"
echo "  • Rust version: $(rustc --version)"
```

### 8. User-Friendly Messages

Clear, actionable instructions:

```
========================================
 USAGE INSTRUCTIONS
========================================

 To start Rusty IDE:
   rusty

 To start in interactive mode:
   rusty interactive

 To start in learning mode:
   rusty learning

 For help:
   rusty --help

 NOTE: If 'rusty' command is not found, please:
   1. Restart your terminal
   2. Or run: source ~/.bashrc
   3. Or use full path: ~/.local/bin/rusty
========================================
```

## Code Quality Highlights

### 1. Safety First
```bash
set -e  # Exit on error (Unix)
```

### 2. Helpful Defaults
```bash
# Default to NOT removing user data
read -p "Remove old agent data? (y/N): " CONFIRM
if [[ $CONFIRM =~ ^[Yy]$ ]]; then
    # Only remove if explicitly confirmed
fi
```

### 3. Idempotent Operations
```bash
# Can run multiple times safely
if [ ! -d "$INSTALL_DIR" ]; then
    mkdir -p "$INSTALL_DIR"
fi
```

### 4. Clean Heredocs
```bash
cat > file.txt << 'EOF'
Content here
No variable expansion
Clean and readable
