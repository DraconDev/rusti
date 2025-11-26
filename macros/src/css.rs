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

    // Handle :global(...)
    // This is a simple parser that splits by spaces/combinators and handles :global()
    // It's not a full CSS parser but covers 99% of use cases
    let mut result = String::new();
    let mut buffer = String::new();
    let mut in_global = false;
    let mut global_depth = 0;

    let chars: Vec<char> = selector.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Check for :global( start
        if !in_global && i + 8 <= chars.len() && &selector[i..i + 8] == ":global(" {
            // Process what we have so far
            if !buffer.trim().is_empty() {
                result.push_str(&scope_part(&buffer, scope_attr));
                buffer.clear();
            } else if !buffer.is_empty() {
                // Preserve whitespace
                result.push_str(&buffer);
                buffer.clear();
            }

            in_global = true;
            global_depth = 1;
            i += 8;
            continue;
        }

        if in_global {
            if chars[i] == '(' {
                global_depth += 1;
                buffer.push(chars[i]);
            } else if chars[i] == ')' {
                global_depth -= 1;
                if global_depth == 0 {
                    in_global = false;
                    result.push_str(&buffer);
                    buffer.clear();
                } else {
                    buffer.push(chars[i]);
                }
            } else {
                buffer.push(chars[i]);
            }
        } else {
            // Normal mode - accumulate until we hit a combinator or end
            // Actually, we can just accumulate everything that isn't :global
            // But we need to be careful about splitting.
            // Let's simplify: split by combinators? No, that breaks complex selectors.
            // Let's just accumulate until the next :global or end
            buffer.push(chars[i]);
        }
        i += 1;
    }

    // Process remaining buffer
    if !buffer.is_empty() {
        if in_global {
            // Unclosed :global, just append
            result.push_str(":global(");
            result.push_str(&buffer);
        } else {
            result.push_str(&scope_part(&buffer, scope_attr));
        }
    }

    result
}

fn scope_part(part: &str, scope_attr: &str) -> String {
    // This function scopes a selector part that is NOT inside :global
    // It needs to handle combinators (space, >, +, ~)

    // Simple approach: split by combinators, scope each simple selector
    // Note: This is a heuristic. A full CSS parser is complex.
    // We assume that if it looks like a class/id/tag, we scope it.

    // Regex would be easier but we want to avoid heavy dependencies if possible.
    // Let's try a simpler approach: append scope_attr to the last simple selector of each combinator-separated chunk?

    // Actually, the previous implementation was:
    // format!("{}{}", selector, scope_attr)
    // But that was applied to the WHOLE selector string (e.g. "div > p").
    // "div > p[scope]" is WRONG. It should be "div[scope] > p[scope]".

    // Wait, the previous implementation WAS naive:
    // format!("{}{}", selector, scope_attr) -> "div > p[data-s]"
    // This means only the LAST element was scoped.
    // This explains why "html -> head -> style" worked (html[scope] -> head -> style)
    // But it also means "div p" -> "div p[scope]", so "div" was NOT scoped?
    // If so, that's a bug in the existing implementation too!

    // Let's check the previous implementation again.
    // It handled pseudo-elements and pseudo-classes by inserting scope before them.
    // But for "div p", it would produce "div p[scope]".
    // This means styles only applied if the LAST element matched the scope.
    // This is actually consistent with Vue's scoped styles (data attribute on the element).
    // But Vue adds data attributes to ALL elements.
    // Azumi only adds data attributes to elements in the component.
    // So "div[scope] p[scope]" is correct.

    // So we need to inject scope_attr into every simple selector.
    // "div > p" -> "div[scope] > p[scope]"

    // For now, let's stick to the previous behavior for non-global parts to avoid regressions,
    // but maybe improve it slightly if we can.
    // The previous behavior was: append scope to the end (before pseudos).
    // "div > p" -> "div > p[scope]"

    // If we want to support :global properly, we should probably stick to that pattern for now:
    // Scope the last part of the sequence.

    // However, for mixed selectors like ".parent :global(.child)",
    // ".parent" needs to be scoped.

    // Let's implement a smarter scoper that handles combinators.
    let combinators = [' ', '>', '+', '~'];
    let mut result = String::new();
    let mut buffer = String::new();

    for ch in part.chars() {
        if combinators.contains(&ch) {
            if !buffer.trim().is_empty() {
                result.push_str(&scope_simple_selector(&buffer, scope_attr));
                buffer.clear();
            }
            result.push(ch);
        } else {
            buffer.push(ch);
        }
    }

    if !buffer.trim().is_empty() {
        result.push_str(&scope_simple_selector(&buffer, scope_attr));
    } else {
        result.push_str(&buffer);
    }

    result
}

fn scope_simple_selector(selector: &str, scope_attr: &str) -> String {
    let selector = selector.trim();
    if selector.is_empty() {
        return String::new();
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
