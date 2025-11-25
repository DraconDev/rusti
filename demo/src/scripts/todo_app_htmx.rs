use rusti::rusti;

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


fn add_todo() -> impl rusti::Component {
    
