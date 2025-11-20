#[cfg(test)]
mod tests {
    use crate::parser::parse_nodes;

    #[test]
    fn test_spaced_attributes_and_script() {
        // Test 1: Attribute with hyphen and spaces (hx-post -> hx - post)
        let input_attr = r#"<div hx - post = "val"></div>"#;
        let (remaining, _) = parse_nodes(input_attr).unwrap();
        assert!(
            remaining.trim().is_empty(),
            "Attribute failed. Remaining: '{}'",
            remaining
        );

        // Test 2: Script tag with spaced closing tag
        let input_script = r#"<script> console.log("hi"); < / script >"#;
        let (remaining, _) = parse_nodes(input_script).unwrap();
        assert!(
            remaining.trim().is_empty(),
            "Script failed. Remaining: '{}'",
            remaining
        );
    }
}
