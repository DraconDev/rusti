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

