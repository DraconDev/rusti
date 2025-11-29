//! Lesson 32: String Optimization
//!
//! Demonstrating the string concatenation optimization pattern.

use azumi::html;

#[azumi::component]
pub fn string_optimization_demo() -> impl azumi::Component {
    let username = "Alice";
    let count = 42;

    html! {
        <style src="/static/pages/lesson32.css" />
        <div class="container">
            <h1>"Lesson 32: String Optimization"</h1>

            <div class="demo-section">
                <h2>"The Pattern"</h2>
                <p>
                    "Azumi has a special optimization for string concatenation. "
                    "When a string literal is immediately followed by an expression inside braces, "
                    "it compiles to a single " <code>"format!"</code> " call."
                </p>

                <div class="comparison">
                    <div class="code-block">
                        <span class="code-title">"Standard Interpolation"</span>
                        <pre>"<p>\"Hello \" {name}</p>"</pre>
                        <div class="explanation">
                            "Compiles to separate write calls:"
                            <br/>
                            <code>"write!(f, \"Hello \")?;"</code>
                            <br/>
                            <code>"write!(f, \"{}\", name)?;"</code>
                        </div>
                    </div>

                    <div class="code-block">
                        <span class="code-title">"Optimized Pattern"</span>
                        <pre>"<p>{\"Hello \" name}</p>"</pre>
                        <div class="explanation">
                            "Compiles to a single write call:"
                            <br/>
                            <code>"write!(f, \"Hello {}\", name)?;"</code>
                        </div>
                    </div>
                </div>
            </div>

            <div class="demo-section">
                <h2>"Live Examples"</h2>

                <div class="result">
                    // Optimized: {"Prefix " expr}
                    <p>{"User: " username}</p>

                    // Optimized: {"Prefix " expr}
                    <p>{"Notification count: " count}</p>

                    // Complex expression
                    <p>{"Status: " if count > 0 { "Active" } else { "Inactive" }}</p>
                </div>
            </div>
        </div>
    }
}

pub async fn lesson32_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(
        &html! { @string_optimization_demo() },
    ))
}
