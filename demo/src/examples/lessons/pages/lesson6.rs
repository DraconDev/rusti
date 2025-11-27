use azumi::prelude::*;

// 1. Define an enum for pattern matching
#[derive(Debug, Clone, PartialEq)]
pub enum UserRole {
    Admin,
    Moderator,
    User,
    Guest,
}

// 2. Define a struct to hold user data
#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub role: UserRole,
}

pub fn lesson6() -> impl azumi::Component {
    // Create some sample users
    let users = vec![
        User {
            name: "Alice".to_string(),
            role: UserRole::Admin,
        },
        User {
            name: "Bob".to_string(),
            role: UserRole::Moderator,
        },
        User {
            name: "Charlie".to_string(),
            role: UserRole::User,
        },
        User {
            name: "Dave".to_string(),
            role: UserRole::Guest,
        },
    ];

    html! {
        <style src="/static/pages/lesson6.css" />
        <div class="lesson-container">
            <h1 class="lesson-title">"Lesson 6: Pattern Matching"</h1>
            <p class="lesson-description">
                "Azumi's " <code>"@match"</code> " block allows you to use Rust's powerful pattern matching directly in your templates."
            </p>

            <div class="demo-section">
                <h2>"User Role Badges"</h2>
                <div class="user-list">
                    @for user in users {
                        <div class="user-card">
                            <span class="user-name">{&user.name}</span>
                            // Match on the user's role to render the appropriate badge
                            @match user.role {
                                UserRole::Admin => {
                                    <span class="badge badge-admin">"Admin"</span>
                                    <span class="icon">"🛡️"</span>
                                },
                                UserRole::Moderator => {
                                    <span class="badge badge-mod">"Mod"</span>
                                    <span class="icon">"⚖️"</span>
                                },
                                UserRole::User => {
                                    <span class="badge badge-user">"User"</span>
                                },
                                UserRole::Guest => {
                                    <span class="badge badge-guest">"Guest"</span>
                                    <small>" (Read Only)"</small>
                                },
                            }
                        </div>
                    }
                </div>
            </div>

            <div class="code-preview">
                <h3>"Source Code"</h3>
                <pre><code>
"// Using @match for control flow
@match user.role {
    UserRole::Admin => <span class=\"badge badge-admin\">\"Admin\"</span>,
    UserRole::Moderator => <span class=\"badge badge-mod\">\"Mod\"</span>,
    UserRole::User => <span class=\"badge badge-user\">\"User\"</span>,
    _ => <span class=\"badge badge-guest\">\"Guest\"</span>,
}"
                </code></pre>
            </div>
        </div>
    }
}
