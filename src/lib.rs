pub use rusti_macros::{component, rusti};

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
    let mut result = String::new();
    let scope_attr = format!("[data-{}]", scope_id);

    for line in css.lines() {
        let trimmed = line.trim();

        // Skip empty lines, comments, at-rules
        if trimmed.is_empty() || trimmed.starts_with("/*") || trimmed.starts_with("@") {
            result.push_str(line);
            result.push('\n');
            continue;
        }

        // Add scope attribute to selectors
        if let Some(brace_pos) = line.find('{') {
            let selector_part = &line[..brace_pos];
            let rest = &line[brace_pos..];

            // Split selectors by comma
            let selectors: Vec<&str> = selector_part.split(',').collect();
            let scoped_selectors: Vec<String> = selectors
                .iter()
                .map(|s| format!("{}{}", s.trim(), scope_attr))
                .collect();

            result.push_str(&scoped_selectors.join(", "));
            result.push_str(rest);
            result.push('\n');
        } else {
            // No brace, just copy line as-is (probably inside a rule)
            result.push_str(line);
            result.push('\n');
        }
    }

    result
}

#[cfg(test)]
mod tests;
