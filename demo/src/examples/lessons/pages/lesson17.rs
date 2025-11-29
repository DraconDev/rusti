//! Lesson 17: HTMX Integration

use azumi::html;

/// HTMX Todo app demo
#[azumi::component]
pub fn todo_with_htmx() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson17.css" />
        <div class="lesson17-container">
            <header class="lesson17-header">
                <h1 class="lesson17-title">"Lesson 17: HTMX Integration"</h1>
                <p class="lesson17-subtitle">"Interactive apps with zero JavaScript"</p>
            </header>

            <section class="lesson17-section">
                <h2 class="section-title">"HTMX Todo Demo"</h2>
                <p class="section-text">"This demonstrates HTMX attributes working with server-side rendering. Click the buttons to see interactivity without writing JavaScript!"</p>

                <div class="todo-container">
                    <div class="todo-item">
                        <input type="checkbox" class="todo-checkbox" />
                        <span class="todo-text">"Learn Azumi basics"</span>
                        <button
                            class="todo-delete"
                            hx-delete="/api/todos/1"
                            hx-target="closest .todo-item"
                            hx-swap="outerHTML swap:0.3s"
                        >"Delete"</button>
                    </div>

                    <div class="todo-item">
                        <input type="checkbox" class="todo-checkbox" checked />
                        <span class="todo-text">"Build HTMX app"</span>
                        <button
                            class="todo-delete"
                            hx-delete="/api/todos/2"
                            hx-target="closest .todo-item"
                            hx-swap="outerHTML swap:0.3s"
                        >"Delete"</button>
                    </div>
                </div>

                <div class="info-box note">
                    <strong>"Note:"</strong>" These HTMX attributes work when connected to a backend. The demo shows the markup structure."
                </div>
            </section>

            <section class="lesson17-section">
                <h2 class="section-title">"Key HTMX Attributes"</h2>
                <ul class="lesson17-list">
                    <li><code>"hx-get"</code>" - Load content from a URL"</li>
                    <li><code>"hx-post"</code>" - Send POST request to URL"</li>
                    <li><code>"hx-delete"</code>" - Send DELETE request"</li>
                    <li><code>"hx-target"</code>" - CSS selector for where to put response"</li>
                    <li><code>"hx-swap"</code>" - How to swap content (innerHTML, outerHTML, etc.)"</li>
                </ul>
            </section>

            <section class="lesson17-section">
                <h2 class="section-title">"Why HTMX + Azumi?"</h2>
                <ul class="lesson17-list">
                    <li>"Type-safe HTML at compile time"</li>
                    <li>"Server-side rendering with Axum"</li>
                    <li>"Progressive enhancement with HTMX"</li>
                    <li>"No JavaScript build step"</li>
                    <li>"Full interactivity without SPA complexity"</li>
                </ul>
            </section>

            <div class="lesson-nav">
                <a href="/" class="nav-btn secondary">"← Back to Lessons"</a>
                <a href="/lesson-18" class="nav-btn">"Next Lesson →"</a>
            </div>
        </div>
    }
}

pub async fn lesson17_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! { @todo_with_htmx() }))
}

// HTMX API endpoint - returns empty to remove todo item
pub async fn delete_todo_handler() -> impl axum::response::IntoResponse {
    // In a real app, this would delete from database
    // Return empty HTML to remove the element
    axum::response::Html("")
}
