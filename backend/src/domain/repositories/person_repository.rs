use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::entities::person::{Person, WorkPerson};
use crate::application::errors::AppError;

/// Repository per la gestione delle persone
#[async_trait]
pub trait PersonRepository: Send + Sync {
    async fn create(&self, person: &Person) -> Result<Person, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Person>, AppError>;
    async fn find_all(&self) -> Result<Vec<Person>, AppError>;
    async fn find_by_work_id(&self, work_id: Uuid) -> Result<Vec<WorkPerson>, AppError>;
    async fn update(&self, person: &Person) -> Result<Person, AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}
