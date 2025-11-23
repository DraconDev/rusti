// Demo for base_layout and navbar using rusti macro

use axum::response::{Html, IntoResponse};
use rusti::rusti;

/// Head component with meta tags, scripts, and styles
fn page_head<'a>(title: &'a str) -> impl rusti::Component + 'a {
    rusti! {
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>{ title }</title>
            <script src="https://unpkg.com/htmx.org@1.9.10"></script>
            <script src="https://cdn.tailwindcss.com"></script>
        </head>
    }
}

/// Test function with content parameter
fn test_with_content<'a>(
    title: &'a str,
    content: impl rusti::Component + 'a,
) -> impl rusti::Component + 'a {
    rusti! {
        <html lang="en">
            @page_head(title)
            <body>
                @content
            </body>
        </html>
    }
}

/// Wrapper function to expose a demo page using the base layout.
pub fn base_layout_demo() -> impl rusti::Component {
    let content = rusti! {
        <h1>"Welcome to the Base Layout Demo"</h1>
        <p>"This page demonstrates the base_layout component with navbar."</p>
    };
    test_with_content("Test Title", content)
}

/// Axum handler for the `/base-layout` route.
pub async fn base_layout_demo_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&base_layout_demo()))
}
