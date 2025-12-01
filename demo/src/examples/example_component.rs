//! Lesson 1: hello_world.rs
//!
//! Simple template showing basic structure
use azumi::html;

/// Simple hello world with fancy styling
/// Maybe use underscores?
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
            @let subtitle = style! {
                .subtitle {
                    color: "#666";
                    font-size: "1em";
                }
            };
            <h2 class={subtitle}>
                "Hello World!"
            </h2>
        </div>
    }
}

#[azumi::component]
pub fn hello_world() -> impl azumi::Component {
    // style! {
    //     .hello-container {
    //         background-color: "#f5f5f5";
    //         padding: "20px";
    //         border-radius: "8px";
    //         box-shadow: "0 2px 4px rgba(0, 0, 0, 0.1)";
    //     }
    //     .hello-title {
    //         color: "#333";
    //         font-size: "1.5em";
    //         font-weight: "bold";
    //     }
    // }

    html! {
        // or this style perhaps with underscores
        <style>
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
        </style>
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
    style! {
        .basic-template {
            background-color: "#f5f5f5";
            padding: "20px";
            border-radius: "8px";
            box-shadow: "0 2px 4px rgba(0, 0, 0, 0.1)";
        }
        .basic-h1 {
            color: "#333";
            font-size: "1.5em";
            font-weight: "bold";
        }
        .basic-h2 {
            color: "#666";
            font-size: "1.2em";
        }
        .basic-p {
            color: "#999";
            font-size: "1em";
        }
    }
    html! {
        <div class={basic-template} >
            <h1 class={basic-h1}>
                "Hello, World!"
            </h1>
            <h2 class={basic-h2}>
                "Welcome to Azumi"
            </h2>
            <p class={basic-p}>
                "This is a simple styled template"
            </p>
        </div>
    }
}

// Handler for Axum
pub async fn lesson1_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&hello_world()))
}
