// ! Lesson 3: Introduction to CSS
//!
//! Shows how to link external CSS files
use azumi::html;

/// Example: Basic CSS linking
pub fn styled_card() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson3.css" />
        <div class="card">
            <h2 class="card-title">"Styled Card"</h2>
            <p class="card-text">"This card uses external CSS."</p>
        </div>
    }
}

/// Example: Multiple components, same CSS
pub fn another_card() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson3.css" />
        <div class="card">
            <h2 class="card-title">"Another Card"</h2>
            <p class="card-text">"Same CSS file, different content."</p>
        </div>
    }
}

/// Main lesson
pub fn lesson3() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson3.css" />
        <div class="lesson-container">
            <h1 class="lesson-title">"Lesson 3: CSS Integration"</h1>
            @styled_card()
            @another_card()
        </div>
    }
}

// Handler
pub async fn lesson3_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson3()))
}
