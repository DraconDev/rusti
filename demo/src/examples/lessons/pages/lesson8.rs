use azumi::html;

/// Lesson 8: Action System Deep Dive
///
/// Server-side interactivity patterns
#[azumi::component]
pub fn counter_display() -> impl azumi::Component {
    html! {
        <style>
            .counter { padding: "2rem"; text-align: "center"; border: "1px solid #eee"; }
            .count_display { font-size: "3rem"; margin: "1rem 0"; }
            .counter_button { padding: "1rem 2rem"; background: "#4caf50"; color: "white"; border: "none"; cursor: "pointer"; }
            .timestamp { font-size: "0.8rem"; color: "#666"; }
        </style>
        <div class={counter}>
            <div class={count_display}>"0"</div>
            <button class={counter_button}>
                "Increment"
            </button>
            <div class={timestamp}>"Last updated: 12:00:00"</div>
        </div>
    }
}

/// Example: Action with state management
#[azumi::component]
pub fn state_management_example() -> impl azumi::Component {
    html! {
        <style>
            .state_container { padding: "1.5rem"; background: "#f9f9f9"; }
            .state_info { margin: "0.5rem 0"; padding: "0.5rem"; background: "white"; }
            .action_button { padding: "0.75rem 1.5rem"; background: "#2196f3"; color: "white"; border: "none"; cursor: "pointer"; }
        </style>
        <div class={state_container}>
            <h3>"State Management"</h3>

            <div class={state_info}>
                <p>"Current State: Active"</p>
                <p>"Counter: 0"</p>
            </div>

            <button class={action_button}>
                "Update State"
            </button>
        </div>
    }
}

/// Example: Action composition
#[azumi::component]
pub fn action_composition_example() -> impl azumi::Component {
    html! {
        <style>
            .composition_container { padding: "1.5rem"; }
            .action_card { margin: "0.5rem 0"; padding: "1rem"; background: "#f5f5f5"; border: "1px solid #eee"; }
            .compose_button { padding: "0.75rem 1.5rem"; background: "#ff4081"; color: "white"; border: "none"; cursor: "pointer"; }
        </style>
        <div class={composition_container}>
            <h3>"Action Composition"</h3>

            <div class={action_card}>
                <p>"Multiple actions can be composed together"</p>
                <p>"Each action maintains its own state"</p>
            </div>

            <button class={compose_button}>
                "Compose Actions"
            </button>
        </div>
    }
}

/// Main lesson demonstration component
#[azumi::component]
pub fn lesson8() -> impl azumi::Component {
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
                <h1 class={main_title}>"Lesson 8: Action System Deep Dive"</h1>
                <p class={subtitle}>"Server-side interactivity patterns"</p>
            </header>

            <section class={key_points}>
                <h2 class={section_title}>"Key Concepts"</h2>
                <ul class={points_list}>
                    <li class={point}>"✅ Server-side actions for interactivity"</li>
                    <li class={point}>"✅ State management patterns"</li>
                    <li class={point}>"✅ Action composition"</li>
                    <li class={point}>"✅ Type-safe action parameters"</li>
                    <li class={point}>"✅ Compile-time action validation"</li>
                </ul>
            </section>

            <section class={examples}>
                <div class={example_card}>
                    @counter_display()
                </div>
                <div class={example_card}>
                    @state_management_example()
                </div>
                <div class={example_card}>
                    @action_composition_example()
                </div>
            </section>
        </div>
    }
}

// Handler for Axum
pub async fn lesson8_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson8()))
}
