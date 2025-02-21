use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use crate::config::database_url;

pub async fn connect() -> SqlitePool {
    SqlitePoolOptions::new()
        .connect(&database_url())
        .await
        .expect("Failed to connect to database")
}