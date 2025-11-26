//! Lesson 3: conditional_rendering.rs
//!
//! Using @if for conditional content
use azumi::html;

/// User status display with conditional rendering
pub fn user_status(is_logged_in: bool) -> impl azumi::Component {
    html! {
        <div>
            @if is_logged_in {
                <p>"Welcome back!"</p>
            } else {
                <p>"Please log in"</p>
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
    user_status(false)
}