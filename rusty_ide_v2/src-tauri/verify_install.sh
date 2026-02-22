#!/bin/bash
# Verification script for Rusty TUI installation

echo "=== Rusty TUI Installation Verification ==="
echo ""

# Check binaries
echo "1. Checking binaries..."
if [ -f "$HOME/.local/bin/rusty-tui" ]; then
    echo "   ✓ rusty-tui found at $HOME/.local/bin/rusty-tui"
    echo "     Size: $(du -h $HOME/.local/bin/rusty-tui | cut -f1)"
else
    echo "   ✗ rusty-tui NOT found"
fi

if [ -f "$HOME/.local/bin/rusty" ]; then
    echo "   ✓ rusty launcher found at $HOME/.local/bin/rusty"
else
    echo "   ✗ rusty launcher NOT found"
fi

echo ""

# Check PATH
echo "2. Checking PATH..."
if echo $PATH | grep -q "$HOME/.local/bin"; then
    echo "   ✓ $HOME/.local/bin is in PATH"
else
    echo "   ⚠ $HOME/.local/bin is NOT in PATH"
    echo "     Add this to your ~/.bashrc:"
    echo "     export PATH=\"\$HOME/.local/bin:\$PATH\""
fi

echo ""

# Check if commands are accessible
echo "3. Checking command accessibility..."
if command -v rusty &> /dev/null; then
    echo "   ✓ 'rusty' command is accessible"
    echo "     Location: $(which rusty)"
else
    echo "   ✗ 'rusty' command NOT accessible"
fi

if command -v rusty-tui &> /dev/null; then
    echo "   ✓ 'rusty-tui' command is accessible"
    echo "     Location: $(which rusty-tui)"
else
    echo "   ✗ 'rusty-tui' command NOT accessible"
fi

echo ""

# Check terminal emulators
echo "4. Available terminal emulators:"
for term in gnome-terminal xterm konsole alacritty kitty terminator; do
    if command -v $term &> /dev/null; then
        echo "   ✓ $term"
    fi
done

echo ""
echo "=== Installation Complete! ==="
echo ""
echo "To launch Rusty IDE:"
echo "  - In new window: rusty"
echo "  - In current terminal: rusty-tui"
echo ""
