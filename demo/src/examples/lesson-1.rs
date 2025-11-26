use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 0: Getting Started - Your first Azumi component
pub fn lesson0() -> impl azumi::Component {
    let simple_message = r#"<div>
    <h1>"Hello from Azumi!"</h1>
    <p>"This is your first lesson"</p>
    <p>"Azumi makes HTML type-safe"</p>
</div>"#;