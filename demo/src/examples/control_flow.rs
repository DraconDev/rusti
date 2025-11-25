use axum::response::{Html, IntoResponse};
use azumi::html;

pub async fn control_flow_handler() -> impl IntoResponse {
    let users = vec!["Alice", "Bob", "Charlie"];
    let status = Status::Active;
    let count = 5;

    Html(azumi::render_to_string(&control_flow_demo(
        users, status, count,
    )))
}

enum Status {
    Active,
    Pending,
    Suspended,
}

fn control_flow_demo(users: Vec<&str>, status: Status, count: i32) -> impl azumi::Component {
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
                        <h2>"1. If / Else Statements"</h2>
                        @if count > 10 {
                            <p>"Count is high: " {count}</p>
                        } @else if count > 0 {
                            <p>"Count is positive: " {count}</p>
                        } @else {
                            <p>"Count is zero or negative"</p>
                        }
                    </div>

                    <div class="demo-box">
                        <h2>"2. For Loops"</h2>
                        <ul>
                            @for user in users {
                                <li>"User: " {user}</li>
                            }
                        </ul>
                    </div>

                    <div class="demo-box">
                        <h2>"3. Match Expressions"</h2>
                        <p>
                            "Current Status: "
                            @match status {
                                Status::Active => { <span class="status-active">"Active"</span> }
                                Status::Pending => { <span class="status-pending">"Pending"</span> }
                                Status::Suspended => { <span class="status-suspended">"Suspended"</span> }
                            }
                        </p>
                    </div>
                </div>
            </body>
        </html>
    }
}
