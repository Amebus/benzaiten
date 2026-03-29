use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::AppState;
use crate::application::dto::tag_dto::{CreateTagRequest, UpdateTagRequest};
use crate::application::errors::AppError;

/// Lista tutti i tag
pub async fn list_tags(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let tags = state.tag_service.list_tags().await?;
    Ok(Json(tags))
}

/// Recupera un tag per ID
pub async fn get_tag(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let tag = state.tag_service.get_tag(id).await?;
    Ok(Json(tag))
}

/// Crea un nuovo tag
pub async fn create_tag(
    State(state): State<AppState>,
    Json(req): Json<CreateTagRequest>,
) -> Result<impl IntoResponse, AppError> {
    let tag = state.tag_service.create_tag(req).await?;
    Ok((StatusCode::CREATED, Json(tag)))
}

/// Aggiorna un tag esistente
pub async fn update_tag(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTagRequest>,
) -> Result<impl IntoResponse, AppError> {
    let tag = state.tag_service.update_tag(id, req).await?;
    Ok(Json(tag))
}

/// Elimina un tag
pub async fn delete_tag(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    state.tag_service.delete_tag(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
