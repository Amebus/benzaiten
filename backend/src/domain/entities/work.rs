use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use crate::domain::value_objects::work_type::WorkType;

/// Entità principale che rappresenta un'opera (manga, anime, film, musica)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Work {
    pub id: Uuid,
    pub work_type: WorkType,
    pub title: String,
    pub original_title: Option<String>,
    pub synopsis: Option<String>,
    pub year: Option<i32>,
    pub metadata: Option<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Work {
    pub fn new(work_type: WorkType, title: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            work_type,
            title,
            original_title: None,
            synopsis: None,
            year: None,
            metadata: None,
            created_at: now,
            updated_at: now,
        }
    }
}
