use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 1: Hello World Template
pub fn lesson1() -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Lesson 1: Hello World - Azumi"</title>
                <style src="/static/pages/lesson1.css" />
            </head>
            <body>
                <div class="lesson-container">
                    <header class="lesson-header">
                        <a href="/" class="back-link">"← Back to Lessons"</a>
                        <div class="lesson-number">"Lesson 1"</div>
                        <h1 class="lesson-title">"Hello World Template"</h1>
                    </header>

                    <section class="section">
                        <h2 class="section-title">"Working Example"</h2>
                        <div class="demo-output">
                            <div class="hello-demo">
                                <h1>"Hello, World!"</h1>
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"Code"</h2>
                        <pre class="code-block">{"pub fn hello_world() -> impl azumi::Component {"}
    {"    html! {"}
    {"        <div>"}
    {"            <h1>\"Hello, World!\"</h1>"}
    {"        </div>"}
    {"    }"}
    {"}"}</pre>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"Key Points"</h2>
                        <ul>
                            <li>"Every template starts with <code>html!</code> macro"</li>
                            <li>"All text content must be quoted"</li>
                            <li>"Returns <code>impl azumi::Component</code>"</li>
                        </ul>
                    </section>

                    <nav class="nav-buttons">
                        <a href="/" class="btn btn-secondary">"← Home"</a>
                        <a href="/lesson-2" class="btn">"Next: Data Binding →"</a>
                    </nav>
                </div>
            </body>
        </html>
    }
}

pub async fn lesson1_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson1()))
}
