//! Lesson 2: Data Binding
//!
//! Passing data to templates
//! This demonstrates how to pass data and render it in templates.

use azumi::html;

/// Simple greeting with data binding
pub fn simple_greeting(name: String) -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson2.css" />
        <div class="lesson2-container">
            <h1 class="lesson2-title">"Hello, " {name}</h1>
        </div>
    }
}

/// Multiple data fields in template
pub fn user_info(name: String, role: String) -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson2.css" />
        <div class="lesson2-container">
            <h1 class="lesson2-title">"Welcome, " {name}</h1>
            <p class="lesson2-text">"Role: " {role}</p>
        </div>
    }
}

/// Data binding with conditional content
pub fn user_status(is_logged_in: bool, name: String) -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson2.css" />
        <div class="lesson2-container">
            @if is_logged_in {
                <p class="lesson2-text success">"Welcome back, " {name} "!"</p>
            } else {
                <p class="lesson2-text warning">"Please log in"</p>
            }
        </div>
    }
}

/// Simple counter component
pub fn counter_component(count: i32) -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson2.css" />
        <div class="lesson2-container">
            <h1 class="lesson2-title counter">"Current count: " {count.to_string()}</h1>
        </div>
    }
}

/// Product display with formatted data
pub fn product_info(name: String, price: f64, in_stock: bool) -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson2.css" />
        <div class="lesson2-container product">
            <h2 class="lesson2-subtitle">{name}</h2>
            <p class="lesson2-text price">"Price: $" {price.to_string()}</p>
            @if in_stock {
                <p class="lesson2-text success">"In stock"</p>
            } else {
                <p class="lesson2-text error">"Out of stock"</p>
            }
        </div>
    }
}
