use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::application::errors::AppError;
use crate::domain::entities::library_item::LibraryItem;
use crate::domain::repositories::library_repository::LibraryRepository;

/// Implementazione PostgreSQL del repository per la libreria personale
pub struct PostgresLibraryRepository {
    pool: PgPool,
}

impl PostgresLibraryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn row_to_library_item(row: &sqlx::postgres::PgRow) -> Result<LibraryItem, AppError> {
    let purchase_price: Option<f64> =
        row.try_get("purchase_price").map_err(|e| AppError::DatabaseError(e.into()))?;

    Ok(LibraryItem {
        id: row.try_get("id").map_err(|e| AppError::DatabaseError(e.into()))?,
        user_id: row.try_get("user_id").map_err(|e| AppError::DatabaseError(e.into()))?,
        work_id: row.try_get("work_id").map_err(|e| AppError::DatabaseError(e.into()))?,
        owned_volumes: row.try_get("owned_volumes").map_err(|e| AppError::DatabaseError(e.into()))?,
        current_episode: row.try_get("current_episode").map_err(|e| AppError::DatabaseError(e.into()))?,
        total_episodes: row.try_get("total_episodes").map_err(|e| AppError::DatabaseError(e.into()))?,
        purchase_price,
        variant_notes: row.try_get("variant_notes").map_err(|e| AppError::DatabaseError(e.into()))?,
        personal_rating: row.try_get("personal_rating").map_err(|e| AppError::DatabaseError(e.into()))?,
        notes: row.try_get("notes").map_err(|e| AppError::DatabaseError(e.into()))?,
        created_at: row.try_get("created_at").map_err(|e| AppError::DatabaseError(e.into()))?,
        updated_at: row.try_get("updated_at").map_err(|e| AppError::DatabaseError(e.into()))?,
    })
}

#[async_trait]
impl LibraryRepository for PostgresLibraryRepository {
    async fn create(&self, item: &LibraryItem) -> Result<LibraryItem, AppError> {
        let row = sqlx::query(
            r#"
            INSERT INTO user_library_items
                (id, user_id, work_id, owned_volumes, current_episode, total_episodes,
                 purchase_price, variant_notes, personal_rating, notes, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING id, user_id, work_id, owned_volumes, current_episode, total_episodes,
                      CAST(purchase_price AS FLOAT8) AS purchase_price,
                      variant_notes, personal_rating, notes, created_at, updated_at
            "#,
        )
        .bind(item.id)
        .bind(&item.user_id)
        .bind(item.work_id)
        .bind(&item.owned_volumes)
        .bind(item.current_episode)
        .bind(item.total_episodes)
        .bind(item.purchase_price)
        .bind(&item.variant_notes)
        .bind(item.personal_rating)
        .bind(&item.notes)
        .bind(item.created_at)
        .bind(item.updated_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("unique") || e.to_string().contains("duplicate") {
                AppError::Conflict("Quest'opera è già presente nella libreria".to_string())
            } else {
                AppError::DatabaseError(e)
            }
        })?;

        row_to_library_item(&row)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<LibraryItem>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, user_id, work_id, owned_volumes, current_episode, total_episodes,
                   CAST(purchase_price AS FLOAT8) AS purchase_price,
                   variant_notes, personal_rating, notes, created_at, updated_at
            FROM user_library_items WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        row.map(|r| row_to_library_item(&r)).transpose()
    }

    async fn find_by_user_id(&self, user_id: &str) -> Result<Vec<LibraryItem>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, work_id, owned_volumes, current_episode, total_episodes,
                   CAST(purchase_price AS FLOAT8) AS purchase_price,
                   variant_notes, personal_rating, notes, created_at, updated_at
            FROM user_library_items
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        rows.iter().map(row_to_library_item).collect()
    }

    async fn find_by_user_and_work(
        &self,
        user_id: &str,
        work_id: Uuid,
    ) -> Result<Option<LibraryItem>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, user_id, work_id, owned_volumes, current_episode, total_episodes,
                   CAST(purchase_price AS FLOAT8) AS purchase_price,
                   variant_notes, personal_rating, notes, created_at, updated_at
            FROM user_library_items
            WHERE user_id = $1 AND work_id = $2
            "#,
        )
        .bind(user_id)
        .bind(work_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        row.map(|r| row_to_library_item(&r)).transpose()
    }

    async fn update(&self, item: &LibraryItem) -> Result<LibraryItem, AppError> {
        let row = sqlx::query(
            r#"
            UPDATE user_library_items
            SET owned_volumes = $2, current_episode = $3, total_episodes = $4,
                purchase_price = $5, variant_notes = $6, personal_rating = $7,
                notes = $8, updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, work_id, owned_volumes, current_episode, total_episodes,
                      CAST(purchase_price AS FLOAT8) AS purchase_price,
                      variant_notes, personal_rating, notes, created_at, updated_at
            "#,
        )
        .bind(item.id)
        .bind(&item.owned_volumes)
        .bind(item.current_episode)
        .bind(item.total_episodes)
        .bind(item.purchase_price)
        .bind(&item.variant_notes)
        .bind(item.personal_rating)
        .bind(&item.notes)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        row_to_library_item(&row)
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM user_library_items WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;
        Ok(())
    }
}
