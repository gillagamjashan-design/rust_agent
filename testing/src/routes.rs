use axum::{
    extract::{Path, Json},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Root endpoint
pub async fn index() -> &'static str {
    "Welcome to the Multi-threaded Web Server! 🚀"
}

/// Health check endpoint
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

/// Echo endpoint - returns what you send
#[derive(Deserialize, Serialize)]
pub struct EchoRequest {
    message: String,
}

#[derive(Serialize)]
pub struct EchoResponse {
    echo: String,
    length: usize,
}

pub async fn echo(Json(payload): Json<EchoRequest>) -> impl IntoResponse {
    let response = EchoResponse {
        length: payload.message.len(),
        echo: payload.message,
    };
    
    (StatusCode::OK, Json(response))
}

/// Get user by ID
#[derive(Serialize)]
pub struct User {
    id: u32,
    name: String,
    email: String,
}

pub async fn get_user(Path(id): Path<u32>) -> Response {
    // Simulate database lookup
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let user = User {
        id,
        name: format!("User {}", id),
        email: format!("user{}@example.com", id),
    };
    
    (StatusCode::OK, Json(user)).into_response()
}

/// CPU-intensive task to demonstrate multi-threading
pub async fn cpu_intensive_task() -> impl IntoResponse {
    // Spawn blocking task to avoid blocking async runtime
    let result = tokio::task::spawn_blocking(|| {
        // Simulate CPU-intensive work
        let mut sum = 0u64;
        for i in 0..10_000_000 {
            sum = sum.wrapping_add(i);
        }
        sum
    })
    .await
    .expect("Task panicked");

    (StatusCode::OK, Json(serde_json::json!({
        "message": "CPU-intensive task completed",
        "result": result,
        "note": "This ran on a separate thread pool to avoid blocking async tasks"
    })))
}
