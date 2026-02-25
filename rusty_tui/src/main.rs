//! Rusty - Beautiful GUI for Rust Learning Agent
//!
//! A graphical interface that provides instant access to a comprehensive
//! Rust knowledge database through an AI-powered chat agent.

use eframe::egui;

mod gui;

fn main() -> Result<(), eframe::Error> {
    // Check if database exists (first run detection)
    let db_path = dirs::home_dir()
        .expect("Could not find home directory")
        .join(".agent")
        .join("data")
        .join("knowledge.db");

    if !db_path.exists() {
        println!("🦀 Rusty - Rust Learning Agent");
        println!();
        println!("📚 First run detected - loading knowledge database...");
        println!("   This will take about 1-2 seconds.");
        println!();

        // Ensure data directory exists
        std::fs::create_dir_all(db_path.parent().unwrap()).ok();
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_min_inner_size([600.0, 400.0])
            .with_title("Rusty 🦀 - Rust Learning Agent"),
        ..Default::default()
    };

    eframe::run_native(
        "Rusty",
        options,
        Box::new(|cc| Ok(Box::new(gui::RustyApp::new(cc)))),
    )
}
