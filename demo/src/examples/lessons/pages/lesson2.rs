//! Lesson 2: The Quoting Rule
//!
//! Demonstrates Azumi's strict quoting requirements
use azumi::html;

/// Example 1: Text content must be quoted
#[azumi::component]
pub fn quoted_text() -> impl azumi::Component {
    html! {
        <style>
            .demo { padding: "20px"; }
            .demo h1 { font-size: "24px"; }
            .demo p { font-size: "16px"; }
        </style>
        <div class={demo}>
            <h1>"Text must always be quoted"</h1>
            <p>"This is correct syntax."</p>
        </div>
    }
}

/// Example 2: Attribute values must be quoted
#[azumi::component]
pub fn quoted_attributes() -> impl azumi::Component {
    html! {
        <style>
            .demo { padding: "20px"; }
            .demo h1 { font-size: "24px"; }
            .demo p { font-size: "16px"; }
        </style>
        <div class={demo}>
            <button type="button" class="btn">"Attributes are quoted"</button>
            <input type="text" placeholder="Always use quotes" />
        </div>
    }
}

/// Example 3: Variables don't need quotes
#[azumi::component]
pub fn unquoted_variables<'a>(name: &'a str, count: i32) -> impl azumi::Component + 'a {
    html! {
        <style>
            .demo { padding: "20px"; }
            .demo h1 { font-size: "24px"; }
            .demo p { font-size: "16px"; }
        </style>
        <div class={demo}>
            <p>"Hello, " {name} "!"</p>
            <p>"Count: " {count}</p>
        </div>
    }
}

/// Main lesson demonstration
#[azumi::component]
pub fn lesson2() -> impl azumi::Component {
    html! {
        <style>
            .lesson-container { padding: "20px"; }
            .lesson-title { font-size: "24px"; }
        </style>
        <div class={lesson-container}>
            <h1 class={lesson-title}>
                "Lesson 2: The Quoting Rule"
            </h1>

            @quoted_text()
            @quoted_attributes()
            @unquoted_variables(name="Alice", count=42)
        </div>
    }
}

// Handler for Axum
pub async fn lesson2_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson2()))
}
