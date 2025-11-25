use crate::db;
use crate::scripts::todo_app_htmx::{self, AddTodoForm, DbState, FilterQuery};
use axum::{
    extract::Query,
    extract::{Path, State},
    response::{Html, IntoResponse},
    Form,
};

pub async fn add_todo_handler(
    State(conn): State<DbState>,
    Form(form): Form<AddTodoForm>,
) -> impl IntoResponse {
    let conn = conn.lock().unwrap();
    let _ = db::insert_todo(&conn, form.text);

    // Also update stats
    let todo_list = todo_app_htmx::render_todo_list(&conn, None);
    Html(rusti::render_to_string(&todo_list))
}

pub async fn clear_completed_handler(State(conn): State<DbState>) -> impl IntoResponse {
    let conn = conn.lock().unwrap();
    let _ = db::clear_completed(&conn);

    let todo_list = todo_app_htmx::render_todo_list(&conn, None);
    Html(rusti::render_to_string(&todo_list))
}

pub async fn delete_handler(State(conn): State<DbState>, Path(id): Path<i64>) -> impl IntoResponse {
    let conn = conn.lock().unwrap();
    let _ = db::delete_todo(&conn, id);

    let todo_list = todo_app_htmx::render_todo_list(&conn, None);
    Html(rusti::render_to_string(&todo_list))
}

pub async fn stats_handler(State(conn): State<DbState>) -> impl IntoResponse {
    use rusti::rusti;

    let conn = conn.lock().unwrap();
    let (total, active, completed) = db::get_stats(&conn).unwrap_or((0, 0, 0));

    let stats_component = rusti! {
        "Total: " { total } " | Active: " { active } " | Completed: " { completed }
    };

    Html(rusti::render_to_string(&stats_component))
}

pub async fn toggle_handler(State(conn): State<DbState>, Path(id): Path<i64>) -> impl IntoResponse {
    let conn = conn.lock().unwrap();
    let _ = db::toggle_todo(&conn, id);

    let todo_list = todo_app_htmx::render_todo_list(&conn, None);
    Html(rusti::render_to_string(&todo_list))
}

pub async fn todo_list_handler(
    State(conn): State<DbState>,
    Query(params): Query<FilterQuery>,
) -> impl IntoResponse {
    let conn = conn.lock().unwrap();
    let todo_list = todo_app_htmx::render_todo_list(&conn, params.filter);
    Html(rusti::render_to_string(&todo_list))
}
