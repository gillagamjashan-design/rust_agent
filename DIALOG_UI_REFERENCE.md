# File Creation Confirmation Dialog - UI Reference

## Dialog Appearance

```
┌────────────────────────────────────────────────────────────┐
│  📝 Confirm File Creation                                  │
├────────────────────────────────────────────────────────────┤
│                                                            │
│  The agent wants to create the following files:           │
│                                                            │
│  ┌──────────────────────────────────────────────────────┐ │
│  │ ╔════════════════════════════════════════════════╗   │ │
│  │ ║  📄 main.rs                                    ║   │ │
│  │ ║                                                ║   │ │
│  │ ║  Preview:                                      ║   │ │
│  │ ║  fn main() {                                   ║   │ │
│  │ ║      println!("Hello, world!");                ║   │ │
│  │ ║  }                                             ║   │ │
│  │ ╚════════════════════════════════════════════════╝   │ │
│  │                                                      │ │
│  │ ╔════════════════════════════════════════════════╗   │ │
│  │ ║  📄 Cargo.toml                                 ║   │ │
│  │ ║                                                ║   │ │
│  │ ║  Preview:                                      ║   │ │
│  │ ║  [package]                                     ║   │ │
│  │ ║  name = "hello_world"                          ║   │ │
│  │ ║  version = "0.1.0"                             ║   │ │
│  │ ║  ... (3 more lines)                            ║   │ │
│  │ ╚════════════════════════════════════════════════╝   │ │
│  └──────────────────────────────────────────────────────┘ │
│                                                            │
│  ──────────────────────────────────────────────────────    │
│                                                            │
│  [ ✅ Create Files ]   [ ❌ Cancel ]                       │
│                                                            │
│  💡 Tip: Press Enter to create, Esc to cancel             │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

## UI Elements

### Header
- **Title**: "📝 Confirm File Creation"
- **Style**: Bold, prominent

### File List Section
- **Scrollable area**: Max height 300px
- **Per-file group**:
  - File icon: 📄
  - File path: Bold text (e.g., `main.rs`)
  - Content preview: First 3 lines
  - Line count indicator: "... (N more lines)" if content exceeds 3 lines
  - Grouped with border/background

### Action Buttons
- **Create Files**: Green checkmark icon, positive action
- **Cancel**: Red X icon, negative action
- Horizontal layout with spacing

### Footer Tip
- Keyboard shortcuts hint
- Small, gray text

## Color Scheme (Tokyo Night Theme)

| Element | Color | Hex |
|---------|-------|-----|
| Background | Dark gray | `#1a1b26` |
| File path | Bright cyan | `#7dcfff` |
| Preview text | Foreground | `#a9b1d6` |
| Create button | Green | `#9ece6a` |
| Cancel button | Yellow | `#e0af68` |
| Tip text | Gray | `#565f89` |

## Interaction Flow

```
User Action              → Dialog State         → Result
─────────────────────────────────────────────────────────────
1. Agent generates code  → Dialog appears       → User sees preview
2. User clicks "Create"  → Dialog closes        → Files created
   OR Press Enter
3. User clicks "Cancel"  → Dialog closes        → No files created
   OR Press Esc          → Message: "Cancelled"

Special Cases:
─────────────────────────────────────────────────────────────
File exists             → Modify instead        → "Modified file: X"
Permission denied       → Error shown           → "File operation failed"
Invalid path            → Error shown           → Security validation message
```

## Example Scenarios

### Scenario 1: Single File
```
User: "Create a hello world program"

Dialog shows:
- 📄 main.rs
- Preview: fn main() { ... }

User clicks "Create Files"
Result: main.rs created with success message
```

### Scenario 2: Multiple Files
```
User: "Create a Rust library with Cargo.toml and lib.rs"

Dialog shows:
- 📄 Cargo.toml (preview: [package]...)
- 📄 lib.rs (preview: pub fn...)

User presses Enter (keyboard shortcut)
Result: Both files created
```

### Scenario 3: User Cancels
```
User: "Create a test file"

Dialog shows:
- 📄 test.rs

User presses Esc
Result: Dialog closes, message "File creation cancelled."
```

### Scenario 4: No Code Blocks
```
User: "What is ownership?"

Result: Normal chat response, NO dialog appears
```

## Implementation Details

### egui Code Structure

```rust
egui::Window::new("📝 Confirm File Creation")
    .collapsible(false)
    .resizable(false)
    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
    .show(ctx, |ui| {
        // Header
        ui.heading("The agent wants to create the following files:");

        // Scrollable file list
        egui::ScrollArea::vertical()
            .max_height(300.0)
            .show(ui, |ui| {
                for file in files {
                    ui.group(|ui| {
                        // File path + preview
                    });
                }
            });

        // Action buttons
        ui.horizontal(|ui| {
            if ui.button("✅ Create Files").clicked() { ... }
            if ui.button("❌ Cancel").clicked() { ... }
        });

        // Tip
        ui.label("💡 Tip: ...");
    });
```

### Keyboard Handling

```rust
// Check for Enter key
if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
    app.approve_file_creation();
}

// Check for Escape key
if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
    app.cancel_file_creation();
}
```

## Accessibility Features

✅ **Keyboard navigation**: Tab through buttons, Enter/Esc shortcuts
✅ **Clear visual hierarchy**: Title → Files → Actions → Tip
✅ **Scrollable content**: Handles many files without overflow
✅ **Icon indicators**: Visual cues for file type and actions
✅ **Centered positioning**: Easy to spot in any window size
✅ **Non-blocking**: Dialog overlays chat without freezing UI

## Edge Cases Handled

1. **Many files (> 10)**: Scrollable list
2. **Long file paths**: Wrapped or truncated
3. **Large file content**: Only first 3 lines shown
4. **Empty code block**: Skipped (no dialog)
5. **Dialog already open**: Prevents duplicate dialogs
6. **Concurrent operations**: Queued via message channel
