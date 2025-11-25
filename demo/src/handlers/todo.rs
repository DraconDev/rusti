use axum::{body::Body, response::IntoResponse};

use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::time::Duration;

pub async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_timeout(Duration::from_secs(3))
        .connect("sqlite::memory:")
        .await
        .expect("Failed to connect to in-memory SQLite database");

    // Optionally, run migrations here if you have them
    // sqlx::migrate!("./migrations").run(&pool).await.expect("Failed to run migrations");

    pool
}


pub async fn add_todo() -> impl IntoResponse {}

pub async fn clear_completed() -> impl IntoResponse {}

pub async fn delete() -> impl IntoResponse {}

pub async fn stats() -> impl IntoResponse {}

pub async fn toggle() -> impl IntoResponse {}

pub async fn todo_list() -> impl IntoResponse {}
