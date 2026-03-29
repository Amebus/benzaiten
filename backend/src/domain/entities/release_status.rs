use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::value_objects::media_status::MediaStatus;

/// Stato di rilascio di un'opera per un determinato paese
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseStatus {
    pub id: Uuid,
    pub work_id: Uuid,
    pub country_code: String,
    pub status: MediaStatus,
    pub started_at: Option<NaiveDate>,
    pub completed_at: Option<NaiveDate>,
}
