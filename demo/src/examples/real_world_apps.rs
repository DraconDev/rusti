use axum::response::{Html, IntoResponse};
use azumi::html;

/// Real-world application examples showcasing practical Azumi usage
pub async fn real_world_apps_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&real_world_apps_demo()))
}

fn real_world_apps_demo() -> impl azumi::Component {
    html! {
        <style src="/static/real_world_apps.css" />
        <html>
            <head>
                <title>"Real-World Applications - Azumi"</title>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            </head>
            <body>
                <header class="page-header">
                    <h1>"🚀 Real-World Applications"</h1>
                    <p>"Production-ready patterns and practical implementations"</p>
                    <nav class="nav-links">
                        <a href="/">"🏠 Home"</a>
                        <a href="/data-processing">"📊 Data Processing"</a>
                        <a href="/advanced-patterns">"🎭 Advanced Patterns"</a>
                        <a href="/ui-library">"🧩 UI Components"</a>
                    </nav>
                </header>

                <main class="content">
                    <section class="app-section">
                        <h2>"📈 E-commerce Product Catalog"</h2>
                        <p class="section-desc">"Complete product browsing with filters, search, and cart functionality"</p>
                        @EcommerceCatalog()
                    </section>

                    <section class="app-section">
                        <h2>"📊 Analytics Dashboard"</h2>
                        <p class="section-desc">"Real-time analytics with charts, metrics, and interactive widgets"</p>
                        @AnalyticsDashboard()
                    </section>

                    <section class="app-section">
                        <h2>"📱 Social Media Feed"</h2>
                        <p class="section-desc">"Social feed with likes, comments, sharing, and infinite scroll"</p>
                        @SocialMediaFeed()
                    </section>

                    <section class="app-section">
                        <h2>"📋 Project Management Board"</h2>
                        <p class="section-desc">"Kanban-style project management with drag-and-drop functionality"</p>
                        @ProjectBoard()
                    </section>
                </main>
            </body>
        </html>
    }
}

#[derive(Debug, Clone)]
struct Product {
    id: u32,
    name: String,
    category: String,
    price: f64,
    rating: f32,
    in_stock: bool,
    tags: Vec<String>,
    images: Vec<String>,
    description: String,
}

