use rust_agent::web_search::{DuckDuckGoClient, SearchQuery};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Testing DuckDuckGo Web Search");
    println!("==============================\n");

    // Create a client
    let client = DuckDuckGoClient::new()?;

    // Test 1: Search for Rust programming language
    println!("Test 1: Searching for 'Rust programming language'");
    let query = SearchQuery::new("Rust programming language").with_max_results(5);

    match client.search(&query).await {
        Ok(response) => {
            println!("Query: {}", response.query);
            println!("Provider: {}", response.provider);
            println!("Total results: {}", response.total_results);
            println!("\nResults:");

            for (i, result) in response.results.iter().enumerate() {
                println!("\n{}. {}", i + 1, result.title);
                println!("   URL: {}", result.url);
                println!("   Snippet: {}", result.snippet.chars().take(100).collect::<String>());
                println!("   Relevance: {:.2}", result.relevance_score);
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!("\n==============================");

    // Test 2: Search for Rust ownership
    println!("\nTest 2: Searching for 'Rust ownership borrow checker'");
    let query = SearchQuery::new("Rust ownership borrow checker").with_max_results(3);

    match client.search(&query).await {
        Ok(response) => {
            println!("Total results: {}", response.total_results);

            for result in response.results.iter() {
                println!("\n- {}", result.title);
                println!("  {}", result.url);
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!("\n==============================");
    println!("Test complete!");

    Ok(())
}
