use axum::response::{Html, IntoResponse};
use azumi::html;

/// Advanced pattern matching and Rust-specific features
pub async fn advanced_patterns_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&advanced_patterns_demo()))
}

fn advanced_patterns_demo() -> impl azumi::Component {
    html! {
        <style src="/static/advanced_patterns.css" />
        <html>
            <head>
                <title>"Advanced Patterns - Azumi"</title>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            </head>
            <body>
                <header class="page-header">
                    <h1>"🎭 Advanced Pattern Matching"</h1>
                    <p>"Deep dive into Rust's pattern matching with Azumi"</p>
                    <nav class="nav-links">
                        <a href="/">"🏠 Home"</a>
                        <a href="/control-flow">"🔀 Control Flow"</a>
                        <a href="/let-examples">"🎯 @let Patterns"</a>
                        <a href="/data-processing">"📊 Data Processing"</a>
                    </nav>
                </header>

                <main class="content">
                    <section class="pattern-section">
                        <h2>"🎨 Struct & Enum Pattern Matching"</h2>
                        <p class="section-desc">"Complex nested pattern matching with destructuring"</p>
                        @ComplexPatterns()
                    </section>

                    <section class="pattern-section">
                        <h2>"🔗 Advanced Binding Patterns"</h2>
                        <p class="section-desc">"Advanced destructuring and variable binding"</p>
                        @BindingPatterns()
                    </section>

                    <section class="pattern-section">
                        <h2>"🎯 Guard Conditions"</h2>
                        <p class="section-desc">"Complex conditional matching with guard clauses"</p>
                        @GuardConditions()
                    </section>

                    <section class="pattern-section">
                        <h2>"🔄 Pattern Matching in Collections"</h2>
                        <p class="section-desc">"Matching patterns within vectors, maps, and tuples"</p>
                        @CollectionPatterns()
                    </section>
                </main>
            </body>
        </html>
    }
}

#[derive(Debug, Clone)]
enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone)]
enum Status {
    Pending,
    InProgress {
        assigned_to: String,
        progress: u8,
    },
    Completed {
        completed_at: String,
        reviewed_by: Option<String>,
    },
    Cancelled {
        reason: String,
        cancelled_by: String,
    },
}

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    priority: Priority,
    status: Status,
    tags: Vec<String>,
    metadata: std::collections::HashMap<String, String>,
}

