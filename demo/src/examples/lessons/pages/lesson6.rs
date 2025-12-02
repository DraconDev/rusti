use azumi::html;

/// Lesson 6: Component Composition
///
/// Demonstrates how to compose components together
#[azumi::component]
pub fn simple_component() -> impl azumi::Component {
    html! {
        <style>
            .simple { padding: "10px"; background: "#f0f0f0"; }
        </style>
        <div class={simple}>
            <p>"Simple component content"</p>
        </div>
    }
}

/// Example: Composing multiple components
#[azumi::component]
pub fn composed_components() -> impl azumi::Component {
    html! {
        <style>
            .container { padding: "20px"; border: "1px solid #ddd"; }
            .title { color: "blue"; }
        </style>
        <div class={container}>
            <h2 class={title}>"Component Composition"</h2>
            @simple_component()
            @simple_component()
            <p>"Multiple components working together"</p>
        </div>
    }
}

/// Example: Nested component composition
#[azumi::component]
pub fn nested_composition() -> impl azumi::Component {
    html! {
        <style>
            .outer { padding: "15px"; background: "#e9e9e9"; }
            .inner { padding: "10px"; background: "white"; }
        </style>
        <div class={outer}>
            <h3>"Nested Components"</h3>
            <div class={inner}>
                @simple_component()
            </div>
        </div>
    }
}

/// Main lesson component
#[azumi::component]
pub fn lesson6() -> impl azumi::Component {
    html! {
        <style>
            .lesson_container { padding: "20px"; }
            .lesson_title { font-size: "24px"; color: "#333"; }
            .examples { display: "grid"; gap: "20px"; margin-top: "20px"; }
        </style>
        <div class={lesson_container}>
            <h1 class={lesson_title}>"Lesson 6: Component Composition"</h1>
            <p>"Learn how to build complex UIs by composing simple components"</p>

            <div class={examples}>
                @simple_component()
                @composed_components()
                @nested_composition()
            </div>
        </div>
    }
}

// Handler for Axum
pub async fn lesson6_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson6()))
}
