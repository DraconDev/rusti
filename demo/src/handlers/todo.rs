use axum::{body::Body, response::IntoResponse};

use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use std::{
    sync::{Arc, Mutex},
    collections::HashMap,
};
use serde::{Deserialize, Serialize};

// Define your Todo struct
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

// Struct for creating a new todo (without an ID)
#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub title: String,
}

// Type alias for our shared state
type SharedState = Arc<Mutex<HashMap<u32, Todo>>>;

pub async fn add_todo(
    State(todos): State<SharedState>,
    Json(create_todo): Json<CreateTodo>,
) -> impl IntoResponse {
    let mut todos = todos.lock().unwrap();

    // Generate a simple ID. In a real app, you might use a UUID or a more robust counter.
    let id = todos.keys().max().map_or(1, |max_id| max_id + 1);

    let new_todo = Todo {
        id,
        title: create_todo.title,
        completed: false,
    };

    todos.insert(id, new_todo.clone());

    (StatusCode::CREATED, Json(new_todo))
}

pub async fn clear_completed() -> impl IntoResponse {}

pub async fn delete() -> impl IntoResponse {}

pub async fn stats() -> impl IntoResponse {}

pub async fn toggle() -> impl IntoResponse {}

pub async fn todo_list() -> impl IntoResponse {}
