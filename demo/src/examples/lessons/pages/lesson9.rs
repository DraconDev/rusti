//! Lesson 9: advanced_list_processing.rs
//!
//! Complex data transformations
use azumi::html;

#[derive(Clone)]
struct Item {
    name: String,
}

/// Filtered search using @let for computed filtered list
#[azumi::component]
pub fn filtered_search<'a>(items: &'a [Item], query: &'a str) -> impl azumi::Component + 'a {
    let filtered: Vec<Item> = items
        .iter()
        .filter(|item| item.name.contains(query))
        .cloned()
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
#[azumi::component]
pub fn matching_search() -> impl azumi::Component {
    let items = vec![
        Item {
            name: "Rust Book".to_string(),
        },
        Item {
            name: "Azumi Docs".to_string(),
        },
        Item {
            name: "Axum Guide".to_string(),
        },
    ];
    html! {
        @filtered_search(items=&items, query="Rust")
    }
}

/// Example with no match
#[azumi::component]
pub fn no_match_search() -> impl azumi::Component {
    let items = vec![
        Item {
            name: "Rust Book".to_string(),
        },
        Item {
            name: "Azumi Docs".to_string(),
        },
    ];
    html! {
        @filtered_search(items=&items, query="JavaScript")
    }
}
pub async fn lesson9_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! { @matching_search() }))
}
