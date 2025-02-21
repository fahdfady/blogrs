pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub type Result<T> = std::result::Result<T, Error>;

pub mod config;
pub mod db;
pub mod controllers;
pub mod models;