# Testing Guide for Rusty IDE Installation Scripts

## Quick Test Commands

### Test Linux Script (Dry Run)
```bash
cd /workspace/jashan/rust_agent/scripts
bash -n install-rusty-linux.sh  # Syntax check only
```

### Test macOS Script (Dry Run)
```bash
cd /workspace/jashan/rust_agent/scripts
bash -n install-rusty-mac.sh   # Syntax check only
```

### Test Windows Script (Syntax Check)
Windows batch files don't have a built-in syntax checker, but you can review the script manually.

## Expected Output Preview

### Linux Installation Output

```
╔════════════════════════════════════════╗
║  Rusty IDE Installer for Linux         ║
╔════════════════════════════════════════╗

[1/4] CLEANUP PHASE
----------------------------------------
[INFO] Cleaning up old PATH entries...
[SUCCESS] Cleaned up old PATH entries

[2/4] INSTALLATION PHASE
----------------------------------------
[INFO] Building Rusty IDE in release mode...
 [⠋]  
[SUCCESS] Build completed successfully
[INFO] Installing Rusty IDE to /home/user/.local/bin...
[SUCCESS] Binary installed successfully
[INFO] Checking PATH configuration...
[SUCCESS] Already in PATH
[SUCCESS] Created data directory at /home/user/.rusty

Create desktop shortcut? (y/N): n

[3/4] VERIFICATION PHASE
----------------------------------------
[INFO] Testing Rusty IDE installation...
[SUCCESS] Rusty IDE is working correctly
[INFO] Version: rust_agent 0.1.0
[INFO] System information:
  • OS: Linux
  • Architecture: x86_64
  • Rust version: rustc 1.75.0

[4/4] INSTALLATION COMPLETE
╔════════════════════════════════════════╗

  ✓ Rusty IDE has been successfully installed!

  Installation location: /home/user/.local/bin/rusty
  Data directory: /home/user/.rusty

╔════════════════════════════════════════╗
║  USAGE INSTRUCTIONS                    ║
╚════════════════════════════════════════╝

  To start Rusty IDE:
    rusty

  To start in interactive mode:
    rusty interactive

  To start in learning mode:
    rusty learning

  For help:
    rusty --help

  Additional commands:
    rusty --version      - Show version information
    rusty config         - Configure settings

  NOTE: If 'rusty' command is not found, please:
    1. Restart your terminal
    2. Or run: source /home/user/.bashrc
    3. Or use the full path: /home/user/.local/bin/rusty

╚════════════════════════════════════════╝

Happy coding with Rusty IDE!
```

### macOS Installation Output

```
========================================
 Rusty IDE Installer for macOS
========================================

[1/4] CLEANUP PHASE
----------------------------------------
[INFO] Cleaning up old PATH entries...
[SUCCESS] Cleaned up old PATH entries

[2/4] INSTALLATION PHASE
----------------------------------------
[INFO] Building Rusty IDE in release mode...
 [|]  
[SUCCESS] Build completed successfully
[INFO] Installing Rusty IDE to /usr/local/bin...
[SUCCESS] Binary installed successfully
[INFO] Checking PATH configuration...
[SUCCESS] Already in PATH
[SUCCESS] Created data directory at /Users/user/.rusty

Create Applications shortcut? (y/N): y
[SUCCESS] Applications shortcut created

[3/4] VERIFICATION PHASE
----------------------------------------
[INFO] Testing Rusty IDE installation...
[SUCCESS] Rusty IDE is working correctly
[INFO] Version: rust_agent 0.1.0

[4/4] INSTALLATION COMPLETE
========================================

  ✓ Rusty IDE has been successfully installed!

  Installation location: /usr/local/bin/rusty
  Data directory: /Users/user/.rusty

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
    2. Or run: source /Users/user/.zshrc

========================================
```

### Windows Installation Output

```
========================================
 Rusty IDE Installer for Windows
========================================

[1/4] CLEANUP PHASE
----------------------------------------
[INFO] Found old agent installation at C:\Users\user\.local\bin\agent.exe
[SUCCESS] Removed old agent binary
[INFO] Found old agent data directory at C:\Users\user\.agent
Do you want to remove old agent data? (y/N): n
[INFO] Keeping old agent data

[2/4] INSTALLATION PHASE
----------------------------------------
[INFO] Building Rusty IDE in release mode...
   Compiling rust_agent v0.1.0
    Finished release [optimized] target(s) in 45.2s
[SUCCESS] Build completed successfully
[INFO] Installing Rusty IDE to C:\Program Files\Rusty...
[SUCCESS] Binary installed successfully
[INFO] Checking PATH configuration...
[INFO] Adding Rusty IDE to system PATH...
[SUCCESS] Added to system PATH
[INFO] Please restart your terminal for PATH changes to take effect
[SUCCESS] Created data directory at C:\Users\user\.rusty

Create desktop shortcut? (y/N): y
[SUCCESS] Desktop shortcut created

[3/4] VERIFICATION PHASE
----------------------------------------
[INFO] Testing Rusty IDE installation...
[SUCCESS] Rusty IDE is working correctly

[4/4] INSTALLATION COMPLETE
========================================

 Rusty IDE has been successfully installed!

 Installation location: C:\Program Files\Rusty\rusty.exe
 Data directory: C:\Users\user\.rusty

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
   1. Restart your terminal/command prompt
   2. Or manually add C:\Program Files\Rusty to your PATH

========================================

Press any key to continue . . .
```

