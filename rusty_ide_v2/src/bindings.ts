/**
 * TypeScript bindings for Rusty IDE Tauri backend
 *
 * This file provides type-safe wrappers around the Tauri invoke API.
 */

import { invoke } from '@tauri-apps/api/tauri';
import { listen, UnlistenFn } from '@tauri-apps/api/event';

// ============================================================================
// Data Types
// ============================================================================

export interface FileInfo {
  name: string;
  path: string;
  is_dir: boolean;
  size: number;
  modified?: string;
}

export interface AgentResponse {
  timestamp: string;
  response_text: string;
  code_suggestions: CodeSuggestion[];
  apply_changes: boolean;
}

export interface CodeSuggestion {
  file: string;
  code: string;
  language: string;
  description: string;
}

export interface AgentInfo {
  agent_dir: string;
  request_path: string;
  response_path: string;
}

export interface TerminalDataEvent {
  id: string;
  data: string;
}

export interface FileSystemEvent {
  kind: string;
  paths: string[];
}

// ============================================================================
// Terminal API
// ============================================================================

export class Terminal {
  static async create(): Promise<string> {
    return invoke<string>('create_terminal');
  }

  static async write(id: string, data: string): Promise<void> {
    return invoke('write_to_terminal', { id, data });
  }

  static async resize(id: string, cols: number, rows: number): Promise<void> {
    return invoke('resize_terminal', { id, cols, rows });
  }

  static async close(id: string): Promise<void> {
    return invoke('close_terminal', { id });
  }

  static async onData(callback: (event: TerminalDataEvent) => void): Promise<UnlistenFn> {
    return listen<TerminalDataEvent>('terminal-data', (event) => {
      callback(event.payload);
    });
  }
}

// ============================================================================
// File System API
// ============================================================================

export class FileSystem {
  static async readFile(path: string): Promise<string> {
    return invoke<string>('read_file', { path });
  }

  static async writeFile(path: string, content: string): Promise<void> {
    return invoke('write_file', { path, content });
  }

  static async listFiles(directory: string): Promise<FileInfo[]> {
    return invoke<FileInfo[]>('list_files', { directory });
  }

  static async watchDirectory(path: string): Promise<void> {
    return invoke('watch_directory', { path });
  }

  static async unwatchDirectory(path: string): Promise<void> {
    return invoke('unwatch_directory', { path });
  }

  static async onFileSystemEvent(callback: (event: FileSystemEvent) => void): Promise<UnlistenFn> {
    return listen<FileSystemEvent>('file-system-event', (event) => {
      callback(event.payload);
    });
  }
}

// ============================================================================
// Agent API
// ============================================================================

export interface AgentQueryParams {
  workspace: string;
  currentFile?: string;
  currentCode?: string;
  files: string[];
  query: string;
}

export class Agent {
  static async query(params: AgentQueryParams): Promise<AgentResponse> {
    return invoke<AgentResponse>('query_agent', {
      workspace: params.workspace,
      currentFile: params.currentFile || null,
      currentCode: params.currentCode || null,
      files: params.files,
      query: params.query,
    });
  }

  static async checkResponse(): Promise<AgentResponse | null> {
    return invoke<AgentResponse | null>('check_agent_response');
  }

  static async clearFiles(): Promise<void> {
    return invoke('clear_agent_files');
  }

  static async getInfo(): Promise<AgentInfo> {
    return invoke<AgentInfo>('get_agent_info');
  }
}

// ============================================================================
// Permissions API
// ============================================================================

export class Permissions {
  static async grant(path: string): Promise<void> {
    return invoke('grant_workspace_access', { path });
  }

  static async check(path: string): Promise<boolean> {
    return invoke<boolean>('check_permission', { path });
  }

  static async getAll(): Promise<string[]> {
    return invoke<string[]>('get_permissions');
  }

  static async revoke(path: string): Promise<void> {
    return invoke('revoke_workspace_access', { path });
  }
}

// ============================================================================
// IDE API
// ============================================================================

export class IDE {
  static async getSourceCode(): Promise<string> {
    return invoke<string>('get_ide_source_code');
  }
}

// ============================================================================
// Convenience Classes
// ============================================================================

/**
 * Managed Terminal instance with event handling
 */
export class ManagedTerminal {
  private id: string;
  private unlistenFn?: UnlistenFn;

  private constructor(id: string) {
    this.id = id;
  }

  static async create(onData: (data: string) => void): Promise<ManagedTerminal> {
    const id = await Terminal.create();
    const terminal = new ManagedTerminal(id);

    terminal.unlistenFn = await Terminal.onData((event) => {
      if (event.id === id) {
        onData(event.data);
      }
    });

    return terminal;
  }

  async write(data: string): Promise<void> {
    return Terminal.write(this.id, data);
  }

  async resize(cols: number, rows: number): Promise<void> {
    return Terminal.resize(this.id, cols, rows);
  }

  async close(): Promise<void> {
    if (this.unlistenFn) {
      this.unlistenFn();
    }
    return Terminal.close(this.id);
  }

  getId(): string {
    return this.id;
  }
}

/**
 * File watcher with managed lifecycle
 */
export class FileWatcher {
  private path: string;
  private unlistenFn?: UnlistenFn;

  private constructor(path: string) {
    this.path = path;
  }

  static async watch(
    path: string,
    onChange: (event: FileSystemEvent) => void
  ): Promise<FileWatcher> {
    await FileSystem.watchDirectory(path);
    const watcher = new FileWatcher(path);

    watcher.unlistenFn = await FileSystem.onFileSystemEvent((event) => {
      // Filter events for this specific path
      if (event.paths.some((p) => p.startsWith(path))) {
        onChange(event);
      }
    });

    return watcher;
  }

  async unwatch(): Promise<void> {
    if (this.unlistenFn) {
      this.unlistenFn();
    }
    return FileSystem.unwatchDirectory(this.path);
  }
}
