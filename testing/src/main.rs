mod routes;

use axum::{
    Router,
    routing::{get, post},
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing for logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "multi_threaded_webserver=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Build the application router
    let app = Router::new()
        .route("/", get(routes::index))
        .route("/health", get(routes::health_check))
        .route("/echo", post(routes::echo))
        .route("/users/:id", get(routes::get_user))
        .route("/cpu-intensive", get(routes::cpu_intensive_task))
        .layer(
            ServiceBuilder::new()
                .layer(tower::limit::ConcurrencyLimitLayer::new(100))
        );

    // Set the address to bind to
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    tracing::info!("Starting multi-threaded web server on {}", addr);
    tracing::info!("Available routes:");
    tracing::info!("  GET  /");
    tracing::info!("  GET  /health");
    tracing::info!("  POST /echo");
    tracing::info!("  GET  /users/:id");
    tracing::info!("  GET  /cpu-intensive");

    // Create the TCP listener
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    // Start serving with multi-threaded runtime
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
