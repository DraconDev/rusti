// Styles demo page – demonstrates gradient background using a CSS variable
use axum::response::{Html, IntoResponse};
use rusti::rusti;

pub fn styles_demo() -> impl rusti::Component {
    let styles = "\
        body {\
            background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);\
            border-radius: 0.5rem;\
            box-shadow: 0 0 20px rgba(6,182,212,0.3);\
        }\
    ";
    rusti! {
        <html lang=\"en\">
            <head>
                <meta charset=\"UTF-8\" />
                <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />
                <title>Styles Demo</title>
                <style>{styles}</style>
            </head>
            <body class=\"bg-gray-900 text-white min-h-screen flex items-center justify-center\">
            </body>
        </html>
    }



pub async fn styles_demo_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&styles_demo()))
}
