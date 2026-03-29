use std::sync::Arc;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;
use crate::application::errors::AppError;
use crate::infrastructure::storage::minio_client::MinioClient;

/// Risposta con i dati di un'immagine
#[derive(Debug, serde::Serialize)]
pub struct ImageResponse {
    pub id: Uuid,
    pub work_id: Uuid,
    pub url: String,
    pub kind: String,
    pub display_order: Option<i32>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub created_at: String,
}

/// Servizio per la gestione delle immagini
pub struct ImageService {
    pool: PgPool,
    storage: Arc<MinioClient>,
}

impl ImageService {
    pub fn new(pool: PgPool, storage: Arc<MinioClient>) -> Self {
        Self { pool, storage }
    }

    pub async fn upload_image(
        &self,
        work_id: Uuid,
        kind: &str,
        data: Bytes,
        content_type: &str,
        display_order: Option<i32>,
    ) -> Result<ImageResponse, AppError> {
        // Genera una chiave S3 univoca
        let extension = match content_type {
            "image/jpeg" | "image/jpg" => "jpg",
            "image/png" => "png",
            "image/webp" => "webp",
            _ => "bin",
        };
        let s3_key = format!("works/{}/{}/{}.{}", work_id, kind, Uuid::new_v4(), extension);

        // Carica l'immagine su MinIO
        self.storage
            .upload(&s3_key, data, content_type)
            .await
            .map_err(|e| AppError::InternalError(format!("Errore upload immagine: {}", e)))?;

        let id = Uuid::new_v4();
        let now = Utc::now();
        let order = display_order.unwrap_or(0);

        // Salva i metadati nel database
        sqlx::query(
            r#"
            INSERT INTO images (id, work_id, s3_key, kind, display_order, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(id)
        .bind(work_id)
        .bind(&s3_key)
        .bind(kind)
        .bind(order)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(ImageResponse {
            id,
            work_id,
            url: self.storage.public_url(&s3_key),
            kind: kind.to_string(),
            display_order: Some(order),
            width: None,
            height: None,
            created_at: now.to_rfc3339(),
        })
    }

    pub async fn list_images(&self, work_id: Uuid) -> Result<Vec<ImageResponse>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, work_id, s3_key, kind, display_order, width, height, created_at
            FROM images
            WHERE work_id = $1
            ORDER BY display_order ASC, created_at ASC
            "#,
        )
        .bind(work_id)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        let images = rows
            .into_iter()
            .map(|row| {
                use sqlx::Row;
                let s3_key: String = row.get("s3_key");
                let created_at: DateTime<Utc> = row.get("created_at");
                ImageResponse {
                    id: row.get("id"),
                    work_id: row.get("work_id"),
                    url: self.storage.public_url(&s3_key),
                    kind: row.get("kind"),
                    display_order: row.get("display_order"),
                    width: row.get("width"),
                    height: row.get("height"),
                    created_at: created_at.to_rfc3339(),
                }
            })
            .collect();

        Ok(images)
    }

    pub async fn delete_image(&self, id: Uuid) -> Result<(), AppError> {
        // Recupera la chiave S3 prima di eliminare
        let row = sqlx::query("SELECT s3_key FROM images WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?
            .ok_or_else(|| AppError::NotFound(format!("Immagine {} non trovata", id)))?;

        use sqlx::Row;
        let s3_key: String = row.get("s3_key");

        // Elimina dal database
        sqlx::query("DELETE FROM images WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;

        // Elimina da MinIO
        self.storage
            .delete(&s3_key)
            .await
            .map_err(|e| AppError::InternalError(format!("Errore eliminazione immagine da storage: {}", e)))?;

        Ok(())
    }
}
