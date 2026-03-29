use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Tag per categorizzare le opere
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Tag {
    pub fn new(name: String, slug: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            slug,
            description: None,
            color: None,
            created_at: Utc::now(),
        }
    }
}
