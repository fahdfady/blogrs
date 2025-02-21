pub mod config;
pub mod db;
pub mod controllers;
pub mod models;
pub mod routes;
pub mod error;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;
