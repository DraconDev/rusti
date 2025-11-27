pub use azumi_macros::{component, html};

pub trait Component {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

pub struct FnComponent<F>(F);

impl<F> Component for FnComponent<F>
where
    F: Fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result,
{
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.0)(f)
    }
}

impl<T: Component + ?Sized> Component for &T {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (**self).render(f)
    }
}

pub fn from_fn<F>(f: F) -> FnComponent<F>
where
    F: Fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result,
{
    FnComponent(f)
}

pub fn render_to_string<C: Component + ?Sized>(component: &C) -> String {
    struct DisplayWrapper<'a, C: Component + ?Sized>(&'a C);
    impl<'a, C: Component + ?Sized> std::fmt::Display for DisplayWrapper<'a, C> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.render(f)
        }
    }
    format!("{}", DisplayWrapper(component))
}

pub struct Escaped<T: std::fmt::Display>(pub T);

impl<T: std::fmt::Display> std::fmt::Display for Escaped<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.0.to_string();
        for c in s.chars() {
            match c {
                '<' => write!(f, "&lt;")?,
                '>' => write!(f, "&gt;")?,
                '&' => write!(f, "&amp;")?,
                '"' => write!(f, "&quot;")?,
                '\'' => write!(f, "&#x27;")?,
                _ => write!(f, "{}", c)?,
            }
        }
        Ok(())
    }
}

// Smart Interpolation Machinery
// Allows {} to handle both Components (render) and Display types (escape)

pub struct RenderWrapper<T>(pub T);

impl<T: Component> RenderWrapper<T> {
    // Priority 1: Component (Render directly)
    // This inherent method takes precedence over the trait implementation below
    pub fn render_azumi(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.render(f)
    }
}

pub trait FallbackRender {
    fn render_azumi(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

// Priority 2: Display (Escape HTML)
impl<T: std::fmt::Display> FallbackRender for RenderWrapper<T> {
    fn render_azumi(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Escaped(&self.0))
    }
}

pub fn js<T: std::fmt::Debug>(v: T) -> String {
    format!("{:?}", v)
}

/// Generate a unique scope ID for CSS scoping
pub fn generate_scope_id() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("s{:x}", id)
}

/// Transform CSS selectors to include scope attribute
/// All CSS is automatically scoped - no escape hatches!
pub fn scope_css(css: &str, scope_id: &str) -> String {
    let scope_attr = format!("[data-{}]", scope_id);
    let mut result = String::new();
    let mut in_rule = false;
    let mut selector_buffer = String::new();

    for ch in css.chars() {
        match ch {
            '{' if !in_rule => {
                // Found opening brace - scope the selector buffer
                let selectors: Vec<&str> = selector_buffer.split(',').collect();
                let scoped: Vec<String> = selectors
                    .iter()
                    .filter(|s| !s.trim().is_empty())
                    .map(|s| {
                        let trimmed = s.trim();
                        // Skip @-rules and comments
                        if trimmed.starts_with('@') || trimmed.starts_with("/*") {
                            trimmed.to_string()
                        } else {
                            format!("{}{}", trimmed, scope_attr)
                        }
                    })
                    .collect();

                result.push_str(&scoped.join(", "));
                result.push('{');
                selector_buffer.clear();
                in_rule = true;
            }
            '}' if in_rule => {
                result.push('}');
                in_rule = false;
            }
            _ => {
                if in_rule {
                    result.push(ch);
                } else {
                    selector_buffer.push(ch);
                }
            }
        }
    }

    // Handle any remaining selector buffer (malformed CSS)
    if !selector_buffer.trim().is_empty() {
        result.push_str(&selector_buffer);
    }

    result
}

#[cfg(test)]
mod tests;
