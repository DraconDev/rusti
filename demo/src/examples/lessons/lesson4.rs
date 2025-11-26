use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 4: Control Flow
pub fn lesson4() -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Lesson 4: Control Flow - Azumi"</title>
                <style src="/static/global.css" />
                <style src="lessons.css" />
                <style src="lesson4.css" />
            </head>
            <body>
                <div class="lesson-container">
                    <header class="lesson-header">
                        <a href="/" class="back-link">"← Back to Lessons"</a>
                        <div class="lesson-number">"Lesson 4"</div>
                        <h1 class="lesson-title">"Control Flow"</h1>
                        <p class="lesson-subtitle">"Use Rust's native control flow directly in templates"</p>
                    </header>

                    <section class="section">
                        <h2 class="section-title">"🎯 What You'll Learn"</h2>
                        <p>"Azumi supports Rust's native control flow: <code>@if</code>, <code>@for</code>, <code>@match</code>, and <code>@let</code>. Write your logic in Rust, not in templates!"</p>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"💻 @if / @else"</h2>
                        <p>"Conditional rendering with Rust expressions:"</p>
                        <pre class="code-block">"@if logged_in {\n    <button>\"Log Out\"</button>\n} else {\n    <button>\"Log In\"</button>\n}"</pre>
                        
                        <div class="control-demo">
                            <h3>"Demo: User Status"</h3>
                            <p>"This shows how <code>@if</code> works:"</p>
                            <div style="margin: 1rem 0;">
                                <span class="status-badge status-active">"User is logged in"</span>
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🔄 @for Loops"</h2>
                        <p>"Iterate over collections directly:"</p>
                        <pre class="code-block">"<ul>\n    @for item in items {\n        <li>{item.name}</li>\n    }\n</ul>"</pre>
                        
                        <div class="control-demo">
                            <h3>"Demo: Item List"</h3>
                            <ul class="item-list">
                                <li>"First item"</li>
                                <li>"Second item"</li>
                                <li>"Third item"</li>
                            </ul>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🎯 @match Expressions"</h2>
                        <p>"Pattern matching for complex conditions:"</p>
                        <pre class="code-block">"@match status {\n    Status::Active => { <span class=\"green\">\"Active\"</span> }\n    Status::Pending => { <span class=\"orange\">\"Pending\"</span> }\n    _ => { <span>\"Unknown\"</span> }\n}"</pre>
                        
                        <div class="control-demo">
                            <h3>"Demo: Status Badges"</h3>
                            <p>"Different statuses rendered with <code>@match</code>:"</p>
                            <div style="margin: 1rem 0; display: flex; gap: 1rem; flex-wrap: wrap;">
                                <span class="status-badge status-active">"Active"</span>
                                <span class="status-badge status-pending">"Pending"</span>
                                <span class="status-badge status-inactive">"Inactive"</span>
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"📝 @let Bindings"</h2>
                        <p>"Create local variables for complex expressions:"</p>
                        <pre class="code-block">"@let formatted_date = format_date(&post.created_at);\n<p>\"Published on \" {formatted_date}</p>"</pre>
                        
                        <div class="control-demo">
                            <h3>"Demo: Formatted Content"</h3>
                            <p>"Published on November 26, 2025"</p>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"✨ Best Practices"</h2>
                        <div class="highlight-box">
                            <p><strong>"Control flow tips:"</strong></p>
                            <ul>
                                <li>"✅ Use <code>@if</code> for simple boolean conditions"</li>
                                <li>"✅ Use <code>@for</code> for iterating over collections"</li>
                                <li>"✅ Use <code>@match</code> for enum variants or complex patterns"</li>
                                <li>"✅ Use <code>@let</code> to avoid repetitive calculations"</li>
                                <li>"❌ Don't put business logic in templates - keep it in Rust!"</li>
                            </ul>
                        </div>
                    </section>

                    <nav class="nav-buttons">
                        <a href="/lesson-3" class="btn btn-secondary">"← Previous: Global Styles"</a>
                        <a href="/" class="btn">"Finish: Back to Home →"</a>
                    </nav>
                </div>
            </body>
        </html>
    }
}

pub async fn lesson4_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson4()))
}
