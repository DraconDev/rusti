use axum::response::{Html, IntoResponse};
use azumi::html;

/// Comprehensive data processing examples
pub async fn data_processing_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&data_processing_demo()))
}

fn data_processing_demo() -> impl azumi::Component {
    html! {
        <style src="/static/data_processing.css" />
        <html>
            <head>
                <title>"Data Processing - Azumi"</title>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            </head>
            <body>
                <header class="page-header">
                    <h1>"📊 Data Processing Mastery"</h1>
                    <p>"Advanced data transformation, filtering, and analysis"</p>
                    <nav class="nav-links">
                        <a href="/">"🏠 Home"</a>
                        <a href="/control-flow">"🔀 Control Flow"</a>
                        <a href="/let-examples">"🎯 @let Patterns"</a>
                        <a href="/ui-library">"🧩 UI Components"</a>
                    </nav>
                </header>

                <main class="content">
                    <section class="data-section">
                        <h2>"📈 Sales Data Analysis"</h2>
                        <p class="section-desc">"Real-world data processing with collections and statistics"</p>

                        @SalesAnalysis()
                    </section>

                    <section class="data-section">
                        <h2>"🔍 Search & Filter Engine"</h2>
                        <p class="section-desc">"Advanced filtering with multiple criteria"</p>

                        @SearchFilter()
                    </section>

                    <section class="data-section">
                        <h2>"📋 Data Transformation Pipeline"</h2>
                        <p class="section-desc">"Chaining transformations and validations"</p>

                        @DataPipeline()
                    </section>

                    <section class="data-section">
                        <h2>"📊 Real-time Statistics Dashboard"</h2>
                        <p class="section-desc">"Dynamic stats with live data updates"</p>

                        @StatsDashboard()
                    </section>
                </main>
            </body>
        </html>
    }
}

#[azumi::component]
fn SalesAnalysis() -> impl azumi::Component {
    html! {
        <div class="analysis-card">
            <h3>"Quarterly Sales Breakdown"</h3>

            @let sales_data = vec![
                ("Q1", 125000, 89),
                ("Q2", 148000, 112),
                ("Q3", 167000, 95),
                ("Q4", 189000, 134),
            ];

            @let total_revenue: i32 = sales_data.iter().map(|(_, revenue, _)| revenue).sum();
            @let total_orders: i32 = sales_data.iter().map(|(_, _, orders)| orders).sum();
            @let avg_revenue_per_quarter = total_revenue / 4;

            <div class="stats-grid">
                <div class="stat-item">
                    <h4>"Total Revenue"</h4>
                    <p class="big-number">{format!("${}", total_revenue)}</p>
                </div>
                <div class="stat-item">
                    <h4>"Total Orders"</h4>
                    <p class="big-number">{total_orders}</p>
                </div>
                <div class="stat-item">
                    <h4>"Avg per Quarter"</h4>
                    <p class="big-number">{format!("${}", avg_revenue_per_quarter)}</p>
                </div>
                <div class="stat-item">
                    <h4>"Best Quarter"</h4>
                    <p class="big-number">"Q4"</p>
                </div>
            </div>

            <div class="sales-chart">
                <h4>"Quarterly Performance"</h4>
                @for (quarter, revenue, orders) in &sales_data {
                    @let performance = match revenue {
                        r if r >= 180000 => "high",
                        r if r >= 140000 => "medium",
                        _ => "low",
                    };

                    <div class="bar-item">
                        <span class="quarter">{quarter}</span>
                        <div class="bar">
                            <div class={format!("bar-fill {}", performance)} style={format!("width: {}%", (revenue / 1000) / 10)}></div>
                        </div>
                        <span class="value">{format!("${}", revenue)}</span>
                    </div>
                }
            </div>
        </div>
    }
}

