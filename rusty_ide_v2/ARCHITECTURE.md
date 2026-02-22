# Rusty IDE v2 - Architecture

## Component Hierarchy

```
App.jsx
├── Header.jsx
│   ├── File Menu
│   ├── Edit Menu
│   ├── View Menu
│   └── Terminal Menu
│
├── Main Content Area
│   ├── FileTree.jsx (toggleable)
│   │   └── File list with icons
│   │
│   ├── Editor Area
│   │   └── MonacoEditor.jsx
│   │       ├── Editor Tabs
│   │       └── Monaco Editor Instance
│   │
│   └── AgentSidebar.jsx (toggleable)
│       ├── Agent Header
│       ├── Workspace Info
│       ├── Grant Access Button
│       ├── Message History
│       └── Input Area
│
└── Terminal.jsx (toggleable)
    ├── Terminal Header
    ├── Resize Handle
    └── xterm.js Instance
```

## Data Flow

### File Operations

```
User clicks file in FileTree
    ↓
FileTree.onFileSelect(filePath)
    ↓
App.handleFileSelect(filePath)
    ↓
window.openFileInEditor(filePath)
    ↓
MonacoEditor.openFile(filePath)
    ↓
invoke('read_file', { path })
    ↓
Tauri Backend reads file
    ↓
Content returned to Monaco
    ↓
File displayed in editor
```

### Save Operations

```
User edits code in Monaco
    ↓
MonacoEditor.handleEditorChange(value)
    ↓
Debounce timer (1000ms)
    ↓
MonacoEditor.handleSave(filePath, content)
    ↓
invoke('write_file', { path, content })
    ↓
Tauri Backend writes file
    ↓
Save complete
```

### Terminal Communication

```
User types in terminal
    ↓
xterm.onData(data)
    ↓
invoke('write_to_pty', { ptyId, data })
    ↓
Tauri Backend → PTY process
    ↓
Process outputs response
    ↓
Backend emits event: 'pty-output-{ptyId}'
    ↓
Terminal.listen() receives event
    ↓
xterm.write(output)
    ↓
Output displayed in terminal
```

### Agent Communication

```
User types message in AI sidebar
    ↓
AgentSidebar.handleSendMessage()
    ↓
invoke('write_agent_query', { query, workspacePath })
    ↓
Tauri Backend writes to agent input file
    ↓
AgentSidebar.startPollingForResponse()
    ↓
Poll every 500ms: invoke('read_agent_response')
    ↓
Agent processes query and writes response
    ↓
Backend reads response file
    ↓
Response returned to sidebar
    ↓
AgentSidebar.handleAgentResponse(response)
    ↓
Message displayed, code changes offered
    ↓
User clicks "Apply Changes"
    ↓
invoke('write_file') for each change
    ↓
Files updated, editor reloaded
```

## State Management

### App.jsx State
```javascript
{
  workspacePath: string | null,      // Current workspace directory
  showFileTree: boolean,             // FileTree visibility
  showTerminal: boolean,             // Terminal visibility
  showAgent: boolean,                // Agent sidebar visibility
  currentFile: string | null         // Currently active file
}
```

### MonacoEditor.jsx State
```javascript
{
  openFiles: Array<{path, name}>,    // All open files
  activeFile: string | null,         // Currently active file
  fileContents: {                    // Content cache
    [filePath]: string
  }
}
```

### AgentSidebar.jsx State
```javascript
{
  messages: Array<{                  // Chat history
    id, role, content, timestamp
  }>,
  inputValue: string,                // Current input
  isWaiting: boolean,                // Waiting for response
  hasGrantedAccess: boolean,         // Access granted flag
  currentContext: {                  // Current code changes
    codeChanges: Array<{file_path, new_content}>
  } | null
}
```

### Terminal.jsx State
```javascript
{
  height: number,                    // Terminal height in pixels
  isDragging: boolean,               // Resize handle dragging
  ptyId: string | null               // PTY process ID
}
```

### FileTree.jsx State
```javascript
{
  files: Array<{                     // File listing
    name, path, isDirectory
  }>,
  expandedDirs: Set<string>,         // Expanded directories
  loading: boolean                   // Loading state
}
```

## Styling System

### CSS Variables (theme.css)
```
Colors:
  --bg-primary       → Main background (#1e1e1e)
  --bg-secondary     → Panels (#252526)
  --bg-tertiary      → Elevated elements (#2d2d30)
  --text-primary     → Main text (#cccccc)
  --accent-primary   → Accent color (#007acc)

Spacing:
  --spacing-xs  → 4px
  --spacing-sm  → 8px
  --spacing-md  → 12px
  --spacing-lg  → 16px
  --spacing-xl  → 24px

Sizes:
  --header-height     → 35px
  --sidebar-width     → 300px (FileTree)
  --terminal-height   → 250px (default)
```

