use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 1: Text & Quotes - Learn mandatory quoting rules
pub fn lesson1() -> impl azumi::Component {
    let wrong_code = r#"<div>
    <h1>Hello World</h1>
    <p>This text is unquoted</p>
</div>"#;

    let correct_code = r#"<div>
    <h1>"Hello, World!"</h1>
    <p>"This text is properly quoted"</p>
    <p>"Azumi requires quotes for ALL text content"</p>
</div>"#;

    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Lesson 1: Text & Quotes - Azumi 2.0"</title>
                <style src="/static/lesson1.css" />
            </head>
            <body>
                <div class="lesson-container">
                    <header class="lesson-header">
                        <a href="/" class="back-link">"← Back to Lessons"</a>
                        <div class="lesson-info">
                            <span class="level-badge">"Level 1"</span>
                            <span class="time">"3 minutes"</span>
                        </div>
                        <h1>"1. Text & Quotes"</h1>
                        <p class="lesson-subtitle">"Learn the fundamental quoting rules that make Azumi type-safe"</p>
                    </header>

                    <main class="lesson-content">
                        <section class="concept-intro">
                            <h2>"🎯 What You'll Learn"</h2>
                            <p>"Azumi requires ALL text content to be quoted. This prevents lexer ambiguity and enables arbitrary content."</p>
                        </section>

                        <section class="code-example">
                            <h2>"💻 Example: Basic Text"</h2>
                            <div class="example-grid">
                                <div class="code-section">
                                    <h3>"❌ Wrong (Won't Compile)"</h3>
                                    <pre class="code-block">
{wrong_code}</pre>
                                </div>
                                <div class="output-section">
                                    <h3>"❌ Error Message"</h3>
                                    <div class="error-output">
                                        <code>"error: expected quoted text content"</code>
                                        <code>"found: Hello World"</code>
                                        <code>"help: wrap text in quotes like \"Hello World\""</code>
                                    </div>
                                </div>
                            </div>
                        </section>

                        <section class="code-example">
                            <h2>"✅ Correct: Everything Quoted"</h2>
                            <div class="example-grid">
                                <div class="code-section">
                                    <h3>"✅ Correct Code"</h3>
                                    <pre class="code-block">
{correct_code}</pre>
                                </div>
                                <div class="output-section">
                                    <h3>"✅ What It Renders"</h3>
                                    <div class="rendered-output">
                                        <div class="demo-output">
                                            <h1>"Hello, World!"</h1>
                                            <p>"This text is properly quoted"</p>
                                            <p>"Azumi requires quotes for ALL text content"</p>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </section>

                        <section class="try-it-section">
                            <h2>"🚀 Try It Yourself"</h2>
                            <p>"Notice how all text must be wrapped in double quotes. This is the foundation of Azumi's type safety."</p>
                            
                            <div class="progress-indicator">
                                <div class="progress-step completed">"1"</div>
                                <div class="progress-line"></div>
                                <div class="progress-step">"2"</div>
                                <div class="progress-line"></div>
                                <div class="progress-step">"3"</div>
                                <div class="progress-line"></div>
                                <div class="progress-step">"4"</div>
                            </div>
                            
                            <div class="lesson-nav">
                                <a href="/lesson-2" class="next-lesson">"Next: Attributes →"</a>
                            </div>
                        </section>
                    </main>
                </div>
            </body>
        </html>
    }
}

pub async fn lesson1_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson1()))
}