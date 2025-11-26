use axum::response::{Html, IntoResponse};
use azumi::html;
use azumi::Component;

pub async fn dashboard_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&dashboard_page()))
}

struct Stat {
    label: &'static str,
    value: &'static str,
    change: &'static str,
    trend: &'static str, // "up" or "down"
}

struct Activity {
    user: &'static str,
    action: &'static str,
    time: &'static str,
}

fn dashboard_page() -> impl Component {
    let stats = vec![
        Stat {
            label: "Total Users",
            value: "12,345",
            change: "+12%",
            trend: "up",
        },
        Stat {
            label: "Revenue",
            value: "$45,678",
            change: "+8%",
            trend: "up",
        },
        Stat {
            label: "Bounce Rate",
            value: "42%",
            change: "-5%",
            trend: "down",
        }, // down is good for bounce rate, but we'll style based on trend
        Stat {
            label: "Active Sessions",
            value: "1,234",
            change: "+23%",
            trend: "up",
        },
    ];

    let activities = vec![
        Activity {
            user: "Alice Smith",
            action: "Created a new project",
            time: "2 mins ago",
        },
        Activity {
            user: "Bob Jones",
            action: "Uploaded a file",
            time: "15 mins ago",
        },
        Activity {
            user: "Charlie Brown",
            action: "Commented on a task",
            time: "1 hour ago",
        },
        Activity {
            user: "Diana Prince",
            action: "Updated profile",
            time: "3 hours ago",
        },
    ];

    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Azumi Dashboard"</title>
                <style src="/static/dashboard.css" />
                <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css" />
            </head>
            <body>
                <div class="dashboard-layout">
                    @sidebar()

                    <main class="main-content">
                        @top_bar()

                        <div class="content-wrapper">
                            <h1 class="page-title">"Dashboard Overview"</h1>

                            <div class="stats-grid">
                                @for stat in &stats {
                                    @stat_card(stat)
                                }
                            </div>

                            <div class="dashboard-grid">
                                <div class="card chart-card">
                                    <div class="card-header">
                                        <h3>"Revenue Overview"</h3>
                                        <button class="btn-icon">"..."</button>
                                    </div>
                                    <div class="chart-placeholder">
                                        <div class="bar" style="height: 60%"></div>
                                        <div class="bar" style="height: 80%"></div>
                                        <div class="bar" style="height: 45%"></div>
                                        <div class="bar" style="height: 90%"></div>
                                        <div class="bar" style="height: 75%"></div>
                                        <div class="bar" style="height: 50%"></div>
                                        <div class="bar" style="height: 65%"></div>
                                    </div>
                                </div>

                                <div class="card activity-card">
                                    <div class="card-header">
                                        <h3>"Recent Activity"</h3>
                                        <a href="#" class="link-sm">"View All"</a>
                                    </div>
                                    <ul class="activity-list">
                                        @for activity in &activities {
                                            @activity_item(activity)
                                        }
                                    </ul>
                                </div>
                            </div>
                        </div>
                    </main>
                </div>
            </body>
        </html>
    }
}

fn sidebar() -> impl Component {
    html! {
        <aside class="sidebar">
            <div class="logo">
                <span class="logo-icon">"⚡"</span>
                <span class="logo-text">"Azumi Admin"</span>
            </div>

            <nav class="nav-menu">
                <a href="/" class="nav-item">
                    <i class="fas fa-home"></i>
                    "Home"
                </a>
                <a href="#" class="nav-item active">
                    <i class="fas fa-chart-line"></i>
                    "Dashboard"
                </a>
                <a href="#" class="nav-item">
                    <i class="fas fa-users"></i>
                    "Users"
                </a>
                <a href="#" class="nav-item">
                    <i class="fas fa-cog"></i>
                    "Settings"
                </a>
            </nav>

            <div class="user-profile">
                <div class="avatar">"AD"</div>
                <div class="user-info">
                    <span class="name">"Admin User"</span>
                    <span class="role">"Super Admin"</span>
                </div>
            </div>
        </aside>
    }
}

fn top_bar() -> impl Component {
    html! {
        <header class="top-bar">
            <div class="search-bar">
                <i class="fas fa-search"></i>
                <input type="text" placeholder="Search..." />
            </div>

            <div class="top-actions">
                <button class="btn-icon">
                    <i class="fas fa-bell"></i>
                    <span class="badge">"3"</span>
                </button>
                <button class="btn-primary">"+ New Report"</button>
            </div>
        </header>
    }
}

fn stat_card<'a>(stat: &'a Stat) -> impl Component + 'a {
    let trend_class = if stat.trend == "up" {
        "trend-up"
    } else {
        "trend-down"
    };
    let trend_icon = if stat.trend == "up" { "↑" } else { "↓" };

    html! {
        <div class="card stat-card">
            <span class="stat-label">{stat.label}</span>
            <div class="stat-value-row">
                <span class="stat-value">{stat.value}</span>
                <span class={format!("stat-change {}", trend_class)}>
                    {trend_icon} {stat.change}
                </span>
            </div>
        </div>
    }
}

fn activity_item<'a>(activity: &'a Activity) -> impl Component + 'a {
    html! {
        <li class="activity-item">
            <div class="activity-icon">"●"</div>
            <div class="activity-content">
                <p class="activity-text">
                    <strong>{activity.user}</strong>
                    " "
                    {activity.action}
                </p>
                <span class="activity-time">{activity.time}</span>
            </div>
        </li>
    }
}
