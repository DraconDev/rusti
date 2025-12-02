use azumi::html;

/// Lesson 10: Event Handling
///
/// Demonstrates basic event handling patterns
#[azumi::component]
pub fn click_counter() -> impl azumi::Component {
    html! {
        <style>
            .counter_container { padding: "20px"; text-align: "center"; }
            .counter_display { font-size: "24px"; margin: "10px 0"; }
            .counter_button { padding: "10px 20px"; background: "#2196f3"; color: "white"; border: "none"; cursor: "pointer"; }
        </style>
        <div class={counter_container}>
            <h3>"Click Counter"</h3>
            <div class={counter_display} id="counter">"0"</div>
            <button
                class={counter_button}
                onclick="document.getElementById('counter').textContent = parseInt(document.getElementById('counter').textContent) + 1"
            >
                "Click Me"
            </button>
        </div>
    }
}

/// Example: Form input handling
#[azumi::component]
pub fn input_handler() -> impl azumi::Component {
    html! {
        <style>
            .input_container { padding: "20px"; }
            .input_field { padding: "10px"; width: "100%"; margin: "10px 0"; }
            .input_display { padding: "10px"; background: "#f0f0f0"; margin-top: "10px"; }
        </style>
        <div class={input_container}>
            <h3>"Input Handler"</h3>
            <input
                class={input_field}
                type="text"
                placeholder="Type something..."
                oninput="document.getElementById('input-display').textContent = this.value"
            />
            <div class={input_display} id="input-display">"Your input will appear here"</div>
        </div>
    }
}

/// Example: Toggle functionality
#[azumi::component]
pub fn toggle_example() -> impl azumi::Component {
    html! {
        <style>
            .toggle_container { padding: "20px"; }
            .toggle_button { padding: "10px 20px"; background: "#4caf50"; color: "white"; border: "none"; cursor: "pointer"; }
            .toggle_content { margin-top: "10px"; padding: "15px"; background: "#f5f5f5"; display: "none"; }
            .visible { display: "block"; }
        </style>
        <div class={toggle_container}>
            <h3>"Toggle Example"</h3>
            <button
                class={toggle_button}
                onclick="document.getElementById('toggle-content').classList.toggle('visible')"
            >
                "Toggle Content"
            </button>
            <div class={toggle_content} id="toggle-content">
                <p>"This content can be toggled on and off!"</p>
            </div>
        </div>
    }
}

/// Main lesson component
#[azumi::component]
pub fn lesson10() -> impl azumi::Component {
    html! {
        <style>
            .lesson_container { padding: "20px"; }
            .lesson_title { font-size: "24px"; color: "#333"; }
            .examples { display: "grid"; gap: "20px"; margin-top: "20px"; }
        </style>
        <div class={lesson_container}>
            <h1 class={lesson_title}>"Lesson 10: Event Handling"</h1>
            <p>"Learn basic event handling patterns in Azumi"</p>

            <div class={examples}>
                @click_counter()
                @input_handler()
                @toggle_example()
            </div>
        </div>
    }
}

// Handler for Axum
pub async fn lesson10_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson10()))
}
