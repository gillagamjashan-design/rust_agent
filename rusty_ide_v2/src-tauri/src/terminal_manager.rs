use anyhow::{Context, Result};
use parking_lot::Mutex;
use portable_pty::{native_pty_system, CommandBuilder, MasterPty, PtySize};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use uuid::Uuid;

pub struct TerminalInstance {
    id: String,
    master: Box<dyn MasterPty + Send>,
    writer: Box<dyn Write + Send>,
    output_buffer: Arc<Mutex<Vec<String>>>,
    _reader_thread: Option<thread::JoinHandle<()>>,
}

impl TerminalInstance {
    pub fn new() -> Result<Self> {
        let id = Uuid::new_v4().to_string();
        let pty_system = native_pty_system();

        // Create a new pty
        let pair = pty_system
            .openpty(PtySize {
                rows: 24,
                cols: 80,
                pixel_width: 0,
                pixel_height: 0,
            })
            .context("Failed to open pty")?;

        // Determine the shell to use
        #[cfg(unix)]
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());

        #[cfg(windows)]
        let shell = "powershell.exe";

        let mut cmd = CommandBuilder::new(shell);
        cmd.cwd(std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/")));

        // Spawn the shell
        let _child = pair
            .slave
            .spawn_command(cmd)
            .context("Failed to spawn shell")?;

        // Get reader and writer
        let mut reader = pair
            .master
            .try_clone_reader()
            .context("Failed to clone reader")?;
        let writer = pair
            .master
            .take_writer()
            .context("Failed to get writer")?;

        // Create output buffer for non-blocking reads
        let output_buffer = Arc::new(Mutex::new(Vec::new()));
        let buffer_clone = output_buffer.clone();

        // Spawn a background thread to read terminal output
        let reader_thread = thread::spawn(move || {
            let mut buf = [0u8; 4096];
            loop {
                match reader.read(&mut buf) {
                    Ok(0) => break, // EOF
                    Ok(n) => {
                        let data = String::from_utf8_lossy(&buf[..n]).to_string();
                        buffer_clone.lock().push(data);
                    }
                    Err(e) => {
                        if e.kind() != std::io::ErrorKind::Interrupted {
                            break;
                        }
                    }
                }
            }
        });

        Ok(Self {
            id,
            master: pair.master,
            writer,
            output_buffer,
            _reader_thread: Some(reader_thread),
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn read(&self) -> Result<Option<String>> {
        // Non-blocking read from the output buffer (filled by background thread)
        let mut buffer = self.output_buffer.lock();
        if buffer.is_empty() {
            Ok(None)
        } else {
            // Drain all buffered output
            let output = buffer.drain(..).collect::<Vec<_>>().join("");
            Ok(Some(output))
        }
    }

    pub fn write(&mut self, data: &str) -> Result<()> {
        self.writer
            .write_all(data.as_bytes())
            .context("Failed to write to terminal")?;

        self.writer
            .flush()
            .context("Failed to flush terminal")?;

        Ok(())
    }

    pub fn resize(&self, cols: u16, rows: u16) -> Result<()> {
        self.master
            .resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .context("Failed to resize terminal")?;

        Ok(())
    }
}

pub struct TerminalManager {
    terminals: Arc<Mutex<Vec<TerminalInstance>>>,
    current_terminal: Arc<Mutex<usize>>,
}

impl TerminalManager {
    pub fn new() -> Result<Self> {
        let mut terminals = Vec::new();

        // Try to create initial terminal, but don't fail if PTY isn't available
        match TerminalInstance::new() {
            Ok(terminal) => terminals.push(terminal),
            Err(e) => {
                eprintln!("Warning: Could not create terminal: {}", e);
                // Continue without terminal - app will still work
            }
        }

        Ok(Self {
            terminals: Arc::new(Mutex::new(terminals)),
            current_terminal: Arc::new(Mutex::new(0)),
        })
    }

    pub fn create_terminal(&self) -> Result<String> {
        let instance = TerminalInstance::new()?;
        let id = instance.id().to_string();

        let mut terminals = self.terminals.lock();
        terminals.push(instance);
        *self.current_terminal.lock() = terminals.len() - 1;

        Ok(id)
    }

    pub fn write(&self, data: &str) -> Result<()> {
        let mut terminals = self.terminals.lock();
        let current_idx = *self.current_terminal.lock();

        if let Some(terminal) = terminals.get_mut(current_idx) {
            terminal.write(data)?;
        }

        Ok(())
    }

    pub fn read(&self) -> Result<Option<String>> {
        let terminals = self.terminals.lock();
        let current_idx = *self.current_terminal.lock();

        if let Some(terminal) = terminals.get(current_idx) {
            terminal.read()
        } else {
            Ok(None)
        }
    }

    pub fn resize(&self, cols: u16, rows: u16) -> Result<()> {
        let terminals = self.terminals.lock();
        let current_idx = *self.current_terminal.lock();

        if let Some(terminal) = terminals.get(current_idx) {
            terminal.resize(cols, rows)?;
        }

        Ok(())
    }

    pub fn switch_terminal(&self, index: usize) -> Result<()> {
        let terminals = self.terminals.lock();
        if index < terminals.len() {
            *self.current_terminal.lock() = index;
            Ok(())
        } else {
            anyhow::bail!("Terminal index {} out of bounds", index)
        }
    }

    pub fn get_terminal_count(&self) -> usize {
        self.terminals.lock().len()
    }

    pub fn current_terminal_index(&self) -> usize {
        *self.current_terminal.lock()
    }
}

impl Default for TerminalManager {
    fn default() -> Self {
        Self::new().expect("Failed to create TerminalManager")
    }
}
