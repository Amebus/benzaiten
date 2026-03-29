use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::entities::library_item::LibraryItem;
use crate::application::errors::AppError;

/// Repository per la gestione della libreria personale
#[async_trait]
pub trait LibraryRepository: Send + Sync {
    async fn create(&self, item: &LibraryItem) -> Result<LibraryItem, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<LibraryItem>, AppError>;
    async fn find_by_user_id(&self, user_id: &str) -> Result<Vec<LibraryItem>, AppError>;
    async fn find_by_user_and_work(&self, user_id: &str, work_id: Uuid) -> Result<Option<LibraryItem>, AppError>;
    async fn update(&self, item: &LibraryItem) -> Result<LibraryItem, AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}
