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
    #[test]
    fn test_components_repro() {
        let input = r#"<div class="bg-white rounded-xl shadow-md overflow-hidden border border-gray-100 hover:shadow-lg transition-shadow duration-300">
    <div class="p-6 border-b border-gray-100 bg-gray-50">
        <h3 class="text-xl font-bold text-gray-800">{ title }</h3>
    </div>
    <div class="p-6">
        <p class="text-gray-600 leading-relaxed">{ body }</p>
    </div>
    @if let Some(footer_text) = footer {
        <div class="p-4 bg-gray-50 border-t border-gray-100 text-sm text-gray-500 text-right">
            { footer_text }
        </div>
    }
</div>"#;
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
    fn test_todo_repro() {
        let input = r#"<div class={format!("{} {}", base_classes, status_classes)}>
    <span class={text_classes}>{ &item.text }</span>
    @if item.completed {
        <span class="px-3 py-1 text-xs font-bold text-green-700 bg-green-200 rounded-full">Done</span>
    } else {
        <span class="px-3 py-1 text-xs font-bold text-yellow-700 bg-yellow-200 rounded-full">Pending</span>
    }
</div>"#;
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
    fn test_attributes() {
        let input = r#"<div class="bg-white" id="main"></div>"#;
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
            #[test]
    fn test_parse_call() {
        use crate::parser::parse_node;
        let input = r#"@page_head("Title")"#;
        let result = parse_node(input);
        match result {
            Ok((remaining, node)) => {
                println!("Remaining: '{}'", remaining);
                println!("Node: {:?}", node);
                assert!(remaining.trim().is_empty(), "Should consume all input");
                if let crate::parser::Node::Call { name, args, .. } = node {
                    assert_eq!(name, "page_head");
                    assert_eq!(args, "\"Title\"");
                } else {
                    panic!("Expected Call node, got {:?}", node);
                }
            }
            Err(e) => {
                panic!("Parse failed: {:?}", e);
            }
        }
    }
}
    }
}
