//! Lesson 1: hello_world.rs
//!
//! Simple template showing basic structure
use azumi::html;

/// Simple hello world with fancy styling
#[azumi::component]
pub fn hello_world() -> impl azumi::Component {
    style! {
        .hello-container {
            background-color: "#f5f5f5";
            padding: "20px";
            border-radius: "8px";
            box-shadow: "0 2px 4px rgba(0, 0, 0, 0.1)";
        }
        .hello-title {
            color: "#333";
            font-size: "1.5em";
            font-weight: "bold";
        }
    }
    html! {

        <div class={hello-container}>
            <h1 class={hello-title}>
                "Hello Azumi!"
            </h1>
        </div>
    }
}

/// Basic template with styling
#[azumi::component]
pub fn basic_template() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson1.css" />
        <div class="basic-template" >
            <h1 class="basic-h1">"Hello, World!"</h1>
            <h2 class="basic-h2">"Welcome to Azumi"</h2>
            <p class="basic-p">"This is a simple styled template"</p>
        </div>
    }
}

// Handler for Axum
pub async fn lesson1_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&hello_world()))
}
