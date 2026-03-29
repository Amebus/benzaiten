use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::entities::tag::Tag;
use crate::application::errors::AppError;

/// Repository per la gestione dei tag
#[async_trait]
pub trait TagRepository: Send + Sync {
    async fn create(&self, tag: &Tag) -> Result<Tag, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tag>, AppError>;
    async fn find_all(&self) -> Result<Vec<Tag>, AppError>;
    async fn find_by_work_id(&self, work_id: Uuid) -> Result<Vec<Tag>, AppError>;
    async fn update(&self, tag: &Tag) -> Result<Tag, AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}
