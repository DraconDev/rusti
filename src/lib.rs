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