#[azumi::component]
fn ComplexPatterns() -> impl azumi::Component {
    html! {
        <div class="patterns-card">
            <h3>"🎨 Complex Nested Pattern Matching"</h3>
            <p class="section-desc">"Multi-level destructuring with enums and structs"</p>

            @let tasks = vec![
                Task {
                    id: 1,
                    title: "Implement login system".to_string(),
                    priority: Priority::Critical,
                    status: Status::InProgress {
                        assigned_to: "Alice".to_string(),
                        progress: 75
                    },
                    tags: vec!["auth".to_string(), "security".to_string()],
                    metadata: std::collections::HashMap::from([
                        ("sprint".to_string(), "Sprint 1".to_string()),
                        ("estimated_hours".to_string(), "16".to_string()),
                    ]),
                },
                Task {
                    id: 2,
                    title: "Database migration".to_string(),
                    priority: Priority::High,
                    status: Status::Completed {
                        completed_at: "2024-01-15".to_string(),
                        reviewed_by: Some("Bob".to_string())
                    },
                    tags: vec!["database".to_string(), "devops".to_string()],
                    metadata: std::collections::HashMap::from([
                        ("sprint".to_string(), "Sprint 1".to_string()),
                        ("downtime_minutes".to_string(), "120".to_string()),
                    ]),
                },
                Task {
                    id: 3,
                    title: "Update documentation".to_string(),
                    priority: Priority::Medium,
                    status: Status::Pending,
                    tags: vec!["docs".to_string()],
                    metadata: std::collections::HashMap::new(),
                },
                Task {
                    id: 4,
                    title: "Refactor legacy code".to_string(),
                    priority: Priority::Low,
                    status: Status::Cancelled {
                        reason: "Scope changed".to_string(),
                        cancelled_by: "Manager".to_string()
                    },
                    tags: vec!["refactor".to_string(), "technical_debt".to_string()],
                    metadata: std::collections::HashMap::from([
                        ("blocked_by".to_string(), "PR #123".to_string()),
                    ]),
                },
            ];

            <div class="task-overview">
                <h4>"📋 Task Status Overview"</h4>
                @let (pending, in_progress, completed, cancelled) = tasks.iter().fold((0, 0, 0, 0), |(p, i, c, ca), task| {
                    match &task.status {
                        Status::Pending => (p + 1, i, c, ca),
                        Status::InProgress { .. } => (p, i + 1, c, ca),
                        Status::Completed { .. } => (p, i, c + 1, ca),
                        Status::Cancelled { .. } => (p, i, c, ca + 1),
                    }
                });

                <div class="status-summary">
                    <div class="status-item pending">
                        <span class="count">{pending}</span>
                        <span class="label">"Pending"</span>
                    </div>
                    <div class="status-item progress">
                        <span class="count">{in_progress}</span>
                        <span class="label">"In Progress"</span>
                    </div>
                    <div class="status-item completed">
                        <span class="count">{completed}</span>
                        <span class="label">"Completed"</span>
                    </div>
                    <div class="status-item cancelled">
                        <span class="count">{cancelled}</span>
                        <span class="label">"Cancelled"</span>
                    </div>
                </div>
            </div>

            <div class="task-list">
                <h4>"📝 Individual Task Analysis"</h4>
                @for task in &tasks {
                    @let (status_display, status_class, extra_info) = match &task.status {
                        Status::Pending => (
                            "⏳ Pending".to_string(),
                            "status-pending",
                            "Waiting for assignment".to_string(),
                        ),
                        Status::InProgress { assigned_to, progress } => (
                            format!("🔄 In Progress ({}%)", progress),
                            "status-in-progress",
                            format!("Assigned to {}", assigned_to),
                        ),
                        Status::Completed { completed_at, reviewed_by } => {
                            let review_info = reviewed_by
                                .as_ref()
                                .map(|reviewer| format!(" (reviewed by {})", reviewer))
                                .unwrap_or_else(|| " (unreviewed)".to_string());
                            (
                                "✅ Completed".to_string(),
                                "status-completed",
                                format!("Completed on {}{}", completed_at, review_info),
                            )
                        }
                        Status::Cancelled { reason, cancelled_by } => (
                            "❌ Cancelled".to_string(),
                            "status-cancelled",
                            format!("Cancelled by {}: {}", cancelled_by, reason),
                        ),
                    };

                    @let priority_display = match &task.priority {
                        Priority::Critical => ("🔴 Critical", "priority-critical"),
                        Priority::High => ("🟠 High", "priority-high"),
                        Priority::Medium => ("🟡 Medium", "priority-medium"),
                        Priority::Low => ("🟢 Low", "priority-low"),
                    };

                    <div class="task-item">
                        <div class="task-header">
                            <span class={format!("priority-badge {}", priority_display.1)}>
                                {priority_display.0}
                            </span>
                            <h4>{&task.title}</h4>
                        </div>

                        <div class="task-status">
                            <span class={format!("status-badge {}", status_class)}>
                                {status_display}
                            </span>
                            <span class="extra-info">{extra_info}</span>
                        </div>

                        <div class="task-tags">
                            @for tag in &task.tags {
                                <span class="tag">{tag}</span>
                            }
                        </div>

                        @if !task.metadata.is_empty() {
                            <div class="task-metadata">
                                <h5>"📊 Metadata:"</h5>
                                @for (key, value) in &task.metadata {
                                    <div class="metadata-item">
                                        <span class="key">{key}":"</span>
                                        <span class="value">{value}</span>
                                    </div>
                                }
                            </div>
                        }
                    </div>
                }
            </div>
        </div>
    }
}

