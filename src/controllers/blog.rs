use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::SqlitePool;

use crate::{
    models::{
        blog::{Blog, CreateBlog, UpdateBlog},
        author::Author,
    },
    Result,
};

pub async fn get_blogs(State(pool): State<SqlitePool>) -> Result<Json<Vec<Blog>>> {
    let blogs = sqlx::query_as::<_, Blog>("SELECT * FROM blogs")
        .fetch_all(&pool)
        .await?;

    Ok(Json(blogs))
}

// pub async fn get
