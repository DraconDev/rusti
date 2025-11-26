use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 4: Loops and Iteration
pub fn lesson4() -> impl azumi::Component {
    html! {
            <!DOCTYPE html>
            <html>
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <title>"Lesson 4: Loops and Iteration - Azumi"</title>
                </head>
                <body>
                    <div>
                        <a href="/">"← Back to Lessons"</a>
                        <h1>"Lesson 4: Loops and Iteration"</h1>

                        <section>
                            <h2>"Working Example"</h2>
                            <ul>
                                <li>"Buy groceries"</li>
                                <li>"Clean the house"</li>
                                <li>"Call mom"</li>
                            </ul>
                        </section>

                        <section>
                            <h2>"Code"</h2>
                            <pre>{"// Using @for to render lists"}
    {"pub fn todo_list(todos: &[Todo]) -> impl azumi::Component {"}
    {"    html! {"}
    {"        <ul>"}
    {"            @for todo in todos {"}
    {"                <li>{&todo.text}</li>"}
    {"            }"}
    {"        </ul>"}
    {"    }"}
    {"}"}</pre>
                        </section>

                        <nav>
                            <a href="/lesson-3">"← Previous"</a>
                            <a href="/lesson-5">"Next: CSS Integration →"</a>
                        </nav>
                    </div>
                </body>
            </html>
        }
}

pub async fn lesson4_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson4()))
}
