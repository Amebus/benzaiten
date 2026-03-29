use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// Errori dell'applicazione
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Non trovato: {0}")]
    NotFound(String),

    #[error("Conflitto: {0}")]
    Conflict(String),

    #[error("Errore di validazione: {0}")]
    ValidationError(String),

    #[error("Non autorizzato")]
    Unauthorized,

    #[error("Accesso negato")]
    Forbidden,

    #[error("Errore database: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Errore interno: {0}")]
    InternalError(String),
}

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        AppError::InternalError(e.to_string())
    }
}

/// Conversione degli errori in risposte HTTP per Axum
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            AppError::ValidationError(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Non autorizzato".to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "Accesso negato".to_string()),
            AppError::DatabaseError(e) => {
                tracing::error!("Errore database: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Errore interno del server".to_string())
            }
            AppError::InternalError(msg) => {
                tracing::error!("Errore interno: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Errore interno del server".to_string())
            }
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
