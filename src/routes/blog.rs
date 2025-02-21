use axum::{
    routing::{get, post, put},
    Router,
};
use sqlx::{Pool, SqlitePool};

use crate::handlers::blog;

pub fn router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/blogs", get(blog::get_blogs).post(blog::create_blog))
        .route("/blogs/:id", put(blog::update_blog))
        .route("/blogs/:id/author", get(blog::get_blog_author))
        .with_state(pool)
}
