# Setup Instructions for Rusty IDE v2

## What's Included

This React frontend includes:

### Components Created
- ✅ `src/App.jsx` - Main application with layout
- ✅ `src/main.jsx` - React entry point
- ✅ `src/components/Header.jsx` - Menu bar
- ✅ `src/components/FileTree.jsx` - File explorer
- ✅ `src/components/MonacoEditor.jsx` - Code editor
- ✅ `src/components/Terminal.jsx` - Integrated terminal
- ✅ `src/components/AgentSidebar.jsx` - AI chat interface

### Styles Created
- ✅ `src/styles/theme.css` - Dark theme variables
- ✅ `src/styles/App.css` - Component styles

### Configuration Files
- ✅ `package.json` - Dependencies
- ✅ `vite.config.js` - Vite configuration
- ✅ `jsconfig.json` - JavaScript/JSX configuration
- ✅ `index.html` - HTML entry point
- ✅ `.gitignore` - Git ignore rules

## Installation Steps

### 1. Install Dependencies

```bash
cd /workspace/jashan/rusty_ide_v2
npm install
```

This will install:
- react & react-dom
- @monaco-editor/react
- @xterm/xterm & @xterm/addon-fit
- @tauri-apps/api
- vite & @vitejs/plugin-react

### 2. Verify Installation

```bash
npm list --depth=0
```

You should see all packages listed in package.json.

### 3. Run Development Server

```bash
# Frontend only (browser)
npm run dev

# Full Tauri app (requires Rust backend)
npm run tauri:dev
```

## Backend Requirements

The frontend expects these Tauri commands to be implemented in your Rust backend:

### File Operations
```rust
#[tauri::command]
fn read_file(path: String) -> Result<String, String>

#[tauri::command]
fn write_file(path: String, content: String) -> Result<(), String>

#[tauri::command]
fn list_workspace_files(path: String) -> Result<Vec<FileInfo>, String>

#[tauri::command]
fn create_new_file(workspace_path: String) -> Result<String, String>
```

### Workspace Management
```rust
#[tauri::command]
fn get_last_workspace() -> Result<String, String>

#[tauri::command]
fn set_workspace(path: String) -> Result<(), String>
```

### Terminal/PTY
```rust
#[tauri::command]
fn create_pty(cols: u16, rows: u16) -> Result<String, String>

#[tauri::command]
fn write_to_pty(pty_id: String, data: String) -> Result<(), String>

#[tauri::command]
fn resize_pty(pty_id: String, cols: u16, rows: u16) -> Result<(), String>

#[tauri::command]
fn close_pty(pty_id: String) -> Result<(), String>

#[tauri::command]
fn kill_pty(pty_id: String) -> Result<(), String>
```

### Agent Communication
```rust
#[tauri::command]
fn write_agent_context(context: String) -> Result<(), String>

#[tauri::command]
fn write_agent_query(query: String, workspace_path: String) -> Result<(), String>

#[tauri::command]
fn read_agent_response() -> Result<String, String>
```

### Events
The backend should emit these events:
- `pty-output-${ptyId}` - Terminal output
- `pty-exit-${ptyId}` - Process exit code

## Layout Structure

```
┌──────────────────────────────────────────────────────┐
│ Header (35px)                                        │
│ [File] [Edit] [View] [Terminal]         Rusty IDE v2│
├──────────┬───────────────────────┬───────────────────┤
│          │                       │                   │
│ FileTree │   Monaco Editor       │   AgentSidebar    │
│ (300px)  │   (Flexible)          │   (400px)         │
│          │                       │                   │
│ 📁 src   │   Code here...        │   💬 AI Chat      │
│ 📄 main  │   with syntax         │   Messages...     │
│ 📄 app   │   highlighting        │   Input box       │
│          │                       │   [Send]          │
│          │                       │                   │
├──────────┴───────────────────────┴───────────────────┤
│ Terminal (250px, resizable)                          │
│ $ cargo run                                          │
│ > Running...                                         │
└──────────────────────────────────────────────────────┘
```

## Features Implemented

### Monaco Editor
- ✅ Syntax highlighting (Rust, JS, Python, etc.)
- ✅ Auto-save with 1s debounce
- ✅ Tab management for multiple files
- ✅ Custom dark theme ("rusty-dark")
- ✅ Keyboard shortcuts (Ctrl+S)
- ✅ Line numbers and minimap
- ✅ Font ligatures support

### Terminal
- ✅ Full xterm.js integration
- ✅ Resizable height (drag to resize)
- ✅ Custom dark theme
- ✅ Clear and kill buttons
- ✅ PTY backend communication
- ✅ Real-time input/output

### AI Sidebar
- ✅ Chat interface
- ✅ Grant access workflow
- ✅ Message history
- ✅ Poll-based responses (500ms)
- ✅ Code change preview
- ✅ Apply changes button
- ✅ Loading indicators
- ✅ Markdown-like formatting

### File Tree
- ✅ File listing
- ✅ Icon indicators
- ✅ Directory expansion
- ✅ File selection
- ✅ Loading state

### Header
- ✅ Menu bar (File, Edit, View, Terminal)
- ✅ Dropdown menus
- ✅ Keyboard shortcuts display
- ✅ Toggle actions

## Customization

### Change Theme Colors
Edit `src/styles/theme.css`:

```css
:root {
  --bg-primary: #1e1e1e;      /* Main background */
  --bg-secondary: #252526;    /* Secondary panels */
  --accent-primary: #007acc;  /* Accent color */
  --text-primary: #cccccc;    /* Main text */
}
```

### Change Editor Font
Edit `src/components/MonacoEditor.jsx`:

```javascript
editor.updateOptions({
  fontSize: 14,
  fontFamily: 'Fira Code, Consolas, Monaco',
  fontLigatures: true
});
```

### Change Terminal Theme
Edit `src/components/Terminal.jsx`:

```javascript
theme: {
  background: '#1e1e1e',
  foreground: '#cccccc',
  // ... other colors
}
```

## Testing

### Test Monaco Editor
1. Open a file
2. Edit content
3. Verify auto-save (check console)
4. Press Ctrl+S (should save)

### Test Terminal
1. Terminal should appear at bottom
2. Drag top border to resize
3. Type commands (requires PTY backend)

### Test AI Sidebar
1. Click "Grant Access"
2. Type a message
3. Verify polling starts (status indicator)
4. Check for response

## Troubleshooting

### Monaco Editor not loading?
Check:
- `@monaco-editor/react` is installed
- No console errors
- Vite is serving correctly

### Terminal blank screen?
Check:
- xterm.css is imported
- PTY backend is running
- No console errors

### Styles not applying?
Check:
- theme.css is imported in App.jsx
- App.css is imported in App.jsx
- CSS variables are defined

### Components not rendering?
Check:
- All imports are correct
- No syntax errors in JSX
- React DevTools for component tree

## Next Steps

1. **Implement Tauri backend** - See backend requirements above
2. **Test all features** - Verify each component works
3. **Customize theme** - Make it your own
4. **Add features** - Extend with your own components

## Resources

- [Monaco Editor Docs](https://microsoft.github.io/monaco-editor/)
- [xterm.js Docs](https://xtermjs.org/)
- [Tauri Docs](https://tauri.app/)
- [React Docs](https://react.dev/)
- [Vite Docs](https://vitejs.dev/)

---

**Ready to build? Run `npm install` and then `npm run dev`!** 🚀
