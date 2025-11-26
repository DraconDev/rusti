//! Lesson 11: simple_component.rs
//!
//! Creating reusable components
use azumi::html;

#[derive(Clone)]
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
pub fn button(props: ButtonProps) -> impl azumi::Component {
    let class = match props.variant {
        ButtonVariant::Primary => "btn btn-primary",
        ButtonVariant::Secondary => "btn btn-secondary",
    };
    html! {
        <style src="/static/pages/lesson11.css" />
        <button class={class}>
            {&props.text}
        </button>
    }
}

/// Primary button example
pub fn primary_button() -> impl azumi::Component {
    button(ButtonProps {
        text: "Primary Button".to_string(),
        variant: ButtonVariant::Primary,
    })
}

/// Secondary button example
pub fn secondary_button() -> impl azumi::Component {
    button(ButtonProps {
        text: "Secondary Button".to_string(),
        variant: ButtonVariant::Secondary,
    })
}
