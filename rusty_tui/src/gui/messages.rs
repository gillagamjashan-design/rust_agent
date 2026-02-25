use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub enum UserCommand {
    Query(String),
    Command(String),
    Quit,
}

#[derive(Debug, Clone)]
pub enum WorkerMessage {
    Response(String),
    SystemMessage(String),
    Error(String),
    Stats(String),
}

#[derive(Debug, Clone)]
pub enum Role {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
    pub timestamp: DateTime<Local>,
}

impl Message {
    pub fn new(role: Role, content: String) -> Self {
        Self {
            role,
            content,
            timestamp: Local::now(),
        }
    }
}
