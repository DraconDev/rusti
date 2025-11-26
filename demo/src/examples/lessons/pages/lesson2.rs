use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 2: Data Binding Basics
pub fn lesson2() -> impl azumi::Component {
    html! {
            <!DOCTYPE html>
            <html>
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <title>"Lesson 2: Data Binding - Azumi"</title>
                </head>
                <body>
                    <div>
                        <a href="/">"← Back to Lessons"</a>
                        <h1>"Lesson 2: Data Binding Basics"</h1>

                        <section>
                            <h2>"Working Example"</h2>
                            <div>
                                <h1>"Hello Alice!"</h1>
                            </div>
                        </section>

                        <section>
                            <h2>"Code"</h2>
                            <pre>{"// Passing data to templates"}
    {"pub fn user_greeting(user: &User) -> impl azumi::Component {"}
    {"    html! {"}
    {"        <div>"}
    {"            <h1>{\"Hello \" user.name \"!\"}</h1>"}
    {"        </div>"}
    {"    }"}
    {"}"}</pre>
                        </section>

                        <nav>
                            <a href="/lesson-1">"← Previous"</a>
                            <a href="/lesson-3">"Next: Conditional Rendering →"</a>
                        </nav>
                    </div>
                </body>
            </html>
        }
}

pub async fn lesson2_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson2()))
}
