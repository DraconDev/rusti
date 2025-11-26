//! Lesson 1: Hello World
//!
//! Simple template showing basic structure
//! This demonstrates the fundamental html! macro usage and basic component structure.

use azumi::html;

/// Simple hello world component demonstrating basic html! macro usage
pub fn hello_world() -> impl azumi::Component {
    html! {
        <div>
            <h1>"Hello, World!"</h1>
        </div>
    }
}

/// Alternative hello world with minimal styling
pub fn hello_world_styled() -> impl azumi::Component {
    html! {
        <div style="text-align: center; padding: 2rem; font-family: Arial, sans-serif;">
            <h1 style="color: #333;">"Hello, World!"</h1>
            <p style="color: #666;">"Welcome to Azumi templates"</p>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world_renders() {
        let component = hello_world();
        let html = azumi::render_to_string(&component);
        assert!(html.contains("Hello, World!"));
    }

    /// Web handler that serves the lesson examples
    use axum::response::{Html, IntoResponse};

    pub async fn lesson1_handler() -> impl IntoResponse {
        // Simple wrapper for web display
        let component = html! {
            <div style="padding: 2rem; font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto;">
                <h1 style="color: #333; border-bottom: 2px solid #007acc; padding-bottom: 0.5rem;">"Lesson 1: Hello World"</h1>

                <div style="background: #f5f5f5; padding: 1rem; border-radius: 0.5rem; margin: 1rem 0;">
                    <h2 style="color: #555;">"Basic Example"</h2>
                    {hello_world()}
                </div>

                <div style="background: #e8f4f8; padding: 1rem; border-radius: 0.5rem; margin: 1rem 0;">
                    <h2 style="color: #555;">"Styled Example"</h2>
                    {hello_world_styled()}
                </div>

                <div style="background: #fff3cd; border: 1px solid #ffeaa7; padding: 1rem; border-radius: 0.5rem;">
                    <p><strong>"Key Concepts:"</strong></p>
                    <ul>
                        <li>"Basic <code>html!</code> macro usage"</li>
                        <li>"Text content must be quoted"</li>
                        <li>"Returns <code>impl azumi::Component</code>"</li>
                    </ul>
                </div>
            </div>
        };

        Html(azumi::render_to_string(&component))
    }
}
