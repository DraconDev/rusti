use axum::response::{Html, IntoResponse};
use rusti::rusti;

pub fn style_src_page() -> impl rusti::Component {
    rusti! {
        <html>
            <head>
                <title>Style Src Demo</title>
                <style src="styles.css" />
            </head>
            <body>
                <h1>This should be red</h1>
            </body>
        </html>
    }
}

pub async fn style_src_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&style_src_page()))
}