#[azumi::component]
fn SearchFilter() -> impl azumi::Component {
    html! {
        <div class="search-card">
            <h3>"Product Search & Filter System"</h3>

            @let products = vec![
                ("Laptop Pro", "Electronics", 1299.99, 4.8, vec!["laptop", "computer", "work"]),
                ("Coffee Mug", "Kitchen", 24.99, 4.2, vec!["mug", "coffee", "kitchen"]),
                ("Gaming Chair", "Furniture", 349.99, 4.6, vec!["chair", "gaming", "furniture"]),
                ("Phone Case", "Electronics", 19.99, 4.0, vec!["phone", "case", "protection"]),
                ("Desk Lamp", "Home", 45.99, 4.4, vec!["lamp", "desk", "lighting"]),
            ];

            @let search_term = "laptop";
            @let min_rating = 4.0;

            @let filtered_products: Vec<_> = products
                .iter()
                .filter(|(_, category, _, rating, tags)| {
                    category.to_lowercase().contains("electronics") ||
                    tags.iter().any(|tag| tag.contains(search_term))
                })
                .filter(|(_, _, _, rating, _)| *rating >= min_rating)
                .collect();

            @let filtered_count = filtered_products.len();
            @let result_text = if search_term.is_empty() {
                format!("All {} products", products.len())
            } else {
                format!("{} results for '{}'", filtered_count, search_term)
            };

            <div class="search-info">
                <p>"🔍 " {result_text}</p>
                <p>"⭐ Minimum rating: " {min_rating}</p>
            </div>

            <div class="product-grid">
                @for (name, category, price, rating, tags) in &filtered_products {
                    @let category_color = match &category[..] {
                        "Electronics" => "category-electronics",
                        "Kitchen" => "category-kitchen",
                        "Furniture" => "category-furniture",
                        "Home" => "category-home",
                        _ => "category-default",
                    };

                    @let price_display = format!("${:.2}", price);
                    @let stars = "⭐".repeat(*rating as usize);

                    <div class="product-item">
                        <div class={format!("category-badge {}", category_color)}>{category}</div>
                        <h4>{name}</h4>
                        <p class="product-price">{price_display}</p>
                        <p class="rating">{stars} " (" {rating} ")"</p>
                        <div class="tags">
                            @for tag in tags {
                                <span class="tag">{tag}</span>
                            }
                        </div>
                    </div>
                }
            </div>
        </div>
    }
}

#[azumi::component]
fn DataPipeline() -> impl azumi::Component {
    html! {
        <div class="pipeline-card">
            <h3>"Data Transformation Pipeline"</h3>
            <p class="section-desc">"Multi-step data processing with validation and transformation"</p>

            @let raw_data = vec![
                "ALICE;25;developer;50000",
                "BOB;30;designer;60000",
                "CAROL;22;intern;30000",
                "DAVID;35;manager;80000",
                "EVE;28;developer;55000",
            ];

            <div class="pipeline-steps">
                <h4>"🔄 Processing Steps"</h4>

                <!-- Step 1: Parse -->
                <div class="pipeline-step">
                    <h5>"Step 1: Parse Raw Data"</h5>
                    @let parsed_data: Result<Vec<_>, _> = raw_data.iter().map(|line| {
                        let parts: Vec<&str> = line.split(';').collect();
                        if parts.len() == 4 {
                            Ok((parts[0], parts[1].parse::<i32>().unwrap_or(0), parts[2], parts[3].parse::<i32>().unwrap_or(0)))
                        } else {
                            Err("Invalid format")
                        }
                    }).collect();

                    @let processed_step1 = if let Ok(data) = &parsed_data {
                        format!("✅ Parsed {} records successfully", data.len())
                    } else {
                        "❌ Parsing failed".to_string()
                    };

                    <p>{processed_step1}</p>
                </div>

                <!-- Step 2: Transform -->
                @if let Ok(data) = &parsed_data {
                    <div class="pipeline-step">
                        <h5>"Step 2: Transform & Clean"</h5>
                        @let cleaned_data: Vec<_> = data
                            .iter()
                            .map(|(name, age, role, salary)| {
                                let clean_name = name.to_lowercase();
                                let clean_role = role.trim();
                                let formatted_salary = format!("${}", salary);
                                (clean_name, age, clean_role, formatted_salary, *salary >= 50000)
                            })
                            .collect();

                        <div class="transformed-data">
                            @for (name, age, role, salary, senior) in &cleaned_data {
                                <div class="data-item">
                                    <span class="name">{name}</span>
                                    <span class="age">"{age} years"</span>
                                    <span class="role">{role}</span>
                                    <span class="salary">{salary}</span>
                                    @if *senior {
                                        <span class="senior-badge">"Senior"</span>
                                    }
                                </div>
                            }
                        </div>
                    </div>

                    <!-- Step 3: Summary -->
                    <div class="pipeline-step">
                        <h5>"Step 3: Generate Summary"</h5>
                        @let total_salary: i32 = data.iter().map(|(_, _, _, salary)| salary).sum();
                        @let avg_age = data.iter().map(|(_, age, _, _)| *age).sum::<i32>() / data.len() as i32;
                        @let senior_count = data.iter().filter(|(_, _, _, salary)| *salary >= 50000).count();
                        @let roles: Vec<_> = data.iter().map(|(_, _, role, _)| *role).collect();

                        <div class="summary-stats">
                            <p>"👥 Total people: " {data.len()}</p>
                            <p>"💰 Total salary: " {format!("${}", total_salary)}</p>
                            <p>"🎂 Average age: " {avg_age}</p>
                            <p>"⭐ Senior roles: " {senior_count}</p>
                            <p>"🎯 Roles: " {roles.join(", ")}</p>
                        </div>
                    </div>
                }
            </div>
        </div>
    }
}

