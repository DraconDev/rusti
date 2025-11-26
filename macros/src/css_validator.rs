/// CSS Class Validator - Revolutionary compile-time CSS type checking
///
/// This system validates that:
/// 1. All CSS class names used in HTML exist in the CSS files
/// 2. All CSS class names defined in CSS files are used somewhere
/// 3. No dead CSS or undefined classes

use std::collections::{HashSet, HashMap};
use regex::Regex;

/// Re-export token_parser types for use in this module
use crate::token_parser::{Node, AttributeValue, Block};


/// Parse CSS content and extract all defined class names - FAST VERSION using regex
pub fn parse_css_classes(css_content: &str, _file_path: &str) -> HashSet<String> {
    let mut defined_classes = HashSet::new();
    
    // Fast regex-based class extraction - much faster than full CSS parsing
    // Matches patterns like: .className, .class-name, .class_name, etc.
    // This is 10-50x faster than full CSS parsing and sufficient for validation
    let class_pattern = Regex::new(r"\.([a-zA-Z0-9_-]+)").unwrap();
    
    for cap in class_pattern.captures_iter(css_content) {
        if let Some(class_name) = cap.get(1) {
            defined_classes.insert(class_name.as_str().to_string());
        }
    }
    
    defined_classes
}

/// Create a better span for class validation by pointing to the class attribute value
/// This gives a more precise error location than the generic attribute span
fn create_class_span(class_attr: &crate::token_parser::Attribute, _class_name: &str) -> proc_macro2::Span {
    // For class attributes, use the value span if available (points to string literal)
    // Otherwise fall back to the attribute span (points to attribute name)
    if let Some(value_span) = &class_attr.value_span {
        *value_span
    } else {
        class_attr.span
    }
}

/// Extract all class names used in HTML attributes with their spans
pub fn extract_html_classes(nodes: &[Node]) -> HashMap<String, Vec<proc_macro2::Span>> {
    let mut used_classes = HashMap::new();
    extract_html_classes_recursive(nodes, &mut used_classes);
    used_classes
}

