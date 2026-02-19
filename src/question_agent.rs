use crate::types::Question;
use anyhow::Result;
use chrono::Utc;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use tokio::time::{sleep, Duration};

pub struct QuestionAgent {
    question_count: usize,
    output_file: PathBuf,
    topics: Vec<&'static str>,
    current_topic_index: usize,
}

impl QuestionAgent {
    pub fn new(output_path: PathBuf) -> Self {
        Self {
            question_count: 0,
            output_file: output_path,
            topics: vec![
                "ownership",
                "borrowing",
                "lifetimes",
                "traits",
                "generics",
                "error_handling",
                "modules",
                "cargo",
                "testing",
                "macros",
                "concurrency",
                "smart_pointers",
                "iterators",
                "closures",
                "pattern_matching",
            ],
            current_topic_index: 0,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        println!("🤔 Question Agent started - generating questions every 2 seconds");

        loop {
            let question = self.generate_question();
            self.save_question(&question)?;

            println!("❓ Q{}: {} [{}]",
                question.id,
                question.text,
                question.category
            );

            sleep(Duration::from_secs(2)).await;
        }
    }

    fn generate_question(&mut self) -> Question {
        self.question_count += 1;
        let topic = self.get_next_topic();

        let question_text = match topic {
            "ownership" => self.generate_ownership_question(),
            "borrowing" => self.generate_borrowing_question(),
            "lifetimes" => self.generate_lifetime_question(),
            "traits" => self.generate_trait_question(),
            "generics" => self.generate_generic_question(),
            "error_handling" => self.generate_error_handling_question(),
            "modules" => self.generate_module_question(),
            "cargo" => self.generate_cargo_question(),
            "testing" => self.generate_testing_question(),
            "macros" => self.generate_macro_question(),
            "concurrency" => self.generate_concurrency_question(),
            "smart_pointers" => self.generate_smart_pointer_question(),
            "iterators" => self.generate_iterator_question(),
            "closures" => self.generate_closure_question(),
            "pattern_matching" => self.generate_pattern_matching_question(),
            _ => "What is Rust?".to_string(),
        };

        Question {
            id: self.question_count,
            text: question_text,
            category: topic.to_string(),
            timestamp: Utc::now(),
        }
    }

    fn get_next_topic(&mut self) -> &'static str {
        let topic = self.topics[self.current_topic_index];
        self.current_topic_index = (self.current_topic_index + 1) % self.topics.len();
        topic
    }

