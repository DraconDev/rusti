use axum::response::{Html, IntoResponse};
use rusti::{rusti, Component};

/// Demonstrates handling of quotes, braces, and tricky CSS units within the rusti! macro.
pub fn quote_demo() -> impl rusti::Component {
    // Using a string literal for the CSS to avoid lexer issues with 2em.
    let css = "margin: \"2em\"; padding: 1rem; background: #222; color: #eee;";
    rusti!(
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Quote & Braces Demo</title>
                <style>{css}``</style>
            </head>
            <body class="p-8">
                <h1 class='text-3xl font-bold text-center'>Quote & Braces Demo</h1>
                <p class="mt-4">
                    This example shows how to embed <strong>single quotes</strong>, "double quotes", and JSON-like <code>{"key": "value"}</code> inside HTML attributes.
                </p>
                <button class='mt-6 px-4 py-2 bg-blue-600 text-white' data-info='{"action":"test","id":42}'>
                    Click Me (data-info with JSON)
                </button>
            </body>
        </html>
    )
}

pub async fn quote_demo_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&quote_demo()))
}
