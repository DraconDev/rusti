//! Lesson 13: component_composition.rs
//!
//! Building complex UIs from simple components
use azumi::html;


/// Simple card component
#[azumi::component]
fn Card(title: &str, children: impl azumi::Component) {
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

/// Profile component
#[derive(Clone)]
pub struct User {
    pub name: String,
    pub email: String,
    pub avatar: String,
}

pub fn user_profile(user: &User) -> impl azumi::Component {
    html! {
        <div class="user-profile">
            <img src={&user.avatar} alt={format!("Avatar of {}", user.name)} class="avatar" />
            <div class="user-info">
                <h4>{&user.name}</h4>
                <p>{&user.email}</p>
            </div>
        </div>
    }
}

/// Activity component
#[derive(Clone)]
pub struct Activity {
    pub description: String,
    pub timestamp: String,
}

pub fn activity_item(activity: &Activity) -> impl azumi::Component {
    html! {
        <div class="activity-item">
            <p class="activity-desc">{&activity.description}</p>
            <span class="activity-time">{&activity.timestamp}</span>
        </div>
    }
}

/// Complex profile with composition
pub fn complex_profile(user: &User, activities: &[Activity]) -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson13.css" />
        <div class="profile-container">
            @Card(
                title="Profile",
                children=html! {
                    <div>
                        <div class="profile-section">
                            <h4>"Personal Information"</h4>
                            <div class="info-grid">
                                <div class="info-item">
                                    <label>"Name:"</label>
                                    <span>{&user.name}</span>
                                </div>
                                <div class="info-item">
                                    <label>"Email:"</label>
                                    <span>{&user.email}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                }
            )

            @Card(
                title="Recent Activity",
                children=html! {
                    <div class="activities">
                        @for activity in activities {
                            {activity_item(activity)}
                        }
                    </div>
                }
            )

            @Card(
                title="Quick Actions",
                children=html! {
                    <div class="actions">
                        <button class="btn btn-primary">"Edit Profile"</button>
                        <button class="btn btn-secondary">"Change Password"</button>
                        <button class="btn btn-outline">"View Settings"</button>
                    </div>
                }
            )
        </div>
    }
}

/// Component composition demo
pub fn composition_demo() -> impl azumi::Component {
    let user = User {
        name: "Alice Johnson".to_string(),
        email: "alice@example.com".to_string(),
        avatar: "https://via.placeholder.com/80".to_string(),
    };

    let activities = vec![
        Activity {
            description: "Updated profile picture".to_string(),
            timestamp: "2 hours ago".to_string(),
        },
        Activity {
            description: "Posted a new article".to_string(),
            timestamp: "1 day ago".to_string(),
        },
        Activity {
            description: "Commented on a discussion".to_string(),
            timestamp: "3 days ago".to_string(),
        },
    ];

    complex_profile(&user, &activities)
}

/// Nested component composition
pub fn dashboard_with_components() -> impl azumi::Component {
    let stats = vec![
        ("Total Users", "1,234", "up"),
        ("Active Sessions", "89", "stable"),
        ("Revenue", "$12,345", "up"),
        ("Conversion", "3.2%", "down"),
    ];

    html! {
        <style src="/static/pages/lesson13.css" />
        <div class="dashboard">
            <h1>"Dashboard"</h1>
            
            @Card(
                title="Statistics",
                children=html! {
                    <div class="stats-grid">
                        @for (label, value, trend) in &stats {
                            <div class="stat-item">
                                <div class="stat-value">{value}</div>
                                <div class="stat-label">{label}</div>
                                <div class={format!("stat-trend trend-{}", trend)}>
                                    @if *trend == "up" {
                                        "↗️"
                                    } else if *trend == "down" {
                                        "↘️"
                                    } else {
                                        "➡️"
                                    }
                                </div>
                            </div>
                        }
                    </div>
                }
            )

            @Card(
                title="Recent Activity",
                children=html! {
                    <div class="activity-feed">
                        <div class="activity-item">
                            <span class="activity-dot"></span>
                            <div>
                                <p>"New user registered"</p>
                                <span>"5 minutes ago"</span>
                            </div>
                        </div>
                        <div class="activity-item">
                            <span class="activity-dot"></span>
                            <div>
                                <p>"Payment processed"</p>
                                <span>"1 hour ago"</span>
                            </div>
                        </div>
                        <div class="activity-item">
                            <span class="activity-dot"></span>
                            <div>
                                <p>"Report generated"</p>
                                <span>"2 hours ago"</span>
                            </div>
                        </div>
                    </div>
                }
            )
        </div>
    }
}

