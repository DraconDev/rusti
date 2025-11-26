//! Lesson 2: Data Binding
//!
//! Passing data to templates
//! This demonstrates how to pass data and render it in templates.

use azumi::html;

/// Simple greeting with data binding
pub fn simple_greeting(name: &str) -> impl azumi::Component {
    html! {
        <div>
            <h1>"Hello, " {name}</h1>
        </div>
    }
}

/// Multiple data fields in template
pub fn user_info(name: &str, role: &str) -> impl azumi::Component {
    html! {
        <div>
            <h1>"Welcome, " {name}</h1>
            <p>"Role: " {role}</p>
        </div>
    }
}

/// Data binding with conditional content
pub fn user_status(is_logged_in: bool, name: &str) -> impl azumi::Component {
    html! {
        <div>
            @if is_logged_in {
                <p>"Welcome back, " {name} "!"</p>
            } else {
                <p>"Please log in"</p>
            }
        </div>
    }
}

/// Simple counter component
pub fn counter_component(count: i32) -> impl azumi::Component {
    html! {
        <div>
            <h1>"Current count: " {count.to_string()}</h1>
        </div>
    }
}
