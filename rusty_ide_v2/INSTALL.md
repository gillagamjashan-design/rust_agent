# Rusty IDE v2 - Installation Guide

Welcome! This guide will help you install Rusty IDE v2 on your system.

## Quick Start

Choose your operating system and follow the instructions:

### Windows

1. **Prerequisites:**
   - Windows 10 or later
   - [Rust](https://rustup.rs/) installed
   - Administrator access

2. **Install:**
   ```batch
   # Navigate to the scripts folder
   cd rusty_ide_v2\scripts
   
   # Right-click install-windows.bat and select "Run as administrator"
   ```

3. **Verify:**
   ```batch
   rusty --version
   ```

---

### macOS

1. **Prerequisites:**
   - macOS 10.15 (Catalina) or later
   - [Rust](https://rustup.rs/) installed
   - Xcode Command Line Tools: `xcode-select --install`

2. **Install:**
   ```bash
   cd rusty_ide_v2/scripts
   chmod +x install-mac.sh
   ./install-mac.sh
   ```

3. **Verify:**
   ```bash
   rusty --version
   ```

---

### Linux

1. **Prerequisites:**
   - Modern Linux distribution
   - [Rust](https://rustup.rs/) installed
   - Build essentials:
     ```bash
     # Ubuntu/Debian
     sudo apt install build-essential pkg-config libssl-dev
     
     # Fedora
     sudo dnf install gcc pkg-config openssl-devel
     
     # Arch
     sudo pacman -S base-devel
     ```

2. **Install:**
   ```bash
   cd rusty_ide_v2/scripts
   chmod +x install-linux.sh
   ./install-linux.sh
   ```

3. **Verify:**
   ```bash
   rusty --version
   ```

---

## What Gets Installed

### Binary Locations
- **Windows:** `C:\Program Files\Rusty\rusty.exe`
- **macOS:** `/usr/local/bin/rusty`
- **Linux:** `~/.local/bin/rusty`

### Data Directory
All platforms use: `~/.rusty/`
```
~/.rusty/
├── agent/           # AI agent communication files
├── workspaces/      # Workspace-specific data
└── permissions.json # User permissions
```

---

## Usage

Once installed, you can use Rusty IDE:

```bash
# Open a project folder
rusty ~/projects/my-rust-app

# Open current directory
rusty .

# Show help
rusty --help
```

---

## Migration from Old Agent

The installer automatically:
- Removes the old `agent` binary
- Cleans up old startup scripts
- Optionally deletes old agent data (you'll be prompted)

Your old agent data is preserved at `~/.agent/` unless you choose to delete it.

---

## Troubleshooting

### Command Not Found

**Windows:**
- Restart your terminal/PowerShell
- Or manually add `C:\Program Files\Rusty` to your PATH

**macOS/Linux:**
```bash
# Reload your shell configuration
source ~/.bashrc  # or ~/.zshrc on macOS
```

### Build Failures

1. **Update Rust:**
   ```bash
   rustup update
   ```

2. **Install dependencies:**
   - See prerequisites for your platform above

3. **Check error messages:**
   - The installer shows detailed build output
   - Look for missing packages or libraries

### Permission Issues

**Windows:**
- Must run installer as Administrator

**macOS/Linux:**
- Install script uses `sudo` for system directories
- You'll be prompted for password

---

## Next Steps

After installation:

1. **Launch Rusty IDE:**
   ```bash
   rusty .
   ```

2. **Configure settings:**
   - The IDE will guide you through first-time setup
   - Set your preferred AI model and API keys

3. **Start coding:**
   - Create or open a Rust project
   - Use the integrated AI agent for assistance

---

## Updates

To update Rusty IDE to a newer version:

1. Pull the latest code:
   ```bash
   cd rusty_ide_v2
   git pull
   ```

2. Re-run the installer:
   - It will rebuild and reinstall the latest version
   - Your settings in `~/.rusty/` are preserved

---

## Uninstallation

See detailed uninstallation instructions in:
- `scripts/README.md`

Quick uninstall:

**Windows:**
```batch
del "C:\Program Files\Rusty\rusty.exe"
rmdir /s "%USERPROFILE%\.rusty"
```

**macOS:**
```bash
sudo rm /usr/local/bin/rusty
rm -rf ~/.rusty
```

**Linux:**
```bash
rm ~/.local/bin/rusty
rm -rf ~/.rusty
```

---

## Support

For issues, questions, or contributions:
- Check `scripts/README.md` for detailed documentation
- Report bugs on the project repository
- Read the main documentation in the project root

---

**Happy coding with Rusty IDE v2!**
