use crate::claude_proxy::ClaudeProxy;
use crate::types::Question;
use anyhow::Result;
use chrono::Utc;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use tokio::time::{sleep, Duration};

pub struct ProxyQuestionAgent {
    claude: ClaudeProxy,
    question_count: usize,
    output_file: PathBuf,
    topics: Vec<(&'static str, u8)>,  // (topic, stage)
    current_topic_index: usize,
    current_stage: u8,
}

impl ProxyQuestionAgent {
    pub fn new(output_path: PathBuf) -> Self {
        // Topics organized by 5-stage curriculum
        // Stage 1: Foundation (move semantics, basic borrowing)
        // Stage 2: Borrowing Mastery (shared vs exclusive refs)
        // Stage 3: Lifetime Patterns (explicit lifetimes)
        // Stage 4: Advanced Ownership (Rc, Arc, RefCell)
        // Stage 5: Systems Thinking (unsafe, raw pointers)
        Self {
            claude: ClaudeProxy::new(),
            question_count: 0,
            output_file: output_path,
            topics: vec![
                // Stage 1: Foundation
                ("Move_Semantics", 1),
                ("Basic_Borrowing", 1),
                ("Ownership_Transfer", 1),
                ("Stack_vs_Heap", 1),
                ("Value_vs_Reference", 1),

                // Stage 2: Borrowing Mastery
                ("Shared_References", 2),
                ("Mutable_References", 2),
                ("Borrow_Checker_Rules", 2),
                ("Multiple_Borrowing", 2),
                ("Reference_Lifetime_Basics", 2),

                // Stage 3: Lifetime Patterns
                ("Explicit_Lifetimes", 3),
                ("Struct_Lifetimes", 3),
                ("Function_Lifetimes", 3),
                ("Lifetime_Bounds", 3),
                ("Lifetime_Elision", 3),

                // Stage 4: Advanced Ownership
                ("Rc_and_Arc", 4),
                ("RefCell_Pattern", 4),
                ("Interior_Mutability", 4),
                ("Smart_Pointers", 4),
                ("Weak_References", 4),

                // Stage 5: Systems Thinking
                ("Unsafe_Rust", 5),
                ("Raw_Pointers", 5),
                ("FFI_Patterns", 5),
                ("Custom_Allocators", 5),
                ("Unsafe_Abstractions", 5),
            ],
            current_topic_index: 0,
            current_stage: 1,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        println!("🤔 Question Agent (CLIProxyAPI) - using Claude Max");
        println!("📚 Teaching Protocol: 5-Stage Rust Mastery Curriculum");

        loop {
            let (topic, stage) = self.topics[self.current_topic_index];
            self.current_topic_index = (self.current_topic_index + 1) % self.topics.len();

            // Update current stage based on progress
            if self.current_topic_index == 0 {
                // Cycle through stages every full rotation
                self.current_stage = ((self.current_stage % 5) + 1).min(5);
            }

            match self.claude.generate_question(topic).await {
                Ok(text) => {
                    self.question_count += 1;
                    let category = format!("Stage_{}_{}",  stage, topic);
                    let question = Question {
                        id: self.question_count,
                        text,
                        category: category.clone(),
                        timestamp: Utc::now(),
                    };

                    let mut file = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&self.output_file)?;

                    writeln!(file, "[{}] Q{}: {} [{}]",
                        question.timestamp.format("%Y-%m-%d %H:%M:%S"),
                        question.id, question.text, question.category)?;

                    println!("❓ [Stage {}] Q{}: {}", stage, question.id, question.text);
                }
                Err(e) => eprintln!("Error generating question: {}", e),
            }

            sleep(Duration::from_secs(2)).await;
        }
    }
}

pub struct ProxyAnswerAgent {
    claude: ClaudeProxy,
    input_file: PathBuf,
    output_file: PathBuf,
    last_processed_id: usize,
}

impl ProxyAnswerAgent {
    pub fn new(input_path: PathBuf, output_path: PathBuf) -> Self {
        Self {
            claude: ClaudeProxy::new(),
            input_file: input_path,
            output_file: output_path,
            last_processed_id: 0,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        println!("💡 Answer Agent (CLIProxyAPI) - using Claude Max");

        loop {
            if let Some((id, question_text, _)) = self.read_latest_question()? {
                if id > self.last_processed_id {
                    match self.claude.generate_answer(&question_text).await {
                        Ok(answer_text) => {
                            let mut file = OpenOptions::new()
                                .create(true)
                                .append(true)
                                .open(&self.output_file)?;

                            writeln!(file, "[{}] A{}: {}\n",
                                Utc::now().format("%Y-%m-%d %H:%M:%S"),
                                id, answer_text)?;

                            println!("✅ A{}: Generated", id);
                            self.last_processed_id = id;
                        }
                        Err(e) => eprintln!("Error generating answer: {}", e),
                    }
                }
            }

            sleep(Duration::from_secs(2)).await;
        }
    }

    fn read_latest_question(&self) -> Result<Option<(usize, String, String)>> {
        if !self.input_file.exists() {
            return Ok(None);
        }

        let file = File::open(&self.input_file)?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

        if let Some(last_line) = lines.last() {
            if let Some(q_pos) = last_line.find("Q") {
                let after_q = &last_line[q_pos + 1..];
                if let Some(colon_pos) = after_q.find(':') {
                    let id_str = &after_q[..colon_pos];
                    if let Ok(id) = id_str.trim().parse::<usize>() {
                        let after_colon = &after_q[colon_pos + 1..];
                        if let Some(bracket_pos) = after_colon.rfind('[') {
                            let question = after_colon[..bracket_pos].trim().to_string();
                            let category = after_colon[bracket_pos + 1..]
                                .trim_end_matches(']').trim().to_string();
                            return Ok(Some((id, question, category)));
                        }
                    }
                }
            }
        }

        Ok(None)
    }
}
