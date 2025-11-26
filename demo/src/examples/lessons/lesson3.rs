use axum::response::{Html, IntoResponse};
use azumi::html;

#[derive(Debug, Clone, Copy)]
enum Status {
    Active,
    Pending,
    Inactive,
}

/// Lesson 3: Control Flow
pub fn lesson3() -> impl azumi::Component {
    let is_logged_in = true;
    let user_role = "admin";
    let status = Status::Active;
    let items = vec!["TypeScript", "Rust", "Python"];

    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Lesson 3: Control Flow - Azumi"</title>
                <style src="global.css" />
                <style src="lessons.css" />
                <style src="lesson3.css" />
            </head>
            <body>
                <div class="lesson-container">
                    <header class="lesson-header">
                        <a href="/" class="back-link">"← Back to Lessons"</a>
                        <div class="lesson-number">"Lesson 3"</div>
                        <h1 class="lesson-title">"Control Flow"</h1>
                        <p class="lesson-subtitle">"Use Rust's native control flow directly in templates"</p>
                    </header>

                    <section class="section">
                        <h2 class="section-title">"🎯 What You'll Learn"</h2>
                        <p>"Azumi supports Rust's native control flow: "<code>"@if"</code>", "<code>"@for"</code>", "<code>"@match"</code>", and "<code>"@let"</code>"."</p>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"💻 @if / @else"</h2>
                        <div class="highlight-box">
                            <pre class="code-block">"@if is_logged_in {\n    <p>\"Welcome back!\"</p>\n} else {\n    <p>\"Please log in\"</p>\n}"</pre>
                        </div>
                        <div class="control-demo">
                            <strong>"Live Example:"</strong>
                            @if is_logged_in {
                                <p>"✅ Welcome back! You are logged in."</p>
                            } else {
                                <p>"❌ Please log in to continue."</p>
                            }
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🔄 @for Loops"</h2>
                        <div class="highlight-box">
                            <pre class="code-block">"@for item in items {\n    <li>{item}</li>\n}"</pre>
                        </div>
                        <div class="control-demo">
                            <strong>"Programming Languages:"</strong>
                            <ul class="item-list">
                                @for item in &items {
                                    <li>{"📝 "}{item}</li>
                                }
                            </ul>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🎯 @match Expressions"</h2>
                        <div class="highlight-box">
                            <pre class="code-block">"@match status {\n    Status::Active => { <span>\"Active\"</span> }\n    Status::Pending => { <span>\"Pending\"</span> }\n    Status::Inactive => { <span>\"Inactive\"</span> }\n}"</pre>
                        </div>
                        <div class="control-demo">
                            <strong>"Current Status: "</strong>
                            @match status {
                                Status::Active => {
                                    <span class="status-badge status-active">"Active"</span>
                                }
                                Status::Pending => {
                                    <span class="status-badge status-pending">"Pending"</span>
                                }
                                Status::Inactive => {
                                    <span class="status-badge status-inactive">"Inactive"</span>
                                }
                            }
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"📝 @let Bindings"</h2>
                        <div class="highlight-box">
                            <pre class="code-block">"@let message = format!(\"Hello, {}!\", name);\n<p>{message}</p>"</pre>
                        </div>
                        <div class="control-demo">
                            @let greeting = format!("Hello, {}!", user_role);
                            <p>{greeting}</p>
                        </div>
                    </section>

                    <nav class="nav-buttons">
                        <a href="/lesson-2" class="btn btn-secondary">"← Previous: Global Styles"</a>
                        <a href="/" class="btn">"Home →"</a>
                    </nav>
                </div>
            </body>
        </html>
    }
}

pub async fn lesson3_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson3()))
}
