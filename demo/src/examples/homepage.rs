pub mod educational_homepage;

// Re-export the educational homepage
pub use educational_homepage::homepage;

/// Homepage showcasing all Azumi examples (legacy - superseded by educational_homepage)
pub fn legacy_homepage() -> impl azumi::Component {
    educational_homepage::homepage()
}

pub async fn homepage_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&homepage()))
}
