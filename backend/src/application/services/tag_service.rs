use std::sync::Arc;
use uuid::Uuid;
use crate::application::dto::tag_dto::{CreateTagRequest, UpdateTagRequest, TagResponse};
use crate::application::errors::AppError;
use crate::domain::entities::tag::Tag;
use crate::domain::repositories::tag_repository::TagRepository;

/// Servizio per la gestione dei tag
pub struct TagService {
    tag_repo: Arc<dyn TagRepository>,
}

impl TagService {
    pub fn new(tag_repo: Arc<dyn TagRepository>) -> Self {
        Self { tag_repo }
    }

    pub async fn create_tag(&self, req: CreateTagRequest) -> Result<TagResponse, AppError> {
        if req.name.trim().is_empty() {
            return Err(AppError::ValidationError("Il nome del tag non può essere vuoto".to_string()));
        }

        // Genera lo slug dal nome se non fornito
        let slug = req.slug.unwrap_or_else(|| slugify(&req.name));

        let mut tag = Tag::new(req.name, slug);
        tag.description = req.description;
        tag.color = req.color;

        let created = self.tag_repo.create(&tag).await?;
        Ok(tag_to_response(created))
    }

    pub async fn get_tag(&self, id: Uuid) -> Result<TagResponse, AppError> {
        self.tag_repo
            .find_by_id(id)
            .await?
            .map(tag_to_response)
            .ok_or_else(|| AppError::NotFound(format!("Tag {} non trovato", id)))
    }

    pub async fn list_tags(&self) -> Result<Vec<TagResponse>, AppError> {
        let tags = self.tag_repo.find_all().await?;
        Ok(tags.into_iter().map(tag_to_response).collect())
    }

    pub async fn update_tag(&self, id: Uuid, req: UpdateTagRequest) -> Result<TagResponse, AppError> {
        let mut tag = self
            .tag_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Tag {} non trovato", id)))?;

        if let Some(name) = req.name {
            if name.trim().is_empty() {
                return Err(AppError::ValidationError("Il nome del tag non può essere vuoto".to_string()));
            }
            tag.name = name;
        }
        if let Some(description) = req.description {
            tag.description = Some(description);
        }
        if let Some(color) = req.color {
            tag.color = Some(color);
        }

        let updated = self.tag_repo.update(&tag).await?;
        Ok(tag_to_response(updated))
    }

    pub async fn delete_tag(&self, id: Uuid) -> Result<(), AppError> {
        self.tag_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Tag {} non trovato", id)))?;

        self.tag_repo.delete(id).await
    }
}

/// Converte un nome in uno slug URL-friendly
fn slugify(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn tag_to_response(tag: Tag) -> TagResponse {
    TagResponse {
        id: tag.id,
        name: tag.name,
        slug: tag.slug,
        description: tag.description,
        color: tag.color,
        created_at: tag.created_at.to_rfc3339(),
    }
}
