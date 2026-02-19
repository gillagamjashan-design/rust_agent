use crate::types::{KnowledgeBase, Pattern};
use anyhow::Result;
use chrono::Utc;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub struct BookReader {
    book_file: PathBuf,
    current_line: usize,
    total_lines: usize,
    knowledge_base: KnowledgeBase,
}

impl BookReader {
    pub fn new(book_path: PathBuf) -> Result<Self> {
        // Count total lines
        let file = File::open(&book_path)?;
        let reader = BufReader::new(file);
        let total_lines = reader.lines().count();

        Ok(Self {
            book_file: book_path,
            current_line: 0,
            total_lines,
            knowledge_base: KnowledgeBase::new(),
        })
    }

    pub fn read_and_learn(&mut self, lines_per_batch: usize) -> Result<bool> {
        let file = File::open(&self.book_file)?;
        let reader = BufReader::new(file);

        let mut current = 0;
        let mut batch_content = String::new();
        let mut code_blocks = Vec::new();
        let mut in_code_block = false;
        let mut current_code = String::new();

        for line in reader.lines() {
            let line = line?;

            // Skip until we reach our current position
            if current < self.current_line {
                current += 1;
                continue;
            }

            // Check for code blocks
            if line.trim().starts_with("```rust") || line.trim().starts_with("```") {
                if in_code_block {
                    // End of code block
                    if !current_code.is_empty() {
                        code_blocks.push(current_code.clone());
                    }
                    current_code.clear();
                    in_code_block = false;
                } else {
                    // Start of code block
                    in_code_block = true;
                }
            } else if in_code_block {
                current_code.push_str(&line);
                current_code.push('\n');
            } else {
                batch_content.push_str(&line);
                batch_content.push('\n');
            }

            self.current_line += 1;

            // Process batch when we've read enough lines
            if self.current_line - current >= lines_per_batch {
                break;
            }
        }

        // Extract knowledge from this batch
        if !batch_content.is_empty() {
            self.extract_knowledge_from_text(&batch_content, &code_blocks)?;
        }

        // Return true if we've finished reading the entire book
        Ok(self.current_line >= self.total_lines)
    }

    fn extract_knowledge_from_text(&mut self, text: &str, code_blocks: &[String]) -> Result<()> {
        // Extract topics/concepts from headers
        let topics = self.extract_topics(text);

        for topic in topics {
            self.knowledge_base.add_topic(topic.clone());

            // Create a pattern for each topic with associated code
            if !code_blocks.is_empty() {
                let pattern = Pattern {
                    name: format!("book_section_{}", self.current_line),
                    description: topic.clone(),
                    code_pattern: code_blocks.join("\n\n"),
                    usage_examples: vec![text.lines().take(3).collect::<Vec<_>>().join(" ")],
                    confidence: 1.0,
                    occurrences: 1,
                };
                self.knowledge_base.add_pattern(pattern);
            }
        }

        Ok(())
    }

    fn extract_topics(&self, text: &str) -> Vec<String> {
        let mut topics = Vec::new();

        for line in text.lines() {
            // Extract from markdown headers
            if line.starts_with("# ") || line.starts_with("## ") || line.starts_with("### ") {
                let topic = line.trim_start_matches('#').trim().to_string();
                if !topic.is_empty() {
                    topics.push(topic);
                }
            }
        }

        // Extract key Rust concepts mentioned
        let keywords = vec![
            "ownership", "borrowing", "lifetime", "trait", "generic",
            "struct", "enum", "impl", "module", "crate", "cargo",
            "closure", "iterator", "panic", "Result", "Option",
            "mutex", "arc", "thread", "async", "await", "macro",
        ];

        let text_lower = text.to_lowercase();
        for keyword in keywords {
            if text_lower.contains(keyword) {
                topics.push(keyword.to_string());
            }
        }

        topics
    }

    pub fn get_progress(&self) -> f64 {
        if self.total_lines == 0 {
            return 0.0;
        }
        (self.current_line as f64 / self.total_lines as f64) * 100.0
    }

    pub fn get_knowledge_base(&self) -> &KnowledgeBase {
        &self.knowledge_base
    }

    pub fn get_current_line(&self) -> usize {
        self.current_line
    }

    pub fn get_total_lines(&self) -> usize {
        self.total_lines
    }

    pub fn is_complete(&self) -> bool {
        self.current_line >= self.total_lines
    }

    pub fn save_knowledge(&self, output_path: &PathBuf) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.knowledge_base)?;
        std::fs::write(output_path, json)?;
        Ok(())
    }
}
