// Styles demo page – demonstrates gradient background using a CSS variable
use axum::response::{Html, IntoResponse};
use rusti::rusti;

pub fn styles_demo() -> impl rusti::Component {
    let styles = "\
        .gradient-demo {\
            width: 100%;\
            height: 100%;\
            background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);\
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
            <body class=\"min-h-screen flex items-center justify-center\">
                <div class=\"gradient-demo\"></div>
            </body>
        </html>
    }



pub async fn styles_demo_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&styles_demo()))
}
