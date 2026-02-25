// Knowledge loader - Populates database from JSON knowledge files

use super::database::{
    CodeExample, CommandFlag, KnowledgeConcept, KnowledgeCommand, KnowledgeDatabase,
    KnowledgePattern,
};
use anyhow::Result;
use serde_json::Value;
use std::fs;
use std::path::Path;

pub struct KnowledgeLoader {
    db: KnowledgeDatabase,
}

impl KnowledgeLoader {
    /// Create a new loader with the given database
    pub fn new(db: KnowledgeDatabase) -> Self {
        Self { db }
    }

    /// Load all knowledge files from the knowledge directory
    pub fn load_all_from_directory<P: AsRef<Path>>(&self, dir: P) -> Result<LoadStats> {
        let mut stats = LoadStats::default();

        // Load core concepts
        let concepts_path = dir.as_ref().join("rust_core_concepts.json");
        if concepts_path.exists() {
            stats.concepts += self.load_core_concepts(&concepts_path)?;
        }

        // Load patterns
        let patterns_path = dir.as_ref().join("rust_patterns_idioms.json");
        if patterns_path.exists() {
            stats.patterns += self.load_patterns(&patterns_path)?;
        }

        // Load toolchain commands
        let toolchain_path = dir.as_ref().join("rust_toolchain_cargo.json");
        if toolchain_path.exists() {
            stats.commands += self.load_commands(&toolchain_path)?;
        }

        Ok(stats)
    }

    /// Load core concepts from rust_core_concepts.json
    fn load_core_concepts<P: AsRef<Path>>(&self, path: P) -> Result<usize> {
        let content = fs::read_to_string(path)?;
        let data: Value = serde_json::from_str(&content)?;

        let mut count = 0;

        if let Some(modules) = data["modules"].as_array() {
            for module in modules {
                let module_id = module["id"].as_str().unwrap_or("unknown");
                let topic = module_id;

                if let Some(concepts) = module["concepts"].as_array() {
                    for concept in concepts {
                        let concept_name = concept["name"].as_str().unwrap_or("Unnamed");
                        let id = format!("{}-{}", module_id, slugify(concept_name));

                        // Parse code examples
                        let code_examples = if let Some(examples) = concept["examples"].as_array() {
                            examples
                                .iter()
                                .map(|ex| CodeExample {
                                    title: ex["title"].as_str().unwrap_or("").to_string(),
                                    code: ex["code"].as_str().unwrap_or("").to_string(),
                                    explanation: ex["explanation"].as_str().unwrap_or("").to_string(),
                                })
                                .collect()
                        } else {
                            vec![]
                        };

                        // Parse common errors
                        let common_mistakes = if let Some(errors) = concept["common_errors"].as_array() {
                            errors
                                .iter()
                                .filter_map(|e| e.as_str().map(|s| s.to_string()))
                                .collect()
                        } else {
                            vec![]
                        };

                        // Build explanation from description and key_points
                        let mut explanation = concept["description"].as_str().unwrap_or("").to_string();

                        if let Some(rules) = concept["rules"].as_array() {
                            explanation.push_str("\n\nRules:\n");
                            for rule in rules {
                                if let Some(r) = rule.as_str() {
                                    explanation.push_str(&format!("- {}\n", r));
                                }
                            }
                        }

                        if let Some(key_points) = concept["key_points"].as_array() {
                            explanation.push_str("\n\nKey Points:\n");
                            for point in key_points {
                                if let Some(p) = point.as_str() {
                                    explanation.push_str(&format!("- {}\n", p));
                                }
                            }
                        }

                        let knowledge_concept = KnowledgeConcept {
                            id: id.clone(),
                            topic: topic.to_string(),
                            title: concept_name.to_string(),
                            explanation,
                            code_examples,
                            common_mistakes,
                            related_concepts: vec![],
                            tags: vec![topic.to_string(), module_id.to_string()],
                        };

                        self.db.store_concept(&knowledge_concept)?;
                        count += 1;
                    }
                }
            }
        }

        Ok(count)
    }

