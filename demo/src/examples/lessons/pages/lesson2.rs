//! Lesson 2: Data Binding
//!
//! Passing data to templates
//! This demonstrates how to pass data and render it in templates.

use azumi::html;

/// Simple greeting with data binding
pub fn simple_greeting(name: String) -> impl azumi::Component {
    html! {
        <div>
            <h1>"Hello, " {name}</h1>
        </div>
    }
}

/// Multiple data fields in template
pub fn user_info(name: String, role: String) -> impl azumi::Component {
    html! {
        <div>
            <h1>"Welcome, " {name}</h1>
            <p>"Role: " {role}</p>
        </div>
    }
}

/// Data binding with conditional content
pub fn user_status(is_logged_in: bool, name: String) -> impl azumi::Component {
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

/// Product display with formatted data
pub fn product_info(name: String, price: f64, in_stock: bool) -> impl azumi::Component {
    html! {
        <div>
            <h2>{name}</h2>
            <p>"Price: $" {price.to_string()}</p>
            @if in_stock {
                <p>"In stock"</p>
            } else {
                <p>"Out of stock"</p>
            }
        </div>
    }
}
