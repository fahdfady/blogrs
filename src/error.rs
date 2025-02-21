use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum Error {
    Database(sqlx::Error),
    NotFound,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let body: String = match self {
            Error::Database(e) => json!({"error": e.to_string()}).to_string(),
            Error::NotFound => json!({"error": "Not Found"}).to_string(),
        };

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Error::NotFound,
            e => Error::Database(e),
        }
    }
}
