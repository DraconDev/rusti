use axum::response::{Html, IntoResponse};
use rusti::{component, rusti};

#[component]
fn alert_box(message: String, is_error: bool) -> impl rusti::Component {
    rusti! {
        <div class={ if is_error { "bg-red-500" } else { "bg-blue-500" } }>
            {message}
        </div>
    }
}

pub fn component_macro_page() -> impl rusti::Component {
    // Manual usage for now, until parser supports named args
    let alert = alert_box::render(alert_box::Props {
        message: "Hello from Component Macro!".to_string(),
        is_error: false,
    });

    rusti! {
        <div>
            <h1>Component Macro Demo</h1>
            {alert}
        </div>
    }
}

pub async fn component_macro_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&component_macro_page()))
}
