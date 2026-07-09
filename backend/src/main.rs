use std::path::Path;
use std::sync::Arc;

use axum::{routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

mod auth;
mod data_hujan;
mod pos;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env(),
        )
        .init();

    let env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
    tracing::info!("Loading .env from {:?}", env_path);
    dotenvy::from_path(&env_path).ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Connected to database");

    let migrations_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    let migrator = sqlx::migrate::Migrator::new(migrations_path)
        .await
        .expect("Failed to load migrations");

    migrator
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    tracing::info!("Migrations applied successfully");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app_state = Arc::new(auth::AppState { pool });

    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/auth/register", axum::routing::post(auth::register))
        .route("/api/auth/login", axum::routing::post(auth::login))
        .route("/api/pos", axum::routing::get(pos::list_pos).post(pos::create_pos))
        .route(
            "/api/pos/:id",
            axum::routing::get(pos::get_pos)
                .put(pos::update_pos)
                .delete(pos::delete_pos),
        )
        .route(
            "/api/data-hujan",
            axum::routing::get(data_hujan::list_data_hujan)
                .post(data_hujan::create_data_hujan),
        )
        .route(
            "/api/data-hujan/:id",
            axum::routing::get(data_hujan::get_data_hujan)
                .put(data_hujan::update_data_hujan)
                .delete(data_hujan::delete_data_hujan),
        )
        .layer(cors)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");

    tracing::info!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}

use axum::extract::State;
use axum::Json;
use shared_types::HealthResponse;

async fn health_check(State(state): State<Arc<auth::AppState>>) -> Json<HealthResponse> {
    sqlx::query("SELECT 1")
        .execute(&state.pool)
        .await
        .ok();

    Json(HealthResponse {
        status: "ok".to_string(),
    })
}
