use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 1: Hello World & Strict Quoting
pub fn lesson1() -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                    <style src="/static/lesson1.css" />
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <title>"Lesson 1: Hello World - Azumi"</title>
            </head>
                <body>
                <div class="lesson-container">
                    <header class="lesson-header">
                        <a href="/" class="back-link">"← Back to Lessons"</a>
                        <div class="lesson-number">"Lesson 1"</div>
                        <h1 class="lesson-title">"Hello World & Strict Quoting"</h1>
                        <p class="lesson-subtitle">"Learn the fundamental quoting rules that make Azumi type-safe"</p>
                    </header>

                    <section class="section">
                        <h2 class="section-title">"🎯 What You'll Learn"</h2>
                        <p>"Azumi requires ALL text content and attribute values to be quoted. This prevents lexer ambiguity and enables arbitrary content."</p>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"💻 Rule 1: Quote All Text"</h2>
                        <div class="example-grid">
                            <div class="example-box">
                                <div class="example-label incorrect">"❌ Wrong (Won't Compile)"</div>
                                <pre class="code-block">"<h1>Hello World</h1>"</pre>
                            </div>
                            <div class="example-box">
                                <div class="example-label correct">"✅ Correct"</div>
                                <pre class="code-block">"<h1>\"Hello World\"</h1>"</pre>
                            </div>
                        </div>
                        <div class="highlight-box">
                            <strong>"Why?"</strong>
                            " Without quotes, Rust's lexer sees special characters like "
                            <code>"<"</code>
                            ", "
                            <code>">"</code>
                            ", "
                            <code>"{"</code>
                            " and gets confused. Quotes make everything clear."
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"💻 Rule 2: Quote All Attributes"</h2>
                        <div class="example-grid">
                            <div class="example-box">
                                <div class="example-label incorrect">"❌ Wrong"</div>
                                <pre class="code-block">"<div class=container>"</pre>
                            </div>
                            <div class="example-box">
                                <div class="example-label correct">"✅ Correct"</div>
                                <pre class="code-block">"<div class=\"container\">"</pre>
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"✨ Live Example"</h2>
                        <div class="highlight-box">
                            <h3>"This text is properly quoted!"</h3>
                            <p>"Every piece of text you see here is wrapped in double quotes in the source code."</p>
                            <p>"This is the foundation of Azumi's type safety."</p>
                        </div>
                    </section>

                    <nav class="nav-buttons">
                        <a href="/" class="btn btn-secondary">"← Home"</a>
                        <a href="/lesson-2" class="btn">"Next: CSS Validation →"</a>
                    </nav>
                </div>
            </body>
        </html>
    }
}

pub async fn lesson1_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson1()))
}
