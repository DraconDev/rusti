#[cfg(test)]
mod tests {
    use crate::parser::{parse_element, AttributeValue, Node};

    #[test]
    fn test_static_attribute() {
        let input = r#"<div class="container"></div>"#;
        let result = parse_element(input);
        assert!(result.is_ok());
        let (_, node) = result.unwrap();
        match node {
            Node::Element { name, attrs, .. } => {
                assert_eq!(name, "div");
                assert_eq!(attrs.len(), 1);
                assert_eq!(attrs[0].0, "class");
                match &attrs[0].1 {
                    AttributeValue::Static(v) => assert_eq!(v, "container"),
                    _ => panic!("Expected static attribute"),
                }
            }
            _ => panic!("Expected element"),
        }
    }

    #[test]
    fn test_dynamic_attribute() {
        let input = r#"<div class={my_class}></div>"#;
        let result = parse_element(input);
        assert!(result.is_ok());
        let (_, node) = result.unwrap();
        match node {
            Node::Element { name, attrs, .. } => {
                assert_eq!(name, "div");
                assert_eq!(attrs.len(), 1);
                assert_eq!(attrs[0].0, "class");
                match &attrs[0].1 {
                    AttributeValue::Dynamic(v) => assert_eq!(v, "my_class"),
                    _ => panic!("Expected dynamic attribute"),
                }
            }
            _ => panic!("Expected element"),
        }
    }

    #[test]
    fn test_multiple_attributes() {
        let input = r#"<a href="/about" class="link"></a>"#;
        let result = parse_element(input);
        assert!(result.is_ok());
        let (_, node) = result.unwrap();
        match node {
            Node::Element { name, attrs, .. } => {
                assert_eq!(name, "a");
                assert_eq!(attrs.len(), 2);
                assert_eq!(attrs[0].0, "href");
                assert_eq!(attrs[1].0, "class");
            }
            _ => panic!("Expected element"),
        }
    }

    #[test]
    fn test_mixed_attributes() {
        let input = r#"<button class="btn" disabled={is_disabled}></button>"#;
        let result = parse_element(input);
        assert!(result.is_ok());
        let (_, node) = result.unwrap();
        match node {
            Node::Element { name, attrs, .. } => {
                assert_eq!(name, "button");
                assert_eq!(attrs.len(), 2);
                match &attrs[0].1 {
                    AttributeValue::Static(_) => {}
                    _ => panic!("Expected static"),
                }
                match &attrs[1].1 {
                    AttributeValue::Dynamic(_) => {}
                    _ => panic!("Expected dynamic"),
                }
            }
            _ => panic!("Expected element"),
        }
    }
}