    fn save_question(&self, question: &Question) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.output_file)?;

        writeln!(
            file,
            "[{}] Q{}: {} [{}]",
            question.timestamp.format("%Y-%m-%d %H:%M:%S"),
            question.id,
            question.text,
            question.category
        )?;

        Ok(())
    }

    // Question generators for different topics
    fn generate_ownership_question(&self) -> String {
        let questions = vec![
            "What is ownership in Rust and why is it important?",
            "How does Rust's ownership system prevent memory leaks?",
            "What are the three rules of ownership in Rust?",
            "When does ownership transfer occur in Rust?",
            "How does move semantics work in Rust?",
        ];
        questions[self.question_count % questions.len()].to_string()
    }

    fn generate_borrowing_question(&self) -> String {
        let questions = vec![
            "What is the difference between mutable and immutable references in Rust?",
            "Why can't you have multiple mutable references to the same data?",
            "What is the borrowing checker and what does it do?",
            "How do you borrow a value without taking ownership?",
            "What happens when you try to modify a borrowed value?",
        ];
        questions[self.question_count % questions.len()].to_string()
    }

    fn generate_lifetime_question(&self) -> String {
        let questions = vec![
            "What are lifetimes in Rust and why do we need them?",
            "How do you annotate lifetimes in function signatures?",
            "What is the 'static lifetime in Rust?",
            "How does the compiler infer lifetimes?",
            "What are lifetime elision rules?",
        ];
        questions[self.question_count % questions.len()].to_string()
    }

    fn generate_trait_question(&self) -> String {
        let questions = vec![
            "What are traits in Rust and how do they work?",
            "How do you implement a trait for a struct?",
            "What is trait inheritance in Rust?",
            "What are trait bounds and when do you use them?",
            "How do trait objects work in Rust?",
        ];
        questions[self.question_count % questions.len()].to_string()
    }

    fn generate_generic_question(&self) -> String {
        let questions = vec![
            "How do generic types work in Rust?",
            "What is monomorphization in Rust?",
            "How do you constrain generic types with trait bounds?",
            "What are associated types in Rust?",
            "How do you write generic functions in Rust?",
        ];
        questions[self.question_count % questions.len()].to_string()
    }

    fn generate_error_handling_question(&self) -> String {
        let questions = vec![
            "What is the difference between Result and Option in Rust?",
            "How do you use the ? operator for error propagation?",
            "What are the best practices for error handling in Rust?",
            "How do you create custom error types in Rust?",
            "When should you use unwrap() vs expect() in Rust?",
        ];
        questions[self.question_count % questions.len()].to_string()
    }

    fn generate_module_question(&self) -> String {
        let questions = vec![
            "How do you organize code into modules in Rust?",
            "What is the difference between mod and use in Rust?",
            "How does Rust's module system handle privacy?",
            "How do you create a library crate in Rust?",
            "What is the purpose of mod.rs files?",
        ];
        questions[self.question_count % questions.len()].to_string()
    }

    fn generate_cargo_question(&self) -> String {
        let questions = vec![
            "What is Cargo and what does it do?",
            "How do you add dependencies in Cargo.toml?",
            "What are the different Cargo commands for building projects?",
            "How do you publish a crate to crates.io?",
            "What are workspace features in Cargo?",
        ];
        questions[self.question_count % questions.len()].to_string()
    }

    fn generate_testing_question(&self) -> String {
        let questions = vec![
            "How do you write unit tests in Rust?",
            "What is the difference between unit tests and integration tests?",
            "How do you use the assert! macro family in Rust?",
            "What is test-driven development in Rust?",
            "How do you test private functions in Rust?",
        ];
        questions[self.question_count % questions.len()].to_string()
    }

    fn generate_macro_question(&self) -> String {
        let questions = vec![
            "What are macros in Rust and how do they differ from functions?",
            "How do you write a declarative macro in Rust?",
            "What are procedural macros in Rust?",
            "When should you use macros vs functions?",
            "How do derive macros work in Rust?",
        ];
        questions[self.question_count % questions.len()].to_string()
    }

    fn generate_concurrency_question(&self) -> String {
        let questions = vec![
            "How does Rust ensure thread safety?",
            "What are channels in Rust and how do they work?",
            "How do you use Arc and Mutex for shared state?",
            "What is the Send trait in Rust?",
            "How do you spawn threads in Rust?",
        ];
        questions[self.question_count % questions.len()].to_string()
    }

    fn generate_smart_pointer_question(&self) -> String {
        let questions = vec![
            "What is Box and when should you use it?",
            "How does Rc differ from Arc in Rust?",
            "What is RefCell and what problem does it solve?",
            "When should you use smart pointers in Rust?",
            "How do weak references work in Rust?",
        ];
        questions[self.question_count % questions.len()].to_string()
    }

    fn generate_iterator_question(&self) -> String {
        let questions = vec![
            "What are iterators in Rust and how do they work?",
            "How do you chain iterator methods in Rust?",
            "What is the difference between iter() and into_iter()?",
            "How do you implement the Iterator trait?",
            "What are iterator adapters in Rust?",
        ];
        questions[self.question_count % questions.len()].to_string()
    }

    fn generate_closure_question(&self) -> String {
        let questions = vec![
            "What are closures in Rust and how do they work?",
            "What are the different closure traits (Fn, FnMut, FnOnce)?",
            "How do closures capture variables from their environment?",
            "When should you use closures vs functions?",
            "How do you move ownership into a closure?",
        ];
        questions[self.question_count % questions.len()].to_string()
    }

    fn generate_pattern_matching_question(&self) -> String {
        let questions = vec![
            "How does pattern matching work in Rust?",
            "What is the match expression and how do you use it?",
            "How do you destructure structs and enums in patterns?",
            "What are match guards in Rust?",
            "When should you use if let vs match?",
        ];
        questions[self.question_count % questions.len()].to_string()
    }
}
