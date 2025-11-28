//! Lesson 23: Global CSS & CDN Links
//!
//! Using global CSS and external CDN stylesheets

use azumi::html;

#[azumi::component]
pub fn global_css_demo() -> impl azumi::Component {
    html! {
        // Global CSS - no scoping, no validation
        <style src="/static/global.css" />

        // Component CSS - scoped automatically
        <style src="/static/pages/lesson23.css" />

        // CDN link - allowed for external libraries
        <link href="https://cdn.jsdelivr.net/npm/animate.css@4.1.1/animate.min.css" rel="stylesheet" />

        <div class="container">
            <h1>"Lesson 23: Global CSS & CDN"</h1>

            <h2>"Global CSS"</h2>
            <p class="description">
                "Files named " <code>"global.css"</code> " bypass scoping and validation:"
            </p>
            <div class="global-styled">
                "This uses global CSS (no data-s attribute)"
            </div>

            <h2>"Scoped Component CSS"</h2>
            <div class="component-styled">
                "This uses scoped CSS (has data-s attribute)"
            </div>

            <h2>"CDN Stylesheets"</h2>
            <div class="animate__animated animate__bounce">
                "Bouncing with animate.css from CDN!"
            </div>

            <div class="rules">
                <h3>"Rules:"</h3>
                <ul>
                    <li>"✅ " <code>"global.css"</code> " - no scoping, no validation"</li>
                    <li>"✅ CDN links (https://) - allowed"</li>
                    <li>"✅ Component CSS - scoped automatically"</li>
                    <li>"❌ Local " <code>"<link>"</code> " to /static - use " <code>"<style src>"</code></li>
                </ul>
            </div>
        </div>
    }
}

pub async fn lesson23_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! { @global_css_demo() }))
}
