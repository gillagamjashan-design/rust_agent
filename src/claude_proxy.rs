use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

// CLIProxyAPI client for Claude Max subscription
pub struct ClaudeProxy {
    client: Client,
    base_url: String,
}

#[derive(Debug, Serialize)]
struct ProxyRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ProxyResponse {
    content: Vec<ContentBlock>,
}

#[derive(Debug, Deserialize)]
struct ContentBlock {
    text: String,
}

impl ClaudeProxy {
    pub fn new() -> Self {
        // CLIProxyAPI runs on localhost:8000 by default
        Self {
            client: Client::new(),
            base_url: "http://localhost:8000".to_string(),
        }
    }

    pub async fn generate_question(&self, topic: &str) -> Result<String> {
        let prompt = format!(
            "Generate ONE practical programming question about {}. \
            Topics: Linux, git, gh CLI, bash, networking, Docker. \
            Just return the question text, no formatting.",
            topic
        );

        self.send_request(prompt).await
    }

    pub async fn generate_answer(&self, question: &str) -> Result<String> {
        let prompt = format!(
            "Answer this question with a detailed explanation and code example:\n\n\
            {}\n\n\
            Format:\n\
            [Explanation]\n\n\
            [CODE_EXAMPLE_1]\n\
            [code here]\n\
            [/CODE_EXAMPLE]",
            question
        );

        self.send_request(prompt).await
    }

    async fn send_request(&self, prompt: String) -> Result<String> {
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
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("CLIProxyAPI not running on localhost:8000"));
        }

        let proxy_response: ProxyResponse = response.json().await?;

        Ok(proxy_response
            .content
            .first()
            .map(|c| c.text.clone())
            .unwrap_or_default())
    }
}
