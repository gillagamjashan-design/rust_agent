use crate::agent_manager::AgentManager;
use crate::file_manager::FileManager;
use anyhow::Result;
use crossterm::event::{KeyEvent, MouseEvent, MouseEventKind, MouseButton};
use std::path::PathBuf;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    FileTree,
    Editor,
    Agent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
    Command,
}

pub struct App {
    pub file_manager: FileManager,
    pub agent_manager: AgentManager,
    pub current_file: Option<PathBuf>,
    pub current_content: String,
    pub focused_panel: Panel,
    pub mode: Mode,
    pub command_buffer: String,
    pub status_message: String,
    pub should_quit: bool,
    pub file_tree_cursor: usize,
    pub file_tree_items: Vec<String>,
    pub current_directory: PathBuf,
    pub agent_query: String,
    pub agent_response: String,
    pub scroll_offset: usize,
    // New fields for VS Code-style input handling
    pub agent_input: String,
    pub agent_chat_history: Vec<(bool, String)>, // (is_user, message)
    pub file_tree_selected: usize,
    // Mouse tracking
    pub last_click: Option<(u16, u16, Instant)>,
    pub terminal_width: u16,
    pub terminal_height: u16,
}

impl App {
    pub fn new() -> Result<Self> {
        let current_directory = std::env::current_dir()?;
        Self::with_directory(current_directory)
    }

    pub fn with_directory(work_dir: PathBuf) -> Result<Self> {
        // Ensure the directory exists
        if !work_dir.is_dir() {
            anyhow::bail!("Not a directory: {:?}", work_dir);
        }

        Ok(Self {
            file_manager: FileManager::new()?,
            agent_manager: AgentManager::new()?,
            current_file: None,
            current_content: String::new(),
            focused_panel: Panel::FileTree,
            mode: Mode::Normal,
            command_buffer: String::new(),
            status_message: format!("Welcome to Rusty TUI! Click anywhere or press Tab to switch panels. Press Enter/i to start typing. Directory: {}", work_dir.display()),
            should_quit: false,
            file_tree_cursor: 0,
            file_tree_items: Vec::new(),
            current_directory: work_dir,
            agent_query: String::new(),
            agent_response: String::new(),
            scroll_offset: 0,
            agent_input: String::new(),
            agent_chat_history: Vec::new(),
            file_tree_selected: 0,
            last_click: None,
            terminal_width: 0,
            terminal_height: 0,
        })
    }

    pub fn refresh_file_tree(&mut self) -> Result<()> {
        let files = self.file_manager.list_files(&self.current_directory)?;
        self.file_tree_items = files.iter().map(|f| f.name.clone()).collect();
        Ok(())
    }

    pub fn open_file(&mut self, path: PathBuf) -> Result<()> {
        let content = self.file_manager.read_file(&path)?;
        self.current_file = Some(path);
        self.current_content = content;
        self.scroll_offset = 0;
        Ok(())
    }

    pub fn save_file(&mut self) -> Result<()> {
        if let Some(ref path) = self.current_file {
            self.file_manager.write_file(path, &self.current_content)?;
            self.status_message = format!("Saved: {:?}", path);
        } else {
            self.status_message = "No file open".to_string();
        }
        Ok(())
    }

