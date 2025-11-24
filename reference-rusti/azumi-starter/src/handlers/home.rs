use crate::templates;
use axum::response::Html;
use rusti::rusti;

pub async fn home_handler() -> Html<String> {
    let page = rusti! {
        @templates::base_layout(title = "Home - Azumi Starter".to_string(), is_authenticated = false) {
            @templates::home_page()
        }
    };
    Html(rusti::render_to_string(&page))
}
