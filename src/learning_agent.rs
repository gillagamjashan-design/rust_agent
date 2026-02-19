use crate::types::{Answer, KnowledgeBase, Pattern, QAPair, Question};
use anyhow::Result;
use chrono::Utc;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use tokio::time::{sleep, Duration};

pub struct LearningAgent {
    questions_file: PathBuf,
    answers_file: PathBuf,
    knowledge_base: KnowledgeBase,
    knowledge_file: PathBuf,
    last_learned_id: usize,
}

impl LearningAgent {
    pub fn new(
        questions_path: PathBuf,
        answers_path: PathBuf,
        knowledge_path: PathBuf,
    ) -> Self {
        Self {
            questions_file: questions_path,
            answers_file: answers_path,
            knowledge_base: KnowledgeBase::new(),
            knowledge_file: knowledge_path,
            last_learned_id: 0,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        println!("🧠 Learning Agent started - processing Q&A pairs and building knowledge");

        loop {
            self.process_new_qa_pairs().await?;
            sleep(Duration::from_secs(2)).await;
        }
    }

    async fn process_new_qa_pairs(&mut self) -> Result<()> {
        // Read all questions
        let questions = self.read_questions()?;

        // Read all answers
        let answers = self.read_answers()?;

        // Match Q&A pairs
        for question in questions {
            if question.id > self.last_learned_id {
                if let Some(answer) = answers.iter().find(|a| a.question_id == question.id) {
                    // Create Q&A pair
                    let qa_pair = QAPair {
                        question: question.clone(),
                        answer: answer.clone(),
                    };

                    // Learn from this pair
                    self.learn_from_qa_pair(&qa_pair)?;

                    self.last_learned_id = question.id;

                    println!(
                        "📚 Learned from Q&A #{}: {} (Total knowledge: {} items)",
                        question.id,
                        question.category,
                        self.knowledge_base.get_total_knowledge_count()
                    );
                }
            }
        }

        Ok(())
    }

    fn read_questions(&self) -> Result<Vec<Question>> {
        if !self.questions_file.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(&self.questions_file)?;
        let reader = BufReader::new(file);
        let mut questions = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if let Some(question) = self.parse_question(&line) {
                questions.push(question);
            }
        }

        Ok(questions)
    }

    fn parse_question(&self, line: &str) -> Option<Question> {
        // Parse: [timestamp] Q<id>: <text> [<category>]
        if let Some(q_pos) = line.find("Q") {
            let after_q = &line[q_pos + 1..];
            if let Some(colon_pos) = after_q.find(':') {
                let id_str = &after_q[..colon_pos];
                if let Ok(id) = id_str.trim().parse::<usize>() {
                    let after_colon = &after_q[colon_pos + 1..];
                    if let Some(bracket_pos) = after_colon.rfind('[') {
                        let text = after_colon[..bracket_pos].trim().to_string();
                        let category = after_colon[bracket_pos + 1..]
                            .trim_end_matches(']')
                            .trim()
                            .to_string();

                        return Some(Question {
                            id,
                            text,
                            category,
                            timestamp: Utc::now(),
                        });
                    }
                }
            }
        }
        None
    }

    fn read_answers(&self) -> Result<Vec<Answer>> {
        if !self.answers_file.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(&self.answers_file)?;
        let reader = BufReader::new(file);
        let mut answers = Vec::new();
        let mut current_answer: Option<Answer> = None;
        let mut current_code = String::new();
        let mut in_code_block = false;

        for line in reader.lines() {
            let line = line?;

            if line.starts_with("[CODE_EXAMPLE") {
                in_code_block = true;
                current_code.clear();
            } else if line.starts_with("[/CODE_EXAMPLE]") {
                in_code_block = false;
                if let Some(ref mut answer) = current_answer {
                    answer.code_examples.push(current_code.clone());
                }
                current_code.clear();
            } else if in_code_block {
                current_code.push_str(&line);
                current_code.push('\n');
            } else if let Some(a_pos) = line.find("A") {
                // Parse: [timestamp] A<id>: <text>
                let after_a = &line[a_pos + 1..];
                if let Some(colon_pos) = after_a.find(':') {
                    let id_str = &after_a[..colon_pos];
                    if let Ok(id) = id_str.trim().parse::<usize>() {
                        // Save previous answer if exists
                        if let Some(answer) = current_answer.take() {
                            answers.push(answer);
                        }

                        let text = after_a[colon_pos + 1..].trim().to_string();
                        current_answer = Some(Answer {
                            question_id: id,
                            text,
                            code_examples: Vec::new(),
                            timestamp: Utc::now(),
                        });
                    }
                }
            }
        }

        // Don't forget the last answer
        if let Some(answer) = current_answer {
            answers.push(answer);
        }

        Ok(answers)
    }