    pub async fn handle_key(&mut self, key: KeyEvent) -> Result<bool> {
        use crossterm::event::{KeyCode, KeyModifiers};

        match self.mode {
            Mode::Normal => {
                match key.code {
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.should_quit = true;
                        return Ok(false);
                    }
                    KeyCode::Char(':') => {
                        self.mode = Mode::Command;
                        self.command_buffer.clear();
                    }
                    // Tab to cycle through panels
                    KeyCode::Tab => {
                        self.focused_panel = match self.focused_panel {
                            Panel::FileTree => Panel::Editor,
                            Panel::Editor => Panel::Agent,
                            Panel::Agent => Panel::FileTree,
                        };
                    }
                    // Arrow keys for file tree navigation (more intuitive)
                    KeyCode::Up | KeyCode::Char('k') if self.focused_panel == Panel::FileTree => {
                        if self.file_tree_cursor > 0 {
                            self.file_tree_cursor -= 1;
                            self.file_tree_selected = self.file_tree_cursor;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') if self.focused_panel == Panel::FileTree => {
                        if self.file_tree_cursor + 1 < self.file_tree_items.len() {
                            self.file_tree_cursor += 1;
                            self.file_tree_selected = self.file_tree_cursor;
                        }
                    }
                    // Enter in FileTree opens file
                    KeyCode::Enter if self.focused_panel == Panel::FileTree => {
                        if let Err(e) = self.open_selected_file() {
                            self.status_message = format!("Error: {}", e);
                        }
                    }
                    // Enter/i in other panels enters Insert mode
                    KeyCode::Char('i') | KeyCode::Enter => {
                        match self.focused_panel {
                            Panel::Editor | Panel::Agent => {
                                self.mode = Mode::Insert;
                            }
                            _ => {}
                        }
                    }
                    // Vim-style panel navigation (keep for power users)
                    KeyCode::Char('h') => {
                        self.focused_panel = Panel::FileTree;
                    }
                    KeyCode::Char('l') => {
                        self.focused_panel = Panel::Editor;
                    }
                    KeyCode::Char('a') => {
                        self.focused_panel = Panel::Agent;
                    }
                    KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        if let Err(e) = self.save_file() {
                            self.status_message = format!("Error saving: {}", e);
                        }
                    }
                    _ => {}
                }
            }
            Mode::Insert => {
                match key.code {
                    KeyCode::Esc => {
                        self.mode = Mode::Normal;
                    }
                    KeyCode::Enter => {
                        match self.focused_panel {
                            Panel::Editor => {
                                self.current_content.push('\n');
                            }
                            Panel::Agent => {
                                if let Err(e) = self.send_to_agent().await {
                                    self.status_message = format!("Error sending to agent: {}", e);
                                }
                            }
                            _ => {}
                        }
                    }
                    KeyCode::Char(c) => {
                        match self.focused_panel {
                            Panel::Editor => {
                                self.current_content.push(c);
                            }
                            Panel::Agent => {
                                self.agent_input.push(c);
                            }
                            _ => {}
                        }
                    }
                    KeyCode::Backspace => {
                        match self.focused_panel {
                            Panel::Editor => {
                                self.current_content.pop();
                            }
                            Panel::Agent => {
                                self.agent_input.pop();
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Mode::Command => {
                match key.code {
                    KeyCode::Esc => {
                        self.mode = Mode::Normal;
                        self.command_buffer.clear();
                    }
                    KeyCode::Enter => {
                        self.execute_command();
                        self.mode = Mode::Normal;
                        self.command_buffer.clear();
                    }
                    KeyCode::Char(c) => {
                        self.command_buffer.push(c);
                    }
                    KeyCode::Backspace => {
                        self.command_buffer.pop();
                    }
                    _ => {}
                }
            }
        }

        Ok(true)
    }

    fn execute_command(&mut self) {
        match self.command_buffer.as_str() {
            "q" | "quit" => {
                self.should_quit = true;
            }
            "w" | "write" => {
                if let Err(e) = self.save_file() {
                    self.status_message = format!("Error saving: {}", e);
                }
            }
            "wq" => {
                if let Err(e) = self.save_file() {
                    self.status_message = format!("Error saving: {}", e);
                } else {
                    self.should_quit = true;
                }
            }
            cmd if cmd.starts_with("e ") => {
                let path = cmd.trim_start_matches("e ").trim();
                let full_path = self.current_directory.join(path);
                if let Err(e) = self.open_file(full_path) {
                    self.status_message = format!("Error opening file: {}", e);
                }
            }
            cmd if cmd.starts_with("cd ") => {
                let path = cmd.trim_start_matches("cd ").trim();
                let new_dir = self.current_directory.join(path);
                if new_dir.is_dir() {
                    self.current_directory = new_dir;
                    if let Err(e) = self.refresh_file_tree() {
                        self.status_message = format!("Error refreshing: {}", e);
                    }
                } else {
                    self.status_message = format!("Not a directory: {}", path);
                }
            }
            _ => {
                self.status_message = format!("Unknown command: {}", self.command_buffer);
            }
        }
    }

    pub async fn send_to_agent(&mut self) -> Result<()> {
        if self.agent_input.trim().is_empty() {
            self.status_message = "Please enter a message for the agent".to_string();
            return Ok(());
        }

        let user_message = self.agent_input.clone();
        self.agent_chat_history.push((true, user_message.clone()));
        self.agent_input.clear();

        self.status_message = "🤖 Sending to agent... (this may take a moment)".to_string();

        // Add timeout to prevent hanging
        match tokio::time::timeout(
            std::time::Duration::from_secs(60),
            self.agent_manager.query(user_message)
        ).await {
            Ok(Ok(response)) => {
                self.agent_chat_history.push((false, response.clone()));
                self.agent_response = response;
                self.status_message = "✓ Agent response received".to_string();
            }
            Ok(Err(e)) => {
                let error_msg = format!("Agent error: {}", e);
                self.agent_chat_history.push((false, error_msg.clone()));
                self.status_message = error_msg;
            }
            Err(_) => {
                let timeout_msg = "Agent query timed out after 60 seconds. Please try again.".to_string();
                self.agent_chat_history.push((false, timeout_msg.clone()));
                self.status_message = timeout_msg;
            }
        }

        Ok(())
    }

    pub fn open_selected_file(&mut self) -> Result<()> {
        if let Some(name) = self.file_tree_items.get(self.file_tree_selected) {
            let path = self.current_directory.join(name);
            if path.is_file() {
                self.open_file(path)?;
                self.focused_panel = Panel::Editor;
                self.status_message = "File opened".to_string();
            } else if path.is_dir() {
                self.current_directory = path;
                self.refresh_file_tree()?;
                self.file_tree_selected = 0;
                self.file_tree_cursor = 0;
            }
        }
        Ok(())
    }

    pub fn handle_mouse(&mut self, mouse: MouseEvent) -> Result<()> {
        match mouse.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                let col = mouse.column;
                let row = mouse.row;

                // Calculate panel boundaries based on layout
                // Title: rows 0-2 (height 3)
                // Main area: rows 3 to status
                // Status: last 3 rows

                let title_height = 3;
                let status_height = 3;

                let main_area_start = title_height;
                let main_area_end = self.terminal_height.saturating_sub(status_height);

                // File tree is 20% of width, Editor is 40%, Agent is 40%
                let file_tree_width = (self.terminal_width * 20) / 100;
                let editor_width = (self.terminal_width * 40) / 100;
                let file_tree_end = file_tree_width;
                let editor_end = file_tree_width + editor_width;

                // Determine which panel was clicked
                if row >= main_area_start && row < main_area_end {
                    // Main area (file tree, editor, agent)
                    if col < file_tree_end {
                        // File tree clicked
                        self.focused_panel = Panel::FileTree;

                        // Calculate which file was clicked (accounting for title and border)
                        let file_index = (row.saturating_sub(main_area_start + 1)) as usize;
                        if file_index < self.file_tree_items.len() {
                            self.file_tree_selected = file_index;
                            self.file_tree_cursor = file_index;

                            // Double-click detection (within 500ms)
                            let now = Instant::now();
                            let is_double_click = if let Some((last_col, last_row, last_time)) = self.last_click {
                                last_col == col && last_row == row && now.duration_since(last_time) < Duration::from_millis(500)
                            } else {
                                false
                            };

                            if is_double_click {
                                // Open file on double-click
                                if let Err(e) = self.open_selected_file() {
                                    self.status_message = format!("Error opening file: {}", e);
                                }
                                self.last_click = None; // Reset to prevent triple-click
                            } else {
                                self.last_click = Some((col, row, now));
                            }
                        }
                    } else if col >= file_tree_end && col < editor_end {
                        // Editor clicked - automatically enter Insert mode for typing
                        self.focused_panel = Panel::Editor;
                        self.mode = Mode::Insert;
                        self.last_click = Some((col, row, Instant::now()));
                    } else {
                        // Agent panel clicked - always enter Insert mode so user can type
                        self.focused_panel = Panel::Agent;
                        self.mode = Mode::Insert;
                        self.status_message = "Type your message and press Enter to send".to_string();
                        self.last_click = Some((col, row, Instant::now()));
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}
