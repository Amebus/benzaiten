use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::application::errors::AppError;
use crate::domain::entities::tag::Tag;
use crate::domain::repositories::tag_repository::TagRepository;

/// Implementazione PostgreSQL del repository per i tag
pub struct PostgresTagRepository {
    pool: PgPool,
}

impl PostgresTagRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn row_to_tag(row: &sqlx::postgres::PgRow) -> Result<Tag, AppError> {
    Ok(Tag {
        id: row.try_get("id").map_err(|e| AppError::DatabaseError(e.into()))?,
        name: row.try_get("name").map_err(|e| AppError::DatabaseError(e.into()))?,
        slug: row.try_get("slug").map_err(|e| AppError::DatabaseError(e.into()))?,
        description: row.try_get("description").map_err(|e| AppError::DatabaseError(e.into()))?,
        color: row.try_get("color").map_err(|e| AppError::DatabaseError(e.into()))?,
        created_at: row.try_get("created_at").map_err(|e| AppError::DatabaseError(e.into()))?,
    })
}

#[async_trait]
impl TagRepository for PostgresTagRepository {
    async fn create(&self, tag: &Tag) -> Result<Tag, AppError> {
        let row = sqlx::query(
            r#"
            INSERT INTO tags (id, name, slug, description, color, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, name, slug, description, color, created_at
            "#,
        )
        .bind(tag.id)
        .bind(&tag.name)
        .bind(&tag.slug)
        .bind(&tag.description)
        .bind(&tag.color)
        .bind(tag.created_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("unique") || e.to_string().contains("duplicate") {
                AppError::Conflict(format!("Tag con nome '{}' o slug '{}' già esistente", tag.name, tag.slug))
            } else {
                AppError::DatabaseError(e)
            }
        })?;

        row_to_tag(&row)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tag>, AppError> {
        let row = sqlx::query(
            "SELECT id, name, slug, description, color, created_at FROM tags WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        row.map(|r| row_to_tag(&r)).transpose()
    }

    async fn find_all(&self) -> Result<Vec<Tag>, AppError> {
        let rows = sqlx::query(
            "SELECT id, name, slug, description, color, created_at FROM tags ORDER BY name ASC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        rows.iter().map(row_to_tag).collect()
    }

    async fn find_by_work_id(&self, work_id: Uuid) -> Result<Vec<Tag>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT t.id, t.name, t.slug, t.description, t.color, t.created_at
            FROM tags t
            INNER JOIN work_tags wt ON wt.tag_id = t.id
            WHERE wt.work_id = $1
            ORDER BY t.name ASC
            "#,
        )
        .bind(work_id)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        rows.iter().map(row_to_tag).collect()
    }

    async fn update(&self, tag: &Tag) -> Result<Tag, AppError> {
        let row = sqlx::query(
            r#"
            UPDATE tags
            SET name = $2, description = $3, color = $4
            WHERE id = $1
            RETURNING id, name, slug, description, color, created_at
            "#,
        )
        .bind(tag.id)
        .bind(&tag.name)
        .bind(&tag.description)
        .bind(&tag.color)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        row_to_tag(&row)
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM tags WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;
        Ok(())
    }
}
