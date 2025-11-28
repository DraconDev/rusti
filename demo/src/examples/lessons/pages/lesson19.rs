//! Lesson 19: Skip Links & ARIA
//!
//! Essential accessibility patterns

use azumi::html;

#[azumi::component]
pub fn accessibility_demo() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson19.css" />

        // Skip link - only visible on focus
        <a href="#main" class="skip-link">"Skip to main content"</a>

        <nav aria-label="Primary navigation">
            <ul>
                <li><a href="/" aria-current="page">"Home"</a></li>
                <li><a href="/about">"About"</a></li>
            </ul>
        </nav>

        <main id="main">
            <h1>"Lesson 19: Accessibility Essentials"</h1>

            <h2>"Form Labels"</h2>
            <div class="form-example">
                <label for="email">"Email" <span class="required">"*"</span></label>
                <input
                    type="email"
                    id="email"
                    aria-required="true"
                    aria-describedby="email-hint"
                />
                <small id="email-hint">"We'll never share your email"</small>
            </div>

            <h2>"Button Labels"</h2>
            <div class="button-examples">
                // ✅ Text content
                <button>"Save"</button>

                // ✅ aria-label for icon buttons
                <button aria-label="Close dialog" class="icon-btn">"×"</button>
            </div>

            <h2>"Live Regions"</h2>
            <div role="status" aria-live="polite" class="status">
                "✓ Changes saved!"
            </div>
        </main>

        <footer role="contentinfo">
            <p>"© 2024 Azumi"</p>
        </footer>
    }
}

pub async fn lesson19_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! { @accessibility_demo() }))
}
