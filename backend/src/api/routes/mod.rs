pub mod health;
pub mod works;
pub mod tags;
pub mod library;
pub mod images;

use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};

use crate::AppState;
use crate::api::middleware::auth::require_auth;

/// Crea il router principale con tutte le rotte dell'API
pub fn create_router(state: AppState) -> Router {
    // Rotte pubbliche (senza autenticazione)
    let public_routes = Router::new()
        .route("/health", get(health::health_check))
        .route("/works", get(works::list_works).post(works::create_work))
        .route(
            "/works/:id",
            get(works::get_work)
                .put(works::update_work)
                .delete(works::delete_work),
        )
        .route("/works/:id/tags", post(works::add_tag))
        .route("/works/:id/tags/:tag_id", delete(works::remove_tag))
        .route("/works/:id/people", post(works::add_person))
        .route(
            "/works/:id/people/:person_id/:role",
            delete(works::remove_person),
        )
        .route("/tags", get(tags::list_tags).post(tags::create_tag))
        .route(
            "/tags/:id",
            get(tags::get_tag)
                .put(tags::update_tag)
                .delete(tags::delete_tag),
        )
        .route("/works/:id/images", get(images::list_images).post(images::upload_image))
        .route("/images/:id", delete(images::delete_image));

    // Rotte protette (richiedono autenticazione)
    let protected_routes = Router::new()
        .route(
            "/library",
            get(library::list_library).post(library::create_library_item),
        )
        .route(
            "/library/:id",
            get(library::get_library_item)
                .put(library::update_library_item)
                .delete(library::delete_library_item),
        )
        .layer(middleware::from_fn(require_auth));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
}
