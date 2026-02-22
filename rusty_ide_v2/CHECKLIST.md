# Rusty IDE v2 - Installation & Testing Checklist

## ✅ Pre-Installation Verification

### System Requirements
- [ ] Node.js 18+ installed (`node --version`)
- [ ] npm installed (`npm --version`)
- [ ] Rust 1.70+ installed (`rustc --version`)
- [ ] Cargo installed (`cargo --version`)

### Project Files Verification
- [ ] `package.json` exists
- [ ] `src/` directory exists with components
- [ ] `src/components/` has 5 .jsx files
- [ ] `src/styles/` has 2 .css files
- [ ] `index.html` exists
- [ ] `vite.config.js` exists

---

## 📦 Installation Steps

### Step 1: Install Dependencies
```bash
cd /workspace/jashan/rusty_ide_v2
npm install
```

**Verify:**
- [ ] `node_modules/` directory created
- [ ] No errors in output
- [ ] All packages installed successfully

**Expected packages:**
- [ ] react & react-dom
- [ ] @monaco-editor/react
- [ ] @xterm/xterm & @xterm/addon-fit
- [ ] @tauri-apps/api
- [ ] vite & @vitejs/plugin-react

### Step 2: Verify Installation
```bash
npm list --depth=0
```

**Check for:**
- [ ] All 6 production dependencies present
- [ ] All 6 dev dependencies present
- [ ] No missing peer dependencies warnings

---

## 🧪 Testing Checklist

### Frontend Testing (Browser Mode)

#### Start Development Server
```bash
npm run dev
```

**Verify:**
- [ ] Server starts on http://localhost:5173
- [ ] Browser opens automatically
- [ ] No console errors

#### Test UI Components

**Header:**
- [ ] Header bar visible at top
- [ ] "Rusty IDE v2" text visible
- [ ] File menu clickable
- [ ] Edit menu clickable
- [ ] View menu clickable
- [ ] Terminal menu clickable
- [ ] Dropdown menus appear on click
- [ ] Keyboard shortcuts displayed

**File Tree (Left Sidebar):**
- [ ] "Explorer" header visible
- [ ] "No files found" or file list shows
- [ ] Loading spinner appears initially
- [ ] Files have appropriate icons
- [ ] Click on file (will error without backend - expected)

**Monaco Editor (Center):**
- [ ] Editor area visible
- [ ] "Rusty IDE v2" welcome screen shows
- [ ] "Open a file to get started" message
- [ ] 🦀 icon displayed
- [ ] Dark theme applied correctly

**AI Agent Sidebar (Right):**
- [ ] Sidebar visible on right
- [ ] "AI Agent" header with status indicator
- [ ] Workspace path displayed
- [ ] "Grant Access" button visible
- [ ] Input area at bottom

**Terminal (Bottom):**
- [ ] Terminal area visible
- [ ] "Terminal" header
- [ ] Resize handle at top (hover shows blue)
- [ ] Clear, Kill, Close buttons visible
- [ ] Black terminal background

#### Test Interactions

**View Menu:**
- [ ] Click View → Toggle File Explorer (sidebar hides)
- [ ] Click again (sidebar shows)
- [ ] Click View → Toggle Terminal (terminal hides)
- [ ] Click again (terminal shows)
- [ ] Click View → Toggle AI Sidebar (sidebar hides)
- [ ] Click again (sidebar shows)

**Terminal Resize:**
- [ ] Hover over top border (cursor changes)
- [ ] Drag up (terminal grows)
- [ ] Drag down (terminal shrinks)
- [ ] Release (size stays)

