use axum::{
    response::{Html, IntoResponse},
    Form,
};
use rusti::rusti;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TodoForm {
    task: String,
}

/// HTMX-powered todo list - server-side rendering
pub fn htmx_todo() -> impl rusti::Component {
    rusti! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <title>"HTMX Todo - Rusti 2.0"</title>
                <script src="https://unpkg.com/htmx.org@1.9.10" />
                <style src="demo/static/todo.css" />
                <style>
                    .todo-item {
                        display: flex;
                        align-items: center;
                        gap: 10px;
                    }
                </style>
                <script>
                    console.log("HTMX Todo loaded");
                </script>
            </head>
            <body>
                <div class="container">
                    <h1>"📝 HTMX Todo List"</h1>
                    <p class="subtitle">"Server-side rendered with Rusti 2.0"</p>

                    <form hx:post="/api/todos" hx:target="#todo-list" hx:swap="beforeend" class="add-form">
                        <input
                            type="text"
                            name="task"
                            placeholder="What needs to be done?"
                            required
                        />
                        <button type="submit" class="btn-primary">"Add"</button>
                    </form>

                    <div id="todo-list" class="todo-list">
                        @todo_item("Learn Rusti 2.0", 1)
                        @todo_item("Build something awesome", 2)
                    </div>
                </div>
            </body>
        </html>
    }
}

fn todo_item<'a>(text: &'a str, id: u32) -> impl rusti::Component + 'a {
    let delete_url = format!("/api/todos/{}", id);

    rusti! {
        <div class="todo-item" id={format!("todo-{}", id)}>
            <input type="checkbox" class="todo-checkbox" />
            <span class="todo-text">{text}</span>
            <button
                class="btn-delete"
                hx:delete={delete_url}
                hx:target={format!("#todo-{}", id)}
                hx:swap="outerHTML"
            >
                "×"
            </button>
        </div>
    }
}

pub async fn htmx_todo_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&htmx_todo()))
}

pub async fn add_todo_handler(Form(form): Form<TodoForm>) -> impl IntoResponse {
    // In real app, save to database and get real ID
    let id = rand::random::<u32>();
    Html(rusti::render_to_string(&todo_item(&form.task, id)))
}

pub async fn delete_todo_handler() -> impl IntoResponse {
    // Return empty response - HTMX will remove the element
    Html("")
}
