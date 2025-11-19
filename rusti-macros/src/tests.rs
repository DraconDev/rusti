#[cfg(test)]
mod tests {
    use crate::parser::parse_nodes;

    #[test]
    fn test_full_html_page() {
        // Actual format produced by TokenStream::to_string()
        // NOTE: No spaces around < and > !
        let input = "<html> <head> <title>{ title }</title> </head> <body> @header(title) <main>\n<p>{ body }</p> </main> @footer(2025) </body> </html>";
        let result = parse_nodes(input);
        match result {
            Ok((remaining, nodes)) => {
                println!("Remaining: '{}'", remaining);
                println!("Nodes: {:?}", nodes);
                assert!(remaining.trim().is_empty(), "Should consume all input");
            }
            Err(e) => {
                panic!("Parse failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_div_structure() {
        let input = "<div> @header(title) <main> <p>{ body }</p> </main> @footer(2023) </div>";
        let result = parse_nodes(input);
        match result {
            Ok((remaining, nodes)) => {
                println!("Remaining: '{}'", remaining);
                println!("Nodes: {:?}", nodes);
                assert!(remaining.trim().is_empty(), "Should consume all input");
            }
            Err(e) => {
                panic!("Parse failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_simple_element() {
        let input = "<div></div>";
        let result = parse_nodes(input);
        match result {
            Ok((remaining, nodes)) => {
                println!("Remaining: '{}'", remaining);
                println!("Nodes: {:?}", nodes);
                assert!(remaining.trim().is_empty(), "Should consume all input");
                assert_eq!(nodes.len(), 1);
            }
            Err(e) => {
                panic!("Parse failed: {:?}", e);
            }
        }
    }
}
