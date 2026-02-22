import React, { useState, useRef, useEffect } from 'react';
import Editor from '@monaco-editor/react';
import { invoke } from '@tauri-apps/api/tauri';

const MonacoEditor = ({ onSave }) => {
  const [openFiles, setOpenFiles] = useState([]);
  const [activeFile, setActiveFile] = useState(null);
  const [fileContents, setFileContents] = useState({});
  const editorRef = useRef(null);
  const saveTimeoutRef = useRef(null);

  useEffect(() => {
    // Keyboard shortcuts
    const handleKeyDown = (e) => {
      // Ctrl+S or Cmd+S for save
      if ((e.ctrlKey || e.metaKey) && e.key === 's') {
        e.preventDefault();
        handleSave();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [activeFile, fileContents]);

  const openFile = async (filePath) => {
    try {
      // Check if file is already open
      if (openFiles.some(f => f.path === filePath)) {
        setActiveFile(filePath);
        return;
      }

      // Read file content from backend
      const content = await invoke('read_file', { path: filePath });

      const fileName = filePath.split('/').pop();
      setOpenFiles([...openFiles, { path: filePath, name: fileName }]);
      setFileContents({ ...fileContents, [filePath]: content });
      setActiveFile(filePath);
    } catch (error) {
      console.error('Failed to open file:', error);
    }
  };

  const closeFile = (filePath, e) => {
    e?.stopPropagation();

    const newOpenFiles = openFiles.filter(f => f.path !== filePath);
    setOpenFiles(newOpenFiles);

    const newContents = { ...fileContents };
    delete newContents[filePath];
    setFileContents(newContents);

    if (activeFile === filePath) {
      setActiveFile(newOpenFiles.length > 0 ? newOpenFiles[0].path : null);
    }
  };

  const handleEditorChange = (value) => {
    if (!activeFile) return;

    setFileContents({ ...fileContents, [activeFile]: value });

    // Auto-save with debounce
    if (saveTimeoutRef.current) {
      clearTimeout(saveTimeoutRef.current);
    }

    saveTimeoutRef.current = setTimeout(() => {
      handleSave(activeFile, value);
    }, 1000); // Auto-save after 1 second of inactivity
  };

  const handleSave = async (filePath = activeFile, content = fileContents[activeFile]) => {
    if (!filePath || !content) return;

    try {
      await invoke('write_file', { path: filePath, content });
      if (onSave) {
        onSave(filePath);
      }
      console.log('File saved:', filePath);
    } catch (error) {
      console.error('Failed to save file:', error);
    }
  };

  const handleEditorMount = (editor, monaco) => {
    editorRef.current = editor;

    // Configure Monaco editor
    monaco.editor.defineTheme('rusty-dark', {
      base: 'vs-dark',
      inherit: true,
      rules: [],
      colors: {
        'editor.background': '#1e1e1e',
        'editor.foreground': '#cccccc',
        'editorLineNumber.foreground': '#858585',
        'editorLineNumber.activeForeground': '#c6c6c6',
        'editor.selectionBackground': '#264f78',
        'editor.inactiveSelectionBackground': '#3a3d41'
      }
    });

    monaco.editor.setTheme('rusty-dark');

    // Additional editor configuration
    editor.updateOptions({
      fontSize: 14,
      fontFamily: 'Fira Code, Consolas, Monaco, Courier New, monospace',
      fontLigatures: true,
      minimap: { enabled: true },
      scrollBeyondLastLine: false,
      renderWhitespace: 'selection',
      tabSize: 2,
      insertSpaces: true,
      wordWrap: 'on',
      smoothScrolling: true,
      cursorBlinking: 'smooth',
      cursorSmoothCaretAnimation: 'on'
    });
  };

  const getLanguage = (filePath) => {
    if (!filePath) return 'plaintext';

    const ext = filePath.split('.').pop();
    const langMap = {
      'rs': 'rust',
      'js': 'javascript',
      'jsx': 'javascript',
      'ts': 'typescript',
      'tsx': 'typescript',
      'py': 'python',
      'md': 'markdown',
      'json': 'json',
      'toml': 'toml',
      'yaml': 'yaml',
      'yml': 'yaml',
      'html': 'html',
      'css': 'css',
      'sh': 'shell',
      'bash': 'shell'
    };

    return langMap[ext] || 'plaintext';
  };

  // Expose openFile method for parent component
  useEffect(() => {
    window.openFileInEditor = openFile;
    return () => {
      delete window.openFileInEditor;
    };
  }, [openFiles, fileContents]);

  return (
    <div className="monaco-container">
      <div className="editor-tabs">
        {openFiles.length === 0 ? (
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', padding: 'var(--spacing-sm)' }}>
            No files open
          </div>
        ) : (
          openFiles.map((file) => (
            <div
              key={file.path}
              className={`editor-tab ${activeFile === file.path ? 'active' : ''}`}
              onClick={() => setActiveFile(file.path)}
            >
              <span>{file.name}</span>
              <div
                className="editor-tab-close"
                onClick={(e) => closeFile(file.path, e)}
              >
                ✕
              </div>
            </div>
          ))
        )}
      </div>
      <div className="editor-wrapper">
        {activeFile ? (
          <Editor
            height="100%"
            language={getLanguage(activeFile)}
            value={fileContents[activeFile] || ''}
            onChange={handleEditorChange}
            onMount={handleEditorMount}
            options={{
              automaticLayout: true
            }}
          />
        ) : (
          <div
            style={{
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              height: '100%',
              flexDirection: 'column',
              gap: 'var(--spacing-md)',
              color: 'var(--text-muted)'
            }}
          >
            <div style={{ fontSize: '48px' }}>🦀</div>
            <div style={{ fontSize: '16px' }}>Rusty IDE v2</div>
            <div style={{ fontSize: '13px' }}>Open a file to get started</div>
          </div>
        )}
      </div>
    </div>
  );
};

export default MonacoEditor;
