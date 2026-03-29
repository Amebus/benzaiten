use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Richiesta di creazione di una nuova persona
#[derive(Debug, Deserialize)]
pub struct CreatePersonRequest {
    pub name: String,
    pub original_name: Option<String>,
    pub country_code: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub metadata: Option<Value>,
}

/// Richiesta di aggiornamento di una persona esistente
#[derive(Debug, Deserialize)]
pub struct UpdatePersonRequest {
    pub name: Option<String>,
    pub original_name: Option<String>,
    pub country_code: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub metadata: Option<Value>,
}

/// Risposta con i dati di una persona
#[derive(Debug, Serialize)]
pub struct PersonResponse {
    pub id: Uuid,
    pub name: String,
    pub original_name: Option<String>,
    pub country_code: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub created_at: String,
}

/// Risposta con i dati di una persona e il suo ruolo nell'opera
#[derive(Debug, Serialize)]
pub struct WorkPersonResponse {
    pub work_id: Uuid,
    pub person_id: Uuid,
    pub role: String,
    pub name: String,
    pub original_name: Option<String>,
    pub country_code: Option<String>,
}
