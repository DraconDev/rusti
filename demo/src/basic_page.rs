// Basic page – demonstrates basic Rusti usage
use axum::response::{Html, IntoResponse};
use rusti::rusti;

pub fn basic_page() -> impl rusti::Component {
    rusti! {
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Basic Page</title>
                
                {/* Inline styles demonstration */}
                <style>
                    body {
                        font-family: sans-serif;
                        margin: "2em";              {/* Quoted to avoid 2e lexer error */}
                        background-color: #f4f4f4;
                        color: #333;
                        padding: 10px;
                    }
                    h1 {
                        color: #0056b3;
                    }
                </style>
            </head>
            <body>
                <h1>Basic Rusti Page</h1>
                <p>This demonstrates a simple Rusti component with inline CSS.</p>
                <p>Note: The margin uses quoted "2em" to avoid Rust's scientific notation parser.</p>
                
                {/* Best practice: Use raw strings for scripts */}
                <script>{r#"
                    console.log("Hello from Rusti!");
                "#}</script>
            </body>
        </html>
    }
}

pub async fn basic_page_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&basic_page()))
}

pub fn extension_page() -> impl rusti::Component {
    rusti! {
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Extension Page Example</title>
            </head>
            <body>
                <h1>Welcome to the Extension Page!</h1>
                <p>This is an additional page demonstrating more Rusti capabilities.</p>
                <a href="/">Back to Basic Page</a>
            </body>
        </html>
    }
}

pub async fn extension_page_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&extension_page()))
}
