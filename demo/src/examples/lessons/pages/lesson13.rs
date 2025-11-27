//! Lesson 13: component_composition.rs
//!
//! Building complex UIs from simple components
use azumi::html;

#[derive(Clone)]
pub struct CardProps<'a> {
    pub title: &'a str,
    pub children: impl azumi::Component + 'a,
}

#[derive(Clone)]
pub struct User {
    pub name: String,
    pub email: String,
    pub role: &'a str,
}

/// Reusable card component
pub fn card<'a>(title: &'a str, children: impl azumi::Component + 'a) -> impl azumi::Component + 'a {
    html! {
        <style src="/static/pages/lesson13.css" />
        <div class="card">
            <h3 class="card-title">{title}</h3>
            <div class="card-content">
                {children}
            </div>
        </div>
    }
}

/// User profile composed with card components
pub fn user_profile(user: &User) -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson13.css" />
        <div class="profile-container">
            {card("Profile", html! {
                <div class="profile-info">
                    <p>{"Name: " user.name}</p>
                    <p>{"Email: " user.email}</p>
                    <p>{"Role: " user.role}</p>
                </div>
            })}

            {card("User Stats", html! {
                <div class="stats-grid">
                    <div class="stat">
                        <span class="stat-number">"24"</span>
                        <span class="stat-label">"Posts"</span>
                    </div>
                    <div class="stat">
                        <span class="stat-number">"156"</span>
                        <span class="stat-label">"Followers"</span>
                    </div>
                    <div class="stat">
                        <span class="stat-number">"89"</span>
                        <span class="stat-label">"Following"</span>
                    </div>
                </div>
            })}
        </div>
    }
}

/// Example with mixed content
pub fn dashboard_example() -> impl azumi::Component {
    let user = User {
        name: "Alice Johnson".to_string(),
        email: "alice@example.com".to_string(),
        role: "Administrator",
    };
    
    html! {
        <div>
            <h2>"User Dashboard"</h2>
            {user_profile(&user)}
        </div>
    }
}