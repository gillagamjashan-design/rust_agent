use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub struct PendingFileCreation {
    pub operations: Vec<FileOperationRequest>,
}

#[derive(Debug, Clone)]
pub struct FileOperationRequest {
    pub path: String,
    pub content: String,
    pub operation_type: String, // "create" or "modify"
}

#[derive(Debug, Clone)]
pub enum UserCommand {
    Query(String),
    Command(String),
    Quit,
    // User's response to confirmation request
    ConfirmFileCreation { approved: bool, operations: Vec<FileOperationRequest> },
}

#[derive(Debug, Clone)]
pub enum WorkerMessage {
    Response(String),
    SystemMessage(String),
    Error(String),
    Stats(String),
    FileCreated { path: String, success: bool, message: String },
    FileModified { path: String, success: bool, message: String },
    FileOperationError { path: String, error: String },
    // Request user confirmation for file operations
    RequestFileConfirmation(PendingFileCreation),
    FilesCreated(Vec<FileCreationInfo>),
}

#[derive(Debug, Clone)]
pub struct FileCreationInfo {
    pub path: String,
    pub appended: bool,
    pub success: bool,
}

#[derive(Debug, Clone)]
pub enum Role {
    User,
    Assistant,
    System,
    FileOperation,
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
