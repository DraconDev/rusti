use azumi::html;

#[azumi::component]
pub fn test_global_styles() -> impl azumi::Component {
    // html! { ... }
    Ok::<(), std::fmt::Error>(())
}

pub async fn handler() -> axum::response::Html<String> {
    axum::response::Html(azumi::render_to_string(&test_global_styles()))
}
