use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Blog {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub author_id: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct CreateBlog{
    pub title: String,
    pub content: String,
    pub author_id: u64,
}

pub struct UpdateBlog{
    pub title: String,
    pub content: String,
}
