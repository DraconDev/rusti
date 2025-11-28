//! Lesson 17: HTMX Integration
//!
//! Building interactive UIs without JavaScript frameworks
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
                <div class="todo-app">
                    <form hx-post="/api/append" hx-swap="beforeend" class="todo-form">
                        <input name="text" type="text" placeholder="Add a todo..." class="todo-input" />
                        <button type="submit" class="btn-add">"Add Todo"</button>
                    </form>

                    <ul id="todo-list" class="todo-list">
                        <li class="todo-item">
                            <span class="todo-text">"Learn Azumi"</span>
                            <button hx-delete="/api/replace" hx-swap="outerHTML" class="btn-delete">"×"</button>
                        </li>
                        <li class="todo-item">
                            <span class="todo-text">"Build HTMX app"</span>
                            <button hx-delete="/api/replace" hx-swap="outerHTML" class="btn-delete">"×"</button>
                        </li>
                    </ul>
                </div>
            </section>

            <section class="lesson17-section">
                <h2 class="section-title">"Key HTMX Attributes"</h2>
                <ul class="lesson17-list">
                    <li>"hx-post - POST request"</li>
                    <li>"hx-swap - DOM swap strategy (beforeend, outerHTML)"</li>
                    <li>"hx-delete - DELETE request"</li>
                    <li>"hx-get - GET request"</li>
                    <li>"hx-trigger - Event triggers"</li>
                </ul>
            </section>

            <section class="lesson17-section">
                <h2 class="section-title">"Why HTMX + Azumi?"</h2>
                <ul class="lesson17-list">
                    <li>"✅ No JS bundle - server-side rendering"</li>
                    <li>"✅ Progressive enhancement"</li>
                    <li>"✅ Type-safe templates with validation"</li>
                    <li>"✅ Automatic CSS scoping"</li>
                    <li>"✅ Zero client-side state"</li>
                </ul>
            </section>
        </div>
    }
}

/// Handler
pub async fn lesson17_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! { @todo_with_htmx() }))
}
