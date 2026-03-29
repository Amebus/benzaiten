use std::sync::Arc;
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod domain;
mod application;
mod infrastructure;
mod api;

use infrastructure::database::postgres::{
    work_repository_impl::PostgresWorkRepository,
    tag_repository_impl::PostgresTagRepository,
    person_repository_impl::PostgresPersonRepository,
    library_repository_impl::PostgresLibraryRepository,
};
use infrastructure::storage::minio_client::MinioClient;
use application::services::{
    work_service::WorkService,
    tag_service::TagService,
    library_service::LibraryService,
    image_service::ImageService,
};
use api::routes;

/// Stato condiviso dell'applicazione
#[derive(Clone)]
pub struct AppState {
    pub work_service: Arc<WorkService>,
    pub tag_service: Arc<TagService>,
    pub library_service: Arc<LibraryService>,
    pub image_service: Arc<ImageService>,
}

#[tokio::main]
async fn main() {
    // Carica variabili di ambiente
    dotenvy::dotenv().ok();

    // Inizializza il tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL deve essere impostato");

    // Connessione al database PostgreSQL
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Impossibile connettersi al database");

    // Esegui le migrazioni
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Impossibile eseguire le migrazioni");

    // Client MinIO per lo storage delle immagini
    let minio_client = Arc::new(MinioClient::new(
        std::env::var("MINIO_ENDPOINT").unwrap_or_else(|_| "http://localhost:9000".into()),
        std::env::var("MINIO_BUCKET").unwrap_or_else(|_| "benzaiten".into()),
        std::env::var("MINIO_ACCESS_KEY").unwrap_or_else(|_| "minioadmin".into()),
        std::env::var("MINIO_SECRET_KEY").unwrap_or_else(|_| "minioadmin".into()),
    ));

    // Repository
    let work_repo = Arc::new(PostgresWorkRepository::new(pool.clone()));
    let tag_repo = Arc::new(PostgresTagRepository::new(pool.clone()));
    let _person_repo = Arc::new(PostgresPersonRepository::new(pool.clone()));
    let library_repo = Arc::new(PostgresLibraryRepository::new(pool.clone()));

    // Servizi applicativi
    let work_service = Arc::new(WorkService::new(work_repo.clone(), tag_repo.clone()));
    let tag_service = Arc::new(TagService::new(tag_repo.clone()));
    let library_service = Arc::new(LibraryService::new(library_repo.clone()));
    let image_service = Arc::new(ImageService::new(pool.clone(), minio_client.clone()));

    let state = AppState {
        work_service,
        tag_service,
        library_service,
        image_service,
    };

    // Configurazione CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Configurazione del router
    let app = Router::new()
        .nest("/api", routes::create_router(state))
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let addr = "0.0.0.0:8080";
    tracing::info!("Server in ascolto su {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
