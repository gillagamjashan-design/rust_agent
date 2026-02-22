import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

const FileTree = ({ workspacePath, onFileSelect }) => {
  const [files, setFiles] = useState([]);
  const [expandedDirs, setExpandedDirs] = useState(new Set());
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (workspacePath) {
      loadFiles();
    }
  }, [workspacePath]);

  const loadFiles = async () => {
    try {
      setLoading(true);
      // Call Tauri backend to list files
      const fileList = await invoke('list_workspace_files', { path: workspacePath });
      setFiles(buildFileTree(fileList));
    } catch (error) {
      console.error('Failed to load files:', error);
    } finally {
      setLoading(false);
    }
  };

  const buildFileTree = (fileList) => {
    // Simple tree structure - can be enhanced
    return fileList.map(file => ({
      name: file.name,
      path: file.path,
      isDirectory: file.is_directory
    }));
  };

  const toggleDirectory = (path) => {
    const newExpanded = new Set(expandedDirs);
    if (newExpanded.has(path)) {
      newExpanded.delete(path);
    } else {
      newExpanded.add(path);
    }
    setExpandedDirs(newExpanded);
  };

  const renderFileIcon = (file) => {
    if (file.isDirectory) {
      return expandedDirs.has(file.path) ? '📂' : '📁';
    }

    const ext = file.name.split('.').pop();
    const iconMap = {
      'rs': '🦀',
      'js': '📜',
      'jsx': '⚛️',
      'ts': '📘',
      'tsx': '⚛️',
      'py': '🐍',
      'md': '📝',
      'json': '📋',
      'toml': '⚙️',
      'yaml': '📄',
      'yml': '📄',
      'txt': '📄',
      'lock': '🔒'
    };

    return iconMap[ext] || '📄';
  };

  if (loading) {
    return (
      <div className="file-tree-container">
        <div className="file-tree-header">Explorer</div>
        <div style={{ padding: 'var(--spacing-md)', textAlign: 'center' }}>
          <div className="spinner" style={{ margin: '0 auto' }}></div>
        </div>
      </div>
    );
  }

  return (
    <div className="file-tree-container">
      <div className="file-tree-header">Explorer</div>
      <div style={{ flex: 1, overflow: 'auto' }}>
        {files.length === 0 ? (
          <div style={{ padding: 'var(--spacing-md)', color: 'var(--text-muted)', fontSize: '12px' }}>
            No files found
          </div>
        ) : (
          <div>
            {files.map((file, index) => (
              <div key={index}>
                <div
                  onClick={() => {
                    if (file.isDirectory) {
                      toggleDirectory(file.path);
                    } else {
                      onFileSelect(file.path);
                    }
                  }}
                  style={{
                    padding: 'var(--spacing-xs) var(--spacing-md)',
                    cursor: 'pointer',
                    display: 'flex',
                    alignItems: 'center',
                    gap: 'var(--spacing-xs)',
                    fontSize: '13px',
                    userSelect: 'none',
                    transition: 'background var(--transition-fast)'
                  }}
                  onMouseEnter={(e) => {
                    e.currentTarget.style.background = 'var(--bg-tertiary)';
                  }}
                  onMouseLeave={(e) => {
                    e.currentTarget.style.background = 'transparent';
                  }}
                >
                  <span style={{ fontSize: '16px' }}>{renderFileIcon(file)}</span>
                  <span>{file.name}</span>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

export default FileTree;
