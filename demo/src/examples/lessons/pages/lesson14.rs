//! Lesson 14: Advanced Component Patterns
//!
//! Demonstrating various component design patterns and best practices

use azumi::html;

// ============================================================================
// SECTION 1: Button Component with Variants
// ============================================================================

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Success,
    Danger,
    Outline,
}

#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

#[derive(Clone)]
pub struct ButtonProps {
    pub text: String,
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub disabled: bool,
}

/// Reusable button component with multiple variants and sizes
pub fn button(props: &ButtonProps) -> impl azumi::Component + '_ {
    let variant_class = match props.variant {
        ButtonVariant::Primary => "btn-primary",
        ButtonVariant::Secondary => "btn-secondary",
        ButtonVariant::Success => "btn-success",
        ButtonVariant::Danger => "btn-danger",
        ButtonVariant::Outline => "btn-outline",
    };

    let size_class = match props.size {
        ButtonSize::Small => "btn-sm",
        ButtonSize::Medium => "btn-md",
        ButtonSize::Large => "btn-lg",
    };

    html! {
        <style src="/static/pages/lesson14.css" />
        <button
            class={format!("btn {} {}", variant_class, size_class)}
            disabled={props.disabled}
        >
            {&props.text}
        </button>
    }
}

// ============================================================================
// SECTION 2: Alert Component with Types
// ============================================================================

#[derive(Clone, PartialEq)]
pub enum AlertType {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Clone)]
pub struct AlertProps {
    pub message: String,
    pub alert_type: AlertType,
    pub dismissible: bool,
}

/// Alert component for showing notifications
pub fn alert(props: &AlertProps) -> impl azumi::Component + '_ {
    let type_class = match props.alert_type {
        AlertType::Info => "alert-info",
        AlertType::Success => "alert-success",
        AlertType::Warning => "alert-warning",
        AlertType::Error => "alert-error",
    };

    let icon = match props.alert_type {
        AlertType::Info => "ℹ️",
        AlertType::Success => "✓",
        AlertType::Warning => "⚠️",
        AlertType::Error => "✕",
    };

    html! {
        <style src="/static/pages/lesson14.css" />
        <div class={format!("alert {}", type_class)}>
            <span class="alert-icon">{icon}</span>
            <span class="alert-message">{&props.message}</span>
            @if props.dismissible {
                <button class="alert-close">"×"</button>
            }
        </div>
    }
}

// ============================================================================
// SECTION 3: Badge Component
// ============================================================================

#[derive(Clone)]
pub struct BadgeProps {
    pub text: String,
    pub color: String,
}

pub fn badge(badge_props: &BadgeProps) -> impl azumi::Component + '_ {
    html! {
        <style src="/static/pages/lesson14.css" />
        <span class={format!("badge badge-{}", badge.color)}>
            {&badge.text}
        </span>
    }
}

// ============================================================================
// SECTION 4: List Component with Custom Items
// ============================================================================

#[derive(Clone)]
pub struct ListItem {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub status: String,
}

pub fn list_item(item: &ListItem) -> impl azumi::Component + '_ {
    html! {
        <style src="/static/pages/lesson14.css" />
        <div class="list-item">
            <div class="list-item-content">
                <h4 class="list-item-title">{&item.title}</h4>
                <p class="list-item-description">{&item.description}</p>
            </div>
            <div class="list-item-meta">
                @badge(badge_props=BadgeProps {
                    text: item.status.clone(),
                    color: match item.status.as_str() {
                        "Active" => "success",
                        "Pending" => "warning",
                        "Completed" => "info",
                        _ => "secondary",
                    }.to_string(),
                })
            </div>
        </div>
    }
}

pub fn item_list(items: &[ListItem]) -> impl azumi::Component + '_ {
    html! {
        <style src="/static/pages/lesson14.css" />
        <div class="item-list">
            @for item in items {
                {list_item(item)}
            }
        </div>
    }
}

// ============================================================================
// SECTION 5: Complex Example - Feature Showcase
// ============================================================================

