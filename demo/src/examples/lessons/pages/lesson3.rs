use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 3: Conditional Rendering
pub fn lesson3() -> impl azumi::Component {
    html! {
            <!DOCTYPE html>
            <html>
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <title>"Lesson 3: Conditional Rendering - Azumi"</title>
                </head>
                <body>
                    <div>
                        <a href="/">"← Back to Lessons"</a>
                        <h1>"Lesson 3: Conditional Rendering"</h1>

                        <section>
                            <h2>"Working Example"</h2>
                            <div>
                                <p>"Welcome back!"</p>
                            </div>
                        </section>

                        <section>
                            <h2>"Code"</h2>
                            <pre>{"// Using @if for conditional content"}
    {"pub fn user_status(user: &User) -> impl azumi::Component {"}
    {"    html! {"}
    {"        <div>"}
    {"            @if user.is_logged_in {"}
    {"                <p>\"Welcome back!\"</p>"}
    {"            } else {"}
    {"                <p>\"Please log in\"</p>"}
    {"            }"}
    {"        </div>"}
    {"    }"}
    {"}"}</pre>
                        </section>

                        <nav>
                            <a href="/lesson-2">"← Previous"</a>
                            <a href="/lesson-4">"Next: Loops and Iteration →"</a>
                        </nav>
                    </div>
                </body>
            </html>
        }
}

pub async fn lesson3_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson3()))
}
