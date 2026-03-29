use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::AppState;
use crate::application::dto::work_dto::{
    AddPersonRequest, AddTagRequest, CreateWorkRequest, UpdateWorkRequest, WorkListQuery,
};
use crate::application::errors::AppError;

/// Lista tutte le opere con filtri opzionali
pub async fn list_works(
    State(state): State<AppState>,
    Query(query): Query<WorkListQuery>,
) -> Result<impl IntoResponse, AppError> {
    let works = state.work_service.list_works(query).await?;
    Ok(Json(works))
}

/// Recupera un'opera per ID
pub async fn get_work(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let work = state.work_service.get_work(id).await?;
    Ok(Json(work))
}

/// Crea una nuova opera
pub async fn create_work(
    State(state): State<AppState>,
    Json(req): Json<CreateWorkRequest>,
) -> Result<impl IntoResponse, AppError> {
    let work = state.work_service.create_work(req).await?;
    Ok((StatusCode::CREATED, Json(work)))
}

/// Aggiorna un'opera esistente
pub async fn update_work(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateWorkRequest>,
) -> Result<impl IntoResponse, AppError> {
    let work = state.work_service.update_work(id, req).await?;
    Ok(Json(work))
}

/// Elimina un'opera
pub async fn delete_work(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    state.work_service.delete_work(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Aggiunge un tag a un'opera
pub async fn add_tag(
    State(state): State<AppState>,
    Path(work_id): Path<Uuid>,
    Json(req): Json<AddTagRequest>,
) -> Result<impl IntoResponse, AppError> {
    state.work_service.add_tag(work_id, req).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Rimuove un tag da un'opera
pub async fn remove_tag(
    State(state): State<AppState>,
    Path((work_id, tag_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, AppError> {
    state.work_service.remove_tag(work_id, tag_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Aggiunge una persona a un'opera
pub async fn add_person(
    State(state): State<AppState>,
    Path(work_id): Path<Uuid>,
    Json(req): Json<AddPersonRequest>,
) -> Result<impl IntoResponse, AppError> {
    state.work_service.add_person(work_id, req).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Rimuove una persona da un'opera
pub async fn remove_person(
    State(state): State<AppState>,
    Path((work_id, person_id, role)): Path<(Uuid, Uuid, String)>,
) -> Result<impl IntoResponse, AppError> {
    state.work_service.remove_person(work_id, person_id, &role).await?;
    Ok(StatusCode::NO_CONTENT)
}
