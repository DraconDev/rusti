mod parser;
use proc_macro::TokenStream;
use std::str::FromStr;
use quote::quote;
use syn::{parse_macro_input, ItemFn, parse::Parse, parse::ParseStream, Token, Ident, Generics, Visibility, Signature, Block};

// We can't use ItemFn because the block is invalid.
// We need a custom struct.
struct RustiFn {
    attrs: Vec<syn::Attribute>,
    vis: Visibility,
    sig: Signature,
    block_content: String,
}

impl Parse for RustiFn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        let sig: Signature = input.parse()?;
        
        let content;
        let _ = syn::braced!(content in input);
        let token_stream: proc_macro2::TokenStream = content.parse()?;
        let block_content = token_stream.to_string();
        
        Ok(RustiFn {
            attrs,
            vis,
            sig,
            block_content,
        })
    }
}

#[proc_macro]
pub fn rusti(item: TokenStream) -> TokenStream {
    let RustiFn { attrs, vis, sig, block_content } = parse_macro_input!(item as RustiFn);
    
    let name = &sig.ident;
    let inputs = &sig.inputs;
    // We ignore the return type in the signature and enforce `impl Component`.
    
    // Parse the block content
    let nodes = match parser::parse_nodes(&block_content) {
        Ok((_, nodes)) => nodes,
        Err(e) => return syn::Error::new_spanned(name, format!("Failed to parse template: {}", e)).to_compile_error().into(),
    };
    
    let body = generate_body(&nodes);
    
    let output = quote! {
        #(#attrs)*
        #vis fn #name(#inputs) -> impl rusti::Component {
            rusti::from_fn(move |f| {
                #body
                Ok(())
            })
        }
    };
    
    TokenStream::from(output)
}

fn generate_body(nodes: &[parser::Node]) -> proc_macro2::TokenStream {
    let mut stream = proc_macro2::TokenStream::new();
    for node in nodes {
        let chunk = match node {
            parser::Node::Element { name, attrs: _, children } => {
                let children_code = generate_body(children);
                quote! {
                    write!(f, "<{}>", #name)?;
                    #children_code
                    write!(f, "</{}>", #name)?;
                }
            }
            parser::Node::Text(text) => {
                let text = text.trim();
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
                    Ok(parsed_expr) => quote! { write!(f, "{}", #parsed_expr)?; },
                    Err(_) => {
                        // Fallback: just emit as string? No, that won't compile.
                        // If we can't parse it, it's probably invalid Rust.
                        // But let's try to emit it as tokens.
                        use std::str::FromStr;
                        let tokens = proc_macro2::TokenStream::from_str(expr).unwrap();
                        quote! { write!(f, "{}", #tokens)?; }
                    }
                }
            }
            parser::Node::Call { name, args, children: _ } => {
                let name_ident = syn::parse_str::<syn::Path>(name).unwrap();
                // args is "(...)" string.
                // We need to parse args.
                // If args is "name", it's "name".
                // The parser returns args without parens?
                // Parser: delimited(char('('), take_until(")"), char(')'))
                // So args is the content inside parens.
                let args_tokens = proc_macro2::TokenStream::from_str(args).unwrap();
                
                quote! {
                    #name_ident(#args_tokens).render(f)?;
                }
            }
        };
        stream.extend(chunk);
    }
    stream
}
