# Rusty IDE v2 - Integration Guide

## Complete Integration in 10 Minutes

This guide shows how to integrate the Tauri backend with your React frontend.

## Step 1: Import Bindings (1 minute)

```tsx
// At the top of your component
import {
  Terminal,
  ManagedTerminal,
  FileSystem,
  FileWatcher,
  Agent,
  Permissions,
  IDE,
} from './bindings';
```

## Step 2: Terminal Integration (3 minutes)

### React Component with xterm.js

```tsx
import React, { useEffect, useRef, useState } from 'react';
import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import { ManagedTerminal } from './bindings';
import '@xterm/xterm/css/xterm.css';

export const IntegratedTerminal: React.FC = () => {
  const xtermRef = useRef<Terminal>();
  const containerRef = useRef<HTMLDivElement>(null);
  const [backend, setBackend] = useState<ManagedTerminal>();

  useEffect(() => {
    // Create xterm.js terminal
    const xterm = new Terminal({
      theme: {
        background: '#1e1e1e',
        foreground: '#cccccc',
        cursor: '#ffffff',
        selection: '#264f78',
      },
      fontSize: 14,
      fontFamily: 'Menlo, Monaco, "Courier New", monospace',
      cursorBlink: true,
    });

    const fitAddon = new FitAddon();
    xterm.loadAddon(fitAddon);

    if (containerRef.current) {
      xterm.open(containerRef.current);
      fitAddon.fit();
    }

    xtermRef.current = xterm;

    // Create backend PTY terminal
    ManagedTerminal.create((data) => {
      xterm.write(data);
    }).then((term) => {
      setBackend(term);

      // Send user input to backend
      xterm.onData((data) => {
        term.write(data);
      });

      // Handle resize
      xterm.onResize(({ cols, rows }) => {
        term.resize(cols, rows);
      });
    });

    // Cleanup
    return () => {
      backend?.close();
      xterm.dispose();
    };
  }, []);

  return (
    <div
      ref={containerRef}
      style={{
        height: '400px',
        width: '100%',
        padding: '10px',
        backgroundColor: '#1e1e1e',
      }}
    />
  );
};
```

## Step 3: File Editor Integration (3 minutes)

### Monaco Editor with File Operations

```tsx
import React, { useEffect, useState } from 'react';
import Editor from '@monaco-editor/react';
import { FileSystem, Permissions } from './bindings';

export const CodeEditor: React.FC<{ filePath: string }> = ({ filePath }) => {
  const [content, setContent] = useState<string>('');
  const [hasAccess, setHasAccess] = useState(false);

  useEffect(() => {
    // Check and grant permissions
    const setupAccess = async () => {
      const allowed = await Permissions.check(filePath);
      if (!allowed) {
        const workspace = filePath.split('/').slice(0, -1).join('/');
        await Permissions.grant(workspace);
      }
      setHasAccess(true);
    };

    setupAccess();
  }, [filePath]);

  useEffect(() => {
    if (!hasAccess) return;

    // Load file content
    FileSystem.readFile(filePath)
      .then(setContent)
      .catch((err) => console.error('Failed to read file:', err));
  }, [filePath, hasAccess]);

  const handleSave = async (value: string | undefined) => {
    if (!value) return;

    try {
      await FileSystem.writeFile(filePath, value);
      console.log('File saved successfully');
    } catch (err) {
      console.error('Failed to save file:', err);
    }
  };

  // Auto-save with debounce
  const [saveTimeout, setSaveTimeout] = useState<NodeJS.Timeout>();

  const handleChange = (value: string | undefined) => {
    setContent(value || '');

    if (saveTimeout) clearTimeout(saveTimeout);

    const timeout = setTimeout(() => {
      handleSave(value);
    }, 1000); // 1 second debounce

    setSaveTimeout(timeout);
  };

  return (
    <Editor
      height="600px"
      defaultLanguage="rust"
      theme="vs-dark"
      value={content}
      onChange={handleChange}
      options={{
        minimap: { enabled: false },
        fontSize: 14,
        lineNumbers: 'on',
        automaticLayout: true,
      }}
    />
  );
};
```

