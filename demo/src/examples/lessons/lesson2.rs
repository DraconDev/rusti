use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 2: CSS Validation & Scoping
pub fn lesson2() -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Lesson 2: CSS Validation - Azumi"</title>
                <style src="lesson2.css" />
                <style src="/demo/src/lesson2.css" />
            </head>
            <body>
                <div class="lesson-container">
                    <header class="lesson-header">
                        <a href="/" class="back-link">"← Back to Lessons"</a>
                        <div class="lesson-number">"Lesson 2"</div>
                        <h1 class="lesson-title">"CSS Validation & Scoping"</h1>
                        <p class="lesson-subtitle">"Catch CSS typos at compile time and prevent style leakage"</p>
                    </header>

                    <section class="section">
                        <h2 class="section-title">"🎯 What You'll Learn"</h2>
                        <p>"Azumi validates your CSS at compile time. Every class you use MUST be defined. Every class you define MUST be used. Plus, CSS is automatically scoped to prevent leakage."</p>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"💻 External CSS Required"</h2>
                        <div class="highlight-box">
                            <p>"Use "<code>"<style src=\"file.css\" />"</code>" to link CSS files."</p>
                            <pre class="code-block">"html! {\n    <style src=\"card.css\" />\n    <div class=\"card\">\"Content\"</div>\n}"</pre>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"✨ Location-Specific Errors"</h2>
                        <p>"If you use a class that doesn't exist, you get a compiler error at the EXACT location:"</p>
                        <pre class="code-block">"error: CSS class 'typo' is not defined in any CSS file\n  --> src/example.rs:8:19\n   |\n 8 |         <div class=\"typo\">\n   |                    ^^^^^^"</pre>
                        <div class="highlight-box">
                            <strong>"Try it yourself:"</strong>
                            " Change "<code>"demo-card"</code>" to "<code>"demo-cardx"</code>" in this component and run "<code>"cargo check"</code>"!"
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🎨 Automatic Scoping"</h2>
                        <p>"CSS is automatically scoped with a unique "<code>"data-scopeid"</code>" attribute. Styles can't leak between components!"</p>

                        <div class="demo-card">
                            <h3>
                                "This Card is Styled"
                                <span class="scoped-indicator">"SCOPED"</span>
                            </h3>
                            <p>"Inspect this element - you'll see a "<code>"data-s[hash]"</code>" attribute!"</p>
                            <p>"The CSS for "<code>".demo-card"</code>" only affects this component, not the whole page."</p>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"⚠️ No Dead CSS"</h2>
                        <p>"If you define a class but never use it, you get a warning at compile time:"</p>
                        <pre class="code-block">"warning: Class 'unused-class' is defined in CSS but never used in HTML"</pre>
                    </section>

                    <nav class="nav-buttons">
                        <a href="/lesson-1" class="btn btn-secondary">"← Previous: Quoting"</a>
                        <a href="/lesson-3" class="btn">"Next: Global Styles →"</a>
                    </nav>
                </div>
            </body>
        </html>
    }
}

pub async fn lesson2_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson2()))
}
