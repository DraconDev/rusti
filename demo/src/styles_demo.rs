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
            <body class="flex items-center justify-center body">
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
                <script src="https://cdn.tailwindcss.com"></script>
                <title>Styles Demo</title>
                <style>
                    .extension-container {
                        border: 1px solid #ccc;
                        padding: 20px;
                        background-color: #f0f0f0;
                        border-radius: 8px;
                    }
                    body {
                        background-color: #222222;
                    }
                    .extension-item {
                        margin-bottom: 15px;
                        padding: 10px;
                        background-color: #ffffff;
                        border-radius: 5px;
                        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
                    }
                    .button {
                        background-color: #007bff;
                        color: white;
                        padding: 10px 15px;
                        border: none;
                        border-radius: 5px;
                        cursor: pointer;
                        font-size: 16px;
                    }
                    .button:hover {
                        background-color: #0056b3;
                    }
                    .input-field {
                        width: 100%;
                        padding: 10px;
                        margin-bottom: 10px;
                        border: 1px solid #ccc;
                        border-radius: 4px;
                        box-sizing: border-box;
                    }
                    a {
                        color: #007bff;
                        text-decoration: none;
                    }
                    a:hover {
                        text-decoration: underline;
                    }
                    .card {
                        background-color: #ffffff;
                        border-radius: 8px;
                        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
                        padding: 20px;
                        margin: 20px;
                        max-width: 400px;
                    }
                    .card:hover {
                        box-shadow: 0 6px 12px rgba(0, 0, 0, 0.2);
                    }
                    .card-title {
                        font-size: 24px;
                        font-weight: bold;
                        margin-bottom: 10px;
                    }
                    .card-content {
                        font-size: 16px;
                        line-height: 1.5;
                    }
                    .card-footer {
                        margin-top: 20px;
                        text-align: center;
                    }
                    .card-footer a {
                        color: #007bff;
                        text-decoration: none;
                    }
                    .card-footer a:hover {
                        text-decoration: underline;
                    }
                    .card-footer a {
                        color: #007bff;
                        text-decoration: none;
                    }
                    .card-footer a:hover {
                        text-decoration: underline;
                    }
                </style>
            </head>
            <body class="flex items-center justify-center">
                <h1 class="text-4xl font-bold text-white">Styles Demo2</h1>
            </body>
        </html>
    }
}

pub async fn styles_demo2_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&styles_demo2()))
}
