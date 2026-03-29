use std::sync::Arc;
use uuid::Uuid;
use crate::application::dto::work_dto::{
    CreateWorkRequest, UpdateWorkRequest, WorkResponse, WorkListQuery, AddTagRequest, AddPersonRequest,
};
use crate::application::errors::AppError;
use crate::domain::entities::work::Work;
use crate::domain::repositories::work_repository::{WorkRepository, WorkFilters};
use crate::domain::repositories::tag_repository::TagRepository;

/// Servizio per la gestione delle opere
pub struct WorkService {
    work_repo: Arc<dyn WorkRepository>,
    tag_repo: Arc<dyn TagRepository>,
}

impl WorkService {
    pub fn new(work_repo: Arc<dyn WorkRepository>, tag_repo: Arc<dyn TagRepository>) -> Self {
        Self { work_repo, tag_repo }
    }

    pub async fn create_work(&self, req: CreateWorkRequest) -> Result<WorkResponse, AppError> {
        if req.title.trim().is_empty() {
            return Err(AppError::ValidationError("Il titolo non può essere vuoto".to_string()));
        }

        let mut work = Work::new(req.work_type, req.title);
        work.original_title = req.original_title;
        work.synopsis = req.synopsis;
        work.year = req.year;
        work.metadata = req.metadata;

        let created = self.work_repo.create(&work).await?;
        Ok(work_to_response(created))
    }

    pub async fn get_work(&self, id: Uuid) -> Result<WorkResponse, AppError> {
        self.work_repo
            .find_by_id(id)
            .await?
            .map(work_to_response)
            .ok_or_else(|| AppError::NotFound(format!("Opera {} non trovata", id)))
    }

    pub async fn list_works(&self, query: WorkListQuery) -> Result<Vec<WorkResponse>, AppError> {
        let filters = WorkFilters {
            work_type: query.work_type,
            search_term: query.search,
            page: query.page,
            page_size: query.page_size,
            ..Default::default()
        };

        let works = self.work_repo.find_all(filters).await?;
        Ok(works.into_iter().map(work_to_response).collect())
    }

    pub async fn update_work(&self, id: Uuid, req: UpdateWorkRequest) -> Result<WorkResponse, AppError> {
        let mut work = self
            .work_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Opera {} non trovata", id)))?;

        if let Some(title) = req.title {
            if title.trim().is_empty() {
                return Err(AppError::ValidationError("Il titolo non può essere vuoto".to_string()));
            }
            work.title = title;
        }
        if let Some(original_title) = req.original_title {
            work.original_title = Some(original_title);
        }
        if let Some(synopsis) = req.synopsis {
            work.synopsis = Some(synopsis);
        }
        if let Some(year) = req.year {
            work.year = Some(year);
        }
        if let Some(metadata) = req.metadata {
            work.metadata = Some(metadata);
        }

        let updated = self.work_repo.update(&work).await?;
        Ok(work_to_response(updated))
    }

    pub async fn delete_work(&self, id: Uuid) -> Result<(), AppError> {
        self.work_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Opera {} non trovata", id)))?;

        self.work_repo.delete(id).await
    }

    pub async fn add_tag(&self, work_id: Uuid, req: AddTagRequest) -> Result<(), AppError> {
        // Verifica che l'opera esista
        self.work_repo
            .find_by_id(work_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Opera {} non trovata", work_id)))?;

        // Verifica che il tag esista
        self.tag_repo
            .find_by_id(req.tag_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Tag {} non trovato", req.tag_id)))?;

        self.work_repo.add_tag(work_id, req.tag_id).await
    }

    pub async fn remove_tag(&self, work_id: Uuid, tag_id: Uuid) -> Result<(), AppError> {
        self.work_repo.remove_tag(work_id, tag_id).await
    }

    pub async fn add_person(&self, work_id: Uuid, req: AddPersonRequest) -> Result<(), AppError> {
        // Verifica che l'opera esista
        self.work_repo
            .find_by_id(work_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Opera {} non trovata", work_id)))?;

        if req.role.trim().is_empty() {
            return Err(AppError::ValidationError("Il ruolo non può essere vuoto".to_string()));
        }

        self.work_repo.add_person(work_id, req.person_id, &req.role).await
    }

    pub async fn remove_person(&self, work_id: Uuid, person_id: Uuid, role: &str) -> Result<(), AppError> {
        self.work_repo.remove_person(work_id, person_id, role).await
    }
}

fn work_to_response(work: Work) -> WorkResponse {
    WorkResponse {
        id: work.id,
        work_type: work.work_type,
        title: work.title,
        original_title: work.original_title,
        synopsis: work.synopsis,
        year: work.year,
        metadata: work.metadata,
        created_at: work.created_at.to_rfc3339(),
        updated_at: work.updated_at.to_rfc3339(),
    }
}
