use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

// CLIProxyAPI client for Claude Max subscription
pub struct ClaudeProxy {
    pub(crate) client: Client,
    pub(crate) base_url: String,
}

#[derive(Debug, Serialize)]
pub struct ProxyRequest {
    pub model: String,
    pub max_tokens: u32,
    pub messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ProxyResponse {
    pub content: Vec<ContentBlock>,
}

#[derive(Debug, Deserialize)]
pub struct ContentBlock {
    pub text: String,
}

impl ClaudeProxy {
    pub fn new() -> Self {
        // CLIProxyAPI runs on localhost:8317 by default
        Self {
            client: Client::new(),
            base_url: "http://localhost:8317".to_string(),
        }
    }

    pub async fn generate_question(&self, topic: &str) -> Result<String> {
        let topic_display = topic.replace('_', " ");
        let prompt = format!(
            "Generate ONE practical Rust programming question about {}. \
            The question MUST be about Rust programming language ONLY. \
            Focus on real-world Rust coding scenarios, best practices, and common patterns. \
            Just return the question text, no formatting or markdown.",
            topic_display
        );

        self.send_request(prompt).await
    }

    pub async fn generate_answer(&self, question: &str) -> Result<String> {
        let prompt = format!(
            "Answer this Rust programming question with a detailed explanation and Rust code example:\n\n\
            {}\n\n\
            IMPORTANT: Your answer MUST include Rust code examples ONLY. No other programming languages.\n\
            Format:\n\
            [Explanation]\n\n\
            [CODE_EXAMPLE]\n\
            ```rust\n\
            // Rust code here\n\
            ```\n\
            [/CODE_EXAMPLE]",
            question
        );

        self.send_request(prompt).await
    }

    pub async fn send_request(&self, prompt: String) -> Result<String> {
        let request = ProxyRequest {
            model: "claude-sonnet-4-5-20250929".to_string(),
            max_tokens: 2048,
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
        };

        let response = self
            .client
            .post(format!("{}/v1/messages", self.base_url))
            .header("Authorization", "Bearer rust-agent-key-123")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("CLIProxyAPI not running on localhost:8317"));
        }

        let proxy_response: ProxyResponse = response.json().await?;

        Ok(proxy_response
            .content
            .first()
            .map(|c| c.text.clone())
            .unwrap_or_default())
    }
}
