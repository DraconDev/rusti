use axum::response::{Html, IntoResponse};
use azumi::html;

pub async fn control_flow_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&control_flow_demo()))
}

fn control_flow_demo() -> impl azumi::Component {
    let users = vec!["Alice", "Bob", "Charlie"];
    let count = 5;

    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <title>"Control Flow Demo"</title>
                <style src="/static/homepage.css" />
                <style src="/static/control_flow.css" />
            </head>
            <body>
                <div class="container">
                    <h1>"Control Flow Examples"</h1>
                    <a href="/">"← Back to Home"</a>

                           <div class="demo-box">
                        <h2>"1. If Statements"</h2>
                        @if count > 10 {
                            <p class="status-active">"Count is high: " {count}</p>
                        }
                        @if count <= 10 {
                            <p class="status-pending">"Count is not high (≤10): " {count}</p>
                        }
                    </div>

                    <div class="demo-box">
                        <h2>"2. For Loops"</h2>
                        <ul>
                            @for user in &users {
                                <li>"User: " {user}</li>
                            }
                        </ul>
                    </div>

                    <div class="demo-box">
                        <h2>"3. Match Expressions"</h2>
                        <p>
                            "Current Status: "
                            @match count {
                                0 => { <span class="status-suspended">"Zero"</span> }
                                1..=5 => { <span class="status-pending">"Low (1-5)"</span> }
                                _ => { <span class="status-active">"Higher than 5"</span> }
                            }
                        </p>
                    </div>
                </div>
            </body>
        </html>
    }
}
