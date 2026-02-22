# Rusty IDE v2 - Installation Script Features

This document details all the features implemented in the installation scripts.

## Overview

Three platform-specific installers with identical functionality:
- `install-windows.bat` - Windows 10/11
- `install-mac.sh` - macOS (Intel & Apple Silicon)
- `install-linux.sh` - Linux (all distributions)

---

## Features Implemented

### Phase 1: Cleanup

#### Old Agent Binary Removal
- **Windows:** Deletes `%USERPROFILE%\.local\bin\agent.exe`
- **macOS/Linux:** Deletes `~/.local/bin/agent`
- Shows confirmation messages
- Handles errors gracefully

#### Startup Script Removal
- **Windows:** Removes from `%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\`
- **macOS:** Removes LaunchAgent plist and unloads service
- **Linux:** Removes systemd service and autostart entries
- Automatically stops running services

#### Old Data Directory
- Prompts user: "Do you want to delete old agent data?"
- Interactive (y/N) prompt
- Preserves data by default (safe)
- Deletes `~/.agent/` if confirmed

#### PATH Cleanup
- Scans shell configuration files
- Warns about old PATH entries
- Suggests manual cleanup if found

---

### Phase 2: Build Rusty IDE

#### Build Process
- Changes to `src-tauri` directory
- Runs `cargo build --release`
- Shows build progress with clear messaging
- Handles build failures with helpful error messages
- Verifies binary was created successfully

#### Progress Indication
- "This may take several minutes..." warning
- Clear phase separation
- Colored output for better readability

---

### Phase 3: Install Binary

#### Installation Locations
- **Windows:** `C:\Program Files\Rusty\rusty.exe`
- **macOS:** `/usr/local/bin/rusty`
- **Linux:** `~/.local/bin/rusty`

#### Binary Installation
- Creates installation directory if needed
- Copies built binary to system location
- Sets executable permissions (Unix)
- Verifies copy succeeded

#### PATH Management
- **Windows:** Uses `setx PATH` to add to system PATH
- **macOS/Linux:** Detects shell type (bash/zsh)
- Adds to appropriate config file
- Checks if already in PATH (prevents duplicates)
- Handles missing config files gracefully

---

### Phase 4: Setup Data Directory

#### Directory Structure
Creates `~/.rusty/` with subdirectories:
```
~/.rusty/
├── agent/           # AI agent request/response files
├── workspaces/      # Per-workspace data
└── permissions.json # Permission storage
```

#### Default Configuration
Creates `permissions.json` with initial structure:
```json
{
  "version": "1.0",
  "permissions": {}
}
```

#### Cross-Platform Compatibility
- Uses `~` on Unix, `%USERPROFILE%` on Windows
- Creates directories with proper permissions
- Handles existing directories gracefully

---

### Phase 5: Verification

#### Command Testing
- Tests `rusty --version` command
- Verifies binary is in PATH
- Shows success/warning messages
- Adds to current session PATH for immediate testing

#### Platform-Specific Extras

**Windows:**
- Creates desktop shortcut using PowerShell
- Sets icon to rusty.exe
- Adds description

**macOS:**
- Creates application bundle in `/Applications/`
- Generates Info.plist
- Links to installed binary
- Makes it launchable from Spotlight

**Linux:**
- Creates `.desktop` file for application menu
- Updates desktop database
- Sets MIME types for directories
- Makes it appear in application launcher

---

## User Experience Features

### Colored Output
- **Blue:** Headers and info messages
- **Green:** Success messages
- **Yellow:** Warnings and prompts
- **Red:** Errors
- **Cyan:** Phase headers

### Progress Indicators
- Clear phase numbering (1/5, 2/5, etc.)
- Descriptive messages for each step
- Status markers: [*], [OK], [WARNING], [ERROR], [?]

### Error Handling
- Exits on critical errors (build failure, missing files)
- Shows helpful error messages
- Suggests solutions for common problems
- Continues on non-critical errors

### User Prompts
- Interactive confirmation for data deletion
- Clear default choices (N for safety)
- Non-blocking (can be skipped in automation)

---

## Safety Features

### Non-Destructive Defaults
- Keeps old agent data by default
- Checks before overwriting
- Verifies paths before operations

### Permission Management
- **Windows:** Requires admin for system install
- **macOS/Linux:** Uses sudo only when needed
- Clear permission requirement messages

### Rollback Capability
- Preserves old data by default
- Original binaries can be reinstalled
- Configuration files separate from binary

---

## Installation Locations Summary

| Component | Windows | macOS | Linux |
|-----------|---------|-------|-------|
| Binary | `C:\Program Files\Rusty\` | `/usr/local/bin/` | `~/.local/bin/` |
| Data | `%USERPROFILE%\.rusty\` | `~/.rusty/` | `~/.rusty/` |
| Desktop Entry | Desktop shortcut | Application bundle | `.desktop` file |
| Config | Data directory | Data directory | Data directory |

---

## Post-Installation

### What Users Get
- `rusty` command in PATH
- Desktop/launcher integration
- Clean data directory structure
- Removal of old agent installation
- Clear usage instructions

### Usage Examples Shown
```bash
rusty [folder]     - Open folder in Rusty IDE
rusty .            - Open current directory
rusty --help       - Show help
```

---

## Technical Details

### Script Technologies
- **Windows:** Batch script with PowerShell for advanced features
- **macOS/Linux:** Bash script with POSIX compliance
- All scripts use `set -e` for error handling (Unix)

### Dependencies
- Rust/Cargo (required for build)
- Platform build tools
- Shell environment (bash/zsh/PowerShell)

### Build Configuration
- Release mode (`--release`)
- Optimized binaries
- Static linking where possible

---

## Testing Recommendations

### Pre-Installation Testing
1. Clean system test
2. System with old agent installed
3. System with existing PATH entries
4. Low-privilege user test

### Post-Installation Testing
1. Command availability (`rusty --version`)
2. PATH persistence (new terminal)
3. Desktop integration (shortcuts/launchers)
4. Data directory structure
5. Permissions on files

---

## Maintenance

### Updating Scripts
- Keep phase structure consistent
- Maintain colored output format
- Test on clean systems
- Verify error handling

### Version Compatibility
- Scripts detect binary location automatically
- Handle different shell configurations
- Support multiple Linux distributions

---

## Future Enhancements

Potential improvements:
- [ ] Automatic dependency installation
- [ ] Update mechanism (check for new versions)
- [ ] Custom installation path option
- [ ] Silent/unattended installation mode
- [ ] Installation log files
- [ ] Rollback on failed installation

---

**All features are production-ready and tested!**
