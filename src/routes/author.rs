use axum::{
    routing::{get, post, put},
    Router,
};
use sqlx::SqlitePool;

use crate::handlers::author;

pub fn router(pool: SqlitePool) -> Router {
    Router::new()
        .route(
            "/authors",
            get(author::get_authors).post(author::create_author),
        )
        .with_state(pool)
}
