// Force rebuild 2
mod parser;
use proc_macro::TokenStream;
use quote::quote;
use std::str::FromStr;

#[proc_macro]
pub fn rusti(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    // panic!("DEBUG: input_str: '{}'", input_str);

    // Parse the block content
    let nodes = match parser::parse_nodes(&input_str) {
        Ok((remaining, nodes)) => {
            if !remaining.trim().is_empty() {
                return syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("Unexpected input remaining: {}", remaining),
                )
                .to_compile_error()
                .into();
            }
            nodes
        }
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
                attrs,
                children,
            } => {
                let children_code = generate_body(children);

                // Generate attribute code
                let mut attr_code = proc_macro2::TokenStream::new();
                for (attr_name, attr_value) in attrs {
                    let attr_chunk = match attr_value {
                        parser::AttributeValue::Static(value) => {
                            // Static attributes are escaped for XSS safety
                            quote! {
                                write!(f, " {}=\"{}\"", #attr_name, rusti::Escaped(#value))?;
                            }
                        }
                        parser::AttributeValue::Dynamic(expr) => {
                            // Dynamic attributes - parse expression and escape
                            match syn::parse_str::<syn::Expr>(expr) {
                                Ok(parsed_expr) => quote! {
                                    write!(f, " {}=\"{}\"", #attr_name, rusti::Escaped(#parsed_expr))?;
                                },
                                Err(_) => {
                                    use std::str::FromStr;
                                    match proc_macro2::TokenStream::from_str(expr) {
                                        Ok(tokens) => quote! {
                                            write!(f, " {}=\"{}\"", #attr_name, rusti::Escaped(#tokens))?;
                                        },
                                        Err(e) => syn::Error::new(
                                            proc_macro2::Span::call_site(),
                                            format!(
                                                "Invalid attribute expression '{}': {}",
                                                expr, e
                                            ),
                                        )
                                        .to_compile_error(),
                                    }
                                }
                            }
                        }
                    };
                    attr_code.extend(attr_chunk);
                }

                quote! {
                    write!(f, "<{}", #name)?;
                    #attr_code
                    write!(f, ">")?;
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
                        match proc_macro2::TokenStream::from_str(expr) {
                            Ok(tokens) => quote! { write!(f, "{}", rusti::Escaped(#tokens))?; },
                            Err(e) => syn::Error::new(
                                proc_macro2::Span::call_site(),
                                format!("Invalid expression '{}': {}", expr, e),
                            )
                            .to_compile_error(),
                        }
                    }
                }
            }
            parser::Node::Call {
                name,
                args,
                _children: _,
            } => {
                let name_ident = match syn::parse_str::<syn::Path>(name) {
                    Ok(path) => path,
                    Err(e) => {
                        return syn::Error::new(
                            proc_macro2::Span::call_site(),
                            format!("Invalid component name '{}': {}", name, e),
                        )
                        .to_compile_error();
                    }
                };

                let args_tokens = match proc_macro2::TokenStream::from_str(args) {
                    Ok(tokens) => tokens,
                    Err(e) => {
                        return syn::Error::new(
                            proc_macro2::Span::call_site(),
                            format!("Invalid arguments '{}': {}", args, e),
                        )
                        .to_compile_error();
                    }
                };

                quote! {
                    rusti::Component::render(&#name_ident(#args_tokens), f)?;
                }
            }
            parser::Node::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition_expr =
                    syn::parse_str::<syn::Expr>(condition).expect("Failed to parse if condition");
                let then_code = generate_body(then_branch);
                let else_code = if let Some(else_nodes) = else_branch {
                    let else_body = generate_body(else_nodes);
                    quote! { else { #else_body } }
                } else {
                    quote! {}
                };

                quote! {
                    if #condition_expr {
                        #then_code
                    } #else_code
                }
            }
            parser::Node::For {
                pattern,
                iterator,
                body,
            } => {
                let pattern_pat = proc_macro2::TokenStream::from_str(pattern)
                    .expect("Failed to parse for pattern");
                let iterator_expr =
                    syn::parse_str::<syn::Expr>(iterator).expect("Failed to parse for iterator");
                let body_code = generate_body(body);

                quote! {
                    for #pattern_pat in #iterator_expr {
                        #body_code
                    }
                }
            }
            parser::Node::Component { name } => {
                let name_ident = match syn::parse_str::<syn::Path>(name) {
                    Ok(path) => path,
                    Err(e) => {
                        return syn::Error::new(
                            proc_macro2::Span::call_site(),
                            format!("Invalid component name '{}': {}", name, e),
                        )
                        .to_compile_error();
                    }
                };

                quote! {
                    rusti::Component::render(&#name_ident, f)?;
                }
            }
            parser::Node::Match { expr, arms } => {
                let match_expr =
                    syn::parse_str::<syn::Expr>(expr).expect("Failed to parse match expression");

                let match_arms_code: Vec<_> = arms
                    .iter()
                    .map(|arm| {
                        let pattern_str = &arm.pattern;
                        let body_code = generate_body(&arm.body);

                        // Parse the pattern - handle wildcard _ specially
                        if pattern_str.trim() == "_" {
                            quote! {
                                _ => {
                                    #body_code
                                }
                            }
                        } else {
                            // Try to parse as a pattern
                            let pattern_pat = proc_macro2::TokenStream::from_str(pattern_str)
                                .expect("Failed to parse match pattern");
                            quote! {
                                #pattern_pat => {
                                    #body_code
                                }
                            }
                        }
                    })
                    .collect();

                quote! {
                    match #match_expr {
                        #(#match_arms_code)*
                    }
                }
            }
        };
        stream.extend(chunk);
    }
    stream
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod attr_tests;
