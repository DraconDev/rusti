//! Lesson 14: component_with_state.rs
//!
//! Managing component state and advanced patterns
use azumi::html;

/// Counter component with mutable state
pub fn counter(initial: u32) -> impl azumi::Component {
    let count = initial;

    html! {
        <style src="/static/pages/lesson14.css" />
        <div class="counter-demo">
            <h3>"Counter: " {count.to_string()}</h3>
            <p>"This demonstrates component state"</p>
        </div>
    }
}

/// Button with variant props
#[derive(Clone)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
    Outline,
}

/// Button component with variants
#[derive(Clone)]
pub struct ButtonProps {
    pub text: String,
    pub variant: ButtonVariant,
    pub disabled: bool,
    pub onclick: Option<String>,
}

pub fn button(props: ButtonProps) -> impl azumi::Component {
    let variant_class = match props.variant {
        ButtonVariant::Primary => "btn-primary",
        ButtonVariant::Secondary => "btn-secondary", 
        ButtonVariant::Danger => "btn-danger",
        ButtonVariant::Outline => "btn-outline",
    };

    html! {
        <button 
            class={format!("btn {}", variant_class)}
            disabled={props.disabled}
            onclick={props.onclick}>
            {&props.text}
        </button>
    }
}

/// Modal component with children
#[derive(Clone)]
pub struct ModalProps {
    pub is_open: bool,
    pub title: String,
    pub children: impl azumi::Component,
    pub on_close: Option<String>,
}

pub fn modal(props: ModalProps) -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson14.css" />
        @if props.is_open {
            <div class="modal-overlay">
                <div class="modal">
                    <div class="modal-header">
                        <h3>{&props.title}</h3>
                        @if let Some(on_close) = &props.on_close {
                            <button class="modal-close" onclick={on_close}>"×"</button>
                        }
                    </div>
                    <div class="modal-content">
                        {props.children}
                    </div>
                </div>
            </div>
        }
    }
}

/// Alert component with different types
#[derive(Clone)]
pub enum AlertType {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Clone)]
pub struct AlertProps {
    pub alert_type: AlertType,
    pub title: String,
    pub message: String,
    pub dismissible: bool,
}

pub fn alert(props: AlertProps) -> impl azumi::Component {
    let type_class = match props.alert_type {
        AlertType::Info => "alert-info",
        AlertType::Success => "alert-success",
        AlertType::Warning => "alert-warning",
        AlertType::Error => "alert-error",
    };

    html! {
        <div class={format!("alert {}", type_class)}>
            <div class="alert-content">
                <strong>{&props.title}</strong>
                <p>{&props.message}</p>
            </div>
            @if props.dismissible {
                <button class="alert-dismiss">"×"</button>
            }
        </div>
    }
}

/// Tabs component with children
#[derive(Clone)]
pub struct TabProps {
    pub label: String,
    pub children: impl azumi::Component,
}

#[derive(Clone)]
pub struct TabsProps {
    pub active_tab: usize,
    pub tabs: Vec<TabProps>,
}

pub fn tabs(props: TabsProps) -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson14.css" />
        <div class="tabs-container">
            <div class="tabs-header">
                @for (index, tab) in props.tabs.iter().enumerate() {
                    <button class={format!("tab {}", if index == props.active_tab { "active" } else { "" })}>
                        {&tab.label}
                    </button>
                }
            </div>
            <div class="tabs-content">
                @if props.active_tab < props.tabs.len() {
                    @props.tabs[props.active_tab].children
                }
            </div>
        </div>
    }
}

/// Advanced component composition demo
pub fn advanced_components_demo() -> impl azumi::Component {
    let button_examples = vec![
        ButtonProps {
            text: "Primary Button".to_string(),
            variant: ButtonVariant::Primary,
            disabled: false,
            onclick: Some("alert('Primary clicked!')".to_string()),
        },
        ButtonProps {
            text: "Danger Button".to_string(),
            variant: ButtonVariant::Danger,
            disabled: false,
            onclick: Some("alert('Danger clicked!')".to_string()),
        },
        ButtonProps {
            text: "Disabled Button".to_string(),
            variant: ButtonVariant::Outline,
            disabled: true,
            onclick: None,
        },
    ];

    let alerts = vec![
        AlertProps {
            alert_type: AlertType::Success,
            title: "Success!".to_string(),
            message: "Your action was completed successfully.".to_string(),
            dismissible: true,
        },
        AlertProps {
            alert_type: AlertType::Warning,
            title: "Warning".to_string(),
            message: "Please check your input and try again.".to_string(),
            dismissible: false,
        },
        AlertProps {
            alert_type: AlertType::Error,
            title: "Error".to_string(),
            message: "Something went wrong. Please try again later.".to_string(),
            dismissible: true,
        },
    ];

    html! {
        <div class="advanced-demo">
            <h1>"Advanced Component Patterns"</h1>
            
            <section class="demo-section">
                <h2>"Counter with State"</h2>
                @counter(42)
            </section>

            <section class="demo-section">
                <h2>"Button Variants"</h2>
                <div class="button-grid">
                    @for button_props in &button_examples {
                        @button(button_props.clone())
                    }
                </div>
            </section>

            <section class="demo-section">
                <h2>"Alert Types"</h2>
                <div class="alert-container">
                    @for alert_props in &alerts {
                        @alert(alert_props.clone())
                    }
                </div>
            </section>

            <section class="demo-section">
                <h2>"Modal Component"</h2>
                @modal(ModalProps {
                    is_open: true,
                    title: "Advanced Modal".to_string(),
                    children: html! {
                        <div>
                            <p>"This is a modal with children content."</p>
                            <p>"Components can compose complex UIs!"</p>
                        </div>
                    },
                    on_close: Some("console.log('Modal closed')".to_string()),
                })
            </section>

            <section class="demo-section">
                <h2>"Tabs Component"</h2>
                @tabs(TabsProps {
                    active_tab: 1,
                    tabs: vec![
                        TabProps {
                            label: "First Tab".to_string(),
                            children: html! {
                                <div>
                                    <h4>"Content of First Tab"</h4>
                                    <p>"This is the content that appears in the first tab."</p>
                                </div>
                            },
                        },
                        TabProps {
                            label: "Second Tab".to_string(),
                            children: html! {
                                <div>
                                    <h4>"Content of Second Tab"</h4>
                                    <p>"This is the content that appears in the second tab."</p>
                                    <ul>
                                        <li>"Tab 2 Item 1"</li>
                                        <li>"Tab 2 Item 2"</li>
                                        <li>"Tab 2 Item 3"</li>
                                    </ul>
                                </div>
                            },
                        },
                        TabProps {
                            label: "Third Tab".to_string(),
                            children: html! {
                                <div>
                                    <h4>"Content of Third Tab"</h4>
                                    <p>"This is the content that appears in the third tab."</p>
                                </div>
                            },
                        },
                    ],
                })
            </section>
        </div>
    }
}

/// Handler for Axum
pub async fn lesson14_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&advanced_components_demo()))
}