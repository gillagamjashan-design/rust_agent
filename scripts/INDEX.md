# Rusty IDE Installation Scripts - Complete Index

Welcome to the Rusty IDE installation scripts directory. This index will guide you to the right files.

## Quick Navigation

### I want to install Rusty IDE
→ **[README.md](README.md)** - Start here for installation instructions

### I want to see what these scripts can do
→ **[SHOWCASE.md](SHOWCASE.md)** - Feature highlights and demonstrations

### I want to understand the features
→ **[FEATURES.md](FEATURES.md)** - Detailed feature comparison matrix

### I want to test the installation
→ **[TEST_GUIDE.md](TEST_GUIDE.md)** - Testing procedures and verification

### I want a quick overview
→ **[INSTALLATION_SUMMARY.md](INSTALLATION_SUMMARY.md)** - Executive summary

---

## File Directory

### Installation Scripts (3 files)

| File | Platform | Size | Description |
|------|----------|------|-------------|
| **install-rusty-windows.bat** | Windows | 6.0 KB | Windows 10/11 installer |
| **install-rusty-mac.sh** | macOS | 8.3 KB | macOS 10.15+ installer |
| **install-rusty-linux.sh** | Linux | 9.7 KB | Linux distributions installer |

### Documentation (5 files)

| File | Purpose | Size | Key Content |
|------|---------|------|-------------|
| **README.md** | Installation guide | 5.6 KB | How to install on each platform |
| **FEATURES.md** | Feature documentation | 6.5 KB | Feature matrix, behavior details |
| **TEST_GUIDE.md** | Testing guide | 11 KB | Testing procedures, verification |
| **INSTALLATION_SUMMARY.md** | Executive summary | 8.1 KB | Quick overview, installation flow |
| **SHOWCASE.md** | Feature highlights | 5.2 KB | What makes scripts special |

---

## Common Tasks

### Fresh Installation

1. Choose your platform's script:
   - Windows: `install-rusty-windows.bat`
   - macOS: `install-rusty-mac.sh`
   - Linux: `install-rusty-linux.sh`

2. Read installation instructions in **README.md**

3. Run the script:
   ```bash
   # Windows (as Administrator)
   install-rusty-windows.bat

   # macOS/Linux
   ./install-rusty-mac.sh
   # or
   ./install-rusty-linux.sh
   ```

### Upgrading from Old Agent

All scripts automatically:
- Detect old `agent` installation
- Remove old binary
- Prompt for old data directory cleanup
- Clean up old PATH entries

See **FEATURES.md** section "Cleanup Phase" for details.

### Troubleshooting

1. Check **README.md** "Troubleshooting" section
2. Review error messages from the script
3. See **TEST_GUIDE.md** "Common Issues and Solutions"
4. Verify Rust installation: `cargo --version`

### Testing Before Distribution

1. Read **TEST_GUIDE.md** completely
2. Follow "Manual Testing Checklist"
3. Test on clean systems and systems with existing installations
4. Verify all platforms

---

## Installation Locations

### Where will Rusty IDE be installed?

| Platform | Binary Location | Data Directory |
|----------|----------------|----------------|
| Windows | `C:\Program Files\Rusty\rusty.exe` | `%USERPROFILE%\.rusty` |
| macOS | `/usr/local/bin/rusty` | `~/.rusty` |
| Linux | `~/.local/bin/rusty` | `~/.rusty` |

### PATH Configuration

All scripts automatically add the installation directory to your PATH:
- **Windows:** System PATH (requires admin)
- **macOS:** Shell config (`~/.zshrc` or `~/.bash_profile`)
- **Linux:** Shell config (`~/.bashrc` or `~/.zshrc`)

---

## What Each Script Does

### All Scripts Follow This Flow:

```
1. CLEANUP PHASE
   ├── Remove old agent binary
   ├── Prompt for old data removal
   └── Clean old PATH entries

2. INSTALLATION PHASE
   ├── Build in release mode
   ├── Copy binary to installation directory
   ├── Update PATH
   ├── Create data directory
   └── Optional: Create desktop shortcut

3. VERIFICATION PHASE
   ├── Test rusty command
   ├── Display version
   └── Show system info (Linux)

4. SUCCESS MESSAGE
   ├── Installation summary
   ├── Usage instructions
   └── Troubleshooting tips
```

---

## Platform-Specific Features

### Windows
- PowerShell desktop shortcut creation
- System-wide PATH modification
- Administrator privilege checking
- Compatible with Command Prompt and PowerShell

### macOS
- Application bundle creation
- Automatic shell detection (bash/zsh)
- /usr/local/bin installation (standard)
- Compatible with Intel and Apple Silicon

### Linux
- Unicode progress indicators
- .desktop file for application launchers
- Multiple shell config support
- XDG-compliant directories

---

## Commands After Installation

```bash
# Start Rusty IDE
rusty

# Interactive mode
rusty interactive

# Learning mode
rusty learning

# Help
rusty --help

# Version
rusty --version
```

---

## Documentation Statistics

| Metric | Value |
|--------|-------|
| Total files | 8 |
| Installation scripts | 3 |
| Documentation files | 5 |
| Total lines of code | ~2,000 |
| Total size | ~80 KB |
| Platforms supported | 3 |

---

## Prerequisites

### All Platforms
- **Rust 1.70.0+** with Cargo
  - Install: https://rustup.rs/

### Windows
- Windows 10 (1809+) or Windows 11
- Administrator privileges

### macOS
- macOS 10.15 Catalina or newer
- Xcode Command Line Tools (optional, for building)

### Linux
- Any modern Linux distribution
- build-essential or equivalent (for building)

---

## Quick Reference

### Installation Commands

```bash
# Windows (as Administrator)
cd C:\path\to\rust_agent\scripts
install-rusty-windows.bat

# macOS
cd /path/to/rust_agent/scripts
chmod +x install-rusty-mac.sh
./install-rusty-mac.sh

# Linux
cd /path/to/rust_agent/scripts
chmod +x install-rusty-linux.sh
./install-rusty-linux.sh
```

### Verification Commands

```bash
# Check installation
which rusty          # Unix
where rusty          # Windows

# Test functionality
rusty --version
rusty --help

# Check data directory
ls -la ~/.rusty/     # Unix
dir %USERPROFILE%\.rusty  # Windows
```

### Uninstallation

See **README.md** "Uninstallation" section for complete instructions.

---

## Support and Resources

### In This Directory
- **README.md** - Installation instructions
- **FEATURES.md** - Feature documentation
- **TEST_GUIDE.md** - Testing and verification
- **SHOWCASE.md** - Feature highlights
- **INSTALLATION_SUMMARY.md** - Quick overview

### Project Root
- **USAGE.md** - How to use Rusty IDE
- **README.md** - Project overview
- **INSTALL.md** - Alternative installation methods

---

## License and Credits

Part of the Rusty IDE project.

Created: 2026-02-21
Version: 1.0.0
Platforms: Windows, macOS, Linux

---

## Next Steps

1. **First time here?** Start with **README.md**
2. **Want to understand features?** Read **FEATURES.md**
3. **Ready to install?** Choose your platform's script
4. **Need to test?** Follow **TEST_GUIDE.md**
5. **Want quick info?** See **INSTALLATION_SUMMARY.md**

---

**Happy Installing!**
