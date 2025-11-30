//! Lesson 16: JavaScript Integration
//!
//! Loading external JavaScript libraries and inline scripts
use azumi::html;

/// Interactive Chart.js example
#[azumi::component]
pub fn interactive_chart() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson16.css" />
        <div class="lesson16-container">
            <div class="lesson16-header">
                <h1 class="lesson16-title">"Lesson 16: Component Composition"</h1>
                <p class="lesson16-subtitle">"Build complex UIs by combining simple, reusable components"</p>
            </div>

            <div class="lesson16-section">
                <h2 class="section-title">"The Power of Composition"</h2>
                <div class="description">
                    <p>"Component composition is about building complex interfaces from simple, reusable pieces."</p>
                    <div id="test_missing_id_value" class="typo_test_class_value">"This should show error on value, not attribute name"</div>
                </div>
            </div>
            <section class="lesson16-section">
                <h2 class="section-title">"Key Points"</h2>
                <ul class="lesson16-list">
                    <li>"✅ External <script src='CDN'> for libraries"</li>
                    <li>"✅ Inline <script> for custom JS"</li>
                    <li>"✅ No inline styles - use classes"</li>
                    <li>"✅ Azumi validates HTML structure/accessibility"</li>
                    <li>"✅ JS runs after DOM ready"</li>
                </ul>
            </section>
        </div>

        // External Chart.js CDN
        <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>

        // External custom JS
        <script src="/static/lesson16.js"></script>
    }
}

/// Handler
pub async fn lesson16_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&interactive_chart()))
}
