//! Lesson 13: Component Composition
//!
//! Building UIs by composing simple components

use azumi::html;

/// A reusable card component
#[azumi::component]
pub fn card(title: String, content: String) -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson13.css" />
        <div class="card">
            <div class="card-header">
                <h3>{title}</h3>
            </div>
            <div class="card-body">
                <p>{content}</p>
            </div>
        </div>
    }
}

/// Dashboard demo showing composition
#[azumi::component]
pub fn composition_demo() -> impl azumi::Component {
    html! {
        // <style src="/static/pages/lesson13.css" />
        <div class="container">
            <h1>"Lesson 13: Composition"</h1>
            <p>"Building complex UIs from simple blocks"</p>

            <div class="grid">
                @card(
                    title="Welcome".to_string(),
                    content="This is a reusable card component.".to_string()
                )

                @card(
                    title="Statistics".to_string(),
                    content="Active Users: 1,234".to_string()
                )

                @card(
                    title="Status".to_string(),
                    content="System Operational".to_string()
                )
            </div>
        </div>
    }
}

pub async fn lesson13_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! { @composition_demo() }))
}
