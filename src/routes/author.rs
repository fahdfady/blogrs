use axum::{
    routing::{get, post, put},
    Router,
};
use sqlx::SqlitePool;

use crate::controllers::author;

pub fn router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/authors", get(author::get_authors))
        .with_state(pool)
}