    /// Load patterns from rust_patterns_idioms.json
    fn load_patterns<P: AsRef<Path>>(&self, path: P) -> Result<usize> {
        let content = fs::read_to_string(path)?;
        let data: Value = serde_json::from_str(&content)?;

        let mut count = 0;

        if let Some(categories) = data["categories"].as_array() {
            for category in categories {
                let category_name = category["name"].as_str().unwrap_or("Unknown");

                if let Some(patterns) = category["patterns"].as_array() {
                    for pattern in patterns {
                        let pattern_name = pattern["name"].as_str().unwrap_or("Unnamed");
                        let id = format!("{}-{}", slugify(category_name), slugify(pattern_name));

                        // Parse examples
                        let examples = if let Some(ex) = pattern["example"].as_str() {
                            vec![CodeExample {
                                title: pattern_name.to_string(),
                                code: ex.to_string(),
                                explanation: pattern["description"].as_str().unwrap_or("").to_string(),
                            }]
                        } else {
                            vec![]
                        };

                        let knowledge_pattern = KnowledgePattern {
                            id: id.clone(),
                            name: pattern_name.to_string(),
                            description: pattern["description"].as_str().unwrap_or("").to_string(),
                            template: pattern["example"].as_str().unwrap_or("").to_string(),
                            when_to_use: category_name.to_string(),
                            when_not_to_use: String::new(),
                            examples,
                        };

                        self.db.store_pattern(&knowledge_pattern)?;
                        count += 1;
                    }
                }
            }
        }

        Ok(count)
    }

    /// Load commands from rust_toolchain_cargo.json
    fn load_commands<P: AsRef<Path>>(&self, path: P) -> Result<usize> {
        let content = fs::read_to_string(path)?;
        let data: Value = serde_json::from_str(&content)?;

        let mut count = 0;

        if let Some(sections) = data["sections"].as_array() {
            for section in sections {
                let section_name = section["name"].as_str().unwrap_or("Unknown");

                // Determine tool name from section
                let tool = if section_name.contains("Cargo") {
                    "cargo"
                } else if section_name.contains("Rustup") {
                    "rustup"
                } else {
                    "rust"
                };

                if let Some(commands) = section["commands"].as_array() {
                    for cmd in commands {
                        let command_str = cmd["command"].as_str().unwrap_or("");
                        let description = cmd["description"].as_str().unwrap_or("");

                        // Parse flags/options
                        let flags = if let Some(opts) = cmd["options"].as_array() {
                            opts.iter()
                                .filter_map(|opt| {
                                    if let Some(opt_str) = opt.as_str() {
                                        // Split on " - " to get flag and description
                                        let parts: Vec<&str> = opt_str.splitn(2, " - ").collect();
                                        if parts.len() == 2 {
                                            Some(CommandFlag {
                                                flag: parts[0].trim().to_string(),
                                                description: parts[1].trim().to_string(),
                                            })
                                        } else {
                                            Some(CommandFlag {
                                                flag: opt_str.to_string(),
                                                description: String::new(),
                                            })
                                        }
                                    } else {
                                        None
                                    }
                                })
                                .collect()
                        } else {
                            vec![]
                        };

                        // Parse examples
                        let examples = if let Some(exs) = cmd["examples"].as_array() {
                            exs.iter()
                                .filter_map(|e| e.as_str().map(|s| s.to_string()))
                                .collect()
                        } else {
                            vec![]
                        };

                        let knowledge_command = KnowledgeCommand {
                            tool: tool.to_string(),
                            command: command_str.to_string(),
                            description: description.to_string(),
                            flags,
                            examples,
                        };

                        self.db.store_command(&knowledge_command)?;
                        count += 1;
                    }
                }
            }
        }

        Ok(count)
    }

    /// Get database statistics
    pub fn get_stats(&self) -> Result<LoadStats> {
        Ok(LoadStats {
            concepts: self.db.count_concepts()?,
            patterns: self.db.count_patterns()?,
            errors: self.db.count_errors()?,
            commands: self.db.count_commands()?,
        })
    }
}

/// Statistics about loaded knowledge
#[derive(Debug, Default, Clone)]
pub struct LoadStats {
    pub concepts: usize,
    pub patterns: usize,
    pub errors: usize,
    pub commands: usize,
}

impl LoadStats {
    pub fn total(&self) -> usize {
        self.concepts + self.patterns + self.errors + self.commands
    }
}

impl std::fmt::Display for LoadStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Loaded {} concepts, {} patterns, {} errors, {} commands (total: {})",
            self.concepts,
            self.patterns,
            self.errors,
            self.commands,
            self.total()
        )
    }
}

/// Convert string to slug (lowercase, hyphens instead of spaces)
fn slugify(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() {
                '-'
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Move Semantics"), "move-semantics");
        assert_eq!(slugify("Builder Pattern"), "builder-pattern");
        assert_eq!(slugify("Error E0382"), "error-e0382");
    }

    #[test]
    fn test_load_core_concepts() {
        let db = KnowledgeDatabase::new_in_memory().unwrap();
        let loader = KnowledgeLoader::new(db);

        // Test with actual knowledge file
        let result = loader.load_core_concepts("knowledge/rust_core_concepts.json");

        if result.is_ok() {
            let count = result.unwrap();
            assert!(count > 0, "Should load at least one concept");

            let stats = loader.get_stats().unwrap();
            assert_eq!(stats.concepts, count);
        }
    }
}
