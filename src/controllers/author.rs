use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::{query_as, SqlitePool};

use crate::{models::author::Author, Result};

pub async fn get_authors(State(pool): State<SqlitePool>) -> Result<Json<Vec<Author>>> {
    let authors = query_as("SELECT * FROM authors").fetch_all(&pool).await?;

    Ok(Json(authors))
}

pub async fn create_author(
    State(pool): State<SqlitePool>,
    Json(author): Json<Author>,
) -> Result<Json<Author>> {
    let author = query_as("INSERT INTO authors(name,email) VALUES(?,?) RETURNING *")
        .bind(author.name)
        .bind(author.email)
        .fetch_one(&pool)
        .await?;

    Ok(Json(author))
}

pub async fn get_author_by_id(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<Json<Author>> {
    let author = query_as("SELECT * FROM authors WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    Ok(Json(author))
}
