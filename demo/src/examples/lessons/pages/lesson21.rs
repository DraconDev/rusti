//! Lesson 21: CSS Scoping Demo
//!
//! How automatic CSS scoping prevents conflicts

use azumi::html;

#[azumi::component]
fn component_a() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson21-a.css" />
        <div class="card">
            <h3 class="title">"Component A"</h3>
            <p class="text">"I have blue styling"</p>
        </div>
    }
}

#[azumi::component]
fn component_b() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson21-b.css" />
        <div class="card">
            <h3 class="title">"Component B"</h3>
            <p class="text">"I have red styling"</p>
        </div>
    }
}

#[azumi::component]
pub fn css_scoping_demo() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson21.css" />
        <div class="container">
            <h1>"Lesson 21: CSS Scoping"</h1>
            <p class="description">
                "Each component's CSS is automatically scoped with a unique hash. "
                "Both use class 'card' but don't conflict!"
            </p>

            <div class="demo">
                @component_a()
                @component_b()
            </div>

            <div class="explanation">
                <h2>"How it works:"</h2>
                <p>"Azumi adds data attributes like " <code>"data-s3a7f"</code> " to scope CSS:"</p>
                <pre>
    "/* Your CSS: */\n"
    ".card { background: blue; }\n\n"
    "/* Compiled CSS: */\n"
    ".card[data-s3a7f] { background: blue; }"
                </pre>
                <p>"This prevents global style leaks between components!"</p>
            </div>
        </div>
    }
}

pub async fn lesson21_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! { @css_scoping_demo() }))
}
