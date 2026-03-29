use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use crate::domain::value_objects::work_type::WorkType;

/// Richiesta di creazione di una nuova opera
#[derive(Debug, Deserialize)]
pub struct CreateWorkRequest {
    pub work_type: WorkType,
    pub title: String,
    pub original_title: Option<String>,
    pub synopsis: Option<String>,
    pub year: Option<i32>,
    pub metadata: Option<Value>,
}

/// Richiesta di aggiornamento di un'opera esistente
#[derive(Debug, Deserialize)]
pub struct UpdateWorkRequest {
    pub title: Option<String>,
    pub original_title: Option<String>,
    pub synopsis: Option<String>,
    pub year: Option<i32>,
    pub metadata: Option<Value>,
}

/// Risposta con i dati di un'opera
#[derive(Debug, Serialize)]
pub struct WorkResponse {
    pub id: Uuid,
    pub work_type: WorkType,
    pub title: String,
    pub original_title: Option<String>,
    pub synopsis: Option<String>,
    pub year: Option<i32>,
    pub metadata: Option<Value>,
    pub created_at: String,
    pub updated_at: String,
}

/// Parametri di query per la lista delle opere
#[derive(Debug, Deserialize, Default)]
pub struct WorkListQuery {
    pub work_type: Option<String>,
    pub search: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

/// Richiesta per aggiungere un tag a un'opera
#[derive(Debug, Deserialize)]
pub struct AddTagRequest {
    pub tag_id: Uuid,
}

/// Richiesta per aggiungere una persona a un'opera
#[derive(Debug, Deserialize)]
pub struct AddPersonRequest {
    pub person_id: Uuid,
    pub role: String,
}
