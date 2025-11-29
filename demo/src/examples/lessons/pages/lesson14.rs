//! Lesson 14: Advanced Patterns (Enums)
//!
//! Using Enums for component variants

use azumi::html;

#[derive()]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
}

#[azumi::component]
pub fn button(text: String, variant: ButtonVariant) -> impl azumi::Component {
    let class_name = match variant {
        ButtonVariant::Primary => "btn-primary",
        ButtonVariant::Secondary => "btn-secondary",
        ButtonVariant::Danger => "btn-danger",
    };

    html! {
        <style src="/static/pages/lesson14.css" />
        <button class={format!("btn {}", class_name)}>
            {text}
        </button>
    }
}

#[azumi::component]
pub fn variants_demo() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson14.css" />
        <div class="container">
            <h1>"Lesson 14: Component Variants"</h1>
            <p>"Using Enums to control component styles"</p>

            <div class="btn-group">
                @button(text="Save Changes".to_string(), variant=ButtonVariant::Primary)
                @button(text="Cancel".to_string(), variant=ButtonVariant::Secondary)
                @button(text="Delete".to_string(), variant=ButtonVariant::Danger)
            </div>
        </div>
    }
}

pub async fn lesson14_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! { @variants_demo() }))
}
