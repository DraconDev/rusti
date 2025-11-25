use axum::response::{Html, IntoResponse};
use rusti::rusti;

/// Simple hello world page demonstrating Rusti 2.0 basics
pub fn hello_world() -> impl rusti::Component {
    rusti! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <title>"Hello World - Rusti 2.0"</title>
                <style src="demo/static/hello.css" />
            </head>
            <body>
                <div class="container">
                    <h1>"Hello, Rusti 2.0!"</h1>
                    <p>"This demonstrates the new mandatory quoting rules."</p>

                    <div class="features">
                        <h2>"Key Features:"</h2>
                        <ul>
                            <li>"All text must be double-quoted"</li>
                            <li>"All attribute values must be quoted"</li>
                            <li>"CSS in external files"</li>
                            <li>"JavaScript in external files"</li>
                        </ul>
                    </div>

                    <button disabled>"Disabled Button"</button>
                    <button class="primary">"Click Me"</button>
                </div>

                <script src="/static/hello.js" />
            </body>
        </html>
    }
}

pub async fn hello_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&hello_world()))
}
