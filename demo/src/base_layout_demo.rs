// Demo for base_layout and navbar using rusti macro

use axum::response::{Html, IntoResponse};
use rusti::rusti;

/// Test function with no parameters
fn test_html() -> impl rusti::Component {
    rusti! {
        <html>
            <head>
                <title>"Test"</title>
            </head>
            <body>
                <h1>"Hello"</h1>
            </body>
        </html>
    }
}

/// Wrapper function to expose a demo page using the base layout.
pub fn base_layout_demo() -> impl rusti::Component {
    test_html()
}

/// Axum handler for the `/base-layout` route.
pub async fn base_layout_demo_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&base_layout_demo()))
}
