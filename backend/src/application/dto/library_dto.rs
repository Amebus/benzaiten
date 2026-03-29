use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Richiesta di aggiunta di un elemento alla libreria
#[derive(Debug, Deserialize)]
pub struct CreateLibraryItemRequest {
    pub work_id: Uuid,
    pub owned_volumes: Option<Vec<String>>,
    pub current_episode: Option<i32>,
    pub total_episodes: Option<i32>,
    pub purchase_price: Option<f64>,
    pub variant_notes: Option<String>,
    pub personal_rating: Option<i32>,
    pub notes: Option<String>,
}

/// Richiesta di aggiornamento di un elemento della libreria
#[derive(Debug, Deserialize)]
pub struct UpdateLibraryItemRequest {
    pub owned_volumes: Option<Vec<String>>,
    pub current_episode: Option<i32>,
    pub total_episodes: Option<i32>,
    pub purchase_price: Option<f64>,
    pub variant_notes: Option<String>,
    pub personal_rating: Option<i32>,
    pub notes: Option<String>,
}

/// Risposta con i dati di un elemento della libreria
#[derive(Debug, Serialize)]
pub struct LibraryItemResponse {
    pub id: Uuid,
    pub user_id: String,
    pub work_id: Uuid,
    pub owned_volumes: Option<Vec<String>>,
    pub current_episode: Option<i32>,
    pub total_episodes: Option<i32>,
    pub purchase_price: Option<f64>,
    pub variant_notes: Option<String>,
    pub personal_rating: Option<i32>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
