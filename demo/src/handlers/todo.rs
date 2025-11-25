use axum::{
    extract::{Form, Path, Query},
    response::IntoResponse,
};

use scripts::todo_app_htmx::{
    htmx_add_todo_handler, htmx_clear_completed_handler, htmx_delete_handler, htmx_stats_handler,
    htmx_todo_list_handler, htmx_toggle_handler, AddTodoForm, FilterQuery,
};

/// Handler for adding a todo item via HTMX.
pub async fn add_todo_handler(Form(form): Form<AddTodoForm>) -> impl IntoResponse {}

/// Handler for clearing completed todo items.
pub async fn clear_completed_handler() -> impl IntoResponse {}

/// Handler for deleting a specific todo item by its id.
pub async fn delete_handler(Path(id): Path<u64>) -> impl IntoResponse {}

/// Handler for retrieving todo statistics.
pub async fn stats_handler() -> impl IntoResponse {}

/// Handler for toggling the completion state of a todo item.
pub async fn toggle_handler(Path(id): Path<u64>) -> impl IntoResponse {}

/// Handler for returning the todo list fragment, optionally filtered.
pub async fn todo_list_handler(Query(params): Query<FilterQuery>) -> impl IntoResponse {}
