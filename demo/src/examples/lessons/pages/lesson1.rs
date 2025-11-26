//! Lesson 1: hello_world.rs
//!
//! Simple template showing basic structure
use azumi::html;

/// Simple hello world with fancy styling
pub fn hello_world() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson1.css" />
        <div class="hello-container"> </div>
    }
}

/// Basic template with styling
pub fn basic_template() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson1.css" />
        <div class="basic-template" style="padding: 20px; border: 2px solid #000;">
            <h1 class="basic-h1">"Hello, World!"</h1>
            <h2 class="basic-h2">"Welcome to Azumi"</h2>
            <p class="basic-p">"This is a simple styled template"</p>
        </div>
    }
}