#[azumi::component]
fn StatsDashboard() -> impl azumi::Component {
    html! {
        <div class="dashboard-card">
            <h3>"📊 Real-time Statistics Dashboard"</h3>
            <p class="section-desc">"Dynamic statistics with live data simulation"</p>

            // Simulated live data - normalized to f64 for consistent typing
            @let live_metrics = (
                "Users Online",
                1247.0,
                vec![1200.0, 1180.0, 1245.0, 1230.0, 1250.0, 1247.0],
                "↗️ +3.2%"
            );

            @let server_metrics = (
                "Server Load",
                68.5,
                vec![45.2, 52.1, 58.9, 62.3, 65.7, 68.5],
                "↑ +5.1%"
            );

            @let response_metrics = (
                "Avg Response Time",
                245.3,
                vec![220.1, 235.4, 240.2, 238.7, 242.1, 245.3],
                "↗️ +1.4%"
            );

            <div class="live-stats">
                @for (name, current, history, trend) in vec![live_metrics, server_metrics, response_metrics] {
                    @let max_value = history.iter().fold(0.0f64, |a, &b| a.max(b));
                    @let min_value = history.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                    @let avg_value = history.iter().sum::<f64>() / history.len() as f64;

                    @let trend_class = if trend.contains("↗") || trend.contains("↑") {
                        "trend-up"
                    } else if trend.contains("↘") || trend.contains("↓") {
                        "trend-down"
                    } else {
                        "trend-neutral"
                    };

                    <div class="metric-card">
                        <div class="metric-header">
                            <h4>{name}</h4>
                            <span class={format!("trend {}", trend_class)}>{trend}</span>
                        </div>

                        <div class="metric-value">
                            @let value_text = match name {
                                "Users Online" => format!("{:.0}", current),
                                "Server Load" => format!("{:.1}%", current),
                                "Avg Response Time" => format!("{:.1}ms", current),
                                _ => format!("{}", current),
                            };
                            <span class="current-value">{value_text}</span>
                        </div>

                        <div class="metric-chart">
                            @for value in &history {
                                @let percentage = ((value - min_value) / (max_value - min_value)) * 80.0 + 10.0;
                                <div class="chart-bar" style={format!("height: {:.1}%", percentage)}></div>
                            }
                        </div>

                        <div class="metric-stats">
                            <span>"Min: " {format!("{:.1}", min_value)}</span>
                            <span>"Avg: " {format!("{:.1}", avg_value)}</span>
                            <span>"Max: " {format!("{:.1}", max_value)}</span>
                        </div>
                    </div>
                }
            </div>

            <div class="last-updated">
                <p>"🕒 Last updated: " {chrono::Utc::now().format("%H:%M:%S UTC")}</p>
                <p>"🔄 Auto-refreshing every 5 seconds"</p>
            </div>
        </div>
    }
}
