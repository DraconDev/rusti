use rusti::rusti;
use axum::{extract::{Form, Query, Path}, response::IntoResponse};
use serde::Deserialize;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use chrono::{Utc, DateTime};

// Shared in‑memory todo list
static TODOS: Lazy<Mutex<Vec<Todo>>> = Lazy::new(|| Mutex::new(Vec::new()));

#[derive(Clone)]
struct Todo {
    id: u64,
    text: String,
    completed: bool,
    created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
struct AddTodoForm {
    text: String,
}

#[derive(Deserialize)]
struct FilterQuery {
    filter: Option<String>,
}

/// Render the full page
pub fn todo_app_htmx() -> impl rusti::Component {
    rusti! {
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <title>Todo App – htmx Demo</title>
            <style>
                body {font-family: sans-serif; max-width: 600px; margin: 40px auto; padding: 20px; background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%); min-height: 100vh;}
                .container {background: white; border-radius: 12px; padding: 30px; box-shadow: 0 10px 30px rgba(0,0,0,0.1);}
                h1 {text-align: center; color: #333; margin-bottom: 20px;}
                .input-section {display: flex; gap: 10px; margin-bottom: 20px;}
                input[type="text"] {flex: 1; padding: 10px; border: 2px solid #ddd; border-radius: 6px;}
                .btn {padding: 10px 20px; border: none; border-radius: 6px; background: #5c6bc0; color: white; cursor: pointer;}
                .btn:hover {background: #3949ab;}
                .todo-item {display: flex; align-items: center; padding: 8px 0; border-bottom: 1px solid #eee;}
                .todo-item.completed .text {text-decoration: line-through; color: #999;}
                .todo-checkbox {margin-right: 10px;}
                .delete-btn {margin-left: auto; background: transparent; border: none; color: #e53935; cursor: pointer;}
                .empty-state {text-align: center; color: #777; padding: 40px 0;}
                .stats {margin-top: 20px; text-align: center; color: #555;}
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Todo List – htmx</h1>
                <div class="input-section">
                    <input type="text" id="todo-input" name="text" placeholder="What needs to be done?" />
                    <!-- hx-post will submit the form to the server and replace the todo list -->
                    <button class="btn" hx-post="/api/add-todo" hx-include="#todo-input" hx-target="#todo-list" hx-swap="outerHTML">Add</button>
                </div>
                <div class="filters" style="margin-bottom: 20px;">
                    <button class="btn" hx-get="/api/todo-list?filter=all" hx-target="#todo-list" hx-swap="outerHTML">All</button>
                    <button class="btn" hx-get="/api/todo-list?filter=active" hx-target="#todo-list" hx-swap="outerHTML">Active</button>
                    <button class="btn" hx-get="/api/todo-list?filter=completed" hx-target="#todo-list" hx-swap="outerHTML">Completed</button>
                </div>
                <!-- The todo list fragment is loaded on page load -->
                <div id="todo-list" hx-get="/api/todo-list" hx-trigger="load" hx-swap="outerHTML"></div>
                <div class="stats" id="stats" hx-get="/api/stats" hx-trigger="load" hx-swap="innerHTML"></div>
                <button class="btn" style="margin-top: 10px;" hx-post="/api/clear-completed" hx-target="#todo-list" hx-swap="outerHTML">Clear Completed</button>
            </div>
        </body>
        </html>
    }
}

// ---------------------------------------------------------------------------
// Helper to render the todo list fragment based on an optional filter
async fn render_todo_list(filter: Option<String>) -> impl rusti::Component {
    let todos = TODOS.lock().unwrap();
    let filtered: Vec<&Todo> = match filter.as_deref() {
        Some("active") => todos.iter().filter(|t| !t.completed).collect(),
        Some("completed") => todos.iter().filter(|t| t.completed).collect(),
        _ => todos.iter().collect(),
    };
    rusti! {
        @if filtered.is_empty() {
            <div class="empty-state">No todos found.</div>
        } @else {
            <ul id="todo-list">
                @for todo in filtered {
                    <li class={format!("todo-item {}", if todo.completed { "completed" } else { "" })}>
                        <input type="checkbox" class="todo-checkbox" hx-post={format!("/api/toggle/{}", todo.id)} hx-target="#todo-list" hx-swap="outerHTML" {if todo.completed { "checked" } else { "" }} />
                        <span class="text">{ &todo.text }</span>
                        <button class="delete-btn" hx-post={format!("/api/delete/{}", todo.id)} hx-target="#todo-list" hx-swap="outerHTML">✕</button>
                    </li>
                }
            </ul>
        }
    }
}

// ---------------------------------------------------------------------------
// API component handlers
pub async fn htmx_add_todo_handler(Form(form): Form<AddTodoForm>) -> impl rusti::Component {
    let mut todos = TODOS.lock().unwrap();
    let id = (todos.len() as u64) + 1;
    todos.push(Todo { id, text: form.text, completed: false, created_at: Utc::now() });
    render_todo_list(None).await
}

pub async fn htmx_clear_completed_handler() -> impl rusti::Component {
    let mut todos = TODOS.lock().unwrap();
    todos.retain(|t| !t.completed);
    render_todo_list(None).await
}

pub async fn htmx_delete_handler(Path(id): Path<u64>) -> impl rusti::Component {
    let mut todos = TODOS.lock().unwrap();
    todos.retain(|t| t.id != id);
    render_todo_list(None).await
}

pub async fn htmx_stats_handler() -> impl rusti::Component {
    let todos = TODOS.lock().unwrap();
    let total = todos.len();
    let completed = todos.iter().filter(|t| t.completed).count();
    let active = total - completed;
    rusti! {
        <div class="stats">Total: { total } | Active: { active } | Completed: { completed }</div>
    }
}

pub async fn htmx_toggle_handler(Path(id): Path<u64>) -> impl rusti::Component {
    let mut todos = TODOS.lock().unwrap();
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.completed = !todo.completed;
    }
    render_todo_list(None).await
}

pub async fn htmx_todo_list_handler(Query(params): Query<FilterQuery>) -> impl rusti::Component {
    render_todo_list(params.filter).await
}


// Simple Todo App using htmx for server‑side rendering.
// The data is stored in a global in‑memory vector protected by a Mutex.
// This example demonstrates how to use htmx attributes (hx-get, hx-post, hx-target, hx-swap)
// to build a fully interactive todo list without any custom JavaScript.

pub fn todo_app_htmx() -> impl rusti::Component {
    rusti! {
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <title>Todo App "–" htmx Demo</title>
            <style>
                body {
                    font-family: sans-serif; max-width: 600px; margin: 40px auto; padding: 20px; background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%); min-height: 100vh;
                }
                .container {
                    background: white; border-radius: 12px; padding: 30px; box-shadow: 0 10px 30px rgba(0,0,0,0.1);
                }
                h1 {
                    text-align: center; color: #333; margin-bottom: 20px;
                }
                .input-section {
                    display: flex; gap: 10px; margin-bottom: 20px;
                }
                input[type="text"] {
                    flex: 1; padding: 10px; border: 2px solid #ddd; border-radius: 6px;
                }
                .btn {
                    padding: 10px 20px; border: none; border-radius: 6px; background: #5c6bc0; color: white; cursor: pointer;
                }
                .btn:hover {
                    background: #3949ab;
                }
                .todo-item {
                    display: flex; align-items: center; padding: 8px 0; border-bottom: 1px solid #eee;
                }
                .todo-item.completed .text {
                    text-decoration: line-through; color: #999;
                }
                .todo-checkbox {
                    margin-right: 10px;
                }
                .delete-btn {
                    margin-left: auto; background: transparent; border: none; color: #e53935; cursor: pointer;
                }
                .empty-state {
                    text-align: center; color: #777; padding: 40px 0;
                }
                .stats {
                    margin-top: 20px; text-align: center; color: #555;
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Todo List "–" htmx</h1>
                <div class="input-section">
                    <input type="text" id="todo-input" name="text" placeholder="What needs to be done?" />
                    <!-- hx-post will submit the form to the server and replace the todo list -->
                    <button class="btn" hx-post="/htmx/add-todo" hx-include="#todo-input" hx-target="#todo-list" hx-swap="outerHTML">Add</button>
                </div>
                <div class="filters" style="margin-bottom: 20px;">
                    <button class="btn" hx-get="/htmx/todo-list?filter=all" hx-target="#todo-list" hx-swap="outerHTML">All</button>
                    <button class="btn" hx-get="/htmx/todo-list?filter=active" hx-target="#todo-list" hx-swap="outerHTML">Active</button>
                    <button class="btn" hx-get="/htmx/todo-list?filter=completed" hx-target="#todo-list" hx-swap="outerHTML">Completed</button>
                </div>
                <!-- The todo list fragment is loaded on page load -->
                <div id="todo-list" hx-get="/htmx/todo-list" hx-trigger="load" hx-swap="outerHTML"></div>
                <div class="stats" id="stats" hx-get="/htmx/stats" hx-trigger="load" hx-swap="innerHTML"></div>
                <button class="btn" style="margin-top: 10px;" hx-post="/htmx/clear-completed" hx-target="#todo-list" hx-swap="outerHTML">Clear Completed</button>
            </div>
        </body>
        </html>
    }
}
