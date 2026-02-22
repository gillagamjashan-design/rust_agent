use anyhow::{Context, Result};
use parking_lot::Mutex;
use portable_pty::{native_pty_system, CommandBuilder, MasterPty, PtySize};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

pub struct TerminalInstance {
    id: String,
    master: Box<dyn MasterPty + Send>,
    writer: Box<dyn Write + Send>,
    reader: Arc<Mutex<Box<dyn Read + Send>>>,
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
        let reader = pair
            .master
            .try_clone_reader()
            .context("Failed to clone reader")?;
        let writer = pair
            .master
            .take_writer()
            .context("Failed to get writer")?;

        Ok(Self {
            id,
            master: pair.master,
            writer,
            reader: Arc::new(Mutex::new(reader)),
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn read(&self) -> Result<Option<String>> {
        let mut buffer = [0u8; 8192];
        let mut reader = self.reader.lock();

        // Set non-blocking mode for reading
        match reader.read(&mut buffer) {
            Ok(n) if n > 0 => {
                let data = String::from_utf8_lossy(&buffer[..n]).to_string();
                Ok(Some(data))
            }
            Ok(_) => Ok(None),
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(None),
            Err(e) => Err(anyhow::Error::from(e)),
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
        // Create initial terminal
        terminals.push(TerminalInstance::new()?);

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