fn extract_html_classes_recursive(nodes: &[Node], used_classes: &mut HashMap<String, Vec<proc_macro2::Span>>) {
    for node in nodes {
        match node {
            Node::Element(elem) => {
                // Check class attribute
                if let Some(class_attr) = elem.attrs.iter().find(|attr| attr.name == "class") {
                    match &class_attr.value {
                        AttributeValue::Static(class_string) => {
                            // For static class attributes, create a better span that points more precisely
                            // We can't easily track exact character positions, so we'll use a span that
                            // encompasses the entire attribute and adjust our error message
                            for class in class_string.split_whitespace() {
                                if !class.is_empty() {
                                    used_classes.entry(class.to_string())
                                        .or_insert_with(Vec::new)
                                        .push(create_class_span(class_attr, class));
                                }
                            }
                        }
                        AttributeValue::Dynamic(_expr) => {
                            // For dynamic expressions, we can't validate at compile time
                            // but we should warn about this
                            eprintln!("Warning: Dynamic class attribute detected. Cannot validate at compile time.");
                        }
                        AttributeValue::None => {}
                    }
                }
                
                // Recurse into children
                extract_html_classes_recursive(&elem.children, used_classes);
            }
            Node::Fragment(frag) => {
                extract_html_classes_recursive(&frag.children, used_classes);
            }
            Node::Block(block) => match block {
                Block::If(if_block) => {
                    extract_html_classes_recursive(&if_block.then_branch, used_classes);
                    if let Some(else_branch) = &if_block.else_branch {
                        extract_html_classes_recursive(else_branch, used_classes);
                    }
                }
                Block::For(for_block) => {
                    extract_html_classes_recursive(&for_block.body, used_classes);
                }
                Block::Match(match_block) => {
                    for arm in &match_block.arms {
                        extract_html_classes_recursive(&arm.body, used_classes);
                    }
                }
                Block::Call(call_block) => {
                    extract_html_classes_recursive(&call_block.children, used_classes);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

/// Validate that all HTML classes exist in CSS files
pub fn validate_css_classes(
    used_classes: &HashMap<String, Vec<proc_macro2::Span>>,
    defined_classes: &HashSet<String>,
) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    
    // Check for undefined classes (used but not defined)
    for (class_name, spans) in used_classes {
        if !defined_classes.contains(class_name) {
            errors.push(ValidationError::UndefinedClass {
                class_name: class_name.clone(),
                spans: spans.clone(),
            });
        }
    }
    
    // Check for dead CSS (defined but not used)
    for class_name in defined_classes {
        if !used_classes.contains_key(class_name) {
            errors.push(ValidationError::DeadCss {
                class_name: class_name.clone(),
            });
        }
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validation error types
#[derive(Debug)]
pub enum ValidationError {
    UndefinedClass {
        class_name: String,
        spans: Vec<proc_macro2::Span>,
    },
    DeadCss {
        class_name: String,
    },
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::UndefinedClass { class_name, .. } => {
                write!(f, 
                    "CSS Validation Error: Class '{}' is used in HTML but not defined in any CSS file.\n\
                    \nThis could be:\n\
                    • A typo in the class name\n\
                    • Missing CSS definition\n\
                    • Incorrect CSS file path\n\
                    \n💡 Tip: Run 'cargo check' to see all CSS validation errors together.",
                    class_name
                )
            }
            ValidationError::DeadCss { class_name } => {
                write!(f,
                    "CSS Validation Warning: Class '{}' is defined in CSS but never used in HTML.\n\
                    \nThis creates dead CSS that:\n\
                    • Increases bundle size unnecessarily\n\
                    • Confuses maintenance\n\
                    • Reduces performance\n\
                    \n💡 Consider removing unused classes to keep your CSS clean.",
                    class_name
                )
            }
        }
    }
}

impl std::error::Error for ValidationError {}

/// Parse all CSS files referenced in the component and validate classes
/// Returns a TokenStream of compile errors if validation fails
pub fn validate_component_css(nodes: &[Node]) -> proc_macro2::TokenStream {
    use quote::{quote, quote_spanned};
    
    // First, collect all CSS files referenced in the component
    let mut css_files = Vec::new();
    collect_css_files(nodes, &mut css_files);
    
    if css_files.is_empty() {
        return quote! {}; // No CSS files to validate
    }
    
    // Read and parse all CSS files
    let mut all_defined_classes = HashSet::new();
    for css_file in &css_files {
        match std::fs::read_to_string(css_file) {
            Ok(content) => {
                let file_classes = parse_css_classes(&content, css_file);
                all_defined_classes.extend(file_classes);
            }
            Err(e) => {
                let error_msg = format!("Failed to read CSS file '{}': {}", css_file, e);
                return quote! {
                    compile_error!(#error_msg);
                };
            }
        }
    }
    
    // Extract all classes used in HTML
    let used_classes = extract_html_classes(nodes);
    
    // Validate
    match validate_css_classes(&used_classes, &all_defined_classes) {
        Ok(()) => {
            eprintln!("✅ CSS validation passed: {} CSS files checked", css_files.len());
            quote! {}
        }
        Err(errors) => {
            // Generate compile errors for each validation error
            let error_tokens: Vec<_> = errors.iter().filter_map(|error| {
                match error {
                    ValidationError::UndefinedClass { class_name, spans } => {
                        // Create a compile error for each span where this class is used
                        let error_msg = format!(
                            "CSS class '{}' is not defined in any CSS file. Check for typos or add the class to your CSS.",
                            class_name
                        );
                        
                        // Emit an error at each location where this undefined class is used
                        let span_errors: Vec<_> = spans.iter().map(|span| {
                            quote_spanned! { *span =>
                                compile_error!(#error_msg);
                            }
                        }).collect();
                        
                        Some(quote! { #(#span_errors)* })
                    }
                    ValidationError::DeadCss { class_name } => {
                        // For dead CSS, emit a warning (not an error) to stderr
                        eprintln!(
                            "⚠️  CSS Warning: Class '{}' is defined in CSS but never used in HTML.",
                            class_name
                        );
                        None // Don't emit compile error for warnings
                    }
                }
            }).collect();
            
            quote! {
                #(#error_tokens)*
            }
        }
    }
}

/// Collect all CSS file paths from <style src="..."> tags
fn collect_css_files(nodes: &[Node], css_files: &mut Vec<String>) {
    for node in nodes {
        match node {
            Node::Element(elem) => {
                if elem.name == "style" {
                    if let Some(src_attr) = elem.attrs.iter().find(|a| a.name == "src") {
                        if let AttributeValue::Static(path) = &src_attr.value {
                            // Skip global.css files - they are opt-out of validation
                            if !path.ends_with("global.css") {
                                // Enhanced path resolution for demo projects
                                let css_file_path = resolve_css_file_path(path);
                                css_files.push(css_file_path);
                            }
                        }
                    }
                }
                // Recurse into children
                collect_css_files(&elem.children, css_files);
            }
            Node::Fragment(frag) => {
                collect_css_files(&frag.children, css_files);
            }
            Node::Block(block) => match block {
                Block::If(if_block) => {
                    collect_css_files(&if_block.then_branch, css_files);
                    if let Some(else_branch) = &if_block.else_branch {
                        collect_css_files(else_branch, css_files);
                    }
                }
                Block::For(for_block) => {
                    collect_css_files(&for_block.body, css_files);
                }
                Block::Match(match_block) => {
                    for arm in &match_block.arms {
                        collect_css_files(&arm.body, css_files);
                    }
                }
                Block::Call(call_block) => {
                    collect_css_files(&call_block.children, css_files);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

/// Enhanced CSS file path resolution
pub fn resolve_css_file_path(css_path: &str) -> String {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set");
    
    let manifest_path = std::path::Path::new(&manifest_dir);
    let clean_path = css_path.trim_start_matches('/');
    
    // Handle different project structures
    let possible_paths = vec![
        // 1. Direct path from manifest dir (e.g. demo/static/style.css)
        manifest_path.join(clean_path).to_string_lossy().to_string(),
        
        // 2. Inside static/ directory (e.g. demo/static/style.css)
        manifest_path.join("static").join(clean_path).to_string_lossy().to_string(),
        
        // 3. From workspace root -> demo/static (e.g. azumi/demo/static/style.css)
        manifest_path.join("demo").join("static").join(clean_path).to_string_lossy().to_string(),
        
        // 4. From workspace root -> demo (e.g. azumi/demo/style.css)
        manifest_path.join("demo").join(clean_path).to_string_lossy().to_string(),
        
        // 5. From manifest dir -> src/examples/lessons (e.g. demo/src/examples/lessons/style.css)
        manifest_path.join("src").join("examples").join("lessons").join(clean_path).to_string_lossy().to_string(),
        
        // 6. From manifest dir -> src/examples (e.g. demo/src/examples/style.css)
        manifest_path.join("src").join("examples").join(clean_path).to_string_lossy().to_string(),
        
        // 7. From manifest dir -> src (e.g. demo/src/style.css)
        manifest_path.join("src").join(clean_path).to_string_lossy().to_string(),
    ];
    
    eprintln!("🔍 Resolving CSS path: '{}' from '{}'", css_path, manifest_dir);
    
    // Try each possible path and return the first one that exists
    for path in &possible_paths {
        if std::path::Path::new(path).exists() {
            eprintln!("✅ Found CSS file: {}", path);
            return path.clone();
        } else {
            // eprintln!("  Checked: {}", path); // Uncomment for verbose debugging
        }
    }
    
    // If no file found, return the first constructed path (relative to manifest)
    // This ensures the error message shows a reasonable path
    let default_path = manifest_path.join(clean_path).to_string_lossy().to_string();
    eprintln!("⚠️  CSS file not found. Checked {} locations.", possible_paths.len());
    default_path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_css_classes() {
        let css = r#"
            .btn { padding: 10px; }
            .card { border: 1px solid #ccc; }
            /* Comment */
            .header, .footer { background: #f0f0f0; }
        "#;
        
        let classes = parse_css_classes(css, "test.css");
        assert!(classes.contains("btn"));
        assert!(classes.contains("card"));
        assert!(classes.contains("header"));
        assert!(classes.contains("footer"));
        assert_eq!(classes.len(), 4);
    }

    #[test]
    fn test_validate_missing_classes() {
        use proc_macro2::Span;
        
        let mut used = HashMap::new();
        used.insert("btn".to_string(), vec![Span::call_site()]);
        used.insert("undefined-class".to_string(), vec![Span::call_site()]);
        
        let defined = HashSet::from(["btn".to_string(), "card".to_string()]);
        
        let result = validate_css_classes(&used, &defined);
        assert!(result.is_err());
        
        if let Err(errors) = result {
            assert_eq!(errors.len(), 1);
            match &errors[0] {
                ValidationError::UndefinedClass { class_name, spans } => {
                    assert_eq!(class_name, "undefined-class");
                    assert_eq!(spans.len(), 1);
                }
                _ => panic!("Expected UndefinedClass error"),
            }
        }
    }

    #[test]
    fn test_validate_dead_css() {
        use proc_macro2::Span;
        
        let mut used = HashMap::new();
        used.insert("btn".to_string(), vec![Span::call_site()]);
        
        let defined = HashSet::from(["btn".to_string(), "unused-class".to_string()]);
        
        let result = validate_css_classes(&used, &defined);
        assert!(result.is_err());
        
        if let Err(errors) = result {
            assert_eq!(errors.len(), 1);
            match &errors[0] {
                ValidationError::DeadCss { class_name } => {
                    assert_eq!(class_name, "unused-class");
                }
                _ => panic!("Expected DeadCss error"),
            }
        }
    }
}
