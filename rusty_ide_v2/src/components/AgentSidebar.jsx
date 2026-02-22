import React, { useState, useRef, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

const AgentSidebar = ({ workspacePath, visible }) => {
  const [messages, setMessages] = useState([]);
  const [inputValue, setInputValue] = useState('');
  const [isWaiting, setIsWaiting] = useState(false);
  const [hasGrantedAccess, setHasGrantedAccess] = useState(false);
  const [currentContext, setCurrentContext] = useState(null);
  const messagesEndRef = useRef(null);
  const pollIntervalRef = useRef(null);

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  useEffect(() => {
    // Clean up polling on unmount
    return () => {
      if (pollIntervalRef.current) {
        clearInterval(pollIntervalRef.current);
      }
    };
  }, []);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  const handleGrantAccess = async () => {
    setHasGrantedAccess(true);
    addMessage('system', 'Access granted! The agent can now read and write files in your workspace.');

    // Send initial context to agent
    try {
      await sendContextToAgent();
    } catch (error) {
      console.error('Failed to send context:', error);
      addMessage('system', 'Warning: Failed to send initial context to agent.');
    }
  };

  const sendContextToAgent = async () => {
    try {
      // Get workspace files list
      const files = await invoke('list_workspace_files', { path: workspacePath });

      // Get IDE source code (optional - for agent to understand the IDE)
      const ideContext = {
        workspace_path: workspacePath,
        files: files,
        ide_version: '2.0.0',
        capabilities: ['read_files', 'write_files', 'execute_commands']
      };

      // Send to agent via file-based communication
      await invoke('write_agent_context', { context: JSON.stringify(ideContext) });
    } catch (error) {
      console.error('Error sending context:', error);
      throw error;
    }
  };

  const addMessage = (role, content) => {
    setMessages(prev => [...prev, {
      id: Date.now() + Math.random(),
      role,
      content,
      timestamp: new Date().toLocaleTimeString()
    }]);
  };

  const handleSendMessage = async () => {
    if (!inputValue.trim() || !hasGrantedAccess) return;

    const userMessage = inputValue.trim();
    setInputValue('');
    addMessage('user', userMessage);
    setIsWaiting(true);

    try {
      // Write query to agent input file
      await invoke('write_agent_query', {
        query: userMessage,
        workspacePath: workspacePath
      });

      // Start polling for response
      startPollingForResponse();
    } catch (error) {
      console.error('Failed to send message:', error);
      addMessage('system', 'Error: Failed to communicate with agent.');
      setIsWaiting(false);
    }
  };

  const startPollingForResponse = () => {
    let pollCount = 0;
    const maxPolls = 120; // 60 seconds max (500ms * 120)

    pollIntervalRef.current = setInterval(async () => {
      pollCount++;

      try {
        // Check for agent response
        const response = await invoke('read_agent_response');

        if (response) {
          // Clear polling
          clearInterval(pollIntervalRef.current);
          setIsWaiting(false);

          // Parse and display response
          handleAgentResponse(response);
        } else if (pollCount >= maxPolls) {
          // Timeout
          clearInterval(pollIntervalRef.current);
          setIsWaiting(false);
          addMessage('system', 'Timeout: No response from agent.');
        }
      } catch (error) {
        console.error('Error polling for response:', error);
      }
    }, 500);
  };

  const handleAgentResponse = (response) => {
    try {
      const parsed = JSON.parse(response);

      // Add agent message
      addMessage('assistant', parsed.message || response);

      // Check for code changes
      if (parsed.code_changes && parsed.code_changes.length > 0) {
        setCurrentContext({ codeChanges: parsed.code_changes });
        addMessage('system', `Agent suggests ${parsed.code_changes.length} code change(s). Click "Apply Changes" to accept.`);
      }

      // Check for file operations
      if (parsed.file_operations) {
        addMessage('system', `Agent performed file operations: ${parsed.file_operations.join(', ')}`);
      }
    } catch (error) {
      // If not JSON, treat as plain text
      addMessage('assistant', response);
    }
  };

  const handleApplyChanges = async () => {
    if (!currentContext?.codeChanges) return;

    try {
      for (const change of currentContext.codeChanges) {
        await invoke('write_file', {
          path: change.file_path,
          content: change.new_content
        });

        addMessage('system', `Applied changes to ${change.file_path}`);

        // Reload file in editor if it's open
        if (window.openFileInEditor) {
          window.openFileInEditor(change.file_path);
        }
      }

      setCurrentContext(null);
    } catch (error) {
      console.error('Failed to apply changes:', error);
      addMessage('system', 'Error: Failed to apply some changes.');
    }
  };

  const formatMessage = (content) => {
    // Simple markdown-like formatting
    const lines = content.split('\n');
    const formatted = [];
    let inCodeBlock = false;
    let codeBlock = [];

    for (let i = 0; i < lines.length; i++) {
      const line = lines[i];

      if (line.startsWith('```')) {
        if (inCodeBlock) {
          // End code block
          formatted.push(
            <pre key={i}>
              <code>{codeBlock.join('\n')}</code>
            </pre>
          );
          codeBlock = [];
          inCodeBlock = false;
        } else {
          // Start code block
          inCodeBlock = true;
        }
      } else if (inCodeBlock) {
        codeBlock.push(line);
      } else {
        // Regular line with inline code support
        const parts = line.split(/(`[^`]+`)/g);
        formatted.push(
          <div key={i}>
            {parts.map((part, j) => {
              if (part.startsWith('`') && part.endsWith('`')) {
                return <code key={j}>{part.slice(1, -1)}</code>;
              }
              return part;
            })}
          </div>
        );
      }
    }

    return formatted;
  };

  if (!visible) return null;

  return (
    <div className="agent-sidebar">
      <div className="agent-header">
        <div className="agent-title">
          <div className={`agent-status ${isWaiting ? 'waiting' : ''}`} />
          AI Agent
        </div>
      </div>

      <div className="agent-workspace">
        Workspace: {workspacePath || 'Not set'}
      </div>

      {!hasGrantedAccess && (
        <div style={{ padding: 'var(--spacing-md)', background: 'var(--bg-tertiary)', margin: 'var(--spacing-md)', borderRadius: 'var(--radius-md)' }}>
          <div style={{ marginBottom: 'var(--spacing-md)', fontSize: '13px' }}>
            The AI agent needs access to read and write files in your workspace to assist you.
          </div>
          <button className="btn btn-success" onClick={handleGrantAccess}>
            Grant Access
          </button>
        </div>
      )}

      <div className="agent-messages">
        {messages.length === 0 && hasGrantedAccess && (
          <div style={{ color: 'var(--text-muted)', fontSize: '13px', textAlign: 'center' }}>
            Ask the AI agent anything about your code!
          </div>
        )}

        {messages.map((message) => (
          <div key={message.id} className="agent-message">
            <div className="message-header">
              {message.role === 'user' ? 'You' : message.role === 'assistant' ? 'AI Agent' : 'System'}
              {' • '}
              {message.timestamp}
            </div>
            <div className={`message-content ${message.role}`}>
              {formatMessage(message.content)}
            </div>
          </div>
        ))}

        {isWaiting && (
          <div className="agent-message">
            <div className="message-header">AI Agent • Thinking...</div>
            <div className="message-content" style={{ display: 'flex', alignItems: 'center', gap: 'var(--spacing-sm)' }}>
              <div className="spinner" />
              <span>Processing your request...</span>
            </div>
          </div>
        )}

        <div ref={messagesEndRef} />
      </div>

      {hasGrantedAccess && (
        <div className="agent-input-area">
          {currentContext?.codeChanges && (
            <button className="btn btn-primary" onClick={handleApplyChanges} style={{ width: '100%', marginBottom: 'var(--spacing-sm)' }}>
              Apply {currentContext.codeChanges.length} Change(s)
            </button>
          )}

          <textarea
            className="agent-input"
            placeholder="Ask the AI agent anything..."
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === 'Enter' && !e.shiftKey) {
                e.preventDefault();
                handleSendMessage();
              }
            }}
            disabled={!hasGrantedAccess || isWaiting}
          />

          <div className="agent-buttons">
            <button
              className="btn btn-primary"
              onClick={handleSendMessage}
              disabled={!hasGrantedAccess || isWaiting || !inputValue.trim()}
              style={{ flex: 1 }}
            >
              {isWaiting ? 'Waiting...' : 'Send'}
            </button>
          </div>
        </div>
      )}
    </div>
  );
};

export default AgentSidebar;
