# Rusty IDE Installation Scripts - Feature Matrix

## Overview

Three comprehensive installation scripts that handle the complete lifecycle of installing Rusty IDE across all major platforms.

## Feature Comparison

| Feature | Windows | macOS | Linux |
|---------|---------|-------|-------|
| **Cleanup Phase** |
| Remove old agent binary | ✓ | ✓ | ✓ |
| Remove old agent data (with prompt) | ✓ | ✓ | ✓ |
| Clean old PATH entries | Manual | ✓ | ✓ |
| **Build & Installation** |
| Release build | ✓ | ✓ | ✓ |
| Binary installation | ✓ | ✓ | ✓ |
| Automatic PATH configuration | ✓ | ✓ | ✓ |
| Data directory creation | ✓ | ✓ | ✓ |
| **User Interface** |
| Colored output | Limited | ✓ | ✓ |
| Progress indicators | ✓ | ✓ | ✓ (Unicode) |
| Error handling | ✓ | ✓ | ✓ |
| User prompts | ✓ | ✓ | ✓ |
| **Shortcuts** |
| Desktop shortcut | ✓ | ✓ (.app) | ✓ (.desktop) |
| **Verification** |
| Command test | ✓ | ✓ | ✓ |
| Version display | ✓ | ✓ | ✓ |
| System info | - | - | ✓ |
| **Documentation** |
| Usage instructions | ✓ | ✓ | ✓ |
| Troubleshooting tips | ✓ | ✓ | ✓ |

## Installation Locations

### Windows
- **Binary:** `C:\Program Files\Rusty\rusty.exe`
- **Data:** `%USERPROFILE%\.rusty\`
- **Shortcut:** `%USERPROFILE%\Desktop\Rusty IDE.lnk`
- **Requirements:** Administrator privileges

### macOS
- **Binary:** `/usr/local/bin/rusty`
- **Data:** `~/.rusty/`
- **Shortcut:** `/Applications/Rusty IDE.app`
- **Requirements:** sudo access, Rust installed

### Linux
- **Binary:** `~/.local/bin/rusty`
- **Data:** `~/.rusty/`
- **Shortcut:** `~/.local/share/applications/rusty-ide.desktop` or `~/Desktop/rusty-ide.desktop`
- **Requirements:** Rust installed

## Script Behavior

### 1. Cleanup Phase (All Platforms)

**Old Agent Binary:**
- Automatically removes `~/.local/bin/agent` (Unix) or `%USERPROFILE%\.local\bin\agent.exe` (Windows)
- Shows success/warning messages

**Old Agent Data:**
- Prompts user: "Do you want to remove old agent data? (y/N)"
- Only removes if user confirms with 'y' or 'Y'
- Preserves data by default (safe choice)

**PATH Cleanup:**
- **Windows:** Notifies user to manually clean up PATH
- **Unix:** Automatically removes old agent PATH entries from shell configs

### 2. Installation Phase (All Platforms)

**Build Process:**
```bash
cargo build --release
```
- Shows progress indicator during build
- Redirects output to log file (Unix: `/tmp/rusty_build.log`)
- Displays last 20 lines of log on failure

**Binary Installation:**
- Copies `target/release/rust_agent` to installation directory
- Renames to `rusty` (Unix) or `rusty.exe` (Windows)
- Sets executable permissions (Unix)

**PATH Configuration:**
- Checks if installation directory is already in PATH
- Adds to PATH if missing
- **Windows:** Uses `setx /M PATH` for system-wide PATH
- **Unix:** Adds export statement to shell config (~/.bashrc, ~/.zshrc, etc.)

**Data Directory:**
- Creates `~/.rusty/` directory
- No initial files (created on first run)

### 3. Desktop Shortcuts (Optional)

**Windows (.lnk):**
```batch
PowerShell script creates shortcut
Target: C:\Program Files\Rusty\rusty.exe
```

**macOS (.app):**
```bash
Creates full Application bundle
Structure:
  /Applications/Rusty IDE.app/
    Contents/
      MacOS/Rusty IDE (launcher script)
      Info.plist (metadata)
```

**Linux (.desktop):**
```ini
[Desktop Entry]
Name=Rusty IDE
Exec=~/.local/bin/rusty
Terminal=true
Categories=Development;IDE;
```

### 4. Verification Phase

**Command Test:**
```bash
rusty --version
```
- Tests if binary is accessible
- Shows version information
- Warns if command not found (PATH not updated yet)

**System Information (Linux only):**
- OS type
- Architecture
- Rust version

### 5. Success Message (All Platforms)

Displays:
- Installation location
- Data directory location
- Usage instructions:
  - `rusty` - Start default mode
  - `rusty interactive` - Interactive mode
  - `rusty learning` - Learning mode
  - `rusty --help` - Show help
- Troubleshooting notes

## Color Coding

### Windows (Limited)
- Standard console output
- Error messages visible

### Unix (Full Color Support)
- **Red:** Errors
- **Green:** Success messages
- **Yellow:** Warnings and info
- **Blue:** Section headers
- **Cyan:** Highlighted text (paths, commands)
- **Magenta:** Special headers

## Progress Indicators

### Windows
```
Standard console spinner: |/-\
```

### macOS
```
Standard spinner: |/-\
```

### Linux (Unicode)
```
Braille patterns: ⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏
```

## Error Handling

All scripts handle:
1. Missing Cargo.toml
2. Build failures
3. Permission issues
4. PATH configuration errors
5. Missing Rust installation

Error messages include:
- Clear description of the problem
- Suggested solutions
- Relevant log file locations

## Shell Support

### Windows
- Command Prompt
- PowerShell
- Windows Terminal

### Unix
- bash
- zsh
- sh (basic support)

Auto-detects current shell and updates appropriate config file.

## Safety Features

1. **Idempotent:** Can run multiple times safely
2. **Non-destructive:** Prompts before deleting user data
3. **Graceful failure:** Doesn't leave system in broken state
4. **Clear messaging:** Every step is communicated
5. **Rollback hints:** Provides uninstallation instructions

## Testing Recommendations

Before release, test each script:

### Windows
- [ ] Windows 10 (Command Prompt)
- [ ] Windows 11 (PowerShell)
- [ ] With/without old agent installation
- [ ] With/without admin privileges
- [ ] PATH already configured
- [ ] PATH not configured

### macOS
- [ ] macOS Monterey (Intel)
- [ ] macOS Ventura (Apple Silicon)
- [ ] bash shell
- [ ] zsh shell
- [ ] With/without old agent installation
- [ ] PATH already configured
- [ ] PATH not configured

### Linux
- [ ] Ubuntu 22.04 (bash)
- [ ] Fedora 38 (bash)
- [ ] Arch Linux (zsh)
- [ ] With/without old agent installation
- [ ] PATH already configured
- [ ] PATH not configured
- [ ] Desktop environment (GNOME/KDE)
- [ ] Server (no desktop)

## Future Enhancements

Potential improvements:
1. Automatic Rust installation if missing
2. Update checker (compare installed vs latest version)
3. Configuration migration from old agent
4. Backup old installation before removing
5. Silent installation mode (no prompts)
6. Custom installation directory option
7. Package manager integration (apt, brew, chocolatey)
8. Uninstaller scripts
9. Auto-update functionality
10. Telemetry opt-in for installation statistics

---

**Last Updated:** 2026-02-21
**Version:** 1.0.0
