use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 2: Attributes - Master HTML attribute syntax
pub fn lesson2() -> impl azumi::Component {
    let wrong_code = r#"<div class=container id=test>
    <button disabled>Click me</button>
    <input type=text placeholder=Name>
</div>"#;

    let correct_code = r#"<div class="container" id="test">
    <button disabled>{"Click me"}</button>
    <input type="text" placeholder="Name" />
</div>"#;

    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Lesson 2: Attributes - Azumi"</title>
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
                        <h1>"2. Attributes"</h1>
                        <p class="lesson-subtitle">"Master HTML attribute syntax with proper quoting"</p>
                    </header>

                    <main class="lesson-content">
                        <section class="concept-intro">
                            <h2>"🎯 What You'll Learn"</h2>
                            <p>"All HTML attribute values must be quoted in Azumi. This ensures consistency and prevents ambiguity."</p>
                            <ul>
                                <li>"String attributes must use double quotes"</li>
                                <li>"Boolean attributes like disabled don't need values"</li>
                                <li>"Self-closing tags need the slash notation"</li>
                                <li>"Attribute names use kebab-case"</li>
                            </ul>
                        </section>

                        <section class="code-example">
                            <h2>"💻 Example: Common Attributes"</h2>
                            <div class="example-grid">
                                <div class="code-section">
                                    <h3>"❌ Wrong (Won't Compile)"</h3>
                                    <pre class="code-block">
{wrong_code}</pre>
                                </div>
                                <div class="output-section">
                                    <h3>"❌ Error Message"</h3>
                                    <div class="error-output">
                                        <code>"error: expected quoted attribute value"</code>
                                        <code>"found: container"</code>
                                        <code>"help: wrap attribute values in quotes"</code>
                                    </div>
                                </div>
                            </div>
                        </section>

                        <section class="code-example">
                            <h2>"✅ Correct: All Attributes Quoted"</h2>
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
                                            <div class="container" style="border: 2px solid #6366f1; padding: 1rem; margin: 0.5rem 0;">
                                                <button disabled style="opacity: 0.5; cursor: not-allowed; padding: 0.5rem 1rem; background: #6366f1; color: white; border: none; border-radius: 0.375rem;">"Click me"</button>
                                                <input type="text" placeholder="Name" style="margin-left: 0.5rem; padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 0.375rem;" />
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </section>

                        <section class="try-it-section">
                            <h2>"🚀 Try It Yourself"</h2>
                            <p>"Notice how all attribute values are wrapped in quotes, and boolean attributes like 'disabled' don't need values."</p>
                            
                            <div class="progress-indicator">
                                <div class="progress-step completed">"1"</div>
                                <div class="progress-line completed"></div>
                                <div class="progress-step completed">"2"</div>
                                <div class="progress-line"></div>
                                <div class="progress-step">"3"</div>
                                <div class="progress-line"></div>
                                <div class="progress-step">"4"</div>
                            </div>
                            
                            <div class="lesson-nav">
                                <a href="/lesson-1" class="prev-lesson">"← Previous: Text & Quotes"</a>
                                <a href="/lesson-3" class="next-lesson">"Next: Expressions →"</a>
                            </div>
                        </section>
                    </main>
                </div>
            </body>
        </html>
    }
}

pub async fn lesson2_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson2()))
}