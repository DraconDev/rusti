use azumi::html;

/// Lesson 6: Control Flow Patterns
///
/// @if, @else, @for, @match patterns
#[azumi::component]
pub fn control_flow_example() -> impl azumi::Component {
    let show_details = true;
    let items = vec!["Item 1", "Item 2", "Item 3"];
    let status = "active";

    html! {
        <style>
            .content { padding: "1rem"; }
            .item { margin: "0.5rem 0"; padding: "0.5rem"; background: "#f5f5f5"; }
            .status_active { color: "green"; }
            .status_inactive { color: "red"; }
        </style>
        <div class={content}>
            <h2>"Control Flow Demo"</h2>

            @if show_details {
                <h3>"Detailed View"</h3>
                @for item in &items {
                    <div class={item}>{item}</div>
                }
            }

            @if !show_details {
                <h3>"Summary View"</h3>
                <p>"Total items: " {items.len()}</p>
            }

            @match status {
                "active" => {
                    <p class={status_active}>"Status: Active"</p>
                }
                "inactive" => {
                    <p class={status_inactive}>"Status: Inactive"</p>
                }
                _ => {
                    <p>"Status: Unknown"</p>
                }
            }
        </div>
    }
}

/// Example: Complex conditional logic
#[azumi::component]
pub fn complex_conditions() -> impl azumi::Component {
    let user_role = "admin";
    let has_permission = true;
    let items = vec!["Item 1", "Item 2", "Item 3"];

    html! {
        <style>
            .conditions_container { padding: "1rem"; background: "#f9f9f9"; }
            .permission_granted { color: "green"; font-weight: "bold"; }
            .permission_denied { color: "red"; font-weight: "bold"; }
        </style>
        <div class={conditions_container}>
            <h3>"Complex Conditions"</h3>

            @if user_role == "admin" && has_permission {
                <p class={permission_granted}>"Full access granted"</p>
                <ul>
                    @for item in &items {
                        <li>{item}</li>
                    }
                </ul>
            }

            @if user_role != "admin" {
                <p class={permission_denied}>"Limited access only"</p>
            }
        </div>
    }
}

/// Example: Pattern matching with enums
#[azumi::component]
pub fn pattern_matching_example() -> impl azumi::Component {
    let current_state = "loading";

    html! {
        <style>
            .pattern_container { padding: "1rem"; }
            .state_loading { color: "blue"; }
            .state_success { color: "green"; }
            .state_error { color: "red"; }
        </style>
        <div class={pattern_container}>
            <h3>"Pattern Matching"</h3>

            @match current_state {
                "loading" => {
                    <p class={state_loading}>"Loading data..."</p>
                }
                "success" => {
                    <p class={state_success}>"Data loaded successfully!"</p>
                }
                "error" => {
                    <p class={state_error}>"Error loading data"</p>
                }
                _ => {
                    <p>"Unknown state"</p>
                }
            }
        </div>
    }
}

/// Main lesson demonstration component
#[azumi::component]
pub fn lesson6() -> impl azumi::Component {
    html! {
        <style>
            .container { padding: "20px"; }
            .header { text-align: "center"; margin-bottom: "30px"; }
            .main_title { font-size: "32px"; color: "#333"; }
            .subtitle { font-size: "18px"; color: "#666"; }
            .key_points { background: "#f9f9f9"; padding: "20px"; border-radius: "8px"; margin-bottom: "30px"; }
            .section_title { font-size: "20px"; margin-bottom: "15px"; }
            .points_list { list-style: "none"; padding: "0"; }
            .point { margin-bottom: "10px"; }
            .examples { display: "grid"; gap: "20px"; }
            .example_card { border: "1px solid #ddd"; padding: "20px"; border-radius: "8px"; }
        </style>
        <div class={container}>
            <header class={header}>
                <h1 class={main_title}>"Lesson 6: Control Flow Patterns"</h1>
                <p class={subtitle}>"@if, @else, @for, @match patterns"</p>
            </header>

            <section class={key_points}>
                <h2 class={section_title}>"Key Concepts"</h2>
                <ul class={points_list}>
                    <li class={point}>"✅ @if for conditional rendering"</li>
                    <li class={point}>"✅ @for for iteration over collections"</li>
                    <li class={point}>"✅ @match for pattern matching"</li>
                    <li class={point}>"✅ All control flow works at compile time"</li>
                    <li class={point}>"✅ Type-safe expressions and patterns"</li>
                </ul>
            </section>

            <section class={examples}>
                <div class={example_card}>
                    @control_flow_example()
                </div>
                <div class={example_card}>
                    @complex_conditions()
                </div>
                <div class={example_card}>
                    @pattern_matching_example()
                </div>
            </section>
        </div>
    }
}

// Handler for Axum
pub async fn lesson6_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson6()))
}
