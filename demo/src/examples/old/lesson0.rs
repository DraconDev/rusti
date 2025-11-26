use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 0: Getting Started - Your first Azumi component
pub fn lesson0() -> impl azumi::Component {
    let simple_message = r#"<div>
    <h1>"Hello from Azumi!"</h1>
    <p>"This is your first lesson"</p>
    <p>"Azumi makes HTML type-safe"</p>
</div>"#;

    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Lesson 0: Getting Started - Azumi"</title>
                <style src="/static/lesson0.css" />
            </head>
            <body>
                <div class="container">
                    <div class="hero">
                        <h1>"🎯 Lesson 0: Getting Started"</h1>
                        <p class="tagline">"Welcome to Azumi! This tutorial will teach you how to build type-safe HTML components in Rust."</p>
                        <div class="badges">
                            <span class="badge">"Type-Safe HTML"</span>
                            <span class="badge">"Compile-Time Validation"</span>
                            <span class="badge">"Rust Powered"</span>
                        </div>
                    </div>

                    <div class="examples">
                        <h2>"💻 Your First Component"</h2>
                        <div class="grid">
                            <div class="card">
                                <div class="card-icon">"📝"</div>
                                <h3 class="card-title">"Code Example"</h3>
                                <p class="card-description">"Here's a simple Azumi component:"</p>
                                <pre class="card-description code-block">{simple_message}</pre>
                                <a href="#" class="card-link">"See Live Demo →"</a>
                            </div>

                            <div class="card">
                                <div class="card-icon">"✅"</div>
                                <h3 class="card-title">"What You See"</h3>
                                <p class="card-description">"The rendered output:"</p>
                                <div class="card-description preview-box">
                                    {html! {<div>
                                        <h1 class="preview-title">"Hello from Azumi!"</h1>
                                        <p class="preview-text">"This is your first lesson"</p>
                                        <p>"Azumi makes HTML type-safe"</p>
                                    </div>}}
                                </div>
                                <a href="#" class="card-link">"Learn More →"</a>
                            </div>
                        </div>
                    </div>

                    <div class="features">
                        <h2>"🚀 Next Steps"</h2>
                        <div class="feature-grid">
                            <div class="feature">
                                <h3>"📚 Lesson 1: Quoting Rules"</h3>
                                <p>"Learn how Azumi requires ALL text content to be quoted for type safety."</p>
                            </div>
                            <div class="feature">
                                <h3>"🎨 CSS Classes"</h3>
                                <p>"Understand how to use CSS classes properly in your components."</p>
                            </div>
                            <div class="feature">
                                <h3>"⚡ Advanced Features"</h3>
                                <p>"Explore control flow, components, and real-world applications."</p>
                            </div>
                        </div>

                        <div class="center-text">
                            <a href="/lesson-1" class="card-link inline-link">"Continue to Lesson 1 →"</a>
                        </div>
                    </div>
                </div>
            </body>
        </html>
    }
}

pub async fn lesson0_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson0()))
}
