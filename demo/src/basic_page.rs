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

