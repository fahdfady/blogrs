use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::{query_as, SqlitePool};

use crate::{
    models::{
        author::Author,
        blog::{Blog, CreateBlog, UpdateBlog},
    },
    Result,
};

pub async fn get_blogs(State(pool): State<SqlitePool>) -> Result<Json<Vec<Blog>>> {
    let blogs = sqlx::query_as::<_, Blog>("SELECT * FROM blogs")
        .fetch_all(&pool)
        .await?;

    Ok(Json(blogs))
}

pub async fn get_blog_by_id(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Blog>> {
    let blog = query_as::<_, Blog>("SELECT * FROM blogs WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    Ok(Json(blog))
}

#[axum_macros::debug_handler]
pub async fn update_blog(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(update): Json<UpdateBlog>,
) -> Result<Json<Blog>> {
    let mut blog: Blog = get_blog_by_id(State(pool.clone()), Path(id)).await?.0;

    if let Some(title) = update.title {
        blog.title = title
    }
    if let Some(content) = update.content {
        blog.content = content;
    }

    let updated_blog = query_as("UPDATE blog SET title = ?, content = ?, updated_at CURRENT_TIMESTAMP WHERE id = ? RETURNING *")
    .bind(&blog.title)
    .bind(&blog.content)
    .bind(id)
    .fetch_one(&pool)
    .await?;

    Ok(Json(updated_blog))
}

pub async fn create_blog(
    State(pool): State<SqlitePool>,
    Json(blog): Json<CreateBlog>,
) -> Result<Json<Blog>> {
    let blog: Blog =
        query_as("INSERT INTO blogs(title,content,author_id) VALUES(?,?,?) RETURNING *")
            .bind(blog.title)
            .bind(blog.content)
            .bind(blog.author_id)
            .fetch_one(&pool)
            .await?;

    Ok(Json(blog))
}

pub async fn get_blog_author(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Author>> {
    let author: Author = query_as(
        "
    SELECT authors.* FROM authors 
    JOIN blogs ON authors.id = blogs.author_id
    WHERE blogs.id=?
    ",
    )
    .bind(id)
    .fetch_one(&pool)
    .await?;

    Ok(Json(author))
}
