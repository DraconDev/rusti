use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 3: Expressions - Insert Rust variables into templates
pub fn lesson3() -> impl azumi::Component {
    let user_name = "Alice";
    let item_count = 5;
    let is_logged_in = true;
    let current_time = chrono::Utc::now().format("%H:%M").to_string();

    let code_example = r#"let user_name = "Alice";
let item_count = 5;
let is_logged_in = true;

html! {
    <div class="user-card">
        <h1>{"Welcome, "} {user_name} "!"</h1>
        <p>{"You have "} {item_count} " items"</p>
        @if is_logged_in {
            <p>{"Logged in since "} {current_time}</p>
        }
    </div>
}"#;

    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Lesson 3: Expressions - Azumi"</title>
                <style src="/static/lesson1.css" />
            </head>
            <body>
                <div class="lesson-container">
                    <header class="lesson-header">
                        <a href="/" class="back-link">"← Back to Lessons"</a>
                        <div class="lesson-info">
                            <span class="level-badge">"Level 1"</span>
                            <span class="time">"4 minutes"</span>
                        </div>
                        <h1>"3. Expressions"</h1>
                        <p class="lesson-subtitle">"Insert Rust variables and expressions into templates"</p>
                    </header>

                    <main class="lesson-content">
                        <section class="concept-intro">
                            <h2>"🎯 What You'll Learn"</h2>
                            <p>"Azumi allows you to insert Rust variables and expressions directly into your templates using curly braces."</p>
                            <ul>
                                <li>"Use {variable} to insert values"</li>
                                <li>"Mix text and variables in any order"</li>
                                <li>"Expressions are evaluated at compile time"</li>
                                <li>"Full Rust type safety is maintained"</li>
                            </ul>
                        </section>

                        <section class="code-example">
                            <h2>"💻 Example: Variable Interpolation"</h2>
                            <div class="example-grid">
                                <div class="code-section">
                                    <h3>"✅ Code with Variables"</h3>
                                    <pre class="code-block">
{code_example}</pre>
                                </div>
                                <div class="output-section">
                                    <h3>"✅ What It Renders"</h3>
                                    <div class="rendered-output">
                                        <div class="demo-output">
                                            <div class="user-card">
                                                <h1>{"Welcome, "} {user_name} "!"</h1>
                                                <p>{"You have "} {item_count} " items"</p>
                                                @if is_logged_in {
                                                    <p>{"Logged in since "} {current_time}</p>
                                                }
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </section>

                        <section class="code-example">
                            <h2>"🔧 Dynamic Values"</h2>
                            <div class="example-grid">
                                <div class="code-section">
                                    <h3>"✅ Complex Expressions"</h3>
                                    <pre class="code-block">{"let temperature = 25;\nlet is_warm = temperature > 20;\nlet message = format!(\"It's {}°C\", temperature);\n\n<div class=\"weather\">\n    <h3>{message}</h3>\n    @if is_warm {\n        <p>\"Perfect weather!\"</p>\n    }\n</div>"}</pre>
                                </div>
                                <div class="output-section">
                                    <h3>"✅ Live Result"</h3>
                                    <div class="rendered-output">
                                        <div class="demo-output">
                                            @let temperature = 25;
                                            @let is_warm = temperature > 20;
                                            @let message = format!("It's {}°C", temperature);
                                            
                                            <div class="weather">
                                                <h3>{message}</h3>
                                                @if is_warm {
                                                    <p>"Perfect weather!"</p>
                                                }
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </section>

                        <section class="try-it-section">
                            <h2>"🚀 Try It Yourself"</h2>
                            <p>"Variables are interpolated directly into the template, maintaining full type safety and compile-time checking."</p>
                            
                            <div class="progress-indicator">
                                <div class="progress-step completed">"1"</div>
                                <div class="progress-line completed"></div>
                                <div class="progress-step completed">"2"</div>
                                <div class="progress-line completed"></div>
                                <div class="progress-step completed">"3"</div>
                                <div class="progress-line"></div>
                                <div class="progress-step">"4"</div>
                            </div>
                            
                            <div class="lesson-nav">
                                <a href="/lesson-2" class="prev-lesson">"← Previous: Attributes"</a>
                                <a href="/lesson-4" class="next-lesson">"Next: Basic Styling →"</a>
                            </div>
                        </section>
                    </main>
                </div>
            </body>
        </html>
    }
}

pub async fn lesson3_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson3()))
}