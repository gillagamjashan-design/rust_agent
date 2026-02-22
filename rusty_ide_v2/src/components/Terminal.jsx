import React, { useEffect, useRef, useState } from 'react';
import { Terminal as XTerm } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import '@xterm/xterm/css/xterm.css';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';

const Terminal = ({ visible, onClose }) => {
  const terminalRef = useRef(null);
  const xtermRef = useRef(null);
  const fitAddonRef = useRef(null);
  const [height, setHeight] = useState(250);
  const [isDragging, setIsDragging] = useState(false);
  const dragStartRef = useRef(null);
  const ptyIdRef = useRef(null);

  useEffect(() => {
    if (!terminalRef.current || xtermRef.current) return;

    // Initialize xterm.js
    const xterm = new XTerm({
      cursorBlink: true,
      cursorStyle: 'block',
      fontFamily: 'Fira Code, Consolas, Monaco, Courier New, monospace',
      fontSize: 14,
      lineHeight: 1.2,
      theme: {
        background: '#1e1e1e',
        foreground: '#cccccc',
        cursor: '#aeafad',
        cursorAccent: '#1e1e1e',
        selectionBackground: '#264f78',
        black: '#000000',
        red: '#cd3131',
        green: '#0dbc79',
        yellow: '#e5e510',
        blue: '#2472c8',
        magenta: '#bc3fbc',
        cyan: '#11a8cd',
        white: '#e5e5e5',
        brightBlack: '#666666',
        brightRed: '#f14c4c',
        brightGreen: '#23d18b',
        brightYellow: '#f5f543',
        brightBlue: '#3b8eea',
        brightMagenta: '#d670d6',
        brightCyan: '#29b8db',
        brightWhite: '#e5e5e5'
      },
      scrollback: 1000,
      tabStopWidth: 4,
      allowProposedApi: true
    });

    const fitAddon = new FitAddon();
    xterm.loadAddon(fitAddon);

    xterm.open(terminalRef.current);
    fitAddon.fit();

    xtermRef.current = xterm;
    fitAddonRef.current = fitAddon;

    // Initialize PTY
    initializePty(xterm);

    // Handle terminal input
    xterm.onData((data) => {
      if (ptyIdRef.current) {
        invoke('write_to_pty', { ptyId: ptyIdRef.current, data })
          .catch(err => console.error('Failed to write to PTY:', err));
      }
    });

    // Handle resize
    const resizeObserver = new ResizeObserver(() => {
      fitAddon.fit();
      if (ptyIdRef.current) {
        const { cols, rows } = xterm;
        invoke('resize_pty', { ptyId: ptyIdRef.current, cols, rows })
          .catch(err => console.error('Failed to resize PTY:', err));
      }
    });

    resizeObserver.observe(terminalRef.current);

    return () => {
      resizeObserver.disconnect();
      if (ptyIdRef.current) {
        invoke('close_pty', { ptyId: ptyIdRef.current })
          .catch(err => console.error('Failed to close PTY:', err));
      }
      xterm.dispose();
    };
  }, []);

  const initializePty = async (xterm) => {
    try {
      // Create PTY process
      const ptyId = await invoke('create_pty', {
        cols: xterm.cols,
        rows: xterm.rows
      });

      ptyIdRef.current = ptyId;

      // Listen for PTY output
      await listen(`pty-output-${ptyId}`, (event) => {
        xterm.write(event.payload);
      });

      // Listen for PTY exit
      await listen(`pty-exit-${ptyId}`, (event) => {
        xterm.write(`\r\n\x1b[1;31mProcess exited with code ${event.payload}\x1b[0m\r\n`);
      });

      xterm.write('\x1b[1;32mTerminal ready\x1b[0m\r\n');
    } catch (error) {
      console.error('Failed to initialize PTY:', error);
      xterm.write('\x1b[1;31mFailed to initialize terminal\x1b[0m\r\n');
    }
  };

  useEffect(() => {
    if (fitAddonRef.current && visible) {
      setTimeout(() => {
        fitAddonRef.current.fit();
      }, 0);
    }
  }, [visible, height]);

  const handleMouseDown = (e) => {
    setIsDragging(true);
    dragStartRef.current = {
      y: e.clientY,
      height: height
    };
  };

  useEffect(() => {
    if (!isDragging) return;

    const handleMouseMove = (e) => {
      if (!dragStartRef.current) return;

      const deltaY = dragStartRef.current.y - e.clientY;
      const newHeight = Math.max(100, Math.min(600, dragStartRef.current.height + deltaY));
      setHeight(newHeight);
    };

    const handleMouseUp = () => {
      setIsDragging(false);
      dragStartRef.current = null;
    };

    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', handleMouseUp);

    return () => {
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
  }, [isDragging]);

  const handleClear = () => {
    if (xtermRef.current) {
      xtermRef.current.clear();
    }
  };

  const handleKillTerminal = () => {
    if (ptyIdRef.current) {
      invoke('kill_pty', { ptyId: ptyIdRef.current })
        .catch(err => console.error('Failed to kill PTY:', err));
    }
  };

  if (!visible) return null;

  return (
    <div className="terminal-container" style={{ height: `${height}px` }}>
      <div
        className={`terminal-resize-handle ${isDragging ? 'dragging' : ''}`}
        onMouseDown={handleMouseDown}
      />
      <div className="terminal-header">
        <div className="terminal-title">Terminal</div>
        <div className="terminal-actions">
          <button className="btn btn-icon" onClick={handleClear} title="Clear">
            🗑️
          </button>
          <button className="btn btn-icon" onClick={handleKillTerminal} title="Kill Process">
            ⏹️
          </button>
          <button className="btn btn-icon" onClick={onClose} title="Close">
            ✕
          </button>
        </div>
      </div>
      <div className="terminal-wrapper" ref={terminalRef} />
    </div>
  );
};

export default Terminal;
