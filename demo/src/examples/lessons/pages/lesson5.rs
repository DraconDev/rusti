//! Lesson 5: CSS Scoping
//!
//! Shows automatic component-scoped CSS
use azumi::html;

/// Component A with scoped styles
pub fn component_a() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson5.css" />
        <div class="scoped-demo">
            <h2 class="demo-title">"Component A"</h2>
            <p class="primary-text">"This uses scoped CSS."</p>
        </div>
    }
}

/// Component B - same CSS file, automatically scoped
pub fn component_b() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson5.css" />
        <div class="scoped-demo alternate">
            <h2 class="demo-title">"Component B"</h2>
            <p class="primary-text">"Same CSS, no conflicts!"</p>
        </div>
    }
}

/// Main lesson
pub fn lesson5() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson5.css" />
        <div class="lesson-container">
            <h1 class="lesson-title">"Lesson 5: CSS Scoping"</h1>
            <p class="lesson-desc">"Azumi automatically scopes CSS to prevent conflicts."</p>
            @component_a()
            @component_b()
        </div>
    }
}

// Handler
pub async fn lesson5_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson5()))
}
