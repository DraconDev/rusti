use axum::response::{Html, IntoResponse};
use azumi::html;

/// Comprehensive @match and @let examples
fn match_let_examples() -> impl azumi::Component {
    html! {
        <style src="/static/let_examples.css" />
        <div class="example-card">
            <h1>"@match & @let Advanced Examples"</h1>
            <p>"Comprehensive pattern matching and variable binding"</p>

            <div class="match-example">
                <h3>"Status Match with @let"</h3>
                @let status = UserStatus::Active;
                @let status_display = match status {
                    UserStatus::Active => "✅ Active User",
                    UserStatus::Inactive => "⏸️ Inactive",
                    UserStatus::Suspended => "🚫 Suspended",
                    UserStatus::Pending => "⏳ Pending Approval",
                };
                @let status_color = match status {
                    UserStatus::Active => "status-active",
                    UserStatus::Inactive => "status-inactive",
                    UserStatus::Suspended => "status-suspended",
                    UserStatus::Pending => "status-pending",
                };
                <div class={format!("status-display {}", status_color)}>
                    {status_display}
                </div>
            </div>

            <div class="match-example">
                <h3>"Priority Match with Dynamic Styling"</h3>
                @let priority = Priority::High;
                @let priority_badge = match priority {
                    Priority::Critical => "🔴 Critical",
                    Priority::High => "🟠 High",
                    Priority::Medium => "🟡 Medium",
                    Priority::Low => "🟢 Low",
                };
                @let badge_class = match priority {
                    Priority::Critical => "badge-critical",
                    Priority::High => "badge-high",
                    Priority::Medium => "badge-medium",
                    Priority::Low => "badge-low",
                };
                <span class={format!("priority-badge {}", badge_class)}>
                    {priority_badge}
                </span>
            </div>

            <div class="match-example">
                <h3>"Complex Match with Calculations"</h3>
                @let notification_count = 7;
                @let (alert_level, urgency_text) = match notification_count {
                    0 => ("low", "No notifications"),
                    1..=3 => ("medium", "Few notifications"),
                    4..=10 => ("high", "Many notifications"),
                    _ => ("urgent", "Excessive notifications"),
                };
                @let progress_percent = match alert_level {
                    "low" => 25,
                    "medium" => 50,
                    "high" => 75,
                    "urgent" => 100,
                    _ => 0,
                };
                <div class="notification-summary">
                    <p>"Alert Level: " {alert_level}</p>
                    <p>"Status: " {urgency_text}</p>
                    <div class="progress-bar">
                        <div class="progress-fill" style={format!("width: {}%", progress_percent)}>
                            {format!("{}%", progress_percent)}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Advanced @match with enums and tuples
fn advanced_match_examples() -> impl azumi::Component {
    html! {
        <div class="example-card">
            <h2>"Advanced @match Patterns"</h2>

            <div class="advanced-match">
                <h3>"Complex Pattern Matching"</h3>
                @let action = Action::Click { button_id: "submit".to_string(), coordinates: (100, 200) };
                @let match_result = match action {
                    Action::Click { button_id, coordinates: (x, y) } => {
                        format!("Clicked '{}' at ({}, {})", button_id, x, y)
                    }
                    Action::Scroll { delta_y, smooth } => {
                        format!("Scrolled {}px (smooth: {})", delta_y, smooth)
                    }
                    Action::KeyPress { key, ctrl_key, shift_key } => {
                        let modifiers = match (ctrl_key, shift_key) {
                            (true, false) => "Ctrl+",
                            (false, true) => "Shift+",
                            (true, true) => "Ctrl+Shift+",
                            (false, false) => "",
                        };
                        format!("{}Pressed: {}{}", modifiers, if key == " " { "Space" } else { &key }, key)
                    }
                    _ => "Unknown action".to_string(),
                };
                <div class="action-display">
                    <p>{match_result}</p>
                </div>
            </div>

            <div class="advanced-match">
                <h3>"Result Type Matching"</h3>
                @let result = Result::<i32, &str>::Ok(42);
                @let display_text = match result {
                    Ok(value) => format!("Success: {}", value),
                    Err(error) => format!("Error: {}", error),
                };
                @let display_class = match result {
                    Ok(_) => "result-ok",
                    Err(_) => "result-error",
                };
                <div class={format!("result-display {}", display_class)}>
                    <p>{display_text}</p>
                </div>
            </div>

            <div class="advanced-match">
                <h3>"Nested Match with @let Chaining"</h3>
                @let user_score = 85;
                @let grade_result = match user_score {
                    90..=100 => "A+",
                    80..=89 => "A",
                    70..=79 => "B",
                    60..=69 => "C",
                    0..=59 => "F",
                    _ => "Invalid",
                };
                @let grade_color = match grade_result {
                    "A+" | "A" => "grade-excellent",
                    "B" | "C" => "grade-good",
                    "F" => "grade-fail",
                    _ => "grade-invalid",
                };
                @let encouragement = match grade_result {
                    "A+" => "Outstanding! Perfect score!",
                    "A" => "Excellent work!",
                    "B" => "Good job!",
                    "C" => "Keep improving!",
                    "F" => "Need more practice!",
                    _ => "Invalid score",
                };
                <div class={format!("grade-display {}", grade_color)}>
                    <h4>"Grade: " {grade_result}</h4>
                    <p>{encouragement}</p>
                    <p>"Score: " {user_score}</p>
                </div>
            </div>
        </div>
    }
}

/// @let with control flow (no @else in Azumi)
fn control_flow_let_example() -> impl azumi::Component {
    html! {
        <div class="example-card">
            <h2>"@let with Control Flow"</h2>
            <p>"Demonstrating @let with @if (Azumi uses separate @if blocks, not @else)"</p>

            @let show_premium = true;
            @let premium_message = if show_premium { "🌟 Premium features enabled" } else { "💎 Upgrade to premium" };
            @let premium_class = if show_premium { "premium-enabled" } else { "premium-disabled" };

            @if show_premium {
                <div class={format!("premium-banner {}", premium_class)}>
                    <p>{premium_message}</p>
                    <p>"Enjoy your premium benefits!"</p>
                </div>
            }

            @let user_level = 15;
            @let level_message = if user_level >= 10 {
                format!("Level {} - Expert User", user_level)
            } else {
                format!("Level {} - Regular User", user_level)
            };
            @let level_class = if user_level >= 10 { "expert-user" } else { "regular-user" };

            @if user_level >= 10 {
                <div class={format!("level-badge {}", level_class)}>
                    <p>{level_message}</p>
                </div>
            }
        </div>
    }
}

/// Mathematical @let examples
fn math_let_examples() -> impl azumi::Component {
    html! {
        <div class="example-card">
            <h2>"Mathematical @let Examples"</h2>

            @let number = 42;
            @let doubled = number * 2;
            @let squared = number * number;
            @let is_even = number % 2 == 0;

            <div class="math-operations">
                <h3>"Basic Operations"</h3>
                <p>"Original: " {number}</p>
                <p>"Doubled: " {doubled}</p>
                <p>"Squared: " {squared}</p>
                <p>"Is Even: " {is_even}</p>
            </div>

            @let temperature = 25;
            @let celsius = temperature;
            @let fahrenheit = (celsius * 9/5) + 32;
            @let temp_description = match celsius {
                temp if temp < 0 => "Freezing",
                temp if temp < 10 => "Cold",
                temp if temp < 25 => "Cool",
                temp if temp < 30 => "Warm",
                _ => "Hot",
            };

            <div class="temperature-converter">
                <h3>"Temperature Converter"</h3>
                <p>{format!("{}°C = {}°F", celsius, fahrenheit)}</p>
                <p>"Weather: " {temp_description}</p>
            </div>

            @let viewport_width = 1200;
            @let container_width = if viewport_width > 768 { 800 } else { viewport_width - 40 };
            @let columns = match container_width {
                width if width >= 1200 => 4,
                width if width >= 992 => 3,
                width if width >= 768 => 2,
                _ => 1,
            };

            <div class="responsive-layout">
                <h3>"Responsive Calculations"</h3>
                <p>{format!("Viewport: {}px", viewport_width)}</p>
                <p>{format!("Container: {}px", container_width)}</p>
                <p>{format!("Columns: {}", columns)}</p>
            </div>
        </div>
    }
}

/// @let with collections and iteration
fn collection_let_examples() -> impl azumi::Component {
    html! {
        <div class="example-card">
            <h2>"@let with Collections"</h2>

            @let fruits = vec!["apple", "banana", "orange", "grape", "strawberry"];
            @let item_count = fruits.len();
            @let first_item = fruits.first().unwrap_or(&"none");
            @let last_item = fruits.last().unwrap_or(&"none");
            @let has_many_items = item_count > 3;

            <div class="collection-info">
                <h3>"Collection Statistics"</h3>
                <p>"Total items: " {item_count}</p>
                <p>"First item: " {first_item}</p>
                <p>"Last item: " {last_item}</p>
                <p>"Has many items: " {has_many_items}</p>
            </div>

            <div class="filtered-items">
                <h3>"Filtered Items (length > 5)"</h3>
                @let filtered_items: Vec<_> = fruits.iter().filter(|item| item.len() > 5).collect();
                @let filtered_count = filtered_items.len();
                <p>"Filtered count: " {filtered_count}</p>
                <ul>
                    @for item in &filtered_items {
                        <li>{item} " (length: " {item.len()} ")"</li>
                    }
                </ul>
            </div>
        </div>
    }
}

/// Complex data @let examples
fn complex_data_let_examples() -> impl azumi::Component {
    html! {
        <div class="example-card">
            <h2>"Complex Data @let Examples"</h2>

            @let user_data = ("Alice Johnson", 28, "developer", UserStatus::Active);
            @let (name, age, profession, status) = user_data;
            @let welcome_message = format!("Welcome {}! Age: {}, Role: {}", name, age, profession);
            @let is_adult = age >= 18;

            <div class="user-profile">
                <h3>"User Profile"</h3>
                <p>{welcome_message}</p>
                <p>"Status: " {format!("{:?}", status)}</p>
                <p>"Adult content access: " {is_adult}</p>
            </div>

            @let config = Config {
                theme: "dark".to_string(),
                language: "en".to_string(),
                notifications: true,
                auto_save: false,
            };
            @let theme_class = match config.theme.as_str() {
                "dark" => "theme-dark",
                "light" => "theme-light",
                "auto" => "theme-auto",
                _ => "theme-default",
            };
            @let language_display = match config.language.as_str() {
                "en" => "English",
                "es" => "Spanish",
                "fr" => "French",
                "de" => "German",
                _ => "Unknown",
            };

            <div class={format!("config-panel {}", theme_class)}>
                <h3>"Configuration"</h3>
                <p>"Theme: " {config.theme}</p>
                <p>"Language: " {language_display}</p>
                <p>"Notifications: " {config.notifications}</p>
                <p>"Auto-save: " {config.auto_save}</p>
            </div>
        </div>
    }
}

/// Data processing and transformation examples
fn data_processing_let_examples() -> impl azumi::Component {
    html! {
        <div class="example-card">
            <h2>"Data Processing @let Examples"</h2>

            @let raw_scores = vec![85, 92, 78, 96, 88, 73, 91];
            @let total_score: i32 = raw_scores.iter().sum();
            @let average_score = total_score as f64 / raw_scores.len() as f64;
            @let highest_score = raw_scores.iter().max().unwrap_or(&0);
            @let lowest_score = raw_scores.iter().min().unwrap_or(&0);
            @let score_range = highest_score - lowest_score;

            <div class="score-analysis">
                <h3>"Score Analysis"</h3>
                <p>"Total scores: " {total_score}</p>
                <p>"Average: " {average_score.round()}</p>
                <p>"Highest: " {highest_score}</p>
                <p>"Lowest: " {lowest_score}</p>
                <p>"Range: " {score_range}</p>
            </div>

            <div class="grade-analysis">
                <h3>"Grade Distribution"</h3>
                @let grades: Vec<_> = raw_scores.iter().map(|&score| {
                    match score {
                        90..=100 => "A",
                        80..=89 => "B",
                        70..=79 => "C",
                        60..=69 => "D",
                        _ => "F",
                    }
                }).collect();
                <p>"Sample grades: " {grades.join(", ")}</p>
            </div>

            <div class="performance-summary">
                <h3>"Performance Summary"</h3>
                @let passing_scores: Vec<_> = raw_scores.iter().filter(|&&score| score >= 70).collect();
                @let passing_rate = (passing_scores.len() * 100) / raw_scores.len();
                @let performance_level = match passing_rate {
                    rate if rate >= 90 => "Excellent",
                    rate if rate >= 75 => "Good",
                    rate if rate >= 60 => "Fair",
                    _ => "Needs Improvement",
                };
                <p>{format!("Passing students: {} / {}", passing_scores.len(), raw_scores.len())}</p>
                <p>{format!("Passing rate: {}%", passing_rate)}</p>
                <p>"Overall performance: " {performance_level}</p>
            </div>
        </div>
    }
}

/// Dynamic styling and CSS class examples
fn dynamic_styling_let_examples() -> impl azumi::Component {
    html! {
        <div class="example-card">
            <h2>"Dynamic Styling with @let"</h2>

            @let priority = "critical";
            @let size = "large";
            @let variant = "outline";

            @let button_classes = format!("btn btn-{} btn-{} btn-{}", size, priority, variant);
            @let status_classes = format!("status-indicator status-{}", priority);
            @let alert_classes = match priority {
                "critical" => "alert alert-critical",
                "warning" => "alert alert-warning",
                "info" => "alert alert-info",
                "success" => "alert alert-success",
                _ => "alert alert-default",
            };

            <div class="styling-examples">
                <h3>"Button Classes"</h3>
                <button class={button_classes}>
                    "Dynamic Button"
                </button>
            </div>

            <div class="status-examples">
                <h3>"Status Indicators"</h3>
                <div class={status_classes}>
                    "System Status"
                </div>
            </div>

            <div class="alert-examples">
                <h3>"Alert Messages"</h3>
                <div class={alert_classes}>
                    @let alert_message = match priority {
                        "critical" => "System failure detected!",
                        "warning" => "Storage running low",
                        "info" => "New update available",
                        "success" => "Operation completed successfully",
                        _ => "General notification",
                    };
                    <p>{alert_message}</p>
                </div>
            </div>
        </div>
    }
}

/// Option and Result type examples
fn option_result_let_examples() -> impl azumi::Component {
    html! {
        <div class="example-card">
            <h2>"Options & Results @let Examples"</h2>

            @let optional_name: Option<&str> = Some("Bob");
            @let optional_age: Option<i32> = None;
            @let result_number: Result<i32, &str> = Ok(42);
            @let result_string: Result<String, &str> = Err("Invalid input");

            @let display_name = optional_name.unwrap_or("Guest User");
            @let display_age = optional_age.unwrap_or(0);
            @let number_value = result_number.unwrap_or(0);
            @let string_value = result_string.unwrap_or("No value");

            <div class="option-examples">
                <h3>"Option Types"</h3>
                <p>"Name: " {display_name}</p>
                <p>"Age: " {display_age}</p>
            </div>

            @let number_status = match result_number {
                Ok(val) => format!("Success: {}", val),
                Err(err) => format!("Error: {}", err),
            };
            @let string_status = match &result_string {
                Ok(val) => format!("Success: {}", val),
                Err(err) => format!("Error: {}", err),
            };

            <div class="result-examples">
                <h3>"Result Types"</h3>
                <p>"Number result: " {number_status}</p>
                <p>"String result: " {string_status}</p>
            </div>

            @let config_valid = optional_name.is_some() && result_number.is_ok();
            @let config_message = if config_valid { "✅ Configuration valid" } else { "❌ Configuration invalid" };

            @if config_valid {
                <div class="config-success">
                    <p>{config_message}</p>
                    <p>"Ready to proceed with: " {display_name}</p>
                </div>
            }

            @if !config_valid {
                <div class="config-error">
                    <p>{config_message}</p>
                    <p>"Please check all required fields"</p>
                </div>
            }
        </div>
    }
}

/// Real-world utility examples
fn utility_let_examples() -> impl azumi::Component {
    html! {
        <div class="example-card">
            <h2>"Real-World @let Utilities"</h2>

            @let file_size = 1_250_000; // bytes
            @let file_size_kb = file_size / 1024;
            @let file_size_mb = file_size_kb / 1024;
            @let readable_size = if file_size_mb > 0 {
                format!("{:.1} MB", file_size_mb as f64 / 1024.0)
            } else if file_size_kb > 0 {
                format!("{} KB", file_size_kb)
            } else {
                format!("{} bytes", file_size)
            };

            <div class="file-info">
                <h3>"File Size Converter"</h3>
                <p>"Raw size: " {file_size} " bytes"</p>
                <p>"Readable: " {readable_size}</p>
            </div>

            @let email = "user@example.com";
            @let email_parts: Vec<&str> = email.split('@').collect();
            @let domain = email_parts.get(1).unwrap_or(&"");
            @let username = email_parts.get(0).unwrap_or(&"");
            @let is_valid_domain = domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.');

            <div class="email-info">
                <h3>"Email Parser"</h3>
                <p>"Original: " {email}</p>
                <p>"Username: " {username}</p>
                <p>"Domain: " {domain}</p>
                <p>"Valid domain: " {is_valid_domain}</p>
            </div>
        </div>
    }
}

pub async fn let_examples_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&let_examples_main()))
}

