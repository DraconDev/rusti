/// CSS Class Validator - Revolutionary compile-time CSS type checking
///
/// This system validates that:
/// 1. All CSS class names used in HTML exist in the CSS files
/// 2. All CSS class names defined in CSS files are used somewhere
/// 3. No dead CSS or undefined classes

use std::collections::{HashSet, HashMap};
use std::path::Path;

/// Re-export token_parser types for use in this module
use crate::token_parser::{Node, Element, AttributeValue, Block, IfBlock, ForBlock, MatchBlock, CallBlock, Fragment};

/// CSS rule with its selector and properties
#[derive(Debug, Clone)]
struct CssRule {
    selector: String,
    properties: String,
    file_path: String,
    line_number: usize,
}

/// Extract class names from CSS selector
fn extract_classes_from_selector(selector: &str) -> HashSet<String> {
    let mut classes = HashSet::new();
    
    // Handle complex selectors separated by commas
    for simple_selector in selector.split(',') {
        let trimmed = simple_selector.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        // Extract classes (.classname)
        let mut chars = trimmed.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '.' {
                let mut class_name = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_whitespace() || next_ch == ',' || next_ch == '{' || next_ch == ';' {
                        break;
                    }
                    class_name.push(chars.next().unwrap());
                }
                if !class_name.is_empty() {
                    classes.insert(class_name);
                }
            }
        }
    }
    
    classes
}

/// Parse CSS content and extract all defined class names
pub fn parse_css_classes(css_content: &str, file_path: &str) -> HashSet<String> {
    let mut defined_classes = HashSet::new();
    let mut lines = css_content.lines().enumerate();
    
    while let Some((line_num, line)) = lines.next() {
        let trimmed = line.trim();
        
        // Skip @-rules, comments, and empty lines
        if trimmed.starts_with('@') || trimmed.starts_with("/*") || trimmed.is_empty() {
            if trimmed.starts_with("/*") {
                // Skip multi-line comments
                while let Some((_, comment_line)) = lines.next() {
                    if comment_line.trim().ends_with("*/") {
                        break;
                    }
                }
            }
            continue;
        }
        
        // Find selector (everything before {)
        if let Some(brace_pos) = trimmed.find('{') {
            let selector = &trimmed[..brace_pos].trim();
            
            // Extract classes from this selector
            let classes = extract_classes_from_selector(selector);
            defined_classes.extend(classes);
            
            // Skip to closing brace
            let mut brace_count = 1;
            while let Some((_, next_line)) = lines.next() {
                for ch in next_line.chars() {
                    if ch == '{' {
                        brace_count += 1;
                    } else if ch == '}' {
                        brace_count -= 1;
                        if brace_count == 0 {
                            break;
                        }
                    }
                }
                if brace_count == 0 {
                    break;
                }
            }
        }
    }
    
    defined_classes
}

/// Extract all class names used in HTML attributes
pub fn extract_html_classes(nodes: &[Node]) -> HashSet<String> {
    let mut used_classes = HashSet::new();
    extract_html_classes_recursive(nodes, &mut used_classes);
    used_classes
}

fn extract_html_classes_recursive(nodes: &[Node], used_classes: &mut HashSet<String>) {
    for node in nodes {
        match node {
            Node::Element(elem) => {
                // Check class attribute
                if let Some(class_attr) = elem.attrs.iter().find(|attr| attr.name == "class") {
                    match &class_attr.value {
                        AttributeValue::Static(class_string) => {
                            // Split by whitespace and add each class
                            for class in class_string.split_whitespace() {
                                if !class.is_empty() {
                                    used_classes.insert(class.to_string());
                                }
                            }
                        }
                        AttributeValue::Dynamic(expr) => {
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
    used_classes: &HashSet<String>,
    defined_classes: &HashSet<String>,
) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    
    // Check for undefined classes (used but not defined)
    let undefined_classes: Vec<_> = used_classes
        .difference(defined_classes)
        .cloned()
        .collect();
    
    if !undefined_classes.is_empty() {
        for class_name in undefined_classes {
            errors.push(ValidationError::UndefinedClass {
                class_name,
            });
        }
    }
    
    // Check for dead CSS (defined but not used)
    let dead_classes: Vec<_> = defined_classes
        .difference(used_classes)
        .cloned()
        .collect();
    
    if !dead_classes.is_empty() {
        for class_name in dead_classes {
            errors.push(ValidationError::DeadCss {
                class_name,
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
    },
    DeadCss {
        class_name: String,
    },
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::UndefinedClass { class_name } => {
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
pub fn validate_component_css(
    nodes: &[Node],
) -> Result<(), Box<dyn std::error::Error>> {
    // First, collect all CSS files referenced in the component
    let mut css_files = Vec::new();
    collect_css_files(nodes, &mut css_files);
    
    if css_files.is_empty() {
        return Ok(()); // No CSS files to validate
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
                return Err(format!("Failed to read CSS file '{}': {}", css_file, e).into());
            }
        }
    }
    
    // Extract all classes used in HTML
    let used_classes = extract_html_classes(nodes);
    
    // Validate
    match validate_css_classes(&used_classes, &all_defined_classes) {
        Ok(()) => {
            eprintln!("✅ CSS validation passed: {} CSS files checked", css_files.len());
            Ok(())
        }
        Err(errors) => {
            // Convert all validation errors into a single error message
            let mut error_message = format!(
                "❌ CSS Validation Failed - Found {} error(s):\n\n",
                errors.len()
            );
            
            for (i, error) in errors.iter().enumerate() {
                error_message.push_str(&format!("{}. {}\n\n", i + 1, error));
            }
            
            error_message.push_str(&format!(
                "📋 Summary:\n• {} CSS files analyzed\n• {} classes defined in CSS\n• {} classes used in HTML\n\n\
                 🔧 Fix these issues by either:\n\
                 1. Adding missing CSS class definitions\n\
                 2. Removing unused CSS classes\n\
                 3. Fixing typos in class names",
                css_files.len(),
                all_defined_classes.len(),
                used_classes.len()
            ));
            
            Err(error_message.into())
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
                            // Enhanced path resolution for demo projects
                            let css_file_path = resolve_css_file_path(path);
                            css_files.push(css_file_path);
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
    fn test_extract_classes_from_simple_selector() {
        let selector = ".btn.primary";
        let classes = extract_classes_from_selector(selector);
        assert!(classes.contains("btn"));
        assert!(classes.contains("primary"));
    }

    #[test]
    fn test_extract_classes_from_multiple_selectors() {
        let selector = ".header, .footer, .sidebar";
        let classes = extract_classes_from_selector(selector);
        assert!(classes.contains("header"));
        assert!(classes.contains("footer"));
        assert!(classes.contains("sidebar"));
    }

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
        let used = HashSet::from(["btn".to_string(), "undefined-class".to_string()]);
        let defined = HashSet::from(["btn".to_string(), "card".to_string()]);
        
        let result = validate_css_classes(&used, &defined);
        assert!(result.is_err());
        
        if let Err(errors) = result {
            assert_eq!(errors.len(), 1);
            match &errors[0] {
                ValidationError::UndefinedClass { class_name } => {
                    assert_eq!(class_name, "undefined-class");
                }
                _ => panic!("Expected UndefinedClass error"),
            }
        }
    }

    #[test]
    fn test_validate_dead_css() {
        let used = HashSet::from(["btn".to_string()]);
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