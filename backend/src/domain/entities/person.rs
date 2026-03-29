use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Persona coinvolta in un'opera (autore, regista, ecc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: Uuid,
    pub name: String,
    pub original_name: Option<String>,
    pub country_code: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub metadata: Option<Value>,
    pub created_at: DateTime<Utc>,
}

impl Person {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            original_name: None,
            country_code: None,
            birth_date: None,
            metadata: None,
            created_at: Utc::now(),
        }
    }
}

/// Associazione tra opera e persona con ruolo specifico
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkPerson {
    pub work_id: Uuid,
    pub person_id: Uuid,
    pub role: String,
    pub person: Option<Person>,
}