/// Supporting types for examples
#[derive(Debug, Clone)]
enum UserStatus {
    Active,
    Inactive,
    Suspended,
    Pending,
}

#[derive(Debug, Clone)]
enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone)]
enum Action {
    Click {
        button_id: String,
        coordinates: (i32, i32),
    },
    Scroll {
        delta_y: i32,
        smooth: bool,
    },
    KeyPress {
        key: String,
        ctrl_key: bool,
        shift_key: bool,
    },
    Hover {
        element_id: String,
    },
    None,
}

#[derive(Debug, Clone)]
struct Config {
    theme: String,
    language: String,
    notifications: bool,
    auto_save: bool,
}

fn let_examples_main() -> impl azumi::Component {
    html! {
        <style src="/static/let_examples.css" />
        <html>
            <head>
                <title>"Comprehensive @let Examples"</title>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            </head>
            <body>
                <header class="page-header">
                    <h1>"🎯 @let & @match Mastery"</h1>
                    <p>"Complete guide to Azumi's variable binding and pattern matching"</p>
                    <nav class="nav-links">
                        <a href="/">"Home"</a>
                        <a href="/components">"Components"</a>
                        <a href="/forms">"Forms"</a>
                        <a href="/control-flow">"Control Flow"</a>
                    </nav>
                </header>

                <main class="examples-container">
                    <div class="intro-section">
                        <h2>"What is @let?"</h2>
                        <p>"The @let syntax allows you to create local variable bindings within your templates. These variables are computed at render time and can be used to simplify complex expressions, format data, or prepare values for multiple uses."</p>

                        <div class="benefits">
                            <h3>"Key Benefits of @let:"</h3>
                            <ul>
                                <li>"Avoid repeating complex expressions"</li>
                                <li>"Format data once and use it multiple times"</li>
                                <li>"Make templates more readable and maintainable"</li>
                                <li>"Compute values needed for multiple attributes"</li>
                                <li>"Separate logic from presentation"</li>
                                <li>"Enable complex pattern matching with @match"</li>
                                <li>"Process collections and transformations"</li>
                            </ul>
                        </div>
                    </div>

                    {match_let_examples()}
                    {advanced_match_examples()}
                    {control_flow_let_example()}
                    {math_let_examples()}
                    {collection_let_examples()}
                    {complex_data_let_examples()}
                    {data_processing_let_examples()}
                    {dynamic_styling_let_examples()}
                    {option_result_let_examples()}
                    {utility_let_examples()}
                </main>

                <footer class="page-footer">
                    <p>"Built with Azumi - Pattern Matching and Variable Binding"</p>
                </footer>
            </body>
        </html>
    }
}
