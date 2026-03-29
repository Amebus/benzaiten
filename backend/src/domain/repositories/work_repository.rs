use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::entities::work::Work;
use crate::application::errors::AppError;

/// Filtri per la ricerca di opere
#[derive(Default, Debug)]
pub struct WorkFilters {
    pub work_type: Option<String>,
    pub tags: Option<Vec<Uuid>>,
    pub search_term: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

/// Repository per la gestione delle opere
#[async_trait]
pub trait WorkRepository: Send + Sync {
    async fn create(&self, work: &Work) -> Result<Work, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Work>, AppError>;
    async fn find_all(&self, filters: WorkFilters) -> Result<Vec<Work>, AppError>;
    async fn update(&self, work: &Work) -> Result<Work, AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
    async fn add_tag(&self, work_id: Uuid, tag_id: Uuid) -> Result<(), AppError>;
    async fn remove_tag(&self, work_id: Uuid, tag_id: Uuid) -> Result<(), AppError>;
    async fn add_person(&self, work_id: Uuid, person_id: Uuid, role: &str) -> Result<(), AppError>;
    async fn remove_person(&self, work_id: Uuid, person_id: Uuid, role: &str) -> Result<(), AppError>;
}
