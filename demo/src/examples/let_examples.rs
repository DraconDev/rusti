use axum::response::{Html, IntoResponse};
use azumi::html;

/// Comprehensive @match and @let examples
fn match_let_examples() -> impl azumi::Component {
    let status = UserStatus::Active;
    let priority = Priority::High;
    let notification_count = 7;

    html! {
        <style src="/static/let_examples.css" />
        <div class="example-card">
            <h1>"@match & @let Advanced Examples"</h1>
            <p>"Comprehensive pattern matching and variable binding"</p>

            <div class="match-example">
                <h3>"Status Match with @let"</h3>
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
                            {progress_percent}%
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Advanced @match with enums and tuples
fn advanced_match_examples() -> impl azumi::Component {
    let action = Action::Click {
        button_id: "submit".to_string(),
        coordinates: (100, 200),
    };
    let result = Result::<i32, &str>::Ok(42);

    html! {
        <div class="example-card">
            <h2>"Advanced @match Patterns"</h2>

            <div class="advanced-match">
                <h3>"Complex Pattern Matching"</h3>
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
                        format!("{}Pressed: {}{}", modifiers, if key == " " { "Space" } else { key }, key)
                    }
                    _ => "Unknown action".to_string(),
                };
                <div class="action-display">
                    <p>{match_result}</p>
                </div>
            </div>

            <div class="advanced-match">
                <h3>"Result Type Matching"</h3>
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

/// @let with conditional expressions
fn conditional_let_example() -> impl azumi::Component {
    let user_role = "admin";
    let status_message = match user_role {
        "admin" => "Full access granted",
        "user" => "Limited access",
        _ => "Unknown role",
    };

    html! {
        <div class="example-card">
            <h2>"@let with Conditionals"</h2>
            <p>{status_message}</p>
            <span class="role-badge">{user_role}</span>
        </div>
    }
}

/// @let with calculations
fn calculation_let_example() -> impl azumi::Component {
    let width = 100;
    let height = 50;
    let area = width * height;
    let perimeter = 2 * (width + height);

    html! {
        <div class="example-card">
            <h2>"@let with Calculations"</h2>
            <p>"Rectangle dimensions: " {width} " x " {height}</p>
            <p>"Area: " {area}</p>
            <p>"Perimeter: " {perimeter}</p>
        </div>
    }
}

/// @let with collections
fn collection_let_example() -> impl azumi::Component {
    let items = vec!["apple", "banana", "orange"];

    html! {
        <div class="example-card">
            <h2>"@let with Collections"</h2>
            @let item_count = items.len();
            @let first_item = items.first().unwrap_or(&"none");
            <p>"Total items: " {item_count}</p>
            <p>"First item: " {first_item}</p>
            <ul>
                @for item in &items {
                    <li>{item}</li>
                }
            </ul>
        </div>
    }
}

/// @let inside control flow
fn let_in_control_flow_example() -> impl azumi::Component {
    html! {
        <div class="example-card">
            <h2>"@let in Control Flow"</h2>

            @let show_details = true;
            @let user_level = 5;
            @let full_name = "John Doe";
            @let role = "Developer";
            @let level_badge = format!("Level {}", user_level);

            @if show_details {
                <div class="user-details">
                    <p>"Name: " {full_name}</p>
                    <p>"Role: " {role}</p>
                    <span class="level-badge">{level_badge}</span>
                </div>
            } @else {
                <p>"User details are hidden"</p>
            }
        </div>
    }
}

/// @let with complex data structures
fn complex_let_example() -> impl azumi::Component {
    let user_data = ("Alice", 28, "developer");
    let (name, age, profession) = user_data;
    let welcome_message = format!("Welcome {}! Age: {}, Role: {}", name, age, profession);
    let is_adult = age >= 18;

    html! {
        <div class="example-card">
            <h2>"@let with Complex Data"</h2>
            <p>{welcome_message}</p>
            <p>"Is adult: " {is_adult}</p>

            @let can_vote = is_adult;
            @let voting_status = if can_vote { "Can vote" } else { "Too young to vote" };
            <p>{voting_status}</p>
        </div>
    }
}

/// @let for component props preparation
fn let_for_props_example() -> impl azumi::Component {
    let button_text = "Click Me!";
    let button_class = "primary-btn";
    let is_disabled = false;

    html! {
        <div class="example-card">
            <h2>"@let for Component Props"</h2>
            <p>"Preparing data for components using @let"</p>

            @let button_id = format!("btn-{}", 123);
            <button
                id={button_id}
                class={button_class}
                disabled={is_disabled}>
                {button_text}
            </button>
        </div>
    }
}

/// @let with options and results
fn option_result_let_example() -> impl azumi::Component {
    let optional_name: Option<&str> = Some("Bob");
    let result_number: Result<i32, &str> = Ok(42);

    html! {
        <div class="example-card">
            <h2>"@let with Options & Results"</h2>
            @let display_name = optional_name.unwrap_or("Guest");
            @let number_value = result_number.unwrap_or(0);
            @let status_text = if number_value > 0 { "Active" } else { "Inactive" };
            <p>"Display name: " {display_name}</p>
            <p>"Number value: " {number_value}</p>
            <p>"Status: " {status_text}</p>
        </div>
    }
}

/// @let for computed CSS classes
fn css_class_let_example() -> impl azumi::Component {
    let priority = "high";
    let count = 5;

    html! {
        <div class="example-card">
            <h2>"@let for CSS Classes"</h2>
            <p>"Priority: " {priority}</p>
            <p>"Count: " {count}</p>
            @let priority_class = match priority {
                "high" => "priority-high",
                "medium" => "priority-medium",
                "low" => "priority-low",
                _ => "priority-unknown",
            };

            @let count_class = if count > 10 { "count-many" } else { "count-few" };
            @let combined_classes = format!("{} {}", priority_class, count_class);
            <div class={combined_classes}>
                "Styled by computed classes"
            </div>
        </div>
    }
}

/// Main function showing all @let examples
pub async fn let_examples_handler() -> impl axum::response::Response {
    let examples = vec![
        ("basic", basic_let_example()),
        ("formatted", formatted_date_example()),
        ("conditional", conditional_let_example()),
        ("calculations", calculation_let_example()),
        ("collections", collection_let_example()),
        ("control-flow", let_in_control_flow_example()),
        ("complex-data", complex_let_example()),
        ("component-props", let_for_props_example()),
        ("options-results", option_result_let_example()),
        ("css-classes", css_class_let_example()),
    ];

    let example_cards: Vec<_> = examples
        .into_iter()
        .map(|(name, component)| {
            html! {
                <section id={name} class="example-section">
                    {component}
                </section>
            }
        })
        .collect();

    html! {
        <style src="/static/let_examples.css" />
        <html>
            <head>
                <title>"Azumi @let Examples"</title>
                <meta charset="utf-8" />
            </head>
            <body>
                <header class="page-header">
                    <h1>"@let Examples in Azumi"</h1>
                    <p>"Demonstrating various uses of @let syntax"</p>
                    <nav class="nav-links">
                        <a href="/">"Home"</a>
                        <a href="/components">"Components"</a>
                        <a href="/forms">"Forms"</a>
                    </nav>
                </header>

                <main class="examples-container">
                    <div class="intro-section">
                        <h2>"What is @let?"</h2>
                        <p>"The @let syntax allows you to create local variable bindings within your templates. These variables are computed at render time and can be used to simplify complex expressions, format data, or prepare values for multiple uses."</p>

                        <div class="benefits">
                            <h3>"Benefits of @let:"</h3>
                            <ul>
                                <li>"Avoid repeating complex expressions"</li>
                                <li>"Format data once and use it multiple times"</li>
                                <li>"Make templates more readable"</li>
                                <li>"Compute values needed for multiple attributes"</li>
                                <li>"Separate logic from presentation"</li>
                            </ul>
                        </div>
                    </div>

                    {example_cards}
                </main>

                <footer class="page-footer">
                    <p>"Built with Azumi 2.0"</p>
                </footer>
            </body>
        </html>
    }
}
