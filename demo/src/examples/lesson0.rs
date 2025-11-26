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
                <style src="/static/homepage.css" />
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
                                <pre class="card-description" style="background: var(--bg-dark); color: var(--text-main); padding: 1rem; border-radius: 0.5rem; font-family: 'Fira Code', monospace; font-size: 0.8rem; overflow-x: auto;">{simple_message}</pre>
                                <a href="#" class="card-link">"See Live Demo →"</a>
                            </div>

                            <div class="card">
                                <div class="card-icon">"✅"</div>
                                <h3 class="card-title">"What You See"</h3>
                                <p class="card-description">"The rendered output:"</p>
                                <div class="card-description" style="background: var(--bg-card); padding: 1rem; border-radius: 0.5rem; border: 1px solid var(--border);">
                                    {html! {<div>
                                        <h1 style="font-size: 1.5rem; margin-bottom: 1rem;">"Hello from Azumi!"</h1>
                                        <p style="margin-bottom: 0.5rem;">"This is your first lesson"</p>
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

                        <div style="text-align: center; margin-top: 2rem;">
                            <a href="/lesson-1" class="card-link" style="display: inline-block; margin: 0;">"Continue to Lesson 1 →"</a>
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
