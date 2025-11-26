//! Lesson 9: advanced_list_processing.rs
//!
//! Complex data transformations
use azumi::html;

#[derive(Clone)]
struct Item<'a> {
    name: &'a str,
}

/// Filtered search using @let for computed filtered list
pub fn filtered_search(items: &[Item], query: &str) -> impl azumi::Component + '_{
    let filtered: Vec<_> = items.iter()
        .filter(|item| item.name.contains(query))
        .collect();
    html! {
        <style src="/static/pages/lesson9.css" />
        <div class="search-container">
            @if filtered.is_empty() {
                <p class="no-results">"No results found"</p>
            } else {
                <ul class="filtered-list">
                    @for item in &filtered {
                        <li class="search-item">{item.name}</li>
                    }
                </ul>
            }
        </div>
    }
}

/// Example with matching query
pub fn matching_search() -> impl azumi::Component {
    let items = &[
        Item { name: "Rust Book" },
        Item { name: "Azumi Docs" },
        Item { name: "Axum Guide" },
    ];
    filtered_search(items, "Rust")
}

/// Example with no match
pub fn no_match_search() -> impl azumi::Component {
    let items = &[
        Item { name: "Rust Book" },
        Item { name: "Azumi Docs" },
    ];
    filtered_search(items, "JavaScript")
}