### Component Classes (App.css)
```
Layout:
  .app-container     → Root container
  .header            → Top menu bar
  .main-content      → Center content area
  .terminal-container → Bottom terminal

Components:
  .file-tree-container   → File explorer
  .monaco-container      → Editor area
  .agent-sidebar         → AI chat sidebar
  .terminal-wrapper      → Terminal content

Utilities:
  .btn               → Base button style
  .btn-primary       → Primary action button
  .btn-success       → Success button
  .btn-icon          → Icon-only button
  .spinner           → Loading spinner
```

## Monaco Editor Configuration

### Theme: "rusty-dark"
```javascript
{
  base: 'vs-dark',
  colors: {
    'editor.background': '#1e1e1e',
    'editor.foreground': '#cccccc',
    'editor.selectionBackground': '#264f78'
  }
}
```

### Options
```javascript
{
  fontSize: 14,
  fontFamily: 'Fira Code, Consolas, Monaco',
  fontLigatures: true,
  minimap: { enabled: true },
  scrollBeyondLastLine: false,
  renderWhitespace: 'selection',
  tabSize: 2,
  insertSpaces: true,
  wordWrap: 'on',
  smoothScrolling: true,
  cursorBlinking: 'smooth'
}
```

### Language Detection
```javascript
Extension → Language mapping:
  .rs   → rust
  .js   → javascript
  .jsx  → javascript
  .ts   → typescript
  .tsx  → typescript
  .py   → python
  .md   → markdown
  .json → json
  .toml → toml
  .yaml → yaml
```

## xterm.js Configuration

### Theme
```javascript
{
  background: '#1e1e1e',
  foreground: '#cccccc',
  cursor: '#aeafad',
  selectionBackground: '#264f78',
  black: '#000000',
  red: '#cd3131',
  green: '#0dbc79',
  yellow: '#e5e510',
  blue: '#2472c8',
  // ... full 16-color palette
}
```

### Options
```javascript
{
  cursorBlink: true,
  cursorStyle: 'block',
  fontFamily: 'Fira Code, Consolas, Monaco',
  fontSize: 14,
  lineHeight: 1.2,
  scrollback: 1000,
  tabStopWidth: 4
}
```

## Event Flow

### Keyboard Shortcuts
```
Ctrl+S → Save current file
Ctrl+O → Open file dialog
Ctrl+N → Create new file
Ctrl+Q → Exit application
Ctrl+` → Toggle terminal

(In Monaco Editor)
All Monaco built-in shortcuts work
```

### Mouse Events
```
Click file in tree → Open in editor
Click editor tab → Switch to file
Click tab close → Close file
Drag terminal handle → Resize terminal
Hover menu item → Show highlight
Click outside menu → Close menu
```

### Window Events
```
Resize window → Fit terminal & Monaco
Focus editor → Set active file
Blur editor → Auto-save (if changed)
```

## Performance Optimizations

### Monaco Editor
- Auto-layout on container resize
- Lazy loading of language features
- Minimap enabled for large files
- Virtual scrolling for long files

### Terminal
- FitAddon for responsive sizing
- Scrollback limited to 1000 lines
- Efficient event listeners
- Cleanup on unmount

### File Tree
- Load files on demand
- Cache expanded state
- Debounced search (if implemented)

### Agent Sidebar
- Polling with interval clear on response
- Message virtualization (future)
- Debounced input (future)
- Response caching

## Browser Compatibility

### Minimum Requirements
- Chrome 105+
- Safari 13+
- Firefox 100+
- Edge 105+

### Features Used
- ES2020 syntax
- CSS Grid & Flexbox
- CSS Variables
- Fetch API
- Promises & Async/Await
- ResizeObserver
- IntersectionObserver (future)

## Build Configuration

### Vite Config
```javascript
{
  plugins: [react()],
  server: {
    port: 5173,
    strictPort: true
  },
  build: {
    target: 'chrome105',
    minify: 'esbuild',
    sourcemap: true
  }
}
```

### Output Structure
```
dist/
├── index.html
├── assets/
│   ├── index.[hash].js
│   ├── index.[hash].css
│   └── monaco-editor/
│       ├── workers/
│       └── languages/
└── vite.svg
```

## Future Enhancements

### Planned Features
- [ ] Multi-cursor support in Monaco
- [ ] Split editor (side-by-side)
- [ ] Git integration (status, diff, commit)
- [ ] Search across files (Ctrl+Shift+F)
- [ ] Command palette (Ctrl+Shift+P)
- [ ] Settings panel
- [ ] Theme switcher (light/dark)
- [ ] Extension system

### Performance Improvements
- [ ] Virtual scrolling for file tree
- [ ] Worker threads for large files
- [ ] Incremental file parsing
- [ ] Lazy component loading
- [ ] Service worker for offline support

### UX Enhancements
- [ ] Drag & drop files
- [ ] Breadcrumb navigation
- [ ] Recently opened files
- [ ] Keyboard shortcut customization
- [ ] Panel layout persistence
- [ ] Zoom controls

---

**This architecture provides a solid foundation for a modern, performant IDE!** 🚀
