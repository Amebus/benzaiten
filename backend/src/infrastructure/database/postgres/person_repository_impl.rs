use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::application::errors::AppError;
use crate::domain::entities::person::{Person, WorkPerson};
use crate::domain::repositories::person_repository::PersonRepository;

/// Implementazione PostgreSQL del repository per le persone
pub struct PostgresPersonRepository {
    pool: PgPool,
}

impl PostgresPersonRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn row_to_person(row: &sqlx::postgres::PgRow) -> Result<Person, AppError> {
    Ok(Person {
        id: row.try_get("id").map_err(|e| AppError::DatabaseError(e.into()))?,
        name: row.try_get("name").map_err(|e| AppError::DatabaseError(e.into()))?,
        original_name: row.try_get("original_name").map_err(|e| AppError::DatabaseError(e.into()))?,
        country_code: row.try_get("country_code").map_err(|e| AppError::DatabaseError(e.into()))?,
        birth_date: row.try_get("birth_date").map_err(|e| AppError::DatabaseError(e.into()))?,
        metadata: row.try_get("metadata").map_err(|e| AppError::DatabaseError(e.into()))?,
        created_at: row.try_get("created_at").map_err(|e| AppError::DatabaseError(e.into()))?,
    })
}

#[async_trait]
impl PersonRepository for PostgresPersonRepository {
    async fn create(&self, person: &Person) -> Result<Person, AppError> {
        let row = sqlx::query(
            r#"
            INSERT INTO people (id, name, original_name, country_code, birth_date, metadata, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, name, original_name, country_code, birth_date, metadata, created_at
            "#,
        )
        .bind(person.id)
        .bind(&person.name)
        .bind(&person.original_name)
        .bind(&person.country_code)
        .bind(person.birth_date)
        .bind(&person.metadata)
        .bind(person.created_at)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        row_to_person(&row)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Person>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, name, original_name, country_code, birth_date, metadata, created_at
            FROM people WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        row.map(|r| row_to_person(&r)).transpose()
    }

    async fn find_all(&self) -> Result<Vec<Person>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, original_name, country_code, birth_date, metadata, created_at
            FROM people ORDER BY name ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        rows.iter().map(row_to_person).collect()
    }

    async fn find_by_work_id(&self, work_id: Uuid) -> Result<Vec<WorkPerson>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT wp.work_id, wp.person_id, wp.role,
                   p.id, p.name, p.original_name, p.country_code, p.birth_date, p.metadata, p.created_at
            FROM work_people wp
            INNER JOIN people p ON p.id = wp.person_id
            WHERE wp.work_id = $1
            ORDER BY wp.role ASC, p.name ASC
            "#,
        )
        .bind(work_id)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        rows.iter()
            .map(|row| {
                let person = row_to_person(row)?;
                Ok(WorkPerson {
                    work_id: row.try_get("work_id").map_err(|e| AppError::DatabaseError(e.into()))?,
                    person_id: row.try_get("person_id").map_err(|e| AppError::DatabaseError(e.into()))?,
                    role: row.try_get("role").map_err(|e| AppError::DatabaseError(e.into()))?,
                    person: Some(person),
                })
            })
            .collect()
    }

    async fn update(&self, person: &Person) -> Result<Person, AppError> {
        let row = sqlx::query(
            r#"
            UPDATE people
            SET name = $2, original_name = $3, country_code = $4, birth_date = $5, metadata = $6
            WHERE id = $1
            RETURNING id, name, original_name, country_code, birth_date, metadata, created_at
            "#,
        )
        .bind(person.id)
        .bind(&person.name)
        .bind(&person.original_name)
        .bind(&person.country_code)
        .bind(person.birth_date)
        .bind(&person.metadata)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        row_to_person(&row)
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM people WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;
        Ok(())
    }
}
