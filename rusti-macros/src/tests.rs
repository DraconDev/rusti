#[cfg(test)]
mod tests {
    use crate::parser::parse_nodes;

    #[test]
    #[test]
    fn test_simple_div() {
        let input = "< div >";
        let result = crate::parser::parse_node(input);
        println!("Result: {:?}", result);
        assert!(result.is_ok());
    }

    fn test_reproduction() {
        let input = "< div > @ header (title) < main > < p > {body} </ p > </ main > @ footer (2023) </ div >";
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
}