## Step 4: File Tree Integration (2 minutes)

### File Explorer with Live Updates

```tsx
import React, { useEffect, useState } from 'react';
import { FileSystem, FileWatcher, FileInfo } from './bindings';

export const FileExplorer: React.FC<{
  workspace: string;
  onFileSelect: (path: string) => void;
}> = ({ workspace, onFileSelect }) => {
  const [files, setFiles] = useState<FileInfo[]>([]);
  const [watcher, setWatcher] = useState<FileWatcher>();

  const loadFiles = async () => {
    try {
      const fileList = await FileSystem.listFiles(workspace);
      setFiles(fileList.sort((a, b) => {
        // Directories first, then alphabetically
        if (a.is_dir && !b.is_dir) return -1;
        if (!a.is_dir && b.is_dir) return 1;
        return a.name.localeCompare(b.name);
      }));
    } catch (err) {
      console.error('Failed to list files:', err);
    }
  };

  useEffect(() => {
    // Load initial files
    loadFiles();

    // Start watching for changes
    FileWatcher.watch(workspace, () => {
      loadFiles();
    }).then(setWatcher);

    return () => {
      watcher?.unwatch();
    };
  }, [workspace]);

  return (
    <div style={{ padding: '10px', maxHeight: '600px', overflow: 'auto' }}>
      {files.map((file) => (
        <div
          key={file.path}
          onClick={() => !file.is_dir && onFileSelect(file.path)}
          style={{
            padding: '5px 10px',
            cursor: file.is_dir ? 'default' : 'pointer',
            backgroundColor: 'transparent',
            ':hover': { backgroundColor: '#2d2d30' },
          }}
        >
          {file.is_dir ? '📁' : '📄'} {file.name}
          {!file.is_dir && (
            <span style={{ color: '#858585', fontSize: '12px', marginLeft: '10px' }}>
              {formatBytes(file.size)}
            </span>
          )}
        </div>
      ))}
    </div>
  );
};

const formatBytes = (bytes: number): string => {
  if (bytes < 1024) return bytes + ' B';
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
  return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
};
```

## Step 5: AI Agent Integration (1 minute)

### Agent Query with Code Suggestions

```tsx
import React, { useState } from 'react';
import { Agent, FileSystem, AgentResponse } from './bindings';

export const AIAssistant: React.FC<{
  workspace: string;
  currentFile: string;
  currentCode: string;
}> = ({ workspace, currentFile, currentCode }) => {
  const [query, setQuery] = useState('');
  const [response, setResponse] = useState<AgentResponse>();
  const [loading, setLoading] = useState(false);

  const handleQuery = async () => {
    setLoading(true);
    try {
      const result = await Agent.query({
        workspace,
        currentFile,
        currentCode,
        files: [currentFile],
        query,
      });
      setResponse(result);
    } catch (err) {
      console.error('Agent query failed:', err);
    } finally {
      setLoading(false);
    }
  };

  const applySuggestion = async (file: string, code: string) => {
    try {
      await FileSystem.writeFile(file, code);
      console.log('Code applied successfully');
    } catch (err) {
      console.error('Failed to apply code:', err);
    }
  };

  return (
    <div style={{ padding: '20px' }}>
      <h3>AI Assistant</h3>

      <textarea
        value={query}
        onChange={(e) => setQuery(e.target.value)}
        placeholder="Ask the AI to improve your code..."
        style={{ width: '100%', height: '100px', marginBottom: '10px' }}
      />

      <button onClick={handleQuery} disabled={loading}>
        {loading ? 'Thinking...' : 'Ask AI'}
      </button>

      {response && (
        <div style={{ marginTop: '20px' }}>
          <p>{response.response_text}</p>

          {response.code_suggestions.map((suggestion, idx) => (
            <div key={idx} style={{ marginTop: '10px', border: '1px solid #444', padding: '10px' }}>
              <p>
                <strong>{suggestion.file}</strong> - {suggestion.description}
              </p>
              <pre style={{ backgroundColor: '#2d2d30', padding: '10px', overflow: 'auto' }}>
                {suggestion.code}
              </pre>
              <button onClick={() => applySuggestion(suggestion.file, suggestion.code)}>
                Apply Changes
              </button>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
```

