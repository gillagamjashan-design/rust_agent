use crate::claude_proxy::ClaudeProxy;
use crate::types::{Answer, Question};
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
    topics: Vec<&'static str>,
    current_topic_index: usize,
}

impl ProxyQuestionAgent {
    pub fn new(output_path: PathBuf) -> Self {
        Self {
            claude: ClaudeProxy::new(),
            question_count: 0,
            output_file: output_path,
            topics: vec![
                "Linux", "Git", "GitHub_CLI", "Bash",
                "Networking", "Docker", "System", "Packages",
            ],
            current_topic_index: 0,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        println!("🤔 Question Agent (CLIProxyAPI) - using Claude Max");

        loop {
            let topic = self.topics[self.current_topic_index];
            self.current_topic_index = (self.current_topic_index + 1) % self.topics.len();

            match self.claude.generate_question(topic).await {
                Ok(text) => {
                    self.question_count += 1;
                    let question = Question {
                        id: self.question_count,
                        text,
                        category: topic.to_string(),
                        timestamp: Utc::now(),
                    };

                    let mut file = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&self.output_file)?;

                    writeln!(file, "[{}] Q{}: {} [{}]",
                        question.timestamp.format("%Y-%m-%d %H:%M:%S"),
                        question.id, question.text, question.category)?;

                    println!("❓ Q{}: {}", question.id, question.text);
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
