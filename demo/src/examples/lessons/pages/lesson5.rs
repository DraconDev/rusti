use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 5: Components with Props
pub fn lesson5() -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Lesson 5: Components with Props - Azumi"</title>
                <style src="/static/page/lessons.css" />
                <style src="/static/page/lesson5.css" />
            </head>
            <body>
                <div class="lesson-container">
                    <header class="lesson-header">
                        <a href="/" class="back-link">"← Back to Lessons"</a>
                        <div class="lesson-number">"Lesson 5"</div>
                        <h1 class="lesson-title">"Components with Props"</h1>
                        <p class="lesson-subtitle">"Create reusable components with type-safe props"</p>
                    </header>

                    <section class="section">
                        <h2 class="section-title">"🎯 What You'll Learn"</h2>
                        <p>"Use <code>#[azumi::component]</code> to create reusable components with type-safe props. Components are just Rust functions that return <code>impl azumi::Component</code>."</p>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"💻 Basic Component"</h2>
                        <p>"A simple component with props:"</p>
                        <pre class="code-block">"#[azumi::component]\nfn UserCard(\n    name: &str,\n    #[prop(default = \"\\\"Member\\\"\")] role: &str,\n) -> impl azumi::Component {\n    html! {\n        <style src=\"user_card.css\" />\n        <div class=\"user-card\">\n            <h2>{name}</h2>\n            <span class=\"role\">{role}</span>\n        </div>\n    }\n}"</pre>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"✨ Live Components"</h2>
                        <p>"Here are some example components in action:"</p>
                        
                        <div class="component-demo-grid">
                            <div class="user-card-demo">
                                <h2>"Alice Johnson"</h2>
                                <span class="role">"Admin"</span>
                            </div>
                            <div class="user-card-demo">
                                <h2>"Bob Smith"</h2>
                                <span class="role">"Member"</span>
                            </div>
                            <div class="user-card-demo">
                                <h2>"Carol Davis"</h2>
                                <span class="role">"Moderator"</span>
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🔧 Component Usage"</h2>
                        <p>"Use components with the <code>@</code> syntax:"</p>
                        <pre class="code-block">"@UserCard(name=\"Alice\", role=\"Admin\")\n@UserCard(name=\"Bob\")  // Uses default role=\"Member\""</pre>
                        
                        <div class="highlight-box">
                            <p><strong>"Component syntax:"</strong></p>
                            <ul>
                                <li>"✅ <code>@ComponentName(prop1=\"value\", prop2=\"value\")</code>"</li>
                                <li>"✅ <code>@ComponentName</code> for components without props"</li>
                                <li>"✅ Default values with <code>#[prop(default = \"value\")]</code>"</li>
                                <li>"✅ Type-safe props - wrong types won't compile!"</li>
                            </ul>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🎨 Component Styling"</h2>
                        <p>"Each component can have its own CSS file:"</p>
                        <pre class="code-block">"/* user_card.css */\n.user-card {\n    background: var(--azumi-surface);\n    border-radius: var(--radius-md);\n    padding: var(--spacing-md);\n    text-align: center;\n}\n\n.role {\n    background: var(--azumi-primary);\n    color: white;\n    padding: 0.25rem 0.5rem;\n    border-radius: 0.25rem;\n    font-size: 0.75rem;\n}"</pre>
                        
                        <div class="highlight-box">
                            <p><strong>"Component benefits:"</strong></p>
                            <ul>
                                <li>"✅ Encapsulated styling - no conflicts"</li>
                                <li>"✅ Reusable across your application"</li>
                                <li>"✅ Type-safe props prevent runtime errors"</li>
                                <li>"✅ Self-contained - easy to test and maintain"</li>
                            </ul>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🚀 Advanced Props"</h2>
                        <p>"Components support complex prop types:"</p>
                        <pre class="code-block">"#[azumi::component]\nfn TodoItem(\n    title: &str,\n    completed: bool,\n    #[prop(default = \"\")] description: &str,\n) -> impl azumi::Component {\n    html! {\n        <div class=\"todo-item\">\n            <input type=\"checkbox\" checked={completed} />\n            <span class={if completed { \"completed\" } else { \"\" }}>{title}</span>\n            @if !description.is_empty() {\n                <p>{description}</p>\n            }\n        </div>\n    }\n}"</pre>
                    </section>

                    <nav class="nav-buttons">
                        <a href="/lesson-4" class="btn btn-secondary">"← Previous: Control Flow"</a>
                        <a href="/lesson-6" class="btn">"Next: HTMX Integration →"</a>
                    </nav>
                </div>
            </body>
        </html>
    }
}

pub async fn lesson5_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson5()))
}