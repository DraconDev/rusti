use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 5: CSS Integration
pub fn lesson5() -> impl azumi::Component {
    html! {
            <!DOCTYPE html>
            <html>
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <title>"Lesson 5: CSS Integration - Azumi"</title>
                </head>
                <body>
                    <div>
                        <a href="/">"← Back to Lessons"</a>
                        <h1>"Lesson 5: CSS Integration"</h1>

                        <section>
                            <h2>"Working Example"</h2>
                            <button>"Click Me"</button>
                        </section>

                        <section>
                            <h2>"Code"</h2>
                            <pre>{"// Adding styles to templates"}
    {"pub fn styled_button() -> impl azumi::Component {"}
    {"    html! {"}
    {"        <style src=\"/static/button.css\" />"}
    {"        <button class=\"btn-primary\">\"Click Me\"</button>"}
    {"    }"}
    {"}"}</pre>
                        </section>

                        <nav>
                            <a href="/lesson-4">"← Previous"</a>
                            <a href="/lesson-6">"Next: Pattern Matching →"</a>
                        </nav>
                    </div>
                </body>
            </html>
        }
}

pub async fn lesson5_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson5()))
}
