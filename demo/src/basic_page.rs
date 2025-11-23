// Basic page – demonstrates basic Rusti usage
use axum::response::{Html, IntoResponse};
use rusti::rusti;

pub fn basic_page() -> impl rusti::Component {
    rusti! {
        <html lang="en">
            <head>
                <!-- Page metadata -->
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Basic Page</title>
                <!-- Add your CSS stylesheets here -->
                <!-- <link rel="stylesheet" href="/styles.css" /> -->

                <!-- Add any meta tags for SEO or social media here -->
                <!-- <meta name="description" content="A simple Rusti application." /> -->
                <!-- <meta property="og:title" content="Basic Page" /> -->

                <!-- You can also include inline styles if necessary -->
                <style>
                    body {
                        font-family: sans-serif;
                        margin: 2 em;
                        background-color: #f4f4f4;
                        color: #333;
                    }
                    h1 {
                        color: #0056b3;
                    }
                </style>
            </head>
            <body>
                <!-- Main content section -->
                <h1>Basic Page</h1>
                <p>This is a basic Rusti page.</p>
                <!-- Inline script for demonstration -->
                <script>
                    console.log("Hello from Rusti!");
                </script>
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
