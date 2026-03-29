use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use bytes::Bytes;
use uuid::Uuid;

use crate::AppState;
use crate::application::errors::AppError;

/// Lista le immagini di un'opera
pub async fn list_images(
    State(state): State<AppState>,
    Path(work_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let images = state.image_service.list_images(work_id).await?;
    Ok(Json(images))
}

/// Carica una nuova immagine per un'opera
pub async fn upload_image(
    State(state): State<AppState>,
    Path(work_id): Path<Uuid>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let mut data: Option<Bytes> = None;
    let mut content_type = "image/jpeg".to_string();
    let mut kind = "cover".to_string();
    let mut display_order: Option<i32> = None;

    // Elabora i campi del form multipart
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::ValidationError(format!("Errore lettura multipart: {}", e)))?
    {
        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "file" => {
                if let Some(ct) = field.content_type() {
                    content_type = ct.to_string();
                }
                data = Some(
                    field
                        .bytes()
                        .await
                        .map_err(|e| AppError::ValidationError(format!("Errore lettura file: {}", e)))?,
                );
            }
            "kind" => {
                kind = field
                    .text()
                    .await
                    .map_err(|e| AppError::ValidationError(format!("Errore lettura campo: {}", e)))?;
            }
            "display_order" => {
                let text = field
                    .text()
                    .await
                    .map_err(|e| AppError::ValidationError(format!("Errore lettura campo: {}", e)))?;
                display_order = text.parse::<i32>().ok();
            }
            _ => {}
        }
    }

    let file_data = data.ok_or_else(|| AppError::ValidationError("File mancante nel form".to_string()))?;

    if file_data.is_empty() {
        return Err(AppError::ValidationError("Il file non può essere vuoto".to_string()));
    }

    let image = state
        .image_service
        .upload_image(work_id, &kind, file_data, &content_type, display_order)
        .await?;

    Ok((StatusCode::CREATED, Json(image)))
}

/// Elimina un'immagine
pub async fn delete_image(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    state.image_service.delete_image(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
