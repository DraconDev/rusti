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

/// Basic template with multiple elements
pub fn basic_template() -> impl azumi::Component {
    html! {
        <div>
            <h1>"Hello, World!"</h1>
            <h2>"Welcome to Azumi"</h2>
            <p>"This is a simple template showing basic structure"</p>
        </div>
    }
}

/// Template with nested elements
pub fn nested_template() -> impl azumi::Component {
    html! {
        <div>
            <header>
                <h1>"Site Title"</h1>
            </header>
            <main>
                <section>
                    <h2>"Introduction"</h2>
                    <p>"Welcome to our website"</p>
                </section>
            </main>
        </div>
    }
}
