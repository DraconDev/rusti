// Demo for base_layout and navbar using rusti macro

use axum::response::{Html, IntoResponse};
use rusti::rusti;

/// Head component with meta tags, scripts, and styles
fn page_head<'a>(title: &'a str) -> impl rusti::Component + 'a {
    rusti! {
        <head>
            <title>{ title }</title>
            <script src="https://unpkg.com/htmx.org@1.9.10"></script>
            <script src="https://cdn.tailwindcss.com"></script>
        </head>
    }
}

/// Test function with parameters and component calls
fn test_with_params<'a>(title: &'a str) -> impl rusti::Component + 'a {
    rusti! {
        <html lang="en">
            @page_head(title)
            <body>
                <h1>"Hello"</h1>
            </body>
        </html>
    }
}

/// Wrapper function to expose a demo page using the base layout.
pub fn base_layout_demo() -> impl rusti::Component {
    test_with_params("Test Title")
}

/// Axum handler for the `/base-layout` route.
pub async fn base_layout_demo_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&base_layout_demo()))
}
