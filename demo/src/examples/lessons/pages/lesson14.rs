//! Lesson 14: component_with_state.rs
//!
//! Managing component state
use azumi::html;

/// Counter component with internal state
/// Note: This demonstrates client-side state with HTMX-like onclick
pub fn counter(initial: u32) -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson14.css" />
        <div class="counter">
            <p class="count-display">"Count