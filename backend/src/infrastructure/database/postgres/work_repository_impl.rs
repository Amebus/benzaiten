use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use std::str::FromStr;

use crate::application::errors::AppError;
use crate::domain::entities::work::Work;
use crate::domain::repositories::work_repository::{WorkFilters, WorkRepository};
use crate::domain::value_objects::work_type::WorkType;

/// Implementazione PostgreSQL del repository per le opere
pub struct PostgresWorkRepository {
    pool: PgPool,
}

impl PostgresWorkRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn row_to_work(row: &sqlx::postgres::PgRow) -> Result<Work, AppError> {
    let work_type_str: String = row.try_get("work_type").map_err(|e| AppError::DatabaseError(e.into()))?;
    let work_type = WorkType::from_str(&work_type_str)
        .map_err(|e| AppError::InternalError(format!("Tipo opera non valido: {}", e)))?;

    Ok(Work {
        id: row.try_get("id").map_err(|e| AppError::DatabaseError(e.into()))?,
        work_type,
        title: row.try_get("title").map_err(|e| AppError::DatabaseError(e.into()))?,
        original_title: row.try_get("original_title").map_err(|e| AppError::DatabaseError(e.into()))?,
        synopsis: row.try_get("synopsis").map_err(|e| AppError::DatabaseError(e.into()))?,
        year: row.try_get("year").map_err(|e| AppError::DatabaseError(e.into()))?,
        metadata: row.try_get("metadata").map_err(|e| AppError::DatabaseError(e.into()))?,
        created_at: row.try_get("created_at").map_err(|e| AppError::DatabaseError(e.into()))?,
        updated_at: row.try_get("updated_at").map_err(|e| AppError::DatabaseError(e.into()))?,
    })
}

#[async_trait]
impl WorkRepository for PostgresWorkRepository {
    async fn create(&self, work: &Work) -> Result<Work, AppError> {
        let row = sqlx::query(
            r#"
            INSERT INTO works (id, work_type, title, original_title, synopsis, year, metadata, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, work_type, title, original_title, synopsis, year, metadata, created_at, updated_at
            "#,
        )
        .bind(work.id)
        .bind(work.work_type.to_string())
        .bind(&work.title)
        .bind(&work.original_title)
        .bind(&work.synopsis)
        .bind(work.year)
        .bind(&work.metadata)
        .bind(work.created_at)
        .bind(work.updated_at)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        row_to_work(&row)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Work>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, work_type, title, original_title, synopsis, year, metadata, created_at, updated_at
            FROM works WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        row.map(|r| row_to_work(&r)).transpose()
    }

    async fn find_all(&self, filters: WorkFilters) -> Result<Vec<Work>, AppError> {
        let page = filters.page.unwrap_or(1).max(1);
        let page_size = filters.page_size.unwrap_or(20).min(100);
        let offset = (page - 1) * page_size;

        // Costruzione dinamica della query con i filtri
        let mut conditions = vec!["1=1".to_string()];
        let mut param_idx = 1i32;

        if filters.work_type.is_some() {
            param_idx += 1;
            conditions.push(format!("work_type = ${}", param_idx));
        }
        if filters.search_term.is_some() {
            param_idx += 1;
            conditions.push(format!(
                "(title ILIKE ${0} OR original_title ILIKE ${0})",
                param_idx
            ));
        }

        let where_clause = conditions.join(" AND ");
        let sql = format!(
            r#"
            SELECT id, work_type, title, original_title, synopsis, year, metadata, created_at, updated_at
            FROM works
            WHERE {}
            ORDER BY created_at DESC
            LIMIT $1 OFFSET ${}
            "#,
            where_clause,
            param_idx + 1
        );

        let mut query = sqlx::query(&sql).bind(page_size).bind(offset);

        if let Some(ref work_type) = filters.work_type {
            query = query.bind(work_type);
        }
        if let Some(ref search) = filters.search_term {
            query = query.bind(format!("%{}%", search));
        }

        let rows = query
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;

        rows.iter().map(row_to_work).collect()
    }

    async fn update(&self, work: &Work) -> Result<Work, AppError> {
        let row = sqlx::query(
            r#"
            UPDATE works
            SET title = $2, original_title = $3, synopsis = $4, year = $5, metadata = $6, updated_at = NOW()
            WHERE id = $1
            RETURNING id, work_type, title, original_title, synopsis, year, metadata, created_at, updated_at
            "#,
        )
        .bind(work.id)
        .bind(&work.title)
        .bind(&work.original_title)
        .bind(&work.synopsis)
        .bind(work.year)
        .bind(&work.metadata)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        row_to_work(&row)
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM works WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;
        Ok(())
    }

    async fn add_tag(&self, work_id: Uuid, tag_id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO work_tags (work_id, tag_id)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            "#,
        )
        .bind(work_id)
        .bind(tag_id)
        .execute(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;
        Ok(())
    }

    async fn remove_tag(&self, work_id: Uuid, tag_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM work_tags WHERE work_id = $1 AND tag_id = $2")
            .bind(work_id)
            .bind(tag_id)
            .execute(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;
        Ok(())
    }

    async fn add_person(&self, work_id: Uuid, person_id: Uuid, role: &str) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO work_people (work_id, person_id, role)
            VALUES ($1, $2, $3)
            ON CONFLICT DO NOTHING
            "#,
        )
        .bind(work_id)
        .bind(person_id)
        .bind(role)
        .execute(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;
        Ok(())
    }

    async fn remove_person(&self, work_id: Uuid, person_id: Uuid, role: &str) -> Result<(), AppError> {
        sqlx::query(
            "DELETE FROM work_people WHERE work_id = $1 AND person_id = $2 AND role = $3",
        )
        .bind(work_id)
        .bind(person_id)
        .bind(role)
        .execute(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;
        Ok(())
    }
}
