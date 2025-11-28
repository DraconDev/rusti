//! Lesson 11: simple_component.rs
//!
//! Creating reusable components
use azumi::html;

#[derive(Clone, Debug)]
pub enum ButtonVariant {
    Primary,
    Secondary,
}

#[derive(Clone)]
pub struct ButtonProps {
    pub text: String,
    pub variant: ButtonVariant,
}

/// Reusable button component
#[azumi::component]
pub fn button(text: String, variant: ButtonVariant) -> impl azumi::Component {
    let class = match variant {
        ButtonVariant::Primary => "btn btn-primary",
        ButtonVariant::Secondary => "btn btn-secondary",
    };
    html! {
        <style src="/static/pages/lesson11.css" />
        <button class={class}>
            {text}
        </button>
    }
}

/// Primary button example
#[azumi::component]
pub fn primary_button() -> impl azumi::Component {
    html! {
        @button(text="Primary Button".to_string(), variant=ButtonVariant::Primary)
    }
}

/// Secondary button example
#[azumi::component]
pub fn secondary_button() -> impl azumi::Component {
    html! {
        @button(text="Secondary Button".to_string(), variant=ButtonVariant::Secondary)
    }
}
pub async fn lesson11_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! {
        <style src="/static/pages/lesson11.css" />
        <div>
            @primary_button()
            @secondary_button()
        </div>
    }))
}