/// Handler for Axum
pub async fn lesson13_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&composition_demo()))
}

    html! {
        <style src="/static/pages/lesson13.css" />
        <div class="card">
            <h3 class="card-title">{&props.title}</h3>
            <div class="card-content">
                {props.children}
            </div>
        </div>
    }
}

/// Profile component
#[derive(Clone)]
pub struct User {
    pub name: String,
    pub email: String,
    pub avatar: String,
}

pub fn user_profile(user: &User) -> impl azumi::Component {
    html! {
        <div class="user-profile">
            <img src={&user.avatar} alt={format!("Avatar of {}", user.name)} class="avatar" />
            <div class="user-info">
                <h4>{&user.name}</h4>
                <p>{&user.email}</p>
            </div>
        </div>
    }
}

/// Activity component
#[derive(Clone)]
pub struct Activity {
    pub description: String,
    pub timestamp: String,
}

pub fn activity_item(activity: &Activity) -> impl azumi::Component {
    html! {
        <div class="activity-item">
            <p class="activity-desc">{&activity.description}</p>
            <span class="activity-time">{&activity.timestamp}</span>
        </div>
    }
}

/// Complex profile with composition
pub fn complex_profile(user: &User, activities: &[Activity]) -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson13.css" />
        <div class="profile-container">
            @card(CardProps {
                title: "Profile".to_string(),
                children: html! {
                    <div>
                        <div class="profile-section">
                            <h4>"Personal Information"</h4>
                            <div class="info-grid">
                                <div class="info-item">
                                    <label>"Name:"</label>
                                    <span>{&user.name}</span>
                                </div>
                                <div class="info-item">
                                    <label>"Email:"</label>
                                    <span>{&user.email}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                }
            })

            @card(CardProps {
                title: "Recent Activity".to_string(),
                children: html! {
                    <div class="activities">
                        @for activity in &activities {
                            @activity_item(activity)
                        }
                    </div>
                }
            })

            @card(CardProps {
                title: "Quick Actions".to_string(),
                children: html! {
                    <div class="actions">
                        <button class="btn btn-primary">"Edit Profile"</button>
                        <button class="btn btn-secondary">"Change Password"</button>
                        <button class="btn btn-outline">"View Settings"</button>
                    </div>
                }
            })
        </div>
    }
}

/// Component composition demo
pub fn composition_demo() -> impl azumi::Component {
    let user = User {
        name: "Alice Johnson".to_string(),
        email: "alice@example.com".to_string(),
        avatar: "https://via.placeholder.com/80".to_string(),
    };

    let activities = vec![
        Activity {
            description: "Updated profile picture".to_string(),
            timestamp: "2 hours ago".to_string(),
        },
        Activity {
            description: "Posted a new article".to_string(),
            timestamp: "1 day ago".to_string(),
        },
        Activity {
            description: "Commented on a discussion".to_string(),
            timestamp: "3 days ago".to_string(),
        },
    ];

    complex_profile(&user, &activities)
}

/// Nested component composition
pub fn dashboard_with_components() -> impl azumi::Component {
    let stats = vec![
        ("Total Users", "1,234", "up"),
        ("Active Sessions", "89", "stable"),
        ("Revenue", "$12,345", "up"),
        ("Conversion", "3.2%", "down"),
    ];

    html! {
        <style src="/static/pages/lesson13.css" />
        <div class="dashboard">
            <h1>"Dashboard"</h1>
            
            @card(CardProps {
                title: "Statistics".to_string(),
                children: html! {
                    <div class="stats-grid">
                        @for (label, value, trend) in &stats {
                            <div class="stat-item">
                                <div class="stat-value">{value}</div>
                                <div class="stat-label">{label}</div>
                                <div class={format!("stat-trend trend-{}", trend)}>
                                    @if *trend == "up" {
                                        "↗️"
                                    } else if *trend == "down" {
                                        "↘️"
                                    } else {
                                        "➡️"
                                    }
                                </div>
                            </div>
                        }
                    </div>
                }
            })

            @card(CardProps {
                title: "Recent Activity".to_string(),
                children: html! {
                    <div class="activity-feed">
                        <div class="activity-item">
                            <span class="activity-dot"></span>
                            <div>
                                <p>"New user registered"</p>
                                <span>"5 minutes ago"</span>
                            </div>
                        </div>
                        <div class="activity-item">
                            <span class="activity-dot"></span>
                            <div>
                                <p>"Payment processed"</p>
                                <span>"1 hour ago"</span>
                            </div>
                        </div>
                        <div class="activity-item">
                            <span class="activity-dot"></span>
                            <div>
                                <p>"Report generated"</p>
                                <span>"2 hours ago"</span>
                            </div>
                        </div>
                    </div>
                }
            })
        </div>
    }
}

/// Handler for Axum
pub async fn lesson13_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&composition_demo()))
}