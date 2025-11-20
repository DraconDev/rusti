#[cfg(test)]
mod tests {
    use crate::parser::parse_nodes;

    #[test]
    fn test_spaced_input() {
        let input = "< head > < title > { title } </ title > </ head >";
        let (remaining, _nodes) = parse_nodes(input).unwrap();
        assert!(remaining.trim().is_empty(), "Remaining: '{}'", remaining);
    }
}
