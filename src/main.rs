mod book_reader;
mod types;

use book_reader::BookReader;
use anyhow::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║        Rust Book Learning Agent                             ║");
    println!("║        Reading and Understanding The Rust Book              ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    let book_path = PathBuf::from("rust.book");
    let knowledge_path = PathBuf::from("data/rust_book_knowledge.json");

    // Create data directory
    std::fs::create_dir_all("data")?;

    println!("📚 Initializing Book Reader...");
    let mut reader = BookReader::new(book_path)?;

    println!("📖 Book loaded: {} lines total", reader.get_total_lines());
    println!("🧠 Starting to read and learn...");
    println!();

    let lines_per_batch = 500; // Read 500 lines at a time
    let mut batch_count = 0;

    loop {
        batch_count += 1;

        // Read and learn from next batch
        let is_complete = reader.read_and_learn(lines_per_batch)?;

        let progress = reader.get_progress();
        let kb = reader.get_knowledge_base();

        println!("📊 Batch {}: Read {}/{} lines ({:.1}%)",
            batch_count,
            reader.get_current_line(),
            reader.get_total_lines(),
            progress
        );
        println!("   🎓 Topics learned: {}", kb.topics_covered.len());
        println!("   🔧 Patterns extracted: {}", kb.patterns.len());
        println!();

        // Save knowledge periodically
        if batch_count % 5 == 0 {
            reader.save_knowledge(&knowledge_path)?;
            println!("   💾 Knowledge saved to {:?}", knowledge_path);
            println!();
        }

        if is_complete {
            break;
        }
    }

    // Final save
    reader.save_knowledge(&knowledge_path)?;

    println!("═══════════════════════════════════════════════════════════════");
    println!("✅ BOOK READING COMPLETE!");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("📚 Total lines read: {}", reader.get_total_lines());
    println!("🎓 Topics learned: {}", reader.get_knowledge_base().topics_covered.len());
    println!("🔧 Patterns extracted: {}", reader.get_knowledge_base().patterns.len());
    println!();
    println!("💾 Knowledge saved to: {:?}", knowledge_path);
    println!();
    println!("🎉 YOUR agent has finished reading and understanding the Rust Book!");
    println!();

    Ok(())
}
