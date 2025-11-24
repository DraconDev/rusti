// Basic page – demonstrates basic Rusti usage with comments showing best practices
use axum::response::{Html, IntoResponse};
use rusti::{component, rusti};

#[component]
pub fn basic_page() -> impl rusti::Component {
    rusti! {
        <html lang="en">
            <head>
                <!-- Meta tags work with standard HTML comments -->
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Basic Page</title>

                <!-- Inline styles work great! Just quote "2em" to avoid lexer issues -->
                <style>
                    body {
                        font-family: sans-serif;
                        margin: 2 em;               /* Space required for 'em' due to Rust float syntax, but parser fixes it! */
                        background-color: #f4f4f4;  /* Hex colors work fine */
                        color: #333;
                        padding: 10px;              /* px works fine without space! */
                    }
                    h1 {
                        color: #0056b3;
                    }
                </style>
            </head>
            <body>
                @let name = "Basic Rusti Page";
                <h1>{namea}</h1>
                <p>This demonstrates a simple Rusti component with inline styles.</p>
                <p>Check the source to see comment examples!</p>

                <!-- Use raw strings for inline scripts - best practice -->
                <script>{r#"
                    console.log("Hello from Rusti!");
                    // You can use single quotes inside raw strings
                    const msg = 'This works!';
                "#}</script>
            </body>
        </html>
    }
}

pub async fn basic_page_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&basic_page::render(
        basic_page::Props {},
    )))
}
