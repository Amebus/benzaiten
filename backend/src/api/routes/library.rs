use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::AppState;
use crate::api::extractors::user::AuthUser;
use crate::application::dto::library_dto::{CreateLibraryItemRequest, UpdateLibraryItemRequest};
use crate::application::errors::AppError;

/// Lista tutti gli elementi della libreria dell'utente autenticato
pub async fn list_library(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let items = state.library_service.list_user_items(&user.user_id).await?;
    Ok(Json(items))
}

/// Recupera un elemento della libreria per ID
pub async fn get_library_item(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let item = state.library_service.get_item(id, &user.user_id).await?;
    Ok(Json(item))
}

/// Aggiunge un'opera alla libreria dell'utente
pub async fn create_library_item(
    State(state): State<AppState>,
    user: AuthUser,
    Json(req): Json<CreateLibraryItemRequest>,
) -> Result<impl IntoResponse, AppError> {
    let item = state.library_service.create_item(&user.user_id, req).await?;
    Ok((StatusCode::CREATED, Json(item)))
}

/// Aggiorna un elemento della libreria
pub async fn update_library_item(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateLibraryItemRequest>,
) -> Result<impl IntoResponse, AppError> {
    let item = state.library_service.update_item(id, &user.user_id, req).await?;
    Ok(Json(item))
}

/// Rimuove un elemento dalla libreria
pub async fn delete_library_item(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    state.library_service.delete_item(id, &user.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