#[azumi::component]
fn BindingPatterns() -> impl azumi::Component {
    html! {
        <div class="binding-card">
            <h3>"🔗 Advanced Binding & Destructuring"</h3>
            <p class="section-desc">"Complex variable binding and tuple/struct destructuring"</p>

            @let coordinate_data = vec![
                (10.0, 20.0, 30.0),
                (15.5, 25.1, 35.2),
                (0.0, 0.0, 0.0),
                (-10.0, 20.0, -30.0),
            ];

            <div class="coordinate-analysis">
                <h4>"🎯 Coordinate System Analysis"</h4>

                @for (x, y, z) in coordinate_data {
                    @let distance = (x * x + y * y + z * z).sqrt();
                    @let quadrant = match (x >= 0.0, y >= 0.0, z >= 0.0) {
                        (true, true, true) => "Positive space",
                        (false, true, true) => "Negative X space",
                        (true, false, true) => "Negative Y space",
                        (true, true, false) => "Negative Z space",
                        (false, false, true) => "Negative X,Y space",
                        (false, true, false) => "Negative X,Z space",
                        (true, false, false) => "Negative Y,Z space",
                        (false, false, false) => "Negative space",
                    };

                    @let distance_class = match distance {
                        d if d == 0.0 => "origin",
                        d if d <= 20.0 => "near",
                        d if d <= 50.0 => "medium",
                        _ => "far",
                    };

                    <div class="coordinate-item">
                        <div class="coord-display">
                            <span class="coord">"(" {x} ", " {y} ", " {z} ")"</span>
                            <span class="distance {distance_class}">
                                "Distance: " {format!("{:.2}", distance)}
                            </span>
                        </div>
                        <div class="quadrant-info">
                            <span class="quadrant-badge">{quadrant}</span>
                        </div>
                    </div>
                }
            </div>

            <div class="user-profile-analysis">
                <h4>"👤 Complex User Profile Destructuring"</h4>

                @let user_profiles = vec![
                    "Alice Johnson;28;developer;NYC;2022-05-15;true;premium;4.8;15000",
                    "Bob Smith;35;designer;LA;2021-03-20;false;basic;4.2;8000",
                    "Carol Davis;42;manager;SF;2020-01-10;true;premium;4.9;25000",
                ];

                @for profile_line in &user_profiles {
                    @let parts: Vec<&str> = profile_line.split(';').collect();
                    @if parts.len() >= 9 {
                        @let (name, age_str, role, location, joined_date_str, active_str, tier, rating_str, posts_str) = (
                            parts[0], parts[1], parts[2], parts[3], parts[4], parts[5], parts[6], parts[7], parts[8]
                        );

                        @let age: i32 = age_str.parse().unwrap_or(0);
                        @let rating: f32 = rating_str.parse().unwrap_or(0.0);
                        @let posts: i32 = posts_str.parse().unwrap_or(0);

                        @let profile_class = match (active_str.parse::<bool>().unwrap_or(false), tier) {
                            (true, "premium") => "profile-premium-active",
                            (true, "basic") => "profile-basic-active",
                            (false, "premium") => "profile-premium-inactive",
                            (false, "basic") => "profile-basic-inactive",
                            _ => "profile-unknown",
                        };

                        @let experience_level = match age {
                            a if a < 25 => "Junior",
                            a if a < 35 => "Mid-level",
                            a if a < 45 => "Senior",
                            _ => "Expert",
                        };

                        @let engagement_score = match (rating, posts) {
                            (r, p) if r >= 4.8 && p >= 20000 => "Super Engaged",
                            (r, p) if r >= 4.5 && p >= 10000 => "Highly Engaged",
                            (r, p) if r >= 4.0 && p >= 5000 => "Moderately Engaged",
                            (r, p) if r >= 3.5 && p >= 1000 => "Minimally Engaged",
                            _ => "Low Engagement",
                        };

                        <div class="user-profile-item">
                            <div class="profile-header">
                                <h4>{name}</h4>
                                <span class={format!("profile-badge {}", profile_class)}>
                                    {tier} " " {if active_str == "true" { "✅" } else { "⏸️" }}
                                </span>
                            </div>

                            <div class="profile-details">
                                <p>"👤 Role: " {role}</p>
                                <p>"📍 Location: " {location}</p>
                                <p>"🎂 Age: " {age}</p>
                                <p>"📅 Joined: " {joined_date_str}</p>
                                <p>"⭐ Rating: " {rating}</p>
                                <p>"📝 Posts: " {format!("{:,}", posts)}</p>
                            </div>

                            <div class="profile-insights">
                                <span class="experience-badge">{experience_level}</span>
                                <span class="engagement-badge">{engagement_score}</span>
                            </div>
                        </div>
                    }
                }
            </div>
        </div>
    }
}

