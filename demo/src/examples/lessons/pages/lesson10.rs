use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 10: Error States and Loading
pub fn lesson10() -> impl azumi::Component {
    html! {
            <!DOCTYPE html>
            <html>
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <title>"Lesson 10: Error States and Loading - Azumi"</title>
                </head>
                <body>
                    <div>
                        <a href="/">"← Back to Lessons"</a>
                        <h1>"Lesson 10: Error States and Loading"</h1>

                        <section>
                            <h2>"Working Example"</h2>
                            <p>"Error: Failed to load data"</p>
                        </section>

                        <section>
                            <h2>"Code"</h2>
                            <pre>{"// Handling different data states"}
    {"pub fn data_view(data: &Result<Vec<Item>, String>) -> impl azumi::Component {"}
    {"    html! {"}
    {"        <div>"}
    {"            @match data {"}
    {"                Ok(items) => {"}
    {"                    <ul>"}
    {"                        @for item in items {"}
    {"                            <li>{&item.name}</li>"}
    {"                        }"}
    {"                    </ul>"}
    {"                },"}
    {"                Err(error) => <p>{\"Error: \" error}</p>,"}
    {"            }"}
    {"        </div>"}
    {"    }"}
    {"}"}</pre>
                        </section>

                        <nav>
                            <a href="/lesson-9">"← Previous"</a>
                            <a href="/lesson-11">"Next: Simple Component →"</a>
                        </nav>
                    </div>
                </body>
            </html>
        }
}

pub async fn lesson10_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson10()))
}
