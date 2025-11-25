use axum::{body::Body, response::IntoResponse};

pub async fn add_todo() -> impl IntoResponse {}

pub async fn clear_completed() -> impl IntoResponse {}

pub async fn delete() -> impl IntoResponse {}

pub async fn stats() -> impl IntoResponse {}

pub async fn toggle() -> impl IntoResponse {}

pub async fn todo_list() -> impl IntoResponse {}
