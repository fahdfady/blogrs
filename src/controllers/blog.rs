use axum::{routing::get, Json, Router};
use sqlx::query_as;

pub async fn get_blogs() -> Result<Json<Vec<Blog>>> {
    let blogs = query_as::<_, Blog>("SELECT * FROM blogs")
        .fetch_all(&pool())
        .await?;

    Ok(Json((blogs)))
}
