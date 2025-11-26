use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 2: Global Styles & Design Tokens
pub fn lesson2() -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Lesson 2: Global Styles - Azumi"</title>
                <style src="global.css" />
                <style src="lessons.css" />
                <style src="lesson2.css" />
            </head>
            <body>
                <div class="lesson-container">
                    <header class="lesson-header">
                        <a href="/" class="back-link">"← Back to Lessons"</a>
                        <div class="lesson-number">"Lesson 2"</div>
                        <h1 class="lesson-title">"Global Styles & Design Tokens"</h1>
                        <p class="lesson-subtitle">"Share CSS variables across all components with global.css"</p>
                    </header>

                    <section class="section">
                        <h2 class="section-title">"🎯 The global.css Convention"</h2>
                        <p>"Files ending in "<code>"global.css"</code>" are special. They are:"</p>
                        <ul>
                            <li>"❌ "<strong>"NOT scoped"</strong>" - Can style "<code>"body"</code>", "<code>"html"</code>", etc."</li>
                            <li>"✅ "<strong>"Injected first"</strong>" - Before scoped styles"</li>
                            <li>"⚠️ "<strong>"Skip validation"</strong>" - No class checking required"</li>
                            <li>"🌍 "<strong>"Globally available"</strong>" - CSS variables work everywhere"</li>
                        </ul>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"💻 Design Tokens Pattern"</h2>
                        <div class="highlight-box">
                            <p>"Define design tokens in "<code>"global.css"</code>":"</p>
                            <pre class="code-block">":root {\n    --primary: #4f46e5;\n    --spacing: 1rem;\n}\n\nbody {\n    margin: 0;\n    font-family: system-ui;\n}"</pre>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"✨ Live Design Tokens"</h2>
                        <p>"This page uses design tokens from "<code>"global.css"</code>":"</p>

                        <div class="tokens-grid">
                            <div class="token-box">
                                <div class="token-label">"Primary Color"</div>
                                <div class="token-value">"--azumi-primary"</div>
                                <div class="color-preview" style="background: var(--azumi-primary);"></div>
                            </div>
                            <div class="token-box">
                                <div class="token-label">"Secondary Color"</div>
                                <div class="token-value">"--azumi-secondary"</div>
                                <div class="color-preview" style="background: var(--azumi-secondary);"></div>
                            </div>
                            <div class="token-box">
                                <div class="token-label">"Surface Color"</div>
                                <div class="token-value">"--azumi-surface"</div>
                                <div class="color-preview" style="background: var(--azumi-surface);"></div>
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🎨 Using Global Variables"</h2>
                        <p>"Scoped CSS can use global variables:"</p>
                        <pre class="code-block">".btn {\n    background: var(--azumi-primary);\n    padding: var(--spacing-md);\n}"</pre>
                        <div class="highlight-box">
                            <button class="demo-button-primary">"Primary Button"</button>
                            <button class="demo-button-secondary">"Secondary Button"</button>
                            <p>"These buttons use colors from "<code>"global.css"</code>"!"</p>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"✅ Best Practices"</h2>
                        <div class="highlight-box">
                            <p><strong>"Use global.css for:"</strong></p>
                            <ul>
                                <li>"✅ CSS variables ("<code>":root { --var: value; }"</code>")"</li>
                                <li>"✅ Resets ("<code>"*"</code>", "<code>"body"</code>", "<code>"html"</code>")"</li>
                                <li>"✅ Font faces ("<code>"@font-face"</code>")"</li>
                            </ul>
                            <p><strong>"Don't use global.css for:"</strong></p>
                            <ul>
                                <li>"❌ Component-specific classes"</li>
                                <li>"❌ Layouts"</li>
                            </ul>
                        </div>
                    </section>

                    <nav class="nav-buttons">
                        <a href="/lesson-1" class="btn btn-secondary">"← Previous: CSS Validation"</a>
                        <a href="/lesson-3" class="btn">"Next: Control Flow →"</a>
                    </nav>
                </div>
            </body>
        </html>
    }
}

pub async fn lesson2_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson2()))
}
