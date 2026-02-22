# Rusty IDE v2 - Installation Scripts

This directory contains platform-specific installation scripts for Rusty IDE v2.

## Available Scripts

### 1. Windows Installation (`install-windows.bat`)
For Windows 10/11 users.

**Requirements:**
- Windows 10 or later
- Administrator privileges
- Rust and Cargo installed

**Usage:**
```batch
# Right-click and select "Run as administrator"
install-windows.bat
```

**Installation Location:** `C:\Program Files\Rusty\rusty.exe`

---

### 2. macOS Installation (`install-mac.sh`)
For macOS users (Intel and Apple Silicon).

**Requirements:**
- macOS 10.15 (Catalina) or later
- Rust and Cargo installed
- Xcode Command Line Tools

**Usage:**
```bash
chmod +x install-mac.sh
./install-mac.sh
```

**Installation Location:** `/usr/local/bin/rusty`

---

### 3. Linux Installation (`install-linux.sh`)
For Linux users (Ubuntu, Debian, Fedora, Arch, etc.).

**Requirements:**
- Modern Linux distribution
- Rust and Cargo installed
- Build essentials (gcc, pkg-config, etc.)

**Usage:**
```bash
chmod +x install-linux.sh
./install-linux.sh
```

**Installation Location:** `~/.local/bin/rusty`

---

## What Each Script Does

All installation scripts follow the same 5-phase process:

### Phase 1: Cleanup
- Removes old agent binary from `~/.local/bin/agent` (or `%USERPROFILE%\.local\bin\agent.exe` on Windows)
- Deletes old agent startup scripts
- Prompts to optionally delete old agent data directory (`~/.agent/`)
- Cleans up old PATH entries

### Phase 2: Build
- Navigates to `src-tauri` directory
- Runs `cargo build --release` to compile Rusty IDE
- Shows build progress and errors if any

### Phase 3: Install
- Copies the built binary to system location:
  - **Windows:** `C:\Program Files\Rusty\rusty.exe`
  - **macOS:** `/usr/local/bin/rusty`
  - **Linux:** `~/.local/bin/rusty`
- Sets executable permissions (Unix systems)
- Adds installation directory to system PATH

### Phase 4: Setup Data Directory
Creates `~/.rusty/` directory structure:
```
~/.rusty/
├── agent/           # Request/response files for AI agent
├── workspaces/      # Workspace-specific data
└── permissions.json # Permission storage
```

### Phase 5: Verify
- Tests that `rusty` command works
- Displays version information
- Shows usage instructions
- Creates desktop shortcuts (optional on Windows/macOS)
- Creates desktop entry (Linux)

---

## Post-Installation

After installation completes, you can use Rusty IDE:

```bash
# Open a specific folder
rusty /path/to/project

# Open current directory
rusty .

# Show help
rusty --help
```

---

## Troubleshooting

### Windows

**Problem:** "This installer requires administrator privileges"
- **Solution:** Right-click the script and select "Run as administrator"

**Problem:** "rusty: command not found"
- **Solution:** Restart your terminal or add `C:\Program Files\Rusty` to your PATH manually

### macOS

**Problem:** "Permission denied"
- **Solution:** Run `chmod +x install-mac.sh` before executing

**Problem:** Binary won't run due to security settings
- **Solution:** Go to System Preferences > Security & Privacy and allow the app

### Linux

**Problem:** "rusty: command not found"
- **Solution:** Restart terminal or run `source ~/.bashrc` (or `~/.zshrc`)

**Problem:** Build fails with missing dependencies
- **Solution:** Install build essentials:
  ```bash
  # Ubuntu/Debian
  sudo apt install build-essential pkg-config libssl-dev
  
  # Fedora
  sudo dnf install gcc pkg-config openssl-devel
  
  # Arch
  sudo pacman -S base-devel
  ```

---

## Data Directory Structure

The installer creates `~/.rusty/` with the following structure:

```
~/.rusty/
├── agent/
│   ├── request.json      # AI agent requests (created at runtime)
│   └── response.json     # AI agent responses (created at runtime)
├── workspaces/
│   └── <workspace-id>/   # Per-workspace data (created as needed)
└── permissions.json      # Stores user permission choices
```

---

## Uninstallation

To uninstall Rusty IDE:

### Windows
```batch
# Delete binary
del "C:\Program Files\Rusty\rusty.exe"
rmdir "C:\Program Files\Rusty"

# Delete data
rmdir /s "%USERPROFILE%\.rusty"

# Remove from PATH manually via System Properties
```

### macOS
```bash
# Delete binary
sudo rm /usr/local/bin/rusty

# Delete application bundle
sudo rm -rf "/Applications/Rusty IDE.app"

# Delete data
rm -rf ~/.rusty
```

### Linux
```bash
# Delete binary
rm ~/.local/bin/rusty

# Delete desktop entry
rm ~/.local/share/applications/rusty-ide.desktop

# Delete data
rm -rf ~/.rusty

# Remove PATH entry from shell config if added
```

---

## Contributing

If you encounter issues with the installation scripts, please report them on the project repository.

---

## License

Same as the main Rusty IDE project.
