//! Lesson 1: hello_world.rs
//!
//! Simple template showing basic structure
use azumi::html;

/// Simple hello world with fancy styling
#[azumi::component]
pub fn hello_world() -> impl azumi::Component {
    html! {
        <style>
            .hello_container {
                text-align: "center";
                margin-top: "cake";
                background-color: "#f00";
            }
            .hello_title {
                color: "green";
                font-size: "3em";
            }
        </style>
        <div class={hello_con tainer}>
            <h1 class={hello_title}>"Hello Azumi!"</h1>
            @basic_template()
        </div>
    }
}

/// Basic template with styling
#[azumi::component]
pub fn basic_template() -> impl azumi::Component {
    html! {
        <style>
            .basic_template {
                padding: "20px";
                background-color: "#f0f0f0";
            }
            .basic_h1 { color: "#333"; }
            .basic_h2 { color: "#666"; }
            .basic_p { font-size: "14px"; }
        </style>
        <div class={basic_template}>
            <h1 class={basic_h1}>"Hello, World!"</h1>
            <h2 class={basic_h2}>"Welcome to Azumi"</h2>
            <p class={basic_p}>"This is a simple styled template"</p>
        </div>
    }
}

// Handler for Axum
pub async fn lesson1_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&hello_world()))
}
