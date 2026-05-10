use axum::{routing::get, Router};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/signals", get(get_signals))
        .route("/api/health", get(health))
        .fallback_service(ServeDir::new("src/public"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await.unwrap();

    println!("Running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn get_signals() -> &'static str {
    "Stock Scanner API"
}

async fn health() -> &'static str {
    "OK"
}