#[azumi::component]
pub fn advanced_patterns_demo() -> impl azumi::Component {
    let sample_items = vec![
        ListItem {
            id: 1,
            title: "Implement Authentication".to_string(),
            description: "Add user login and registration functionality".to_string(),
            status: "Active".to_string(),
        },
        ListItem {
            id: 2,
            title: "Design Database Schema".to_string(),
            description: "Create optimized database structure".to_string(),
            status: "Completed".to_string(),
        },
        ListItem {
            id: 3,
            title: "API Documentation".to_string(),
            description: "Document all API endpoints".to_string(),
            status: "Pending".to_string(),
        },
        ListItem {
            id: 4,
            title: "Performance Optimization".to_string(),
            description: "Improve page load times".to_string(),
            status: "Active".to_string(),
        },
    ];

    html! {
            <style src="/static/pages/lesson14.css" />
            <div class="lesson-container">
                <header class="lesson-header">
                    <h1 class="lesson-title">"Lesson 14: Advanced Component Patterns"</h1>
                    <p class="lesson-subtitle">
                        "Master component design with variants, compositions, and best practices"
                    </p>
                </header>

                <section class="lesson-section">
                    <h2 class="section-title">"Introduction"</h2>
                    <p class="section-text">
                        "Advanced component patterns help you build flexible, reusable UI elements. "
                        "This lesson covers enums for variants, composition strategies, and real-world examples."
                    </p>
                </section>

                <section class="lesson-section">
                    <h2 class="section-title">"Pattern 1: Button Variants"</h2>
                    <p class="section-text">"Using enums to create flexible button components with different styles and sizes."</p>

                    <div class="demo-grid">
                        <div class="demo-column">
                            <h3 class="demo-heading">"Variants"</h3>
                            {button(&ButtonProps {
                                text: "Primary".to_string(),
                                variant: ButtonVariant::Primary,
                                size: ButtonSize::Medium,
                                disabled: false,
                            })}
                            {button(&ButtonProps {
                                text: "Secondary".to_string(),
                                variant: ButtonVariant::Secondary,
                                size: ButtonSize::Medium,
                                disabled: false,
                            })}
                            {button(&ButtonProps {
                                text: "Success".to_string(),
                                variant: ButtonVariant::Success,
                                size: ButtonSize::Medium,
                                disabled: false,
                            })}
                            {button(&ButtonProps {
                                text: "Danger".to_string(),
                                variant: ButtonVariant::Danger,
                                size: ButtonSize::Medium,
                                disabled: false,
                            })}
                            {button(&ButtonProps {
                                text: "Outline".to_string(),
                                variant: ButtonVariant::Outline,
                                size: ButtonSize::Medium,
    disabled: false,
                            })}
                        </div>

                        <div class="demo-column">
                            <h3 class="demo-heading">"Sizes"</h3>
                            {button(&ButtonProps {
                                text: "Small".to_string(),
                                variant: ButtonVariant::Primary,
                                size: ButtonSize::Small,
                                disabled: false,
                            })}
                            {button(&ButtonProps {
                                text: "Medium".to_string(),
                                variant: ButtonVariant::Primary,
                                size: ButtonSize::Medium,
                                disabled: false,
                            })}
                            {button(&ButtonProps {
                                text: "Large".to_string(),
                                variant: ButtonVariant::Primary,
                                size: ButtonSize::Large,
                                disabled: false,
                            })}
                        </div>

                        <div class="demo-column">
                            <h3 class="demo-heading">"States"</h3>
                            {button(&ButtonProps {
                                text: "Enabled".to_string(),
                                variant: ButtonVariant::Primary,
                                size: ButtonSize::Medium,
                                disabled: false,
                            })}
                            {button(&ButtonProps {
                                text: "Disabled".to_string(),
                                variant: ButtonVariant::Primary,
                                size: ButtonSize::Medium,
                                disabled: true,
                            })}
                        </div>
                    </div>
                </section>

                <section class="lesson-section">
                    <h2 class="section-title">"Pattern 2: Alert Components"</h2>
                    <p class="section-text">"Contextual alerts with different types and dismissible options."</p>

                    <div class="alert-demo">
                        {alert(&AlertProps {
                            message: "This is an informational message.".to_string(),
                            alert_type: AlertType::Info,
                            dismissible: true,
                        })}
                        {alert(&AlertProps {
                            message: "Operation completed successfully!".to_string(),
                            alert_type: AlertType::Success,
                            dismissible: true,
                        })}
                        {alert(&AlertProps {
                            message: "Warning: Please review your input.".to_string(),
                            alert_type: AlertType::Warning,
                            dismissible: false,
                        })}
                        {alert(&AlertProps {
                            message: "Error: Something went wrong.".to_string(),
                            alert_type: AlertType::Error,
                            dismissible: true,
                        })}
                    </div>
                </section>

                <section class="lesson-section">
                    <h2 class="section-title">"Pattern 3: List Components"</h2>
                    <p class="section-text">"Composable list items with badges and status indicators."</p>

                    {item_list(&sample_items)}
                </section>

                <section class="lesson-section">
                    <h2 class="section-title">"Key Takeaways"</h2>
                    <ul class="takeaways-list">
                        <li>"✓ Use enums to define component variants and types"</li>
                        <li>"✓ Create Props structs with clear, semantic fields"</li>
                        <li>"✓ Compose small components to build complex UIs"</li>
                        <li>"✓ Use pattern matching to generate variant-specific classes"</li>
                        <li>"✓ Keep components focused on a single responsibility"</li>
                        <li>"✓ Always add lifetime bounds (`+ '_`) when borrowing data"</li>
                        <li>"✓ Import CSS in every component for validation"</li>
                    </ul>
                </section>
            </div>
        }
}

/// Axum handler for Lesson 14
pub async fn lesson14_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&advanced_patterns_demo()))
}