#[azumi::component]
fn EcommerceCatalog() -> impl azumi::Component {
    html! {
        <div class="ecommerce-card">
            <h3>"🛒 Advanced Product Catalog"</h3>
            <p class="section-desc">"Full-featured e-commerce with search, filtering, and cart simulation"</p>

            @let products = vec![
                Product {
                    id: 1,
                    name: "MacBook Pro 16-inch".to_string(),
                    category: "Laptops".to_string(),
                    price: 2499.00,
                    rating: 4.8,
                    in_stock: true,
                    tags: vec!["apple".to_string(), "professional".to_string(), "high-end".to_string()],
                    images: vec!["macbook-pro.jpg".to_string()],
                    description: "Professional laptop with M3 Pro chip".to_string(),
                },
                Product {
                    id: 2,
                    name: "Gaming Mouse RGB".to_string(),
                    category: "Accessories".to_string(),
                    price: 89.99,
                    rating: 4.6,
                    in_stock: true,
                    tags: vec!["gaming".to_string(), "wireless".to_string(), "rgb".to_string()],
                    images: vec!["gaming-mouse.jpg".to_string()],
                    description: "High-precision wireless gaming mouse".to_string(),
                },
                Product {
                    id: 3,
                    name: "USB-C Hub 8-in-1".to_string(),
                    category: "Accessories".to_string(),
                    price: 149.99,
                    rating: 4.3,
                    in_stock: false,
                    tags: vec!["usb-c".to_string(), "hub".to_string(), "ports".to_string()],
                    images: vec!["usb-c-hub.jpg".to_string()],
                    description: "Multi-port USB-C hub for laptops".to_string(),
                },
                Product {
                    id: 4,
                    name: "4K Monitor 27-inch".to_string(),
                    category: "Monitors".to_string(),
                    price: 399.99,
                    rating: 4.7,
                    in_stock: true,
                    tags: vec!["4k".to_string(), "27-inch".to_string(), "ips".to_string()],
                    images: vec!["4k-monitor.jpg".to_string()],
                    description: "Professional 4K IPS display".to_string(),
                },
                Product {
                    id: 5,
                    name: "Mechanical Keyboard".to_string(),
                    category: "Accessories".to_string(),
                    price: 199.99,
                    rating: 4.5,
                    in_stock: true,
                    tags: vec!["mechanical".to_string(), "rgb".to_string(), "tactile".to_string()],
                    images: vec!["mech-keyboard.jpg".to_string()],
                    description: "RGB mechanical keyboard with tactile switches".to_string(),
                },
            ];

            <!-- Search and Filter Controls -->
            <div class="catalog-controls">
                <div class="search-bar">
                    <input type="text" placeholder="Search products..." class="search-input" />
                    <button class="search-btn">"🔍 Search"</button>
                </div>

                <div class="filter-controls">
                    <div class="filter-group">
                        <label>"Category:"</label>
                        <select class="category-select">
                            <option value="">"All Categories"</option>
                            <option value="Laptops">"Laptops"</option>
                            <option value="Monitors">"Monitors"</option>
                            <option value="Accessories">"Accessories"</option>
                        </select>
                    </div>

                    <div class="filter-group">
                        <label>"Price Range:"</label>
                        <select class="price-select">
                            <option value="">"All Prices"</option>
                            <option value="0-100">"$0 - $100"</option>
                            <option value="100-500">"$100 - $500"</option>
                            <option value="500+">"$500+"</option>
                        </select>
                    </div>

                    <div class="filter-group">
                        <label>"Availability:"</label>
                        <select class="stock-select">
                            <option value="">"All Items"</option>
                            <option value="in-stock">"In Stock Only"</option>
                        </select>
                    </div>
                </div>
            </div>

            <!-- Product Grid -->
            <div class="product-grid">
                @for product in &products {
                    @let stock_status = if product.in_stock {
                        ("✅ In Stock", "status-in-stock")
                    } else {
                        ("⏳ Out of Stock", "status-out-stock")
                    };

                    @let price_category = match product.price {
                        p if p < 100.0 => "price-budget",
                        p if p < 500.0 => "price-mid",
                        _ => "price-premium",
                    };

                    @let rating_stars = "⭐".repeat(product.rating as usize);
                    @let rating_decimal = format!("{:.1}", product.rating);

                    <div class="product-card">
                        <div class="product-image">
                            <div class="image-placeholder">
                                📷 {product.name.chars().next().unwrap_or('?')}
                            </div>
                            @if !product.in_stock {
                                <div class="out-of-stock-overlay">"Out of Stock"</div>
                            }
                        </div>

                        <div class="product-info">
                            <h4 class="product-name">{product.name}</h4>
                            <p class="product-category">{product.category}</p>
                            <p class="product-description">{product.description}</p>

                            <div class="product-rating">
                                <span class="stars">{rating_stars}</span>
                                <span class="rating-value">{rating_decimal}</span>
                            </div>

                            <div class="product-price">
                                <span class={format!("price {} {}", price_category, if product.in_stock { "" } else { "price-disabled" })}>
                                    {format!("${:.2}", product.price)}
                                </span>
                                <span class={format!("stock-status {}", stock_status.1)}>
                                    {stock_status.0}
                                </span>
                            </div>

                            <div class="product-tags">
                                @for tag in &product.tags {
                                    <span class="product-tag">{tag}</span>
                                }
                            </div>

                            <div class="product-actions">
                                @if product.in_stock {
                                    <button class="add-to-cart-btn">"Add to Cart"</button>
                                } else {
                                    <button class="notify-btn">"Notify When Available"</button>
                                }
                                <button class="wishlist-btn">"♡ Wishlist"</button>
                            </div>
                        </div>
                    </div>
                }
            </div>

            <!-- Cart Summary -->
            <div class="cart-summary">
                <h4>"🛒 Shopping Cart"</h4>
                <div class="cart-items">
                    <div class="cart-empty">"Your cart is empty"</div>
                    <!-- Cart items would go here -->
                </div>
                <div class="cart-total">
                    <span>"Subtotal: $0.00"</span>
                    <span>"Tax: $0.00"</span>
                    <span class="total">"Total: $0.00"</span>
                </div>
                <button class="checkout-btn">"Proceed to Checkout"</button>
            </div>
        </div>
    }
}

