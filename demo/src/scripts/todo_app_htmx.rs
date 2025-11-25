use crate::db;
use rusqlite::Connection;
use rusti::rusti;
use serde::Deserialize;
use std::sync::{Arc, Mutex};

// Type alias for shared database state
pub type DbState = Arc<Mutex<Connection>>;

#[derive(Deserialize)]
pub struct AddTodoForm {
    pub text: String,
}

#[derive(Deserialize)]
pub struct FilterQuery {
    pub filter: Option<String>,
}

/// Main todo app page component with HTMX attributes and modern styling
pub fn todo_app_htmx() -> impl rusti::Component {
    rusti! {
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <title>"Todo App – HTMX + SQLite"</title>
            <script src="https://unpkg.com/htmx.org@1.9.10"></script>
            <style>
                
                * {
                    margin: 0;
                    padding: 0;
                    box-sizing: border-box;
                }
                
                body {
                    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
                    background: linear-gradient(135deg, #"667eea" 0%, #764ba2 100%);
                    min-height: 100vh;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    padding: 20px;
                }
                
                .container {
                    background: white;
                    border-radius: 24px;
                    padding: 48px;
                    box-shadow: 0 20px 60px rgba(0,0,0,0.3);
                    max-width: 600px;
                    width: 100%;
                }
                
                h1 {
                    text-align: center;
                    color: #"667eea";
                    margin-bottom: 12px;
                    font-size: 2.5rem;
                    font-weight: 800;
                    background: linear-gradient(135deg, #"667eea" 0%, #764ba2 100%);
                    -webkit-background-clip: text;
                    -webkit-text-fill-color: transparent;
                    background-clip: text;
                }
                
                .subtitle {
                    text-align: center;
                    color: #888;
                    margin-bottom: 32px;
                    font-size: 0.9rem;
                }
                
                .input-section {
                    display: flex;
                    gap: 12px;
                    margin-bottom: 24px;
                }
                
                input[type="text"] {
                    flex: 1;
                    padding: 14px 18px;
                    border: 2px solid #e0e0e0;
                    border-radius: 12px;
                    font-size: 16px;
                    transition: all 0.3s ease;
                }
                
                input[type="text"]:focus {
                    outline: none;
                    border-color: #"667eea";
                    box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
                }
                
                .btn {
                    padding: 14px 28px;
                    border: none;
                    border-radius: 12px;
                    font-weight: 600;
                    cursor: pointer;
                    transition: all 0.3s ease;
                    font-size: 15px;
                }
                
                .btn-primary {
                    background: linear-gradient(135deg, #"667eea" 0%, #764ba2 100%);
                    color: white;
                }
                
                .btn-primary:hover {
                    transform: translateY(-2px);
                    box-shadow: 0 6px 20px rgba(102, 126, 234, 0.4);
                }
                
                .filters {
                    display: flex;
                    gap: 8px;
                    margin-bottom: 20px;
                    justify-content: center;
                    flex-wrap: wrap;
                }
                
                .filters .btn {
                    padding: 8px 20px;
                    background: #f5f5f5;
                    color: #666;
                    font-size: 14px;
                }
                
                .filters .btn:hover {
                    background: #e0e0e0;
                    transform: none;
                }
                
                .todo-list {
                    list-style: none;
                    margin-bottom: 24px;
                }
                
                .todo-item {
                    display: flex;
                    align-items: center;
                    padding: 16px;
                    background: #f9f9f9;
                    border-radius: 12px;
                    margin-bottom: 8px;
                    transition: all 0.3s ease;
                }
                
                .todo-item:hover {
                    background: #f0f0f0;
                    transform: translateX(4px);
                }
                
                .todo-item.completed {
                    opacity: 0.6;
                }
                
                .todo-item.completed .text {
                    text-decoration: line-through;
                    color: #999;
                }
                
                .todo-checkbox {
                    width: 20px;
                    height: 20px;
                    margin-right: 14px;
                    cursor: pointer;
                    accent-color: #"667eea";
                }
                
                .text {
                    flex: 1;
                    font-size: 16px;
                    color: #333;
                }
                
                .delete-btn {
                    background: transparent;
                    border: none;
                    color: #e74c3c;
                    font-size: 20px;
                    cursor: pointer;
                    padding: 4px 8px;
                    border-radius: 6px;
                    transition: all 0.3s ease;
                }
                
                .delete-btn:hover {
                    background: #fee;
                }
                
                .empty-state {
                    text-align: center;
                    padding: 60px 20px;
                    color: #bbb;
                    font-size: 18px;
                }
                
                .stats {
                    text-align: center;
                    color: #666;
                    font-size: 14px;
                    padding: 16px;
                    background: #f9f9f9;
                    border-radius: 12px;
                    margin-bottom: 16px;
                }
                
                .clear-completed-btn {
                    width: 100%;
                    background: #e74c3c;
                    color: white;
                }
                
                .clear-completed-btn:hover {
                    background: #c0392b;
                    transform: translateY(-2px);
                    box-shadow: 0 4px 12px rgba(231, 76, 60, 0.3);
                }
                
                .htmx-indicator {
                    opacity: 0;
                    transition: opacity 200ms ease-in;
                }
                
                .htmx-request .htmx-indicator {
                    opacity: 1;
                }
                
            </style>
        </head>
        <body>
            <div class="container">
                <h1>"Todo List"</h1>
                <div class="subtitle">"HTMX + SQLite + Rusti"</div>

                <div class="input-section">
                    <input
                        type="text"
                        id="todo-input"
                        name="text"
                        placeholder="What needs to be done?"
                        autocomplete="off"
                    />
                    <button
                        class="btn btn-primary"
                        hx-post="/api/add-todo"
                        hx-include="#todo-input"
                        hx-target="#todo-list"
                        hx-swap="outerHTML"
                    >
                        "Add Todo"
                    </button>
                </div>

                <div class="filters">
                    <button
                        class="btn"
                        hx-get="/api/todo-list?filter=all"
                        hx-target="#todo-list"
                        hx-swap="outerHTML"
                    >
                        "All"
                    </button>
                    <button
                        class="btn"
                        hx-get="/api/todo-list?filter=active"
                        hx-target="#todo-list"
                        hx-swap="outerHTML"
                    >
                        "Active"
                    </button>
                    <button
                        class="btn"
                        hx-get="/api/todo-list?filter=completed"
                        hx-target="#todo-list"
                        hx-swap="outerHTML"
                    >
                        "Completed"
                    </button>
                </div>

                <div
                    id="todo-list"
                    hx-get="/api/todo-list"
                    hx-trigger="load"
                    hx-swap="outerHTML"
                ></div>

                <div
                    class="stats"
                    id="stats"
                    hx-get="/api/stats"
                    hx-trigger="load"
                    hx-swap="innerHTML"
                ></div>

                <button
                    class="btn clear-completed-btn"
                    hx-post="/api/clear-completed"
                    hx-target="#todo-list"
                    hx-swap="outerHTML"
                >
                    "Clear Completed"
                </button>
            </div>
        </body>
        </html>
    }
}

/// Helper to render the todo list fragment based on optional filter
pub fn render_todo_list(conn: &Connection, filter: Option<String>) -> impl rusti::Component {
    let todos = db::list_todos(conn, filter).unwrap_or_else(|_| vec![]);

    rusti! {
        @if todos.is_empty() {
            <div id="todo-list">
                <div class="empty-state">"No todos found. Add one above!"</div>
            </div>
        } else {
            <ul id="todo-list" class="todo-list">
                @for todo in &todos {
                    <li class={format!("todo-item {}", if todo.completed { "completed" } else { "" })}>
                        @if todo.completed {
                            <input
                                type="checkbox"
                                class="todo-checkbox"
                                hx-post={format!("/api/toggle/{}", todo.id)}
                                hx-target="#todo-list"
                                hx-swap="outerHTML"
                                checked
                            />
                        } else {
                            <input
                                type="checkbox"
                                class="todo-checkbox"
                                hx-post={format!("/api/toggle/{}", todo.id)}
                                hx-target="#todo-list"
                                hx-swap="outerHTML"
                            />
                        }
                        <span class="text">{ &todo.text }</span>
                        <button
                            class="delete-btn"
                            hx-post={format!("/api/delete/{}", todo.id)}
                            hx-target="#todo-list"
                            hx-swap="outerHTML"
                            title="Delete"
                        >
                            "✕"
                        </button>
                    </li>
                }
            </ul>
        }
    }
}
