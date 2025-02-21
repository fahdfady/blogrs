use axum::{
    routing::{get, post, put},
    Router,
};
use sqlx::{Pool, SqlitePool};

use crate::controllers::blog;

pub fn router(pool: SqlitePool) -> Router {
    Router::new()
    .route("/blogs", get(blog::get_blogs))
    .with_state(pool)
}