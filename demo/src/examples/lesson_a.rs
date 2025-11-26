use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 0: Getting Started - Your first Azumi component
pub fn lesson_a() -> impl azumi::Component {
    html! {
        <style src="/static/lesson_a.css" />
        <div class="babya">
            <h1>"Hello from Azumi!"</h1>
            <p>"This is your first lesson"</p>
            <p>"Azumi makes HTML type-safe"</p>
        </div>
    }
}

pub async fn lesson_a_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson_a()))
}
