//! Lesson 10: error_states_and_loading.rs
//!
//! Handling different data states with @match on Result
use azumi::html;

/// Data view handling Ok/Err states
pub fn data_view(ok_data: bool) -> impl azumi::Component {
    let data = if ok_data {
        Ok(vec!["Item 1", "Item 2"])
    } else {
        Err("Failed to load data".to_string())
    };
    html! {
        <style src="/static/pages/lesson10.css" />
        <div>
            @match &data {
                Ok(items) => {
                    <ul>
                        @for item in items {
                            <li>{item}</li>
                        }
                    </ul>
                },
                Err(error) => {
                    @let error_msg = format!("Error: {}", error);
                    <p class="error">{error_msg}</p>
                },
            }
        </div>
    }
}

/// Success state example
pub fn success_data() -> impl azumi::Component {
    data_view(true)
}

/// Error state example
pub fn error_data() -> impl azumi::Component {
    data_view(false)
}
pub async fn lesson10_handler() -> impl axum::response::IntoResponse { axum::response::Html("Lesson 10") }
