//! Lesson 18: CSS Variables
//!
//! Passing Rust values to CSS custom properties

use azumi::html;

#[azumi::component]
pub fn css_variables_demo() -> impl azumi::Component {
    let primary_color = "#3498db";
    let card_shadow = 8;
    
    html! {
        <style src="/static/pages/lesson18.css" />
        <div class="container">
        <h1>"Lesson 18: CSS Variables"</h1>
        <p class="description">"Pass Rust values directly to CSS with --custom-properties"</p>

        <h2>"Color Theming"</h2>
        <div class="theme-box" --primary={primary_color}>
        "Primary: " {primary_color}
        </div>
        
        <h2>"Progress Bar"</h2>
            @let progress = 75;
            <div class="progress" --width={progress}>
                <div class="progress-bar"></div>
                <span class="progress-label">{progress} "%"</span>
            </div>

            <h2>"Dynamic Shadows"</h2>
            <div class="card" --shadow={card_shadow}>
                "Shadow depth: " {card_shadow} "px"
            </div>

            <div class="code-example">
                <h3>"How it works:"</h3>
                <pre>
    "// Rust\n"
    "let progress = 75;\n"
    "<div --width={progress}>...</div>\n\n"
    "/* CSS */\n"
    ".progress-bar {\n"
    "  width: calc(var(--width) * 1%);\n"
    "}"
                </pre>
            </div>
        </div>
    }
}

pub async fn lesson18_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! { @css_variables_demo() }))
}
