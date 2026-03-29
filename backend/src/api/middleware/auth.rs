use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
    Json,
};
use serde_json::json;

/// Middleware per richiedere autenticazione
pub async fn require_auth(
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "));

    match auth_header {
        Some(_token) => {
            // In produzione: validare il token con Keycloak
            Ok(next.run(request).await)
        }
        None => Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Header Authorization mancante o non valido" })),
        )),
    }
}