#[azumi::component]
fn GuardConditions() -> impl azumi::Component {
    html! {
        <div class="guard-card">
            <h3>"🎯 Pattern Guards & Complex Conditions"</h3>
            <p class="section-desc">"Advanced conditional matching with complex expressions"</p>

            @let price_data = vec![
                ("Laptop", 1200.0, "Electronics", true, 0.1),
                ("Book", 25.0, "Education", false, 0.0),
                ("Phone", 800.0, "Electronics", true, 0.15),
                ("Coffee", 12.0, "Food", true, 0.05),
            ];

            <div class="pricing-analysis">
                <h4>"💰 Smart Pricing with Guards"</h4>

                @for (product, base_price, category, in_stock, discount) in &price_data {
                    @let final_price = base_price * (1.0 - discount);
                    @let is_expensive = final_price > 500.0;
                    @let is_affordable = final_price < 50.0;
                    @let stock_status = if *in_stock { "Available" } else { "Out of Stock" };

                    @let pricing_strategy = match (category, final_price, *in_stock, discount) {
                        ("Electronics", price, true, d) if price > 1000.0 && d > 0.1 => "Premium Tech Sale",
                        ("Electronics", price, true, d) if price <= 1000.0 && d > 0.1 => "Electronics Sale",
                        ("Education", price, _, d) if price < 30.0 => "Student Discount",
                        ("Food", price, true, _) if price < 15.0 => "Quick Bite Special",
                        ("Electronics", price, false, _) => "Coming Soon",
                        (cat, price, true, _) if cat != "Electronics" && price > 100.0 => "Standard Pricing",
                        (cat, price, true, d) if d > 0.0 => "Regular Sale",
                        _ => "Standard Rate",
                    };

                    @let price_class = if is_expensive {
                        "price-expensive"
                    } else if is_affordable {
                        "price-affordable"
                    } else {
                        "price-standard"
                    };

                    <div class="product-item">
                        <div class="product-header">
                            <h4>{product}</h4>
                            <span class={format!("price-display {}", price_class)}>
                                {format!("${:.2}", final_price)}
                            </span>
                        </div>

                        <div class="product-details">
                            <p>"Category: " {category}</p>
                            <p>"Base Price: " {format!("${:.2}", base_price)}</p>
                            @if discount > &0.0 {
                                <p>"Discount: " {format!("{:.0}%", discount * 100.0)}</p>
                            }
                            <p>"Status: " {stock_status}</p>
                        </div>

                        <div class="pricing-info">
                            <span class="strategy-badge">{pricing_strategy}</span>
                        </div>
                    </div>
                }
            </div>

            <div class="validation-system">
                <h4>"✅ Input Validation with Guards"</h4>

                @let user_inputs = vec![
                    ("email", "user@example.com", "string"),
                    ("age", "25", "i32"),
                    ("score", "98.5", "f32"),
                    ("age", "-5", "i32"),
                    ("score", "150", "f32"),
                    ("email", "invalid-email", "string"),
                ];

                @for (field_name, input_value, expected_type) in &user_inputs {
                    @let validation_result = match (field_name, input_value, expected_type) {
                        ("email", value, "string") if value.contains('@') && value.contains('.') =>
                            ("✅ Valid", "validation-valid"),
                        ("email", value, "string") =>
                            ("❌ Invalid email", "validation-error"),
                        ("age", value, "i32") => {
                            match value.parse::<i32>() {
                                Ok(age) if age >= 0 && age <= 150 => ("✅ Valid age", "validation-valid"),
                                Ok(_) => ("❌ Age out of range", "validation-error"),
                                Err(_) => ("❌ Not a number", "validation-error"),
                            }
                        },
                        ("score", value, "f32") => {
                            match value.parse::<f32>() {
                                Ok(score) if score >= 0.0 && score <= 100.0 => ("✅ Valid score", "validation-valid"),
                                Ok(_) => ("❌ Score out of range", "validation-error"),
                                Err(_) => ("❌ Not a number", "validation-error"),
                            }
                        },
                        _ => ("❌ Unknown validation", "validation-error"),
                    };

                    <div class="validation-item">
                        <span class="field-name">{field_name}:</span>
                        <span class="input-value">"{input_value}"</span>
                        <span class={format!("result-badge {}", validation_result.1)}>
                            {validation_result.0}
                        </span>
                    </div>
                }
            </div>
        </div>
    }
}

