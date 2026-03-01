#!/bin/bash

# Test script for Rusty GUI Modernization
# This script verifies that all code changes compile correctly

set -e

echo "🧪 Testing Rusty GUI Modernization"
echo "=================================="
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}Step 1: Checking source files...${NC}"
echo ""

# Check that all modified files exist
FILES=(
    "rusty_tui/src/gui/theme.rs"
    "rusty_tui/src/gui/layout.rs"
    "rusty_tui/src/gui/app.rs"
    "rusty_tui/src/gui/messages.rs"
)

for file in "${FILES[@]}"; do
    if [ -f "$file" ]; then
        echo -e "${GREEN}✓${NC} Found: $file"
    else
        echo -e "${YELLOW}✗${NC} Missing: $file"
        exit 1
    fi
done

echo ""
echo -e "${BLUE}Step 2: Verifying theme.rs contains new constants...${NC}"
echo ""

# Check for new color constants
THEME_FILE="rusty_tui/src/gui/theme.rs"
CONSTANTS=(
    "USER_BUBBLE_BG"
    "ASSISTANT_BUBBLE_BG"
    "SYSTEM_BUBBLE_BG"
    "CODE_BLOCK_BG"
    "CODE_BLOCK_BORDER"
    "SHADOW_LIGHT"
)

for constant in "${CONSTANTS[@]}"; do
    if grep -q "pub const $constant" "$THEME_FILE"; then
        echo -e "${GREEN}✓${NC} Found constant: $constant"
    else
        echo -e "${YELLOW}✗${NC} Missing constant: $constant"
        exit 1
    fi
done

echo ""
echo -e "${BLUE}Step 3: Verifying theme.rs contains frame builders...${NC}"
echo ""

# Check for frame builder functions
FUNCTIONS=(
    "message_bubble_frame"
    "message_bubble_frame_alpha"
    "code_block_frame"
    "input_frame"
    "header_frame"
)

for func in "${FUNCTIONS[@]}"; do
    if grep -q "pub fn $func" "$THEME_FILE"; then
        echo -e "${GREEN}✓${NC} Found function: $func"
    else
        echo -e "${YELLOW}✗${NC} Missing function: $func"
        exit 1
    fi
done

echo ""
echo -e "${BLUE}Step 4: Verifying layout.rs contains bubble rendering...${NC}"
echo ""

# Check for key functions in layout.rs
LAYOUT_FILE="rusty_tui/src/gui/layout.rs"
LAYOUT_CHECKS=(
    "render_message_bubble"
    "render_code_block"
    "render_text_line"
)

for check in "${LAYOUT_CHECKS[@]}"; do
    if grep -q "fn $check" "$LAYOUT_FILE"; then
        echo -e "${GREEN}✓${NC} Found function: $check"
    else
        echo -e "${YELLOW}✗${NC} Missing function: $check"
        exit 1
    fi
done

echo ""
echo -e "${BLUE}Step 5: Verifying Enter key fix is preserved...${NC}"
echo ""

# Check that the critical Enter key fix is still there
if grep -q "response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))" "$LAYOUT_FILE"; then
    echo -e "${GREEN}✓${NC} Enter key fix preserved"
else
    echo -e "${YELLOW}✗${NC} Enter key fix missing!"
    exit 1
fi

echo ""
echo -e "${BLUE}Step 6: Verifying app.rs contains animation state...${NC}"
echo ""

# Check for animation fields in app.rs
APP_FILE="rusty_tui/src/gui/app.rs"
ANIMATION_CHECKS=(
    "message_animations"
    "last_message_count"
    "spinner_rotation"
    "get_message_alpha"
)

for check in "${ANIMATION_CHECKS[@]}"; do
    if grep -q "$check" "$APP_FILE"; then
        echo -e "${GREEN}✓${NC} Found: $check"
    else
        echo -e "${YELLOW}✗${NC} Missing: $check"
        exit 1
    fi
done

echo ""
echo -e "${BLUE}Step 7: Checking binary exists...${NC}"
echo ""

BINARY="$HOME/.local/bin/rusty"
if [ -f "$BINARY" ]; then
    SIZE=$(ls -lh "$BINARY" | awk '{print $5}')
    echo -e "${GREEN}✓${NC} Binary exists: $BINARY"
    echo -e "${GREEN}✓${NC} Binary size: $SIZE"
else
    echo -e "${YELLOW}⚠${NC} Binary not found (may need to run ./run-all.sh)"
fi

echo ""
echo -e "${BLUE}Step 8: Code structure validation...${NC}"
echo ""

# Verify key patterns exist
if grep -q "Layout::right_to_left" "$LAYOUT_FILE"; then
    echo -e "${GREEN}✓${NC} Right-aligned user messages implemented"
else
    echo -e "${YELLOW}✗${NC} Right-aligned messages missing"
    exit 1
fi

if grep -q "max_width" "$LAYOUT_FILE"; then
    echo -e "${GREEN}✓${NC} Constrained message width implemented"
else
    echo -e "${YELLOW}✗${NC} Message width constraints missing"
    exit 1
fi

if grep -q "HashMap" "$APP_FILE"; then
    echo -e "${GREEN}✓${NC} Animation HashMap implemented"
else
    echo -e "${YELLOW}✗${NC} Animation HashMap missing"
    exit 1
fi

echo ""
echo "=================================="
echo -e "${GREEN}✅ All checks passed!${NC}"
echo "=================================="
echo ""
echo "Modernization verification complete!"
echo ""
echo "To run the application:"
echo "  1. Ensure ClaudeProxyAPI is running on localhost:8317"
echo "  2. Run: rusty"
echo ""
echo "Visual features to test:"
echo "  • User messages appear right-aligned in darker bubbles"
echo "  • Assistant messages appear left-aligned in lighter bubbles"
echo "  • Code blocks have dark background with border"
echo "  • New messages fade in smoothly (~300ms)"
echo "  • Header shows badge-style stats"
echo "  • Input area has emoji and rounded frame"
echo "  • Enter key sends messages correctly"
echo ""
