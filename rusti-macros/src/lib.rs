mod parser;
use proc_macro::TokenStream;
use quote::quote;
use std::str::FromStr;

#[proc_macro]
pub fn rusti(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();

    // Parse the block content
    let nodes = match parser::parse_nodes(&input_str) {
        Ok((_, nodes)) => nodes,
        Err(e) => {
            return syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Failed to parse template: {}", e),
            )
            .to_compile_error()
            .into()
        }
    };

    let body = generate_body(&nodes);

    let output = quote! {
        rusti::from_fn(move |f| {
            #body
            Ok(())
        })
    };

    TokenStream::from(output)
}

fn generate_body(nodes: &[parser::Node]) -> proc_macro2::TokenStream {
    let mut stream = proc_macro2::TokenStream::new();
    for node in nodes {
        let chunk = match node {
            parser::Node::Element {
                name,
                _attrs: _,
                children,
            } => {
                let children_code = generate_body(children);
                quote! {
                    write!(f, "<{}>", #name)?;
                    #children_code
                    write!(f, "</{}>", #name)?;
                }
            }
            parser::Node::Text(text) => {
                if text.is_empty() {
                    quote! {}
                } else {
                    quote! { write!(f, "{}", #text)?; }
                }
            }
            parser::Node::Expression(expr) => {
                // expr is a string like "name". We need to parse it as an expression to quote it safely?
                // Or just emit it as tokens.
                // "name" -> Ident(name).
                // "x + 1" -> Expr...
                // We can parse it using syn::parse_str::<Expr>
                match syn::parse_str::<syn::Expr>(expr) {
                    Ok(parsed_expr) => quote! { write!(f, "{}", rusti::Escaped(#parsed_expr))?; },
                    Err(_) => {
                        // Fallback: just emit as string? No, that won't compile.
                        // If we can't parse it, it's probably invalid Rust.
                        // But let's try to emit it as tokens.
                        use std::str::FromStr;
                        let tokens = proc_macro2::TokenStream::from_str(expr).unwrap();
                        quote! { write!(f, "{}", rusti::Escaped(#tokens))?; }
                    }
                }
            }
            parser::Node::Call {
                name,
                args,
                _children: _,
            } => {
                let name_ident = syn::parse_str::<syn::Path>(name).unwrap();
                // args is "(...)" string.
                // We need to parse args.
                // If args is "name", it's "name".
                // The parser returns args without parens?
                // Parser: delimited(char('('), take_until(")"), char(')'))
                // So args is the content inside parens.
                let args_tokens = proc_macro2::TokenStream::from_str(args).unwrap();

                quote! {
                    rusti::Component::render(&#name_ident(#args_tokens), f)?;
                }
            }
        };
        stream.extend(chunk);
    }
    stream
}
