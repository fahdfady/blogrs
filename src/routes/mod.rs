use axum::Router;
use sqlx::SqlitePool;

pub mod author;
pub mod blog;

pub fn create_router(pool: SqlitePool)-> Router{
    Router::new()
    .merge(blog::router(pool.clone()))
    .merge(author::router(pool.clone()))
}