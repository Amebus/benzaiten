use std::sync::Arc;
use uuid::Uuid;
use crate::application::dto::library_dto::{
    CreateLibraryItemRequest, UpdateLibraryItemRequest, LibraryItemResponse,
};
use crate::application::errors::AppError;
use crate::domain::entities::library_item::LibraryItem;
use crate::domain::repositories::library_repository::LibraryRepository;

/// Servizio per la gestione della libreria personale
pub struct LibraryService {
    library_repo: Arc<dyn LibraryRepository>,
}

impl LibraryService {
    pub fn new(library_repo: Arc<dyn LibraryRepository>) -> Self {
        Self { library_repo }
    }

    pub async fn create_item(
        &self,
        user_id: &str,
        req: CreateLibraryItemRequest,
    ) -> Result<LibraryItemResponse, AppError> {
        // Verifica che non esista già un elemento per quest'opera nella libreria dell'utente
        if let Some(_) = self
            .library_repo
            .find_by_user_and_work(user_id, req.work_id)
            .await?
        {
            return Err(AppError::Conflict(
                "Quest'opera è già presente nella tua libreria".to_string(),
            ));
        }

        if let Some(rating) = req.personal_rating {
            if rating < 1 || rating > 10 {
                return Err(AppError::ValidationError(
                    "Il voto personale deve essere compreso tra 1 e 10".to_string(),
                ));
            }
        }

        let mut item = LibraryItem::new(user_id.to_string(), req.work_id);
        item.owned_volumes = req.owned_volumes;
        item.current_episode = req.current_episode.or(Some(0));
        item.total_episodes = req.total_episodes;
        item.purchase_price = req.purchase_price;
        item.variant_notes = req.variant_notes;
        item.personal_rating = req.personal_rating;
        item.notes = req.notes;

        let created = self.library_repo.create(&item).await?;
        Ok(item_to_response(created))
    }

    pub async fn get_item(&self, id: Uuid, user_id: &str) -> Result<LibraryItemResponse, AppError> {
        let item = self
            .library_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Elemento libreria {} non trovato", id)))?;

        // Verifica che l'elemento appartenga all'utente
        if item.user_id != user_id {
            return Err(AppError::Forbidden);
        }

        Ok(item_to_response(item))
    }

    pub async fn list_user_items(&self, user_id: &str) -> Result<Vec<LibraryItemResponse>, AppError> {
        let items = self.library_repo.find_by_user_id(user_id).await?;
        Ok(items.into_iter().map(item_to_response).collect())
    }

    pub async fn update_item(
        &self,
        id: Uuid,
        user_id: &str,
        req: UpdateLibraryItemRequest,
    ) -> Result<LibraryItemResponse, AppError> {
        let mut item = self
            .library_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Elemento libreria {} non trovato", id)))?;

        if item.user_id != user_id {
            return Err(AppError::Forbidden);
        }

        if let Some(rating) = req.personal_rating {
            if rating < 1 || rating > 10 {
                return Err(AppError::ValidationError(
                    "Il voto personale deve essere compreso tra 1 e 10".to_string(),
                ));
            }
        }

        if let Some(owned_volumes) = req.owned_volumes {
            item.owned_volumes = Some(owned_volumes);
        }
        if let Some(current_episode) = req.current_episode {
            item.current_episode = Some(current_episode);
        }
        if let Some(total_episodes) = req.total_episodes {
            item.total_episodes = Some(total_episodes);
        }
        if let Some(purchase_price) = req.purchase_price {
            item.purchase_price = Some(purchase_price);
        }
        if let Some(variant_notes) = req.variant_notes {
            item.variant_notes = Some(variant_notes);
        }
        if let Some(personal_rating) = req.personal_rating {
            item.personal_rating = Some(personal_rating);
        }
        if let Some(notes) = req.notes {
            item.notes = Some(notes);
        }

        let updated = self.library_repo.update(&item).await?;
        Ok(item_to_response(updated))
    }

    pub async fn delete_item(&self, id: Uuid, user_id: &str) -> Result<(), AppError> {
        let item = self
            .library_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Elemento libreria {} non trovato", id)))?;

        if item.user_id != user_id {
            return Err(AppError::Forbidden);
        }

        self.library_repo.delete(id).await
    }
}

fn item_to_response(item: LibraryItem) -> LibraryItemResponse {
    LibraryItemResponse {
        id: item.id,
        user_id: item.user_id,
        work_id: item.work_id,
        owned_volumes: item.owned_volumes,
        current_episode: item.current_episode,
        total_episodes: item.total_episodes,
        purchase_price: item.purchase_price,
        variant_notes: item.variant_notes,
        personal_rating: item.personal_rating,
        notes: item.notes,
        created_at: item.created_at.to_rfc3339(),
        updated_at: item.updated_at.to_rfc3339(),
    }
}
