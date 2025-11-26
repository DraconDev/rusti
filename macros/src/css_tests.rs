#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_css_basic() {
        let css = ".container { color: red; }";
        let scoped = scope_css(css, "123");
        assert_eq!(scoped, ".container[data-123]{ color: red; }");
    }

    #[test]
    fn test_scope_css_nested() {
        let css = ".parent .child { color: blue; }";
        let scoped = scope_css(css, "123");
        // Note: The new implementation scopes ALL parts
        assert_eq!(scoped, ".parent[data-123] .child[data-123]{ color: blue; }");
    }

    #[test]
    fn test_scope_css_global_simple() {
        let css = ":global(.container) { color: red; }";
        let scoped = scope_css(css, "123");
        assert_eq!(scoped, ".container{ color: red; }");
    }

    #[test]
    fn test_scope_css_global_nested() {
        let css = ".parent :global(.child) { color: blue; }";
        let scoped = scope_css(css, "123");
        assert_eq!(scoped, ".parent[data-123] .child{ color: blue; }");
    }

    #[test]
    fn test_scope_css_global_complex() {
        let css = ":global(.lib-class) .local-class { color: green; }";
        let scoped = scope_css(css, "123");
        assert_eq!(scoped, ".lib-class .local-class[data-123]{ color: green; }");
    }

    #[test]
    fn test_scope_css_multiple_selectors() {
        let css = ".a, .b { color: red; }";
        let scoped = scope_css(css, "123");
        assert_eq!(scoped, ".a[data-123], .b[data-123]{ color: red; }");
    }

    #[test]
    fn test_scope_css_combinators() {
        let css = "div > p + span { color: red; }";
        let scoped = scope_css(css, "123");
        assert_eq!(
            scoped,
            "div[data-123]>p[data-123]+span[data-123]{ color: red; }"
        );
    }
}
