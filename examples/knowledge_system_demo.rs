// Example demonstrating the knowledge system
//
// This shows how to:
// 1. Load knowledge from JSON files into the database
// 2. Query the knowledge database
// 3. Use the knowledge fetcher tool
// 4. Generate direct instruction prompts

use anyhow::Result;
use rust_agent::knowledge::{KnowledgeDatabase, KnowledgeLoader, KnowledgeQuery};
use rust_agent::tools::KnowledgeFetcher;
use rust_agent::training::{DirectInstruction, LearningStage};

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Knowledge System Demo ===\n");

    // Step 1: Create database
    println!("1. Creating knowledge database...");
    let db = KnowledgeDatabase::new("data/knowledge.db")?;
    println!("   ✓ Database created\n");

    // Step 2: Load knowledge from JSON files
    println!("2. Loading knowledge from JSON files...");
    let loader = KnowledgeLoader::new(db);
    let stats = loader.load_all_from_directory("knowledge")?;
    println!("   ✓ {}\n", stats);

    // Step 3: Create query interface
    println!("3. Creating query interface...");
    let db2 = KnowledgeDatabase::new("data/knowledge.db")?;
    let query = KnowledgeQuery::new(db2);
    println!("   ✓ Query interface ready\n");

    // Step 4: Search concepts
    println!("4. Searching for concepts about 'ownership'...");
    let concepts = query.search_concepts("ownership")?;
    println!("   Found {} concepts:", concepts.len());
    for concept in concepts.iter().take(3) {
        println!("   - {} (topic: {})", concept.title, concept.topic);
    }
    println!();

    // Step 5: Find patterns
    println!("5. Searching for patterns about 'builder'...");
    let patterns = query.find_patterns("builder")?;
    println!("   Found {} patterns:", patterns.len());
    for pattern in patterns.iter().take(3) {
        println!("   - {}", pattern.name);
    }
    println!();

    // Step 6: Use knowledge fetcher
    println!("6. Using knowledge fetcher...");
    let fetcher = KnowledgeFetcher::new(query);

    let response = fetcher.explain_concept("ownership")?;
    println!("   Query: Explain 'ownership'");
    println!("   Confidence: {:.2}", response.confidence);
    println!("   Has results: {}", response.has_results());
    println!();

    // Step 7: Generate direct instruction
    println!("7. Generating direct instruction...");
    let db3 = KnowledgeDatabase::new("data/knowledge.db")?;
    let query2 = KnowledgeQuery::new(db3);
    let instructor = DirectInstruction::new(query2);

    let instruction = instructor.generate_instruction("ownership")?;
    println!("   Generated instruction ({} chars)", instruction.len());
    println!("   Preview:");
    println!("   ---");
    for line in instruction.lines().take(15) {
        println!("   {}", line);
    }
    println!("   ...");
    println!();

    // Step 8: Generate curriculum
    println!("8. Generating beginner curriculum...");
    let curriculum = instructor.generate_curriculum(LearningStage::Beginner)?;
    println!("   Generated curriculum ({} chars)", curriculum.len());
    println!("   Covers topics: {:?}", LearningStage::Beginner.topics());
    println!();

    // Step 9: Inject context for agent
    println!("9. Context injection demo...");
    let user_query = "How do I implement a builder pattern in Rust?";
    let context = instructor.inject_context(user_query)?;
    println!("   User query: {}", user_query);
    println!("   Injected context length: {} chars", context.len());
    println!();

    println!("=== Demo Complete ===");
    println!("\nThe knowledge system provides:");
    println!("  ✓ Queryable database of Rust concepts, patterns, and commands");
    println!("  ✓ Full-text search using SQLite FTS5");
    println!("  ✓ Knowledge fetcher tool for runtime queries");
    println!("  ✓ Direct instruction generator for learning");
    println!("  ✓ Context injection for agent responses");

    Ok(())
}
