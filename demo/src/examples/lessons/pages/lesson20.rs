//! Lesson 20: Conditional CSS Classes
//!
//! Dynamic class names based on state

use azumi::html;

#[azumi::component]
pub fn conditional_classes_demo() -> impl azumi::Component {
    
    
    
    

    html! {
        <style src="/static/pages/lesson20.css" />
        <div class="container">
            <h1>"Lesson 20: Conditional CSS Classes"</h1>

            <h2>"Using format! for dynamic classes"</h2>
            @let is_active = true;
            <button class={format!("btn {}", if is_active { "active" } else { "" })}>
                "Toggle Button"
            </button>

            @let is_loading = false;
            <button class={format!("btn {} {}",
                if is_loading { "loading" } else { "" },
                if is_active { "active" } else { "" }
            )}>
                "Multi-State Button"
            </button>

            <h2>"Status-based styling"</h2>
            @let status = "success"; // "success", "warning", "error"
            <div class={format!("alert alert-{}", status)}>
                "Status: " {status}
            </div>

            <h2>"Count-based classes"</h2>
            @let count = 5;
            <span class={format!("badge {}", if count > 10 { "badge-large" } else { "badge-small" })}>
                {count}
            </span>

            <div class="code-example">
                <pre>
    "let is_active = true;\n"
    "<button class={format!(\"btn {}\", if is_active { \"active\" } else { \"\" })}>\n"
    "  \"Click Me\"\n"
    "</button>"
                </pre>
            </div>
        </div>
    }
}

pub async fn lesson20_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(
        &html! { @conditional_classes_demo() },
    ))
}
