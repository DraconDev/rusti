#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_css_basic() {
        let css = ".button { color: red; }";
        let scoped = scope_css(css, "abc");
        assert!(scoped.contains(".button[data-abc]"));
        assert!(scoped.contains("color: red;"));
    }

    #[test]
    fn test_scope_css_multiple_selectors() {
        let css = ".button, .link { color: blue; }";
        let scoped = scope_css(css, "s123");
        assert!(scoped.contains(".button[data-s123], .link[data-s123]"));
    }

    #[test]
    fn test_scope_css_global_escape() {
        let css = ":global(.utility) { margin: 0; }";
        let scoped = scope_css(css, "s456");
        assert!(scoped.contains(".utility"));
        assert!(!scoped.contains("[data-s456]"));
    }

    #[test]
    fn test_generate_scope_id() {
        let id1 = generate_scope_id();
        let id2 = generate_scope_id();
        assert_ne!(id1, id2);
        assert!(id1.starts_with('s'));
    }
}
