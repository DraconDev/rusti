use axum::response::{Html, IntoResponse};
use azumi::html;

/// Test global.css convention
pub fn global_test() -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <title>"Global CSS Test"</title>
                <style src="/static/global.css" />
                <style src="/static/global-test.css" />
            </head>
            <body>
                <div class="container">
                    <h1>"Global CSS Test"</h1>
                    <p class="description">"This component uses global.css for variables and a scoped CSS file."</p>
                    <button class="btn-primary">"Primary Button"</button>
                    <button class="btn-secondary">"Secondary Button"</button>
                </div>
            </body>
        </html>
    }
}

pub async fn global_test_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&global_test()))
}
