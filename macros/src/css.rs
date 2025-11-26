/// Transform CSS selectors to include scope attribute
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
                    .map(|s| scope_selector(s.trim(), &scope_attr))
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

fn scope_selector(selector: &str, scope_attr: &str) -> String {
    // Skip @-rules and comments
    if selector.starts_with('@') || selector.starts_with("/*") {
        return selector.to_string();
    }

    // Handle pseudo-elements (::before, ::after, etc.)
    if let Some(pseudo_pos) = selector.find("::") {
        let base = &selector[..pseudo_pos];
        let pseudo = &selector[pseudo_pos..];
        return format!("{}{}{}", base, scope_attr, pseudo);
    }

    // Handle pseudo-classes (:hover, :focus, etc.)
    if let Some(pseudo_pos) = selector.find(':') {
        let base = &selector[..pseudo_pos];
        let pseudo = &selector[pseudo_pos..];
        return format!("{}{}{}", base, scope_attr, pseudo);
    }

    // Default: append scope attribute
    format!("{}{}", selector, scope_attr)
}
