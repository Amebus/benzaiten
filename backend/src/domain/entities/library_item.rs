use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Elemento della libreria personale dell'utente
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryItem {
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl LibraryItem {
    pub fn new(user_id: String, work_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            work_id,
            owned_volumes: None,
            current_episode: Some(0),
            total_episodes: None,
            purchase_price: None,
            variant_notes: None,
            personal_rating: None,
            notes: None,
            created_at: now,
            updated_at: now,
        }
    }
}
