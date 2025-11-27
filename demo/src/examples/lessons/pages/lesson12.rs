//! Lesson 12: component_with_children.rs
//!
//! Passing content to components
use azumi::html;

/// Card component with children content
#[derive(Clone)]
pub struct CardProps<'a> {
    pub title: &'a str,
}

/// Card wrapper that accepts children content
pub fn card<'a>(
    title: &'a str,
    children: impl azumi::Component + 'a,
) -> impl azumi::Component + 'a {
    html! {
        <style src="/static/pages/lesson12.css" />
        <div class="card">
            <h3 cla ss="card-title">{title}</h3>
            <div class="card-content">
                {children}
            </div>
        </div>
    }
}

/// Handler for Axum
pub async fn lesson12_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&example_card()))
}

/// Example card with content
pub fn example_card() -> impl azumi::Component {
    card(
        "Example Card",
        html! {
            <p>"This card has children content."</p>
            <ul>
                <li>"Item 1"</li>
                <li>"Item 2"</li>
            </ul>
        },
    )
}

/// Simple card
pub fn simple_card() -> impl azumi::Component {
    card("Simple Card", html! { <p>"Simple content."</p> })
}
