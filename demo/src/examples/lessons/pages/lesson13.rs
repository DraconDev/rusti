//! Lesson 13: Component Composition
//!
//! Building complex UIs from simple, reusable components

use azumi::html;

// ============================================================================
// SECTION 1: Card Component - The Building Block
// ============================================================================

/// Props for the Card component
#[derive(Clone)]
pub struct CardProps {
    pub title: String,
    pub content: String,
}

/// A reusable card component
/// Cards are fundamental UI building blocks that can contain any content
/// A reusable card component
/// Cards are fundamental UI building blocks that can contain any content
#[azumi::component]
pub fn card(title: String, content: String) -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson13.css" />
        <div class="card">
            <div class="card-header">
                <h3 class="card-title">{title}</h3>
            </div>
            <div class="card-body">
                <p>{content}</p>
            </div>
        </div>
    }
}

// ============================================================================
// SECTION 2: User Profile Components
// ============================================================================

#[derive(Clone)]
pub struct User {
    pub name: String,
    pub email: String,
    pub role: String,
    pub avatar_url: String,
}

/// A compact user profile display component
#[azumi::component]
pub fn user_profile_compact<'a>(user: &'a User) -> impl azumi::Component + 'a {
    html! {
        <style src="/static/pages/lesson13.css" />
        <div class="user-profile-compact">
            <img src={&user.avatar_url} alt={format!("{} avatar", user.name)} class="user-avatar" />
            <div class="user-details">
                <h4 class="user-name">{&user.name}</h4>
                <p class="user-email">{&user.email}</p>
                <span class="user-role">{&user.role}</span>
            </div>
        </div>
    }
}

/// A detailed user profile card
#[azumi::component]
pub fn user_profile_card<'a>(user: &'a User) -> impl azumi::Component + 'a {
    html! {
                <style src="/static/pages/lesson13.css" />
            <div class="user-profile-card">
            <div class="profile-header">
                <img src={&user.avatar_url} alt={format!("{} avatar", user.name)} class="profile-avatar" />
                <div class="profile-info">
                    <h2 class="profile-name">{&user.name}</h2>
                    <p class="profile-email">{&user.email}</p>
                    <span class="profile-role-badge">{&user.role}</span>
                </div>
            </div>
            <div class="profile-stats">
                <div class="stat-item">
                    <span class="stat-value">"127"</span>
                    <span class="stat-label">"Posts"</span>
                </div>
                <div class="stat-item">
                    <span class="stat-value">"1.2k"</span>
                    <span class="stat-label">"Followers"</span>
                </div>
                <div class="stat-item">
                    <span class="stat-value">"43"</span>
                    <span class="stat-label">"Following"</span>
                </div>
            </div>
        </div>
    }
}

// ============================================================================
// SECTION 3: Activity Feed Components
// ============================================================================

#[derive(Clone)]
pub struct Activity {
    pub id: u32,
    pub user_name: String,
    pub action: String,
    pub target: String,
    pub timestamp: String,
}

/// Single activity item component
#[azumi::component]
pub fn activity_item<'a>(activity: &'a Activity) -> impl azumi::Component + 'a {
    html! {
        <style src="/static/pages/lesson13.css" />
        <div class="activity-item">
            <div class="activity-icon">"📝"</div>
            <div class="activity-content">
                <p class="activity-text">
                    <strong>{&activity.user_name}</strong>
                    " "
                    {&activity.action}
                    " "
                    <em>{&activity.target}</em>
                </p>
                <span class="activity-timestamp">{&activity.timestamp}</span>
            </div>
        </div>
    }
}

/// Activity feed component showing multiple activities
#[azumi::component]
pub fn activity_feed<'a>(activities: &'a [Activity]) -> impl azumi::Component + 'a {
    html! {
        <style src="/static/pages/lesson13.css" />
        <div class="activity-feed">
            <h3 class="feed-title">"Recent Activity"</h3>
            <div class="feed-items">
                @for activity in activities {
                    @activity_item(activity=activity)
                }
            </div>
        </div>
    }
}

// ============================================================================
// SECTION 4: Complex Composition Example - Dashboard
// ============================================================================

