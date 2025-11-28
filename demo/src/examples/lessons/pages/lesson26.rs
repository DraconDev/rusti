//! Lesson 26: Multiple CSS Files
//!
//! Loading multiple stylesheets in one component

use azumi::html;

#[azumi::component]
pub fn multiple_css_demo() -> impl azumi::Component {
    html! {
        // Load multiple CSS files - all get scoped
        <style src="/static/pages/lesson26-base.css" />
        <style src="/static/pages/lesson26-theme.css" />
        <style src="/static/pages/lesson26-layout.css" />

        <div class="container">
            <h1 class="title">"Lesson 26: Multiple CSS Files"</h1>
            <p class="description">
                "Load multiple stylesheets - each gets scoped independently"
            </p>

            <div class="card">
                <div class="card-header">
                    <h2>"Card Component"</h2>
                </div>
                <div class="card-body">
                    <p>"This component uses 3 separate CSS files:"</p>
                    <ul>
                        <li><code>"lesson26-base.css"</code> " - typography"</li>
                        <li><code>"lesson26-theme.css"</code> " - colors"</li>
                        <li><code>"lesson26-layout.css"</code> " - grid/spacing"</li>
                    </ul>
                </div>
            </div>

            <div class="grid">
                <div class="grid-item primary">
                    "Primary"
                </div>
                <div class="grid-item secondary">
                    "Secondary"
                </div>
                <div class="grid-item accent">
                    "Accent"
                </div>
            </div>

            <div class="note">
                <strong>"Why multiple files?"</strong>
                <ul>
                    <li>"Separate concerns (base, theme, layout)"</li>
                    <li>"Reuse base styles across components"</li>
                    <li>"Easier theme switching"</li>
                    <li>"Better organization for large components"</li>
                </ul>
                <p>"All files are scoped to this component automatically!"</p>
            </div>
        </div>
    }
}

pub async fn lesson26_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! { @multiple_css_demo() }))
}
