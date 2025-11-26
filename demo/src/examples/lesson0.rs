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
            </head>
            <body>
                <div style="margin: 2rem; background: #f0f8ff; padding: 2rem; border-radius: 8px;">
                    <h1 style="color: #2b6cb0; margin-bottom: 1rem;">"🎯 Lesson 0: Getting Started"</h1>

                    <p style="color: #4a5568; margin-bottom: 1rem;">"Welcome to Azumi! This tutorial will teach you how to build type-safe HTML components in Rust."</p>

                    <h2 style="color: #3182ce; margin: 1.5rem 0 1rem 0;">"💻 Your First Component"</h2>
                    <pre style="background: #1a202c; color: #e2e8f0; padding: 1rem; border-radius: 4px; overflow-x: auto;">{simple_message}</pre>

                    <h2 style="color: #3182ce; margin: 1.5rem 0 1rem 0;">"✅ What You See"</h2>
                    <div style="background: white; border: 1px solid #e2e8f0; padding: 1rem; border-radius: 4px;">
                        {html! {<div>
                            <h1>"Hello from Azumi!"</h1>
                            <p>"This is your first lesson"</p>
                            <p>"Azumi makes HTML type-safe"</p>
                        </div>}}
                    </div>

                    <div style="margin-top: 2rem; padding: 1rem; background: #f7fafc; border-radius: 4px;">
                        <h3 style="color: #2b6cb0; margin-bottom: 0.5rem;">"🚀 Next Steps"</h3>
                        <p style="color: #4a5568;">"Ready to learn about quoting rules? Every piece of text in Azumi must be in quotes!"</p>
                        <a href="/lesson-1" style="display: inline-block; margin-top: 1rem; padding: 0.5rem 1rem; background: #3182ce; color: white; text-decoration: none; border-radius: 4px;">"Continue to Lesson 1 →"</a>
                    </div>
                </div>
            </body>
        </html>
    }
}

pub async fn lesson0_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson0()))
}
