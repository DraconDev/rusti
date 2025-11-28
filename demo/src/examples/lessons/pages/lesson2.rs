//! Lesson 2: The Quoting Rule
//!
//! Demonstrates Azumi's strict quoting requirements
use azumi::html;

/// Example 1: Text content must be quoted
#[azumi::component]
pub fn quoted_text() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson2.css" />
        <div class="demo">
            <h1>"Text must always be quoted"</h1>
            <p>"This is correct syntax."</p>
        </div>
    }
}

/// Example 2: Attribute values must be quoted
#[azumi::component]
pub fn quoted_attributes() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson2.css" />
        <div class="demo">
            <button type="button" class="btn">"Attributes are quoted"</button>
            <input type="text" placeholder="Always use quotes" />
        </div>
    }
}

/// Example 3: Variables don't need quotes
// TODO: Add #[component] when lifetime support is added
pub fn unquoted_variables<'a>(name: &'a str, count: i32) -> impl azumi::Component + 'a {
    html! {
        <style src="/static/pages/lesson2.css" />
        <div class="demo">
            <p>"Hello, " {name} "!"</p>
            <p>"Count: " {count}</p>
        </div>
    }
}

/// Main lesson demonstration
#[azumi::component]
pub fn lesson2() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson2.css" />
        <div class="lesson-container">
            <h1 class="lesson-title">"Lesson 2: The Quoting Rule"</h1>

            @quoted_text()
            @quoted_attributes()
            @unquoted_variables("Alice", 42)
        </div>
    }
}

// Handler for Axum
pub async fn lesson2_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson2()))
}
