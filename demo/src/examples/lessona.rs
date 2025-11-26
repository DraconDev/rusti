use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 0: Getting Started - Your first Azumi component
pub fn lessona() -> impl azumi::Component {
    html! {
        <div class="cente">
            <h1>"Hello from Azumi!"</h1>
            <p>"This is your first lesson"</p>
            <p>"Azumi makes HTML type-safe"</p>
        </div>
    }
}
