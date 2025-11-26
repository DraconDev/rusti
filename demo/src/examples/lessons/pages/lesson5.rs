//! Lesson 5: css_integration.rs
//!
//! Adding styles to templates
use azumi::html;

/// Styled button example
/// Loads external CSS and uses validated classes
pub fn styled_button() -> impl azumi::Component {
    html! {
        <style src="/static/pages/button.css" />
        <button class="btn-primary">"Click Me"</button>
    }
}

/// Button with hover state demonstration
pub fn hover_button() -> impl azumi::Component {
    html! {
        <style src="/static/pages/button.css" />
        <button class="btn-hover">"Hover Me"</button>
    }
}

/// Multiple buttons showcase
pub fn buttons_showcase() -> impl azumi::Component {
    html! {
        <style src="/static/pages/button.css" />
        <div>
            <button class="btn-primary">"Primary"</button>
            <button class="btn-secondary">"Secondary"</button>
        </div>
    }
}