    fn learn_from_qa_pair(&mut self, qa_pair: &QAPair) -> Result<()> {
        // Add Q&A pair to knowledge base
        self.knowledge_base.add_qa_pair(qa_pair.clone());

        // Add topic
        self.knowledge_base
            .add_topic(qa_pair.question.category.clone());

        // Extract and add patterns
        let patterns = self.extract_patterns(qa_pair);
        for pattern in patterns {
            self.knowledge_base.add_pattern(pattern);
        }

        // Save knowledge base to disk
        self.save_knowledge_base()?;

        Ok(())
    }

    fn extract_patterns(&self, qa_pair: &QAPair) -> Vec<Pattern> {
        let mut patterns = Vec::new();
        let category = &qa_pair.question.category;

        // Create a pattern based on the Q&A pair
        let pattern_name = format!("{}_{}", category, qa_pair.question.id);

        // Extract key concepts from the answer
        let concepts = self.extract_concepts(&qa_pair.answer.text);

        // Create pattern from code examples
        for (i, code) in qa_pair.answer.code_examples.iter().enumerate() {
            let pattern = Pattern {
                name: format!("{}_code_{}", pattern_name, i),
                description: format!(
                    "Code pattern for {}: {}",
                    category,
                    concepts.join(", ")
                ),
                code_pattern: code.clone(),
                usage_examples: vec![qa_pair.answer.text.clone()],
                confidence: 1.0,
                occurrences: 1,
            };
            patterns.push(pattern);
        }

        // Create a concept pattern
        if !concepts.is_empty() {
            let pattern = Pattern {
                name: format!("{}_concept", pattern_name),
                description: format!(
                    "{}: {}",
                    category,
                    qa_pair.question.text
                ),
                code_pattern: String::new(),
                usage_examples: vec![qa_pair.answer.text.clone()],
                confidence: 1.0,
                occurrences: 1,
            };
            patterns.push(pattern);
        }

        patterns
    }

    fn extract_concepts(&self, text: &str) -> Vec<String> {
        let keywords = vec![
            "ownership",
            "borrowing",
            "lifetime",
            "trait",
            "generic",
            "error",
            "Result",
            "Option",
            "module",
            "crate",
            "Cargo",
            "test",
            "macro",
            "thread",
            "Arc",
            "Mutex",
            "Box",
            "Rc",
            "RefCell",
            "iterator",
            "closure",
            "match",
            "pattern",
        ];

        let text_lower = text.to_lowercase();
        keywords
            .into_iter()
            .filter(|&keyword| text_lower.contains(keyword))
            .map(|s| s.to_string())
            .collect()
    }

    fn save_knowledge_base(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.knowledge_base)?;
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.knowledge_file)?;

        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn get_statistics(&self) -> KnowledgeStatistics {
        let mut category_counts: HashMap<String, usize> = HashMap::new();

        for qa_pair in &self.knowledge_base.qa_pairs {
            *category_counts
                .entry(qa_pair.question.category.clone())
                .or_insert(0) += 1;
        }

        KnowledgeStatistics {
            total_qa_pairs: self.knowledge_base.qa_pairs.len(),
            total_patterns: self.knowledge_base.patterns.len(),
            topics_covered: self.knowledge_base.topics_covered.len(),
            category_distribution: category_counts,
            last_updated: self.knowledge_base.last_updated,
        }
    }
}

#[derive(Debug)]
pub struct KnowledgeStatistics {
    pub total_qa_pairs: usize,
    pub total_patterns: usize,
    pub topics_covered: usize,
    pub category_distribution: HashMap<String, usize>,
    pub last_updated: chrono::DateTime<Utc>,
}
