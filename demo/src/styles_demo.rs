// Styles demo page – demonstrates gradient background using inline CSS
use axum::response::{Html, IntoResponse};
use rusti::rusti;

pub fn styles_demo() -> impl rusti::Component {
    rusti! {
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Styles Demo</title>
                <style>
                    // :root {
                    //     --gradient-start: #0f172a;
                    //     --gradient-end: #1e293b;
                    // }
                    body {
                        margin: 0;
                        background: linear-gradient(135deg, var(--gradient-start) 0%, var(--gradient-end) 100%);
                        min-height: 100vh;
                        color: red;
                        background-color: #222222;
                    }
                </style>
            </head>
            <body class="flex items-center justify-center">
                <h1 class="text-4xl font-bold text-white">Styles Demo</h1>
            </body>
        </html>
    }
}

pub async fn styles_demo_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&styles_demo()))
}

pub fn styles_demo2() -> impl rusti::Component {
    rusti! {
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Styles Demo</title>
                <style>
                    .extension-container {
                        border: 1px solid #ccc; 
                        padding: 20px;
                        background-color: #f0f0f0;
                        border-radius: 8px;
                    }
                    .extension-item {
                        margin-bottom: 15px;
                        padding: 10px;
                        background-color: #ffffff;
                        border-radius: 5px;
                        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
                    }

        
                </style>
            </head>
            <body class="flex items-center justify-center">
                <h1 class="text-4xl font-bold text-white">Styles Demo</h1>
            </body>
        </html>
    }
}