## Manual Testing Checklist

### Pre-Installation Tests

- [ ] Verify Rust is installed: `cargo --version`
- [ ] Check if old agent exists: `which agent` or `where agent`
- [ ] Note current PATH: `echo $PATH` or `echo %PATH%`
- [ ] Check available disk space

### During Installation

- [ ] Script runs without syntax errors
- [ ] Progress indicators display correctly
- [ ] Color output works (Unix)
- [ ] Build completes successfully
- [ ] Binary is copied to correct location
- [ ] PATH is updated (or already correct)
- [ ] Data directory is created
- [ ] User prompts work correctly
- [ ] Desktop shortcut created (if selected)

### Post-Installation Tests

- [ ] `rusty --version` works
- [ ] `rusty --help` displays help
- [ ] Can start in interactive mode: `rusty interactive`
- [ ] Can start in learning mode: `rusty learning`
- [ ] Data directory exists: `ls ~/.rusty/` or `dir %USERPROFILE%\.rusty`
- [ ] Desktop shortcut works (if created)
- [ ] Old agent binary removed (if existed)
- [ ] Old agent data removed/kept (as selected)

### Cleanup Tests (Re-running Script)

- [ ] Can run script multiple times without errors
- [ ] Re-installation works correctly
- [ ] No duplicate PATH entries created
- [ ] No file conflicts
- [ ] Old installation properly replaced

## Common Issues and Solutions

### Issue: "cargo: command not found"

**Solution:**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Issue: "Permission denied" (Unix)

**Solution:**
```bash
chmod +x install-rusty-linux.sh
# or
chmod +x install-rusty-mac.sh
```

### Issue: Build fails with linker errors

**Solution:**
```bash
# Linux - Install build essentials
sudo apt-get install build-essential  # Debian/Ubuntu
sudo dnf install gcc                   # Fedora

# macOS - Install Xcode Command Line Tools
xcode-select --install
```

### Issue: "This script requires administrator privileges" (Windows)

**Solution:**
Right-click on Command Prompt or PowerShell and select "Run as administrator"

### Issue: PATH not updated after installation

**Solution:**
```bash
# Manually add to PATH

# Linux/macOS (add to ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.local/bin:$PATH"  # Linux
export PATH="/usr/local/bin:$PATH"     # macOS

# Windows (run as administrator)
setx /M PATH "%PATH%;C:\Program Files\Rusty"
```

## Performance Expectations

### Build Time
- Debug build: 1-2 minutes
- Release build: 2-5 minutes
- Depends on: CPU, RAM, disk speed, dependency count

### Total Installation Time
- Fast system: 3-5 minutes
- Average system: 5-10 minutes
- Slow system: 10-15 minutes

## Verification Commands

After installation, verify with these commands:

```bash
# Check installation
which rusty          # Unix
where rusty          # Windows

# Test basic functionality
rusty --version
rusty --help

# Check data directory
ls -la ~/.rusty/     # Unix
dir %USERPROFILE%\.rusty  # Windows

# Check PATH
echo $PATH | grep rusty   # Unix
echo %PATH% | findstr rusty  # Windows
```

## Rollback Procedure

If installation fails or you want to revert:

### Linux
```bash
rm ~/.local/bin/rusty
rm -rf ~/.rusty
# Remove PATH entry from ~/.bashrc or ~/.zshrc
```

### macOS
```bash
sudo rm /usr/local/bin/rusty
rm -rf ~/.rusty
# Remove PATH entry from ~/.zshrc or ~/.bash_profile
```

### Windows
```cmd
del "C:\Program Files\Rusty\rusty.exe"
rmdir "C:\Program Files\Rusty"
rmdir /s "%USERPROFILE%\.rusty"
# Manually remove from PATH in System Environment Variables
```

## Success Criteria

Installation is successful when:

1. `rusty --version` runs without errors
2. `rusty --help` displays help text
3. Binary is in the expected location
4. PATH includes installation directory
5. Data directory exists at `~/.rusty/`
6. Can start application: `rusty`
7. No error messages during installation
8. Old agent installation cleaned up (if existed)

---

**Note:** These tests should be performed on clean systems and systems with existing installations to ensure all scenarios work correctly.
