//! Lesson 4: CSS Validation
//!
//! Demonstrates compile-time CSS validation
use azumi::html;

/// Example: All classes defined - compiles successfully
pub fn valid_classes() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson4.css" />
        <div class="validated-box">
            <h2 class="validated-title">"Valid CSS"</h2>
            <p class="validated-text">"All classes are defined in CSS."</p>
        </div>
    }
}

/// Example: Shows what valid CSS looks like
pub fn another_valid() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson4.css" />
        <di class="validated-box highlight">
            <h2 class="validated-title">"Another Valid Example"</h2>
            <p class="validated-text">"Azumi validates at compile time."</p>
        </di>
    }
}

/// Main lesson
pub fn lesson4() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson4.css" />
        <div class="lesson-container">
            <h1 class="lesson-title">"Lesson 4: CSS Validation"</h1>
            <p class="lesson-desc">"Every class must be defined in CSS or it won't compile."</p>
            @valid_classes()
            @another_valid()
        </div>
    }
}

// Handler
pub async fn lesson4_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson4()))
}