## Complete App Integration

### Main App Component

```tsx
import React, { useState } from 'react';
import { IntegratedTerminal } from './components/IntegratedTerminal';
import { CodeEditor } from './components/CodeEditor';
import { FileExplorer } from './components/FileExplorer';
import { AIAssistant } from './components/AIAssistant';

export const App: React.FC = () => {
  const [workspace] = useState('/workspace');
  const [currentFile, setCurrentFile] = useState<string>('');
  const [currentCode, setCurrentCode] = useState<string>('');

  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: '100vh' }}>
      {/* Header */}
      <div style={{ height: '40px', backgroundColor: '#2d2d30', padding: '10px' }}>
        <h2 style={{ margin: 0 }}>Rusty IDE v2</h2>
      </div>

      {/* Main Content */}
      <div style={{ display: 'flex', flex: 1 }}>
        {/* File Explorer */}
        <div style={{ width: '250px', backgroundColor: '#252526', borderRight: '1px solid #444' }}>
          <FileExplorer workspace={workspace} onFileSelect={setCurrentFile} />
        </div>

        {/* Editor */}
        <div style={{ flex: 1 }}>
          {currentFile ? (
            <CodeEditor filePath={currentFile} />
          ) : (
            <div style={{ padding: '20px' }}>
              <p>Select a file to start editing</p>
            </div>
          )}
        </div>

        {/* AI Sidebar */}
        <div style={{ width: '350px', backgroundColor: '#252526', borderLeft: '1px solid #444' }}>
          <AIAssistant
            workspace={workspace}
            currentFile={currentFile}
            currentCode={currentCode}
          />
        </div>
      </div>

      {/* Terminal */}
      <div style={{ height: '300px', borderTop: '1px solid #444' }}>
        <IntegratedTerminal />
      </div>
    </div>
  );
};
```

## Testing the Integration

### 1. Start the Development Server

```bash
npm run tauri:dev
```

### 2. Test Each Feature

**Terminal:**
- Type commands in the terminal
- Verify output appears
- Test resize by dragging window

**File Operations:**
- Click files in the explorer
- Edit content in Monaco
- Save and verify changes persist

**AI Agent:**
- Type a query
- Check for agent response
- Apply code suggestions

**Permissions:**
- Try accessing files outside workspace
- Grant permission when prompted
- Verify access after grant

## Common Integration Issues

### Terminal Not Responding
```tsx
// Make sure to set up event listener BEFORE creating terminal
const unlisten = await Terminal.onData(handler);
const termId = await Terminal.create();
```

### File Save Not Working
```tsx
// Always grant workspace permission first
await Permissions.grant('/workspace');
```

### Agent Timeout
```tsx
// Increase timeout or poll manually
const response = await Agent.query({...});
// OR
const interval = setInterval(async () => {
  const resp = await Agent.checkResponse();
  if (resp) {
    clearInterval(interval);
    handleResponse(resp);
  }
}, 500);
```

## Next Steps

1. **Customize Themes**: Modify terminal and editor themes
2. **Add Keybindings**: Implement keyboard shortcuts
3. **Error Handling**: Add user-friendly error messages
4. **State Management**: Use Redux/Context for complex state
5. **Testing**: Add unit and integration tests

## Resources

- **Backend API**: `/workspace/jashan/rusty_ide_v2/src-tauri/README.md`
- **Examples**: `/workspace/jashan/rusty_ide_v2/src/example-usage.tsx`
- **Architecture**: `/workspace/jashan/rusty_ide_v2/TECHNICAL.md`
- **TypeScript Types**: `/workspace/jashan/rusty_ide_v2/src/bindings.ts`
