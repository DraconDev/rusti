//! Lesson 9: advanced_list_processing.rs
//!
//! Complex data transformations with @let
use azumi::html;

/// Filtered search with @let computed value
pub fn filtered_search(items: &[&str], query: &str) -> impl azumi::Component {
    html! {
        <div>
            @let filtered: Vec<&str> = items.iter()
                .filter(|item| item.contains(query))
                .copied()
                .collect();
            @if filtered.is_empty()