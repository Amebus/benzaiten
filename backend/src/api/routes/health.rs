use axum::{Json, response::IntoResponse};
use serde_json::json;

/// Handler per il controllo dello stato del servizio
pub async fn health_check() -> impl IntoResponse {
    Json(json!({ "status": "ok", "service": "benzaiten-backend" }))
}