**AI Sidebar:**
- [ ] Click "Grant Access" button
- [ ] Button disappears
- [ ] Success message appears
- [ ] Input box enabled
- [ ] Type message (won't send without backend)

**Styling:**
- [ ] Dark theme throughout
- [ ] Consistent colors (#1e1e1e background)
- [ ] Smooth transitions on hover
- [ ] Custom scrollbars visible
- [ ] No style glitches

---

## 🦀 Tauri Integration Testing

### Prerequisites
- [ ] Tauri backend implemented (see SETUP.md)
- [ ] All Tauri commands registered
- [ ] PTY support configured

### Start Tauri Dev Mode
```bash
npm run tauri:dev
```

**Verify:**
- [ ] Application window opens
- [ ] No console errors
- [ ] UI renders correctly

### Test File Operations

**Open Workspace:**
- [ ] File → Select Workspace (or auto-prompt)
- [ ] Dialog opens
- [ ] Select folder
- [ ] Files appear in FileTree
- [ ] Workspace path shown in AI sidebar

**Open File:**
- [ ] Click file in FileTree
- [ ] File opens in Monaco Editor
- [ ] Tab appears at top
- [ ] Syntax highlighting works
- [ ] Content loads correctly

**Edit File:**
- [ ] Type in editor
- [ ] Changes visible
- [ ] Wait 1 second (auto-save triggers)
- [ ] Check file on disk (should be saved)

**Manual Save:**
- [ ] Edit file
- [ ] Press Ctrl+S
- [ ] File saved immediately
- [ ] No errors in console

**Multiple Files:**
- [ ] Open second file
- [ ] Second tab appears
- [ ] Click first tab (switches)
- [ ] Click second tab (switches)
- [ ] Close tab with ✕ button
- [ ] Tab removed

### Test Terminal

**Basic Terminal:**
- [ ] Terminal visible at bottom
- [ ] Prompt appears ($ or >)
- [ ] Type command (e.g., `ls`)
- [ ] Press Enter
- [ ] Output appears
- [ ] Colors work (ANSI)

**Terminal Actions:**
- [ ] Click Clear button (terminal clears)
- [ ] Type command again (works)
- [ ] Click Kill button (process stops)
- [ ] Click Close button (terminal hides)

**Terminal Resize:**
- [ ] Drag resize handle
- [ ] Terminal resizes smoothly
- [ ] Content reflows
- [ ] No visual glitches

### Test AI Agent

**Grant Access:**
- [ ] Click "Grant Access"
- [ ] System message appears
- [ ] Workspace context sent (check logs)
- [ ] Input box enabled

**Send Message:**
- [ ] Type "Hello" in input
- [ ] Press Enter or click Send
- [ ] Message appears in history
- [ ] Loading indicator shows
- [ ] Agent processes (wait for response)
- [ ] Response appears in chat

**Code Changes:**
- [ ] Agent suggests code change
- [ ] "Apply Changes" button appears
- [ ] Review changes
- [ ] Click "Apply Changes"
- [ ] Files updated
- [ ] Editor reloads with new content

### Test Keyboard Shortcuts

- [ ] Ctrl+S → Saves file
- [ ] Ctrl+O → Opens file dialog
- [ ] Ctrl+N → Creates new file
- [ ] Ctrl+` → Toggles terminal
- [ ] All Monaco shortcuts work (Ctrl+C, Ctrl+V, etc.)

---

## 🎨 Visual Quality Check

### Colors
- [ ] Background: #1e1e1e (dark gray)
- [ ] Secondary: #252526 (slightly lighter)
- [ ] Text: #cccccc (light gray)
- [ ] Accent: #007acc (blue)
- [ ] Consistent across all panels

### Typography
- [ ] Sans-serif font in UI
- [ ] Monospace font in editor (Fira Code if available)
- [ ] Readable font sizes
- [ ] Proper line heights

### Spacing
- [ ] Consistent padding
- [ ] No overlapping elements
- [ ] Proper margins
- [ ] Clean alignment

### Animations
- [ ] Smooth transitions (200ms)
- [ ] No janky animations
- [ ] Hover effects work
- [ ] Loading spinners rotate

### Responsiveness
- [ ] Resize window (UI adapts)
- [ ] Monaco editor resizes
- [ ] Terminal fits properly
- [ ] Panels remain proportional

---

## 🐛 Common Issues & Solutions

### Issue: "Cannot find module 'react'"
**Solution:**
```bash
rm -rf node_modules package-lock.json
npm install
```

### Issue: Monaco Editor blank
**Solution:**
- Check console for errors
- Verify @monaco-editor/react installed
- Try restarting dev server

### Issue: Terminal not working
**Solution:**
- Verify Tauri backend running
- Check PTY implementation
- Look for backend errors

### Issue: AI Sidebar not responding
**Solution:**
- Check backend agent integration
- Verify file-based communication setup
- Look for polling errors in console

### Issue: Styles not applying
**Solution:**
- Clear browser cache
- Check theme.css imported
- Verify CSS variables defined

### Issue: Tauri commands failing
**Solution:**
- Check backend implementation
- Verify command names match
- Look at Tauri console logs

---

## 📊 Performance Check

### Load Times
- [ ] Initial load < 2 seconds
- [ ] File open < 500ms
- [ ] Terminal startup < 1 second
- [ ] No lag when typing

### Memory Usage
- [ ] Browser: < 200MB
- [ ] Tauri: < 150MB
- [ ] Total: < 350MB
- [ ] No memory leaks after 1 hour use

### CPU Usage
- [ ] Idle: < 5%
- [ ] Typing: < 10%
- [ ] Syntax highlighting: < 15%
- [ ] Terminal output: < 20%

---

## ✅ Production Build Test

### Build
```bash
npm run tauri:build
```

**Verify:**
- [ ] Build completes successfully
- [ ] No errors
- [ ] Binary created in `src-tauri/target/release/`

### Run Production Binary
```bash
./src-tauri/target/release/rusty-ide-v2
```

**Check:**
- [ ] Application launches
- [ ] All features work
- [ ] Performance good
- [ ] No console errors

---

## 📝 Final Checklist

### Code Quality
- [ ] No console errors
- [ ] No React warnings
- [ ] No TypeScript errors (if using TS)
- [ ] Clean code formatting

### Documentation
- [ ] README.md comprehensive
- [ ] SETUP.md clear
- [ ] ARCHITECTURE.md accurate
- [ ] All docs up to date

### Features
- [ ] All 5 components working
- [ ] Monaco Editor functional
- [ ] Terminal operational
- [ ] AI Sidebar working
- [ ] File Tree functional

### User Experience
- [ ] Intuitive UI
- [ ] Fast performance
- [ ] Professional look
- [ ] Smooth animations
- [ ] Clear feedback

---

## 🎉 Success Criteria

**The project is ready when:**

✅ All dependencies install without errors
✅ Development server starts and runs
✅ All UI components render correctly
✅ Styling matches VS Code theme
✅ No console errors in browser
✅ File operations work (with backend)
✅ Terminal functions properly (with backend)
✅ AI sidebar communicates (with backend)
✅ Keyboard shortcuts work
✅ Production build succeeds

---

## 📞 Support

If you encounter issues:

1. **Check documentation:**
   - README.md
   - SETUP.md
   - ARCHITECTURE.md

2. **Verify environment:**
   - Node.js version
   - Dependencies installed
   - No conflicting packages

3. **Review logs:**
   - Browser console
   - Terminal output
   - Tauri logs

4. **Test incrementally:**
   - Start with frontend only
   - Add backend integration
   - Test each feature separately

---

**Last Updated:** February 21, 2026
**Status:** Ready for Testing

✅ **All systems go! Start testing!** 🚀
