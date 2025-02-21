use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub async fn connect() -> SqlitePool {
    // Create the data directory if it doesn't exist
    std::fs::create_dir_all("data").expect("Failed to create data directory");

    // Construct the database URL using the data directory
    let database_url = "sqlite:data/blog.db";
    
    SqlitePoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to database")
}
