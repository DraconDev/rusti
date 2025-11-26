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
    InProgress { assigned_to: String, progress: u8 },
    Completed { completed_at: String, reviewed_by: Option<String> },
    Cancelled { reason: String, cancelled_by: String },
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
                            "⏳ Pending",
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
                                "✅ Completed",
                                "status-completed",
                                format!("Completed on {}{}", completed_at, review_info),
                            )
                        }
                        Status::Cancelled { reason, cancelled_by } => (
                            "❌ Cancelled",
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
                                        <span class="key">{key}:</span>
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
            
