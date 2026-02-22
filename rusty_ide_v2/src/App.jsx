import React, { useState, useEffect } from 'react';
import Header from './components/Header';
import FileTree from './components/FileTree';
import MonacoEditor from './components/MonacoEditor';
import AgentSidebar from './components/AgentSidebar';
import Terminal from './components/Terminal';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import './styles/theme.css';
import './styles/App.css';

function App() {
  const [workspacePath, setWorkspacePath] = useState(null);
  const [showFileTree, setShowFileTree] = useState(true);
  const [showTerminal, setShowTerminal] = useState(true);
  const [showAgent, setShowAgent] = useState(true);
  const [currentFile, setCurrentFile] = useState(null);

  useEffect(() => {
    // Load initial workspace from config or prompt user
    loadWorkspace();
  }, []);

  const loadWorkspace = async () => {
    try {
      // Try to load last workspace from config
      const lastWorkspace = await invoke('get_last_workspace');
      if (lastWorkspace) {
        setWorkspacePath(lastWorkspace);
      } else {
        // Prompt user to select workspace
        promptSelectWorkspace();
      }
    } catch (error) {
      console.error('Failed to load workspace:', error);
      promptSelectWorkspace();
    }
  };

  const promptSelectWorkspace = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Select Workspace Folder'
      });

      if (selected && typeof selected === 'string') {
        setWorkspacePath(selected);
        await invoke('set_workspace', { path: selected });
      }
    } catch (error) {
      console.error('Failed to select workspace:', error);
    }
  };

  const handleFileSelect = async (filePath) => {
    setCurrentFile(filePath);
    if (window.openFileInEditor) {
      window.openFileInEditor(filePath);
    }
  };

  const handleNewFile = async () => {
    try {
      const filePath = await invoke('create_new_file', { workspacePath });
      if (filePath) {
        handleFileSelect(filePath);
      }
    } catch (error) {
      console.error('Failed to create new file:', error);
    }
  };

  const handleOpenFile = async () => {
    try {
      const selected = await open({
        multiple: false,
        directory: false,
        title: 'Open File'
      });

      if (selected && typeof selected === 'string') {
        handleFileSelect(selected);
      }
    } catch (error) {
      console.error('Failed to open file:', error);
    }
  };

  const handleSaveFile = () => {
    // Save is handled automatically in MonacoEditor
    console.log('Save triggered');
  };

  const handleToggleFileTree = () => {
    setShowFileTree(!showFileTree);
  };

  const handleToggleTerminal = () => {
    setShowTerminal(!showTerminal);
  };

  const handleToggleAgent = () => {
    setShowAgent(!showAgent);
  };

  return (
    <div className="app-container">
      <Header
        onToggleFileTree={handleToggleFileTree}
        onToggleTerminal={handleToggleTerminal}
        onToggleAgent={handleToggleAgent}
        onNewFile={handleNewFile}
        onOpenFile={handleOpenFile}
        onSaveFile={handleSaveFile}
      />

      <div className="main-content">
        {showFileTree && (
          <FileTree
            workspacePath={workspacePath}
            onFileSelect={handleFileSelect}
          />
        )}

        <div className="editor-area">
          <MonacoEditor
            onSave={(filePath) => {
              console.log('File saved:', filePath);
            }}
          />

          {showAgent && (
            <AgentSidebar
              workspacePath={workspacePath}
              visible={showAgent}
            />
          )}
        </div>
      </div>

      <Terminal
        visible={showTerminal}
        onClose={() => setShowTerminal(false)}
      />
    </div>
  );
}

export default App;
