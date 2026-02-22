use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use notify::{Event, RecursiveMode, Watcher};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: Option<String>,
}

pub struct FileManager {
    permissions: Arc<Mutex<Vec<String>>>,
    watchers: Arc<Mutex<HashMap<String, Box<dyn Watcher + Send>>>>,
}

impl FileManager {
    pub fn new() -> Result<Self> {
        let permissions = Self::load_permissions()?;
        Ok(Self {
            permissions: Arc::new(Mutex::new(permissions)),
            watchers: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub fn read_file(&self, path: &Path) -> Result<String> {
        // For TUI mode, allow reading any file the user opens
        fs::read_to_string(path).context("Failed to read file")
    }

    pub fn write_file(&self, path: &Path, content: &str) -> Result<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).context("Failed to create parent directory")?;
        }

        // For TUI mode, allow writing to any file the user saves
        fs::write(path, content).context("Failed to write file")
    }

    pub fn list_files(&self, directory: &Path) -> Result<Vec<FileInfo>> {
        // For TUI mode, allow access to any directory the user opens
        // (Permission system is for sandboxed mode only)
        let entries = fs::read_dir(directory).context("Failed to read directory")?;

        let mut files = Vec::new();

        for entry in entries {
            let entry = entry.context("Failed to read entry")?;
            let metadata = entry.metadata().context("Failed to read metadata")?;

            let modified = metadata.modified().ok().and_then(|time| {
                let datetime: DateTime<Utc> = time.into();
                Some(datetime.to_rfc3339())
            });

            files.push(FileInfo {
                name: entry.file_name().to_string_lossy().to_string(),
                path: entry.path().to_string_lossy().to_string(),
                is_dir: metadata.is_dir(),
                size: metadata.len(),
                modified,
            });
        }

        Ok(files)
    }

    pub fn grant_permission(&mut self, path: PathBuf) -> Result<()> {
        let mut permissions = self.permissions.lock();

        let canonical = path.canonicalize().context("Failed to canonicalize path")?;
        let canonical_str = canonical.to_string_lossy().to_string();

        if !permissions.contains(&canonical_str) {
            permissions.push(canonical_str);
            Self::save_permissions(&permissions)?;
        }

        Ok(())
    }

    pub fn revoke_permission(&mut self, path: PathBuf) -> Result<()> {
        let mut permissions = self.permissions.lock();

        let canonical = path.canonicalize().context("Failed to canonicalize path")?;
        let canonical_str = canonical.to_string_lossy().to_string();
        permissions.retain(|p| p != &canonical_str);

        Self::save_permissions(&permissions)?;

        Ok(())
    }

    pub fn get_permissions(&self) -> Vec<String> {
        self.permissions.lock().clone()
    }

    pub fn check_permission(&self, path: &Path) -> Result<bool> {
        let permissions = self.permissions.lock();

        if permissions.is_empty() {
            return Ok(true); // No restrictions if no permissions set
        }

        let canonical = path.canonicalize().context("Failed to canonicalize path")?;

        let allowed = permissions.iter().any(|p| {
            let p_path = PathBuf::from(p);
            canonical.starts_with(p_path)
        });

        Ok(allowed)
    }

    // Helper functions
    fn get_rusty_dir() -> Result<PathBuf> {
        let home = home::home_dir().context("Failed to get home directory")?;
        let rusty_dir = home.join(".rusty");
        fs::create_dir_all(&rusty_dir)?;
        Ok(rusty_dir)
    }

    fn get_permissions_file() -> Result<PathBuf> {
        let rusty_dir = Self::get_rusty_dir()?;
        Ok(rusty_dir.join("permissions.json"))
    }

    fn load_permissions() -> Result<Vec<String>> {
        let permissions_file = Self::get_permissions_file()?;
        if permissions_file.exists() {
            let content = fs::read_to_string(&permissions_file)?;
            let permissions: Vec<String> = serde_json::from_str(&content)?;
            Ok(permissions)
        } else {
            Ok(Vec::new())
        }
    }

    fn save_permissions(permissions: &[String]) -> Result<()> {
        let permissions_file = Self::get_permissions_file()?;
        let content = serde_json::to_string_pretty(permissions)?;
        fs::write(&permissions_file, content)?;
        Ok(())
    }

    pub fn watch_directory<F>(&self, path: &Path, callback: F) -> Result<()>
    where
        F: Fn(Event) + Send + 'static,
    {
        let path_str = path.to_string_lossy().to_string();

        let mut watcher = notify::recommended_watcher(move |res: std::result::Result<Event, notify::Error>| {
            match res {
                Ok(event) => callback(event),
                Err(e) => eprintln!("Watch error: {:?}", e),
            }
        })?;

        watcher.watch(path, RecursiveMode::Recursive)?;

        self.watchers.lock().insert(path_str, Box::new(watcher));

        Ok(())
    }

    pub fn unwatch_directory(&self, path: &Path) {
        let path_str = path.to_string_lossy().to_string();
        self.watchers.lock().remove(&path_str);
    }
}

impl Default for FileManager {
    fn default() -> Self {
        Self::new().expect("Failed to create FileManager")
    }
}
