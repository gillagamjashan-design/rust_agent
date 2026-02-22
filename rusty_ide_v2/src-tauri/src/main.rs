// Rusty TUI - A ratatui-based Rust IDE with AI agent integration

mod agent_bridge;
mod agent_manager;
mod app;
mod file_manager;
mod terminal_manager;
mod ui;

use anyhow::Result;
use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::path::PathBuf;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let work_dir = if args.len() > 1 {
        let path = PathBuf::from(&args[1]);
        // Resolve to absolute path
        if path.is_absolute() {
            path
        } else {
            std::env::current_dir()?.join(path)
        }
    } else {
        std::env::current_dir()?
    };

    // Canonicalize the path to resolve . and ..
    let work_dir = work_dir.canonicalize().unwrap_or(work_dir);

    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app with working directory
    let mut app = App::with_directory(work_dir.clone())?;

    // Change to working directory for terminal
    std::env::set_current_dir(&work_dir)?;

    // Initialize file tree
    if let Err(e) = app.refresh_file_tree() {
        app.status_message = format!("Error loading file tree: {}", e);
    }

    // Main event loop
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    let result = loop {
        // Update terminal dimensions for mouse handling
        let size = terminal.size()?;
        app.terminal_width = size.width;
        app.terminal_height = size.height;

        // Draw UI
        terminal.draw(|f| ui::render(f, &app))?;

        // Handle events with timeout
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    match app.handle_key(key).await {
                        Ok(true) => {
                            if app.should_quit {
                                break Ok(());
                            }
                        }
                        Ok(false) => break Ok(()),
                        Err(e) => {
                            app.status_message = format!("Error: {}", e);
                        }
                    }
                }
                Event::Mouse(mouse) => {
                    if let Err(e) = app.handle_mouse(mouse) {
                        app.status_message = format!("Mouse error: {}", e);
                    }
                }
                _ => {}
            }
        }

        // Tick
        if last_tick.elapsed() >= tick_rate {
            // Update terminal output
            if let Err(e) = app.update_terminal() {
                app.status_message = format!("Terminal error: {}", e);
            }
            last_tick = Instant::now();
        }
    };

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}
