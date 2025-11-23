use crate::token_parser::{parse_nodes, Node};
use quote::quote;
use syn::parse::{ParseStream, Parser};
use syn::Result;

fn parse_nodes_wrapper(input: ParseStream) -> Result<Vec<Node>> {
    parse_nodes(input)
}

#[test]
fn test_style_in_head() {
    let input = quote! {
        <head>
            <style>
                body { color: red; }
            </style>
        </head>
    };

    let parser = parse_nodes_wrapper;
    let result = parser.parse2(input);
    match result {
        Ok(_) => println!("Success!"),
        Err(e) => panic!("Failed: {}", e),
    }
}

#[test]
fn test_mismatched_closing_tag() {
    let input = quote! {
        <div></span>
    };
    let parser = parse_nodes_wrapper;
    let result = parser.parse2(input);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Mismatched closing tag: expected </div>, found </span>"
    );
}

#[test]
fn test_component_block_in_div() {
    let input = quote! {
        <div>
            @foo
        </div>
    };
    let parser = parse_nodes_wrapper;
    let result = parser.parse2(input);
    assert!(result.is_ok());
}
