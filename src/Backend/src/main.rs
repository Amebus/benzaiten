use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok", "service": "backend" }))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health));

    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    println!("🚀 Backend running on http://localhost:{}", port);
    axum::serve(listener, app).await.unwrap();
}
