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

    htmx_add_todo_handler,
    htmx_clear_completed_handler,
    htmx_delete_handler,
    htmx_stats_handler,
    htmx_toggle_handler,
    htmx_todo_list_handler,
    AddTodoForm,
    FilterQuery,
};

/// Add a new todo item via HTMX.
pub async fn add_todo_handler(Form(form): Form<AddTodoForm>) -> impl IntoResponse {
    htmx_add_todo_handler(Form(form)).await
}

/// Clear completed todo items.
pub async fn clear_completed_handler() -> impl IntoResponse {
    htmx_clear_completed_handler().await
}

/// Delete a specific todo item by its id.
pub async fn delete_handler(Path(id): Path<u64>) -> impl IntoResponse {
    htmx_delete_handler(Path(id)).await
}

/// Retrieve todo statistics.
pub async fn stats_handler() -> impl IntoResponse {
    htmx_stats_handler().await
}

/// Toggle completion state of a todo item.
pub async fn toggle_handler(Path(id): Path<u64>) -> impl IntoResponse {
    htmx_toggle_handler(Path(id)).await
}

/// Return the todo list fragment, optionally filtered.
pub async fn todo_list_handler(Query(params): Query<FilterQuery>) -> impl IntoResponse {
    htmx_todo_list_handler(Query(params)).await
}
