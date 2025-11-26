use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 9: Advanced List Processing
pub fn lesson9() -> impl azumi::Component {
    html! {
            <!DOCTYPE html>
            <html>
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <title>"Lesson 9: Advanced List Processing - Azumi"</title>
                </head>
                <body>
                    <div>
                        <a href="/">"← Back to Lessons"</a>
                        <h1>"Lesson 9: Advanced List Processing"</h1>

                        <section>
                            <h2>"Working Example"</h2>
                            <ul>
                                <li>"Item 2"</li>
                            </ul>
                        </section>

                        <section>
                            <h2>"Code"</h2>
                            <pre>{"// Complex data transformations"}
    {"pub fn filtered_search(items: &[Item], query: &str) -> impl azumi::Component {"}
    {"    html! {"}
    {"        <div>"}
    {"            @let filtered = items.iter()"}
    {"                .filter(|item| item.name.contains(query))"}
    {"                .collect::<Vec<_>>();"}
    {"            "}
    {"            @if filtered.is_empty() {"}
    {"                <p>\"No results found\"</p>"}
    {"            } else {"}
    {"                <ul>"}
    {"                    @for item in filtered {"}
    {"                        <li>{&item.name}</li>"}
    {"                    }"}
    {"                </ul>"}
    {"            }"}
    {"        </div>"}
    {"    }"}
    {"}"}</pre>
                        </section>

                        <nav>
                            <a href="/lesson-8">"← Previous"</a>
                            <a href="/lesson-10">"Next: Error States and Loading →"</a>
                        </nav>
                    </div>
                </body>
            </html>
        }
}

pub async fn lesson9_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson9()))
}
