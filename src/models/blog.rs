use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Blog {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CreateBlog {
    pub title: String,
    pub content: String,
    pub author_id: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UpdateBlog {
    pub title: String,
    pub content: String,
}
