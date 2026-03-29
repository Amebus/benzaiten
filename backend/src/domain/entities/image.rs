use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Immagine associata ad un'opera (copertina, screenshot, ecc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub id: Uuid,
    pub work_id: Uuid,
    pub s3_key: String,
    pub kind: String,
    pub display_order: Option<i32>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub created_at: DateTime<Utc>,
}
