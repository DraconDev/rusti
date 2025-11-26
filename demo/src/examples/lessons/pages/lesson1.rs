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

/// Alternative hello world with minimal styling using external CSS
pub fn hello_world_styled() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson1.css" />
        <div class="hello-container">
            <h1 class="hello-title">"Hello, World!"</h1>
            <p class="hello-subtitle">"Welcome to Azumi templates"</p>
        </div>
    }
}

/// Multiple greetings demonstration
pub fn multiple_greetings() -> impl azumi::Component {
    html! {
        <div class="greetings">
            <h1>"Hello, World!"</h1>
            <h2>"Greetings from Azumi"</h2>
            <p>"This is a simple template showing basic structure"</p>
        </div>
    }
}