#[azumi::component]
fn AnalyticsDashboard() -> impl azumi::Component {
    html! {
        <div class="analytics-card">
            <h3>"📊 Real-Time Analytics Dashboard"</h3>
            <p class="section-desc">"Live metrics, charts, and KPI monitoring"</p>

            <!-- Key Performance Indicators -->
            <div class="kpi-grid">
                <div class="kpi-card">
                    <div class="kpi-header">
                        <h4>"👥 Active Users"</h4>
                        <span class="kpi-trend trend-up">"+12.5%"</span>
                    </div>
                    <div class="kpi-value">"24,567"</div>
                    <div class="kpi-chart">
                        @let user_data = vec![21.2, 22.1, 23.5, 24.8, 24.1, 24.9, 24.6];
                        @for point in &user_data {
                            @let height = (point * 3.0) as i32;
                            <div class="chart-bar" style={format!("height: {}px", height)}></div>
                        }
                    </div>
                    <p class="kpi-description">"vs last week"</p>
                </div>

                <div class="kpi-card">
                    <div class="kpi-header">
                        <h4>"💰 Revenue"</h4>
                        <span class="kpi-trend trend-up">"+8.3%"</span>
                    </div>
                    <div class="kpi-value">"$127,450"</div>
                    <div class="kpi-chart">
                        @let revenue_data = vec![95.2, 102.1, 108.9, 115.2, 122.8, 125.1, 127.4];
                        @for point in &revenue_data {
                            @let height = (point * 0.8) as i32;
                            <div class="chart-bar" style={format!("height: {}px", height)}></div>
                        }
                    </div>
                    <p class="kpi-description">"monthly recurring"</p>
                </div>

                <div class="kpi-card">
                    <div class="kpi-header">
                        <h4>"⏱️ Response Time"</h4>
                        <span class="kpi-trend trend-down">"-5.1%"</span>
                    </div>
                    <div class="kpi-value">"247ms"</div>
                    <div class="kpi-chart">
                        @let response_data = vec![280, 265, 250, 255, 245, 248, 247];
                        @for point in &response_data {
                            @let height = point / 4;
                            <div class="chart-bar" style={format!("height: {}px", height)}></div>
                        }
                    </div>
                    <p class="kpi-description">"average API response"</p>
                </div>

                <div class="kpi-card">
                    <div class="kpi-header">
                        <h4>"📈 Conversion Rate"</h4>
                        <span class="kpi-trend trend-up">"+15.2%"</span>
                    </div>
                    <div class="kpi-value">"4.8%"</div>
                    <div class="kpi-chart">
                        @let conversion_data = vec![3.8, 4.1, 4.2, 4.5, 4.6, 4.7, 4.8];
                        @for point in &conversion_data {
                            @let height = (point * 20.0) as i32;
                            <div class="chart-bar" style={format!("height: {}px", height)}></div>
                        }
                    </div>
                    <p class="kpi-description">"visit to purchase"</p>
                </div>
            </div>

            <!-- Traffic Sources -->
            <div class="traffic-sources">
                <h4>"🌐 Traffic Sources"</h4>
                <div class="source-list">
                    @let traffic_sources = vec![
                        ("🔍 Search Engines", 12450, 45.2),
                        ("🔗 Social Media", 8230, 29.8),
                        ("📧 Email", 4120, 14.9),
                        ("📱 Direct", 2890, 10.1),
                    ];

                    @for (source, visits, percentage) in &traffic_sources {
                        <div class="source-item">
                            <div class="source-info">
                                <span class="source-name">{source}</span>
                                <span class="source-visits">{format!("{:,} visits", visits)}</span>
                            </div>
                            <div class="source-bar">
                                <div class="source-fill" style={format!("width: {}%", percentage)}></div>
                            </div>
                            <span class="source-percentage">{format!("{:.1}%", percentage)}</span>
                        </div>
                    }
                </div>
            </div>

            <!-- Recent Events -->
            <div class="recent-events">
                <h4>"⚡ Recent Events"</h4>
                <div class="events-timeline">
                    <div class="event-item">
                        <div class="event-time">"2 min ago"</div>
                        <div class="event-content">
                            <span class="event-type">"New user signup"</span>
                            <span class="event-detail">"alice@example.com joined"</span>
                        </div>
                    </div>
                    <div class="event-item">
                        <div class="event-time">"5 min ago"</div>
                        <div class="event-content">
                            <span class="event-type">"Purchase completed"</span>
                            <span class="event-detail">"Order #12345 - $89.99"</span>
                        </div>
                    </div>
                    <div class="event-item">
                        <div class="event-time">"8 min ago"</div>
                        <div class="event-content">
                            <span class="event-type">"Error resolved"</span>
                            <span class="event-detail">"API rate limit issue fixed"</span>
                        </div>
                    </div>
                    <div class="event-item">
                        <div class="event-time">"12 min ago"</div>
                        <div class="event-content">
                            <span class="event-type">"High traffic"</span>
                            <span class="event-detail">"Traffic spike detected (+45%)"</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[azumi::component]
fn SocialMediaFeed() -> impl azumi::Component {
    html! {
        <div class="social-card">
            <h3>"📱 Social Media Feed"</h3>
            <p class="section-desc">"Interactive social feed with real-time updates"</p>

            <div class="feed-container">
                @let posts = vec![
                    (
                        "Alice Johnson",
                        "@alice_dev",
                        "🚀 Just launched my new Rust library for data processing! 🚀",
                        "2h",
                        42,
                        8,
                        vec!["@rustlang".to_string(), "@opensource".to_string(), "@programming".to_string()],
                        true
                    ),
                    (
                        "Bob Smith",
                        "@bob_the_builder",
                        "Working on a new UI library with Azumi. The pattern matching capabilities are incredible! 🔥",
                        "4h",
                        128,
                        23,
                        vec!["@azumi".to_string(), "@ui".to_string(), "@rust".to_string()],
                        false
                    ),
                    (
                        "Carol Davis",
                        "@carol_codes",
                        "Database optimization complete! 60% faster queries. #performance #database",
                        "6h",
                        95,
                        15,
                        vec!["#performance".to_string(), "#database".to_string(), "#optimization".to_string()],
                        true
                    ),
                    (
                        "David Wilson",
                        "@david_devops",
                        "Deploying to production today! Feeling nervous but excited. 🚀",
                        "8h",
                        67,
                        12,
                        vec!["#deployment".to_string(), "#production".to_string(), "#devops".to_string()],
                        false
                    ),
                ];

                @for (username, handle, content, time_ago, likes, comments, tags, verified) in &posts {
                    @let is_liked = *likes > 50;
                    @let engagement_level = match likes {
                        l if l >= 100 => "high",
                        l if l >= 50 => "medium",
                        _ => "low",
                    };

                    <div class="post-card">
                        <div class="post-header">
                            <div class="user-info">
                                <div class="avatar">{username.chars().next().unwrap_or('?')}</div>
                                <div class="user-details">
                                    <div class="user-names">
                                        <span class="display-name">{username}</span>
                                        @if *verified {
                                            <span class="verified-badge">"✓"</span>
                                        }
                                        <span class="username">handle</span>
                                    </div>
                                    <span class="post-time">{time_ago}</span>
                                </div>
                            </div>
                            <button class="more-options">"⋯"</button>
                        </div>

                        <div class="post-content">
                            <p>{content}</p>
                        </div>

                        @if !tags.is_empty() {
                            <div class="post-tags">
                                @for tag in tags {
                                    <span class="post-tag">{tag}</span>
                                }
                            </div>
                        }

                        <div class="post-actions">
                            <div class="action-buttons">
                                <button class={format!("like-btn {}", if is_liked { "liked" } else { "" })}>
                                    @if is_liked { "❤️" } else { "🤍" }
                                    <span class="like-count">{likes}</span>
                                </button>

                                <button class="comment-btn">
                                    "💬"
                                    <span class="comment-count">{comments}</span>
                                </button>

                                <button class="share-btn">"↗️ Share"</button>

                                <button class="save-btn">"🔖 Save"</button>
                            </div>

                            <div class="engagement-info">
                                <span class={format!("engagement-badge {}", engagement_level)}>
                                    @match engagement_level {
                                        "high" => "🔥 Trending",
                                        "medium" => "📈 Popular",
                                        _ => "📝 New",
                                    }
                                </span>
                            </div>
                        </div>
                    </div>
                }
            </div>

            <div class="new-post-composer">
                <div class="composer-header">
                    <h4>"Create New Post"</h4>
                </div>
                <div class="composer-content">
                    <textarea placeholder="What's happening?" class="post-input"></textarea>
                    <div class="composer-actions">
                        <div class="attachment-options">
                            <button class="attach-btn">"📷 Photo"</button>
                            <button class="attach-btn">"🎥 Video"</button>
                            <button class="attach-btn">"📊 Poll"</button>
                        </div>
                        <div class="publish-options">
                            <button class="draft-btn">"Save Draft"</button>
                            <button class="post-btn">"Post"</button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[derive(Debug, Clone)]
enum TaskStatus {
    Todo,
    InProgress,
    Review,
    Done,
}

#[derive(Debug, Clone)]
struct ProjectTask {
    id: u32,
    title: String,
    description: String,
    assignee: String,
    priority: String,
    status: TaskStatus,
    due_date: Option<String>,
    tags: Vec<String>,
    estimate_hours: Option<f32>,
}

#[azumi::component]
fn ProjectBoard() -> impl azumi::Component {
    html! {
        <div class="project-card">
            <h3>"📋 Kanban Project Board"</h3>
            <p class="section-desc">"Agile project management with drag-and-drop cards"</p>

            <!-- Project Overview -->
            <div class="project-overview">
                <div class="project-stats">
                    <div class="stat-item">
                        <span class="stat-label">"Total Tasks"</span>
                        <span class="stat-value">"24"</span>
                    </div>
                    <div class="stat-item">
                        <span class="stat-label">"Completed"</span>
                        <span class="stat-value">"8"</span>
                    </div>
                    <div class="stat-item">
                        <span class="stat-label">"In Progress"</span>
                        <span class="stat-value">"12"</span>
                    </div>
                    <div class="stat-item">
                        <span class="stat-label">"Blocked"</span>
                        <span class="stat-value">"4"</span>
                    </div>
                </div>

                <div class="project-actions">
                    <button class="add-task-btn">"+ Add Task"</button>
                    <button class="filter-btn">"🔍 Filter"</button>
                    <button class="export-btn">"📤 Export"</button>
                </div>
            </div>

            <!-- Kanban Columns -->
            <div class="kanban-board">
                <!-- To Do Column -->
                <div class="kanban-column">
                    <div class="column-header">
                        <h4>"📝 To Do"</h4>
                        <span class="task-count">"6"</span>
                    </div>
                    <div class="column-content">
                        @let todo_tasks = vec![
                            ProjectTask {
                                id: 1,
                                title: "Design login page".to_string(),
                                description: "Create responsive login form with validation".to_string(),
                                assignee: "Alice".to_string(),
                                priority: "High".to_string(),
                                status: TaskStatus::Todo,
                                due_date: Some("2024-01-25".to_string()),
                                tags: vec!["design".to_string(), "ui".to_string()],
                                estimate_hours: Some(8.0),
                            },
                            ProjectTask {
                                id: 2,
                                title: "Setup CI/CD pipeline".to_string(),
                                description: "Configure GitHub Actions for automated testing".to_string(),
                                assignee: "Bob".to_string(),
                                priority: "Medium".to_string(),
                                status: TaskStatus::Todo,
                                due_date: Some("2024-01-28".to_string()),
                                tags: vec!["devops".to_string(), "automation".to_string()],
                                estimate_hours: Some(16.0),
                            },
                        ];

                        @for task in &todo_tasks {
                            <div class="task-card">
                                <div class="task-header">
                                    <h5>{&task.title}</h5>
                                    @let priority_class = match task.priority.as_str() {
                                        "High" => "priority-high",
                                        "Medium" => "priority-medium",
                                        _ => "priority-low",
                                    };
                                    <span class={format!("priority-badge {}", priority_class)}>
                                        {&task.priority}
                                    </span>
                                </div>
                                <p class="task-description">{&task.description}</p>
                                <div class="task-meta">
                                    <span class="assignee">"👤 " {&task.assignee}</span>
                                    @if let Some(hours) = task.estimate_hours {
                                        <span class="estimate">"⏱️ " {format!("{:.1}h", hours)}</span>
                                    }
                                    @if let Some(date) = &task.due_date {
                                        <span class="due-date">"📅 " {date}</span>
                                    }
                                </div>
                                <div class="task-tags">
                                    @for tag in &task.tags {
                                        <span class="task-tag">{tag}</span>
                                    }
                                </div>
                                <div class="task-actions">
                                    <button class="task-btn">"Edit"</button>
                                    <button class="task-btn">"Move"</button>
                                </div>
                            </div>
                        }
                    </div>
                </div>

                <!-- In Progress Column -->
                <div class="kanban-column">
                    <div class="column-header">
                        <h4>"🔄 In Progress"</h4>
                        <span class="task-count">"12"</span>
                    </div>
                    <div class="column-content">
                        @let in_progress_tasks = vec![
                            ProjectTask {
                                id: 3,
                                title: "Implement user authentication".to_string(),
                                description: "JWT-based auth with refresh tokens".to_string(),
                                assignee: "Carol".to_string(),
                                priority: "High".to_string(),
                                status: TaskStatus::InProgress,
                                due_date: Some("2024-01-30".to_string()),
                                tags: vec!["backend".to_string(), "security".to_string()],
                                estimate_hours: Some(24.0),
                            },
                            ProjectTask {
                                id: 4,
                                title: "Create API documentation".to_string(),
                                description: "OpenAPI/Swagger docs for all endpoints".to_string(),
                                assignee: "Dave".to_string(),
                                priority: "Medium".to_string(),
                                status: TaskStatus::InProgress,
                                due_date: Some("2024-02-01".to_string()),
                                tags: vec!["docs".to_string(), "api".to_string()],
                                estimate_hours: Some(12.0),
                            },
                        ];

                        @for task in &in_progress_tasks {
                            <div class="task-card">
                                <div class="task-header">
                                    <h5>{&task.title}</h5>
                                    @let priority_class = match task.priority.as_str() {
                                        "High" => "priority-high",
                                        "Medium" => "priority-medium",
                                        _ => "priority-low",
                                    };
                                    <span class={format!("priority-badge {}", priority_class)}>
                                        {&task.priority}
                                    </span>
                                </div>
                                <p class="task-description">{&task.description}</p>
                                <div class="task-meta">
                                    <span class="assignee">"👤 " {&task.assignee}</span>
                                    @if let Some(hours) = task.estimate_hours {
                                        <span class="estimate">"⏱️ " {format!("{:.1}h", hours)}</span>
                                    }
                                    @if let Some(date) = &task.due_date {
                                        <span class="due-date">"📅 " {date}</span>
                                    }
                                </div>
                                <div class="task-tags">
                                    @for tag in &task.tags {
                                        <span class="task-tag">{tag}</span>
                                    }
                                </div>
                                <div class="task-progress">
                                    <div class="progress-bar">
                                        <div class="progress-fill" style="width: 65%"></div>
                                    </div>
                                    <span class="progress-text">"65% complete"</span>
                                </div>
                            </div>
                        }
                    </div>
                </div>

                <!-- Review Column -->
                <div class="kanban-column">
                    <div class="column-header">
                        <h4>"👀 Review"</h4>
                        <span class="task-count">"4"</span>
                    </div>
                    <div class="column-content">
                        @let review_tasks = vec![
                            ProjectTask {
                                id: 5,
                                title: "Database schema design".to_string(),
                                description: "Review and optimize database relationships".to_string(),
                                assignee: "Eve".to_string(),
                                priority: "High".to_string(),
                                status: TaskStatus::Review,
                                due_date: Some("2024-01-27".to_string()),
                                tags: vec!["database".to_string(), "review".to_string()],
                                estimate_hours: Some(6.0),
                            },
                        ];

                        @for task in &review_tasks {
                            <div class="task-card">
                                <div class="task-header">
                                    <h5>{&task.title}</h5>
                                    @let priority_class = match task.priority.as_str() {
                                        "High" => "priority-high",
                                        "Medium" => "priority-medium",
                                        _ => "priority-low",
                                    };
                                    <span class={format!("priority-badge {}", priority_class)}>
                                        {&task.priority}
                                    </span>
                                </div>
                                <p class="task-description">{&task.description}</p>
                                <div class="task-meta">
                                    <span class="assignee">"👤 " {&task.assignee}</span>
                                    @if let Some(hours) = task.estimate_hours {
                                        <span class="estimate">"⏱️ " {format!("{:.1}h", hours)}</span>
                                    }
                                    @if let Some(date) = &task.due_date {
                                        <span class="due-date">"📅 " {date}</span>
                                    }
                                </div>
                                <div class="task-tags">
                                    @for tag in &task.tags {
                                        <span class="task-tag">{tag}</span>
                                    }
                                </div>
                                <div class="review-actions">
                                    <button class="approve-btn">"✅ Approve"</button>
                                    <button class="request-changes-btn">"📝 Request Changes"</button>
                                </div>
                            </div>
                        }
                    </div>
                </div>

                <!-- Done Column -->
                <div class="kanban-column">
                    <div class="column-header">
                        <h4>"✅ Done"</h4>
                        <span class="task-count">"8"</span>
                    </div>
                    <div class="column-content">
                        @let done_tasks = vec![
                            ProjectTask {
                                id: 6,
                                title: "Project setup".to_string(),
                                description: "Initialize repository, setup tools".to_string(),
                                assignee: "Frank".to_string(),
                                priority: "Medium".to_string(),
                                status: TaskStatus::Done,
                                due_date: Some("2024-01-20".to_string()),
                                tags: vec!["setup".to_string(), "infrastructure".to_string()],
                                estimate_hours: Some(4.0),
                            },
                            ProjectTask {
                                id: 7,
                                title: "Requirements analysis".to_string(),
                                description: "Document functional and non-functional requirements".to_string(),
                                assignee: "Grace".to_string(),
                                priority: "High".to_string(),
                                status: TaskStatus::Done,
                                due_date: Some("2024-01-22".to_string()),
                                tags: vec!["analysis".to_string(), "requirements".to_string()],
                                estimate_hours: Some(16.0),
                            },
                        ];

                        @for task in &done_tasks {
                            <div class="task-card completed">
                                <div class="task-header">
                                    <h5>{&task.title}</h5>
                                    <span class="completion-badge">"✅ Completed"</span>
                                </div>
                                <p class="task-description">{&task.description}</p>
                                <div class="task-meta">
                                    <span class="assignee">"👤 " {&task.assignee}</span>
                                    @if let Some(hours) = task.estimate_hours {
                                        <span class="estimate">"⏱️ " {format!("{:.1}h", hours)}</span>
                                    }
                                    @if let Some(date) = &task.due_date {
                                        <span class="completed-date">"📅 Done: " {date}</span>
                                    }
                                </div>
                                <div class="task-tags">
                                    @for tag in &task.tags {
                                        <span class="task-tag">{tag}</span>
                                    }
                                </div>
                            </div>
                        }
                    </div>
                </div>
            </div>
        </div>
    }
}
