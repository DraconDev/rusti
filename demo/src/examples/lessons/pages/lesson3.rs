//! Lesson 3: conditional_rendering.rs
//!
//! Using @if for conditional content
use azumi::html;

/// User status display with conditional rendering
pub fn user_status(is_logged_in: bool) -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson3.css" />
        <div class="lesson3-container">
            @if is_logged_in {
                <p class="lesson3-text success">"Welcome back!"</p>
            } else {
                <p class="lesson3-text warning">"Please log in"</p>
            }
        </div>
    }
}

/// Example usage with logged in user
pub fn logged_in_example() -> impl azumi::Component {
    user_status(true)
}

/// Example usage with guest user
pub fn guest_example() -> impl azumi::Component {
    user_status(false, "Guest")
}

// Handler for Axum
pub async fn lesson3_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&logged_in_example()))
}
