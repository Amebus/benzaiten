use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use serde::{Deserialize, Serialize};

/// Utente autenticato estratto dall'header Authorization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub user_id: String,
    pub username: Option<String>,
    pub email: Option<String>,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "));

        match auth_header {
            Some(token) => {
                // In produzione: decodificare il JWT e estrarre le claims
                // Per ora usiamo il token stesso come user_id se non è vuoto
                if token.is_empty() {
                    return Err(StatusCode::UNAUTHORIZED);
                }
                Ok(AuthUser {
                    user_id: token.to_string(),
                    username: None,
                    email: None,
                })
            }
            None => Err(StatusCode::UNAUTHORIZED),
        }
    }
}
