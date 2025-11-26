use axum::response::{Html, IntoResponse};
use azumi::html;

// Define enums used in the examples
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
        coordinates: (u32, u32),
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
}

/// Basic @let example - formatting and reusing values
fn basic_let_example() -> impl azumi::Component {
    let name = "Alice";
    let greeting = format!("Hello, {}!", name);

    html! {
        <style src="/static/let_examples.css" />
        <div class="example-card">
            <h1>"@let Basic Example"</h1>
            <p>{greeting}</p>
        </div>
    }
}

/// @let with dynamic formatting
fn formatted_date_example() -> impl azumi::Component {
    let post_date = "2024-12-25";
    let formatted_date = format!("Published on {}", post_date);

    html! {
        <div class="example-card">
            <h2>"@let with Formatting"</h2>
            <p>{formatted_date}</p>
            <p>"This shows how @let can format dates dynamically"</p>
        </div>
    }
}

/// Main function showing all @let examples
pub async fn let_examples_handler() -> impl IntoResponse {
    use axum::response::Html;

    let examples = vec![
        ("basic", basic_let_example()),
        ("formatted", formatted_date_example()),
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

    Html(azumi::render_to_string(&html! {
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
    }))
}