#[azumi::component]
fn CollectionPatterns() -> impl azumi::Component {
    html! {
        <div class="collection-patterns-card">
            <h3>"🔄 Pattern Matching in Collections"</h3>
            <p class="section-desc">"Advanced collection processing with pattern matching"</p>

            @let nested_data = vec![
                vec![1, 2, 3, 4, 5],
                vec![10, 20],
                vec![100],
                vec![5, 10, 15, 20, 25, 30],
                vec![], // Empty vector
            ];

            <div class="vector-analysis">
                <h4>"📊 Vector Pattern Analysis"</h4>

                @for (i, vector) in nested_data.iter().enumerate() {
                    @let analysis = match vector.as_slice() {
                        [] => ("Empty", "analysis-empty", "No data available"),
                        [single] => ("Single element", "analysis-single", format!("Only {}", single)),
                        [first, rest @ ..] => {
                            let sum: i32 = vector.iter().sum();
                            let avg = sum as f64 / vector.len() as f64;
                            ("Multiple elements", "analysis-multiple", format!("Sum: {}, Avg: {:.1}", sum, avg))
                        }
                    };

                    <div class="vector-item">
                        <h5>"Vector " {i + 1}</h5>
                        <div class="vector-display">
                            @for &item in vector {
                                <span class="vector-element">{item}</span>
                            }
                        </div>
                        <div class={format!("analysis-badge {}", analysis.1)}>
                            {analysis.0}
                        </div>
                        <p class="analysis-desc">{analysis.2}</p>
                    </div>
                }
            </div>

            <div class="complex-collections">
                <h4>"🗂️ Complex Nested Collection Patterns"</h4>

                @let user_groups = vec![
                    (
                        "Developers",
                        vec![
                            ("Alice", vec!["Rust", "JavaScript", "Python"]),
                            ("Bob", vec!["Go", "TypeScript"]),
                            ("Carol", vec!["Python", "Django", "PostgreSQL"]),
                        ]
                    ),
                    (
                        "Designers",
                        vec![
                            ("Dave", vec!["Figma", "Photoshop", "Illustrator"]),
                            ("Eve", vec!["Sketch", "Principle", "InVision"]),
                        ]
                    ),
                    (
                        "Managers",
                        vec![
                            ("Frank", vec!["Project Management", "Agile", "Scrum"]),
                        ]
                    ),
                ];

                @for (group_name, users) in &user_groups {
                    @let total_skills: Vec<_> = users.iter()
                        .flat_map(|(_, skills)| skills)
                        .collect();
                    @let unique_skills: std::collections::HashSet<_> = total_skills.iter().copied().collect();
                    @let avg_skills_per_user = total_skills.len() as f64 / users.len() as f64;

                    <div class="user-group">
                        <h5>{group_name}</h5>
                        <div class="group-stats">
                            <span>"Users: " {users.len()}</span>
                            <span>"Total skills: " {total_skills.len()}</span>
                            <span>"Unique skills: " {unique_skills.len()}</span>
                            <span>"Avg skills/user: " {format!("{:.1}", avg_skills_per_user)}</span>
                        </div>

                        <div class="user-list">
                            @for (name, skills) in users {
                                @let skill_count = skills.len();
                                @let proficiency = match skill_count {
                                    0 => "None",
                                    1..=2 => "Beginner",
                                    3..=5 => "Intermediate",
                                    _ => "Expert",
                                };

                                <div class="user-skill-item">
                                    <span class="user-name">{name}</span>
                                    <div class="skills-display">
                                        @for skill in skills {
                                            <span class="skill-tag">{skill}</span>
                                        }
                                    </div>
                                    <span class="proficiency-badge">{proficiency}</span>
                                </div>
                            }
                        </div>
                    </div>
                }
            </div>
        </div>
    }
}
