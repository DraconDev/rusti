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

    let mut result = String::new();
    let mut buffer = String::new();
    let mut in_global = false;
    let mut global_depth = 0;

    let chars: Vec<char> = selector.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Check for :global(
        if !in_global && i + 8 <= chars.len() {
            let slice: String = chars[i..i + 8].iter().collect();
            if slice == ":global(" {
                // Process accumulated buffer
                if !buffer.trim().is_empty() {
                    result.push_str(&scope_part(&buffer, scope_attr));
                    buffer.clear();
                } else if !buffer.is_empty() {
                    result.push_str(&buffer);
                    buffer.clear();
                }

                in_global = true;
                global_depth = 1;
                i += 8;
                continue;
            }
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
            buffer.push(chars[i]);
        }
        i += 1;
    }

    if !buffer.is_empty() {
        if in_global {
            result.push_str(":global(");
            result.push_str(&buffer);
        } else {
            result.push_str(&scope_part(&buffer, scope_attr));
        }
    }

    result
}

fn scope_part(part: &str, scope_attr: &str) -> String {
    let mut result = String::new();
    let mut buffer = String::new();

    for ch in part.chars() {
        match ch {
            ' ' | '>' | '+' | '~' => {
                if !buffer.trim().is_empty() {
                    result.push_str(&scope_simple_selector(&buffer, scope_attr));
                    buffer.clear();
                } else if !buffer.is_empty() {
                    // Preserve whitespace if buffer was just whitespace
                    result.push_str(&buffer);
                    buffer.clear();
                }
                result.push(ch);
            }
            _ => buffer.push(ch),
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
    let trimmed = selector.trim();
    if trimmed.is_empty() {
        return selector.to_string();
    }

    // If the selector has leading/trailing whitespace, we need to preserve it
    // but insert the scope attribute at the right place.
    // Actually, scope_part handles the whitespace between combinators.
    // buffer in scope_part might contain leading whitespace if it's the start of the string?
    // No, scope_part splits by combinators.
    // If part is " div", first char is space.
    // buffer is empty. result pushes space.
    // Next char 'd'. buffer gets 'd'.

    // So buffer passed to scope_simple_selector should be trimmed?
    // Let's just trim it here and return trimmed result.
    // scope_part handles the separators.

    // Handle pseudo-elements
    if let Some(pseudo_pos) = trimmed.find("::") {
        let base = &trimmed[..pseudo_pos];
        let pseudo = &trimmed[pseudo_pos..];
        return format!("{}{}{}", base, scope_attr, pseudo);
    }

    // Handle pseudo-classes
    if let Some(pseudo_pos) = trimmed.find(':') {
        let base = &trimmed[..pseudo_pos];
        let pseudo = &trimmed[pseudo_pos..];
        return format!("{}{}{}", base, scope_attr, pseudo);
    }

    format!("{}{}", trimmed, scope_attr)
}
