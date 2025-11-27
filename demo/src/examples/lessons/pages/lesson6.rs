//! Lesson 6: pattern_matching.rs
//!
//! Using @match for complex conditions
use azumi::html;

#[derive(Clone, Copy)]
enum Role {
    Admin,
    User,
    Guest,
}

/// User role display using @match
pub fn user_role_display(role: Role) -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson6.css" />
        <div>
            @match role {
                Role::Admin => <span class="badge admin">"Admin"</span>,
                Role::User => <span class="badge user">"User"</span>,
                Role::Guest => <span class="badge guest">"Guest"</span>,
            }
        </div>
    }
}

/// Admin example
pub fn admin_example() -> impl azumi::Component {
    user_role_display(Role::Admin)
}

/// User example
pub fn user_example() -> impl azumi::Component {
    user_role_display(Role::User)
}

/// Guest example
pub fn guest_example() -> impl azumi::Component {
    user_role_display(Role::Guest)
}

/// Handler for Axum
pub async fn lesson6_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&admin_example()))
}
