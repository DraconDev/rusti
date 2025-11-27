use azumi::html;

#[derive(Debug, Clone)]
pub enum Widget {
    Chart(Vec<i32>),
    Stats(String, String),
    Alert(String),
}

#[derive(Debug, Clone)]
pub struct DashboardUser {
    pub name: String,
    pub is_admin: bool,
    pub widgets: Vec<Widget>,
}

pub fn lesson8() -> impl azumi::Component {
    let user = DashboardUser {
        name: "Admin Alice".to_string(),
        is_admin: true,
        widgets: vec![
            Widget::Stats("Total Users".to_string(), "1,234".to_string()),
            Widget::Alert("System update scheduled for tonight".to_string()),
            Widget::Chart(vec![10, 25, 15, 30, 45]),
        ],
    };

    html! {
        <style src="/static/pages/lesson8.css" />
        <div class="lesson-container">
            <h1 class="lesson-title">"Lesson 8: Nested Control Flow"</h1>
            <p class="lesson-description">
                "Azumi allows you to nest control flow blocks arbitrarily. You can put " <code>"@match"</code> " inside " <code>"@for"</code> " inside " <code>"@if"</code> ", just like in regular Rust code."
            </p>

            <d iv class="demo-section">
                <h2>"Dashboard"</h2>
                <div class="dashboard">
                    @if user.is_admin {
                        <div class="welcome-banner">
                            "Welcome back, Administrator " {&user.name}
                        </div>

                        <div class="widgets-grid">
                            @for widget in &user.widgets {
                                <div class="widget-card">
                                    @match widget {
                                        Widget::Stats(label, value) => {
                                            <div class="stat-widget">
                                                <span class="stat-label">{label}</span>
                                                <span class="stat-value">{value}</span>
                                            </div>
                                        },
                                        Widget::Alert(msg) => {
                                            <div class="alert-widget">
                                                <span class="alert-icon">"⚠️"</span>
                                                <span class="alert-msg">{msg}</span>
                                            </div>
                                        },
                                        Widget::Chart(data) => {
                                            <div class="chart-widget">
                                                <h4>"Activity Chart"</h4>
                                                <div class="chart-bars">
                                                    @for val in data {
                                                        <div class="bar" --chart-height={val}></div>
                                                    }
                                                </div>
                                            </div>
                                        },
                                    }
                                </div>
                            }
                        </div>
                    } else {
                        <div class="access-denied">
                            "Access Denied. Admin privileges required."
                        </div>
                    }
                </div>
            </d>

            <div class="code-preview">
                <h3>"Source Code"</h3>
                <pre><code>
"@if user.is_admin {
    @for widget in &user.widgets {
        @match widget {
            Widget::Stats(l, v) => { ... },
            Widget::Alert(msg) => { ... },
            Widget::Chart(data) => {
                @for val in data { ... }
            },
        }
    }
}"
                </code></pre>
            </div>
        </div>
    }
}