/// Dashboard demo showing composition of multiple components
#[azumi::component]
pub fn dashboard_demo() -> impl azumi::Component {
    let current_user = User {
        name: "Alice Johnson".to_string(),
        email: "alice.johnson@example.com".to_string(),
        role: "Administrator".to_string(),
        avatar_url: "https://i.pravatar.cc/150?img=1".to_string(),
    };

    let team_members = vec![
        User {
            name: "Bob Smith".to_string(),
            email: "bob.smith@example.com".to_string(),
            role: "Developer".to_string(),
            avatar_url: "https://i.pravatar.cc/150?img=12".to_string(),
        },
        User {
            name: "Carol White".to_string(),
            email: "carol.white@example.com".to_string(),
            role: "Designer".to_string(),
            avatar_url: "https://i.pravatar.cc/150?img=5".to_string(),
        },
        User {
            name: "David Brown".to_string(),
            email: "david.brown@example.com".to_string(),
            role: "Developer".to_string(),
            avatar_url: "https://i.pravatar.cc/150?img=13".to_string(),
        },
    ];

    let recent_activities = vec![
        Activity {
            id: 1,
            user_name: "Bob Smith".to_string(),
            action: "completed".to_string(),
            target: "Feature X implementation".to_string(),
            timestamp: "2 hours ago".to_string(),
        },
        Activity {
            id: 2,
            user_name: "Carol White".to_string(),
            action: "updated".to_string(),
            target: "Design System v2".to_string(),
            timestamp: "5 hours ago".to_string(),
        },
        Activity {
            id: 3,
            user_name: "David Brown".to_string(),
            action: "created".to_string(),
            target: "API Documentation".to_string(),
            timestamp: "1 day ago".to_string(),
        },
        Activity {
            id: 4,
            user_name: "Alice Johnson".to_string(),
            action: "approved".to_string(),
            target: "Budget Request Q4".to_string(),
            timestamp: "2 days ago".to_string(),
        },
    ];

    html! {
        <style src="/static/pages/lesson13.css" />
        <div class="lesson-container">
            <header class="lesson-header">
                <h1 class="lesson-title">"Lesson 13: Component Composition"</h1>
                <p class="lesson-subtitle">
                    "Building complex UIs by composing simple, reusable components"
                </p>
            </header>

            <section class="lesson-section">
                <h2 class="section-title">"Introduction"</h2>
                <p class="section-content">
                    "Component composition is a powerful pattern where you build complex user interfaces "
                    "by combining simple, focused components. This approach promotes code reuse, "
                    "maintainability, and separation of concerns."
                </p>
            </section>

            <section class="lesson-section">
                <h2 class="section-title">"Example 1: Simple Cards"</h2>
                <div class="example-grid">
                    @card(
                        title="Welcome".to_string(),
                        content="This is a simple reusable card component. Cards are versatile building blocks for modern UIs.".to_string()
                    )

                    @card(
                        title="Composability".to_string(),
                        content="By creating small, focused components, we can combine them in various ways to build complex interfaces.".to_string()
                    )

                    @card(
                        title="Reusability".to_string(),
                        content="Each component can be reused across different parts of your application, reducing code duplication.".to_string()
                    )
                </div>
            </section>

            <section class="lesson-section">
                <h2 class="section-title">"Example 2: User Profile Composition"</h2>
                <div class="profile-demo">
                    <div class="demo-item">
                    <div class="demo-item">
                        <h3>"Compact Profile"</h3>
                        @user_profile_compact(user=&current_user)
                    </div>
                    <div class="demo-item">
                        <h3>"Detailed Profile Card"</h3>
                        @user_profile_card(user=&current_user)
                    </div>
                </div>
            </section>

            <section class="lesson-section">
                <h2 class="section-title">"Example 3: Team Dashboard"</h2>
                <div class="dashboard-layout">
                    <div class="dashboard-sidebar">
                        <div class="sidebar-section">
                            <h3>"Your Profile"</h3>
                            <h3>"Your Profile"</h3>
                            @user_profile_card(user=&current_user)
                        </div>
                    </div>

                    <div class="dashboard-main">
                        <div class="dashboard-row">
                            <h3>"Team Members"</h3>
                            <div class="team-grid">
                                @for member in &team_members {
                                    @user_profile_compact(user=member)
                                }
                            </div>
                        </div>

                        <div class="dashboard-row">
                        <div class="dashboard-row">
                            @activity_feed(activities=&recent_activities)
                        </div>
                        </div>

                        <div class="dashboard-row">
                            <h3>"Quick Stats"</h3>
                            <div class="stats-grid">
                                @card(
                                    title="Total Projects".to_string(),
                                    content="12 active projects".to_string()
                                )
                                @card(
                                    title="Tasks Completed".to_string(),
                                    content="87 tasks this week".to_string()
                                )
                                @card(
                                    title="Team Velocity".to_string(),
                                    content="23 points per sprint".to_string()
                                )
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            <section class="lesson-section">
                <h2 class="section-title">"Key Takeaways"</h2>
                <ul class="takeaways-list">
                    <li>"✓ Build small, focused components with clear responsibilities"</li>
                    <li>"✓ Use Props structs to pass data and configuration to components"</li>
                    <li>"✓ Compose complex UIs by combining simple components"</li>
                    <li>"✓ Iterate over collections to render multiple component instances"</li>
                    <li>"✓ Keep components pure and predictable"</li>
                </ul>
            </section>
        </div>
    }
}

/// Axum handler for Lesson 13
pub async fn lesson13_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! { @dashboard_demo() }))
}
