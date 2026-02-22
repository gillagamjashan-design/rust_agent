# Rusty IDE Installation Scripts

This directory contains platform-specific installation scripts for Rusty IDE.

## Available Scripts

### 1. Windows Installation (`install-rusty-windows.bat`)
- **Platform:** Windows 10/11
- **Requirements:** Administrator privileges
- **Installation Location:** `C:\Program Files\Rusty\rusty.exe`

### 2. macOS Installation (`install-rusty-mac.sh`)
- **Platform:** macOS 10.15+
- **Requirements:** Rust/Cargo installed
- **Installation Location:** `/usr/local/bin/rusty`

### 3. Linux Installation (`install-rusty-linux.sh`)
- **Platform:** Ubuntu, Debian, Fedora, Arch, etc.
- **Requirements:** Rust/Cargo installed
- **Installation Location:** `~/.local/bin/rusty`

## Installation Instructions

### Windows

1. Open Command Prompt or PowerShell **as Administrator**
2. Navigate to the scripts directory:
   ```cmd
   cd path\to\rust_agent\scripts
   ```
3. Run the installation script:
   ```cmd
   install-rusty-windows.bat
   ```
4. Follow the on-screen prompts
5. Restart your terminal to use the `rusty` command

### macOS

1. Open Terminal
2. Navigate to the scripts directory:
   ```bash
   cd /path/to/rust_agent/scripts
   ```
3. Make the script executable (if not already):
   ```bash
   chmod +x install-rusty-mac.sh
   ```
4. Run the installation script:
   ```bash
   ./install-rusty-mac.sh
   ```
5. Enter your password when prompted (for sudo operations)
6. Follow the on-screen prompts
7. Restart your terminal or run `source ~/.zshrc` (or `~/.bash_profile`)

### Linux

1. Open Terminal
2. Navigate to the scripts directory:
   ```bash
   cd /path/to/rust_agent/scripts
   ```
3. Make the script executable (if not already):
   ```bash
   chmod +x install-rusty-linux.sh
   ```
4. Run the installation script:
   ```bash
   ./install-rusty-linux.sh
   ```
5. Follow the on-screen prompts
6. Restart your terminal or run `source ~/.bashrc`

## What Each Script Does

### Cleanup Phase
- Removes old `agent` installation from `~/.local/bin/agent`
- Optionally removes old agent data directory (`~/.agent`)
- Cleans up old PATH entries

### Installation Phase
- Builds Rusty IDE in release mode using `cargo build --release`
- Copies the binary to the platform-specific location
- Adds the installation directory to PATH (if needed)
- Creates data directory at `~/.rusty/`
- Optionally creates desktop shortcuts

### Verification Phase
- Tests that the `rusty` command works
- Displays version information
- Shows system information (Linux only)

### Success Message
- Displays installation location
- Shows usage instructions
- Provides troubleshooting tips

## Features

### All Scripts Include:
- **Colored Output:** Easy-to-read, color-coded messages
- **Progress Indicators:** Visual feedback during long operations
- **Error Handling:** Graceful handling of errors with helpful messages
- **User Prompts:** Interactive prompts for optional features
- **PATH Management:** Automatic PATH configuration
- **Desktop Shortcuts:** Optional desktop/application shortcuts

### Platform-Specific Features:

**Windows:**
- Administrator privilege checking
- System-wide PATH configuration
- PowerShell-based desktop shortcut creation

**macOS:**
- Application bundle creation
- Automatic shell detection (bash/zsh)
- /usr/local/bin installation (standard for macOS)

**Linux:**
- Unicode progress indicators
- Desktop file creation (.desktop)
- Multiple shell config support (bash/zsh/profile)

## Usage After Installation

Once installed, you can use Rusty IDE with the following commands:

```bash
# Start Rusty IDE (default mode)
rusty

# Start in interactive mode
rusty interactive

# Start in learning mode
rusty learning

# Show help
rusty --help

# Show version
rusty --version
```

## Troubleshooting

### Command not found after installation

**Solution:** Restart your terminal or source your shell configuration:
- **Linux/macOS:** `source ~/.bashrc` or `source ~/.zshrc`
- **Windows:** Restart Command Prompt or PowerShell

### Build failed

**Solution:** Ensure Rust is installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Permission denied (macOS/Linux)

**Solution:** Make the script executable:
```bash
chmod +x install-rusty-linux.sh
# or
chmod +x install-rusty-mac.sh
```

### Administrator privileges required (Windows)

**Solution:** Right-click on Command Prompt or PowerShell and select "Run as administrator"

## Uninstallation

To uninstall Rusty IDE:

### Windows
```cmd
del "C:\Program Files\Rusty\rusty.exe"
rmdir "C:\Program Files\Rusty"
```

### macOS
```bash
sudo rm /usr/local/bin/rusty
rm -rf ~/.rusty
```

### Linux
```bash
rm ~/.local/bin/rusty
rm -rf ~/.rusty
```

Then remove the PATH entry from your shell configuration file.

## Data Directory

All scripts create a data directory at `~/.rusty/` (or `%USERPROFILE%\.rusty` on Windows) where:
- Configuration files are stored
- Learning progress is saved
- Cache data is maintained

## Support

For issues or questions:
1. Check the troubleshooting section above
2. Review the error messages in the script output
3. Check the build log at `/tmp/rusty_build.log` (Unix) or console output (Windows)
4. Ensure Rust is properly installed: `cargo --version`

## Script Details

| Script | Size | Executable | Platform |
|--------|------|------------|----------|
| install-rusty-windows.bat | ~6KB | No (Windows) | Windows |
| install-rusty-mac.sh | ~8KB | Yes | macOS |
| install-rusty-linux.sh | ~10KB | Yes | Linux |

---

**Note:** These scripts are designed to be idempotent - you can run them multiple times safely. They will clean up old installations before installing the new version.
