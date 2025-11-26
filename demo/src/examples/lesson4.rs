use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 4: Basic Styling - External CSS and automatic scoping
pub fn lesson4() -> impl azumi::Component {
    let css_code = r#"/* component.css */
.card {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    padding: 2rem;
    border-radius: 1rem;
    color: white;
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
}

.card h2 {
    margin: 0 0 1rem 0;
    font-size: 1.5rem;
}

.card p {
    margin: 0;
    opacity: 0.9;
}"#;

    let azumi_code = r#"html! {
    <div class="lesson-container">
        <style src="/static/lesson4.css" />
        <div class="card">
            <h2>{"My Card Title"}</h2>
            <p>{"This card has automatic CSS scoping!"}</p>
        </div>
    </div>
}"#;

    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Lesson 4: Basic Styling - Azumi"</title>
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
                        <h1>"4. Basic Styling"</h1>
                        <p class="lesson-subtitle">"Add external CSS and understand automatic scoping"</p>
                    </header>

                    <main class="lesson-content">
                        <section class="concept-intro">
                            <h2>"🎯 What You'll Learn"</h2>
                            <p>"Azumi uses external CSS files with automatic scoping. This prevents style conflicts and provides full IDE support."</p>
                            <ul>
                                <li>"Use <style src=\"file.css\" /> for local CSS"</li>
                                <li>"CSS is automatically scoped to the component"</li>
                                <li>"No inline styles allowed (for better tooling)"</li>
                                <li>"Full IDE support for CSS files"</li>
                            </ul>
                        </section>

                        <section class="code-example">
                            <h2>"🎨 Step 1: Create CSS File"</h2>
                            <div class="example-grid">
                                <div class="code-section">
                                    <h3>"📄 component.css"</h3>
                                    <pre class="code-block">
{css_code}</pre>
                                </div>
                                <div class="output-section">
                                    <h3>"💡 Key Benefits"</h3>
                                    <div class="benefits-list">
                                        <div class="benefit-item">
                                            <i class="fas fa-shield-alt"></i>
                                            <div>
                                                <h4>"Auto-Scoping"</h4>
                                                <p>"CSS is scoped to prevent conflicts"</p>
                                            </div>
                                        </div>
                                        <div class="benefit-item">
                                            <i class="fas fa-code"></i>
                                            <div>
                                                <h4>"IDE Support"</h4>
                                                <p>"Full syntax highlighting and autocomplete"</p>
                                            </div>
                                        </div>
                                        <div class="benefit-item">
                                            <i class="fas fa-palette"></i>
                                            <div>
                                                <h4>"Reusable"</h4>
                                                <p>"CSS files can be shared between components"</p>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </section>

                        <section class="code-example">
                            <h2>"⚡ Step 2: Use in Component"</h2>
                            <div class="example-grid">
                                <div class="code-section">
                                    <h3>"✅ Azumi Template"</h3>
                                    <pre class="code-block">
{azumi_code}</pre>
                                </div>
                                <div class="output-section">
                                    <h3>"✅ Generated Result"</h3>
                                    <div class="rendered-output">
                                        <div class="demo-output">
                                            <div class="card">
                                                <h2>"My Card Title"</h2>
                                                <p>"This card has automatic CSS scoping!"</p>
                                            </div>
                                        </div>
                                        <div class="scoping-explanation">
                                            <h4>"🛡️ What Azumi Does"</h4>
                                            <ol>
                                                <li>"Reads your CSS file at compile time"</li>
                                                <li>"Generates unique data-scope attributes"</li>
                                                <li>"Applies scope to all matching elements"</li>
                                                <li>"Ensures no style conflicts"</li>
                                            </ol>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </section>

                        <section class="try-it-section">
                            <h2>"🚀 Try It Yourself"</h2>
                            <p>"The CSS is automatically scoped with data attributes. Check the browser dev tools to see the scoping in action!"</p>
                            
                            <div class="progress-indicator">
                                <div class="progress-step completed">"1"</div>
                                <div class="progress-line completed"></div>
                                <div class="progress-step completed">"2"</div>
                                <div class="progress-line completed"></div>
                                <div class="progress-step completed">"3"</div>
                                <div class="progress-line completed"></div>
                                <div class="progress-step completed">"4"</div>
                            </div>
                            
                            <div class="lesson-nav">
                                <a href="/lesson-3" class="prev-lesson">"← Previous: Expressions"</a>
                                <a href="/" class="next-lesson">"Complete! Back to Home →"</a>
                            </div>
                        </section>
                    </main>
                </div>
            </body>
        </html>
    }
}

pub async fn lesson4_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson4()))
}