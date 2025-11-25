// Force rebuild 3
mod component;
mod parser; // Keep for extern crate proc_macro;

mod test_spacing;
mod token_parser;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    component::expand_component(item)
}

struct NodesWrapper(Vec<token_parser::Node>);

impl Parse for NodesWrapper {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        token_parser::parse_nodes(input).map(NodesWrapper)
    }
}

#[proc_macro]
pub fn rusti(input: TokenStream) -> TokenStream {
    // Handle string literal input for emoji support
    let input_tokens = if let Ok(lit) = syn::parse::<syn::LitStr>(input.clone()) {
        match lit.value().parse::<proc_macro2::TokenStream>() {
            Ok(ts) => ts,
            Err(e) => {
                return syn::Error::new(lit.span(), format!("Invalid tokens in string: {}", e))
                    .to_compile_error()
                    .into()
            }
        }
    } else {
        proc_macro2::TokenStream::from(input)
    };

    let nodes = match syn::parse2::<NodesWrapper>(input_tokens) {
        Ok(wrapper) => wrapper.0,
        Err(e) => return e.to_compile_error().into(),
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

/// Strip outer quotes from string literals for cleaner text rendering
/// If the user wants literal quotes, they should use raw strings like r#""Hello""#
fn strip_outer_quotes(s: &str) -> String {
    let trimmed = s.trim();
    if trimmed.len() >= 2 {
        if (trimmed.starts_with('"') && trimmed.ends_with('"'))
            || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
        {
            return trimmed[1..trimmed.len() - 1].to_string();
        }
    }
    s.to_string()
}

#[derive(Copy, Clone, PartialEq)]
enum Context {
    Normal,
    Script,
    Style,
}

fn generate_body(nodes: &[token_parser::Node]) -> proc_macro2::TokenStream {
    generate_body_with_context(nodes, Context::Normal)
}

fn generate_body_with_context(
    nodes: &[token_parser::Node],
    context: Context,
) -> proc_macro2::TokenStream {
    let mut stream = proc_macro2::TokenStream::new();
    for node in nodes {
        let chunk = match node {
            token_parser::Node::Element(elem) => {
                let name = &elem.name;

                // Determine context for children
                let child_context = if name == "script" {
                    Context::Script
                } else if name == "style" {
                    Context::Style
                } else {
                    Context::Normal
                };

                // Special handling for <style src="...">
                if name == "style" {
                    if let Some(src_attr) = elem.attrs.iter().find(|a| a.name == "src") {
                        if let token_parser::AttributeValue::Static(path) = &src_attr.value {
                            return quote! {
                                write!(f, "<style>{}</style>", include_str!(#path))?;
                            };
                        }
                    }
                }

                let children_code = generate_body_with_context(&elem.children, child_context);

                let mut attr_code = proc_macro2::TokenStream::new();
                for attr in &elem.attrs {
                    let attr_name = &attr.name;
                    match &attr.value {
                        token_parser::AttributeValue::Static(val) => {
                            attr_code.extend(quote! {
                                write!(f, " {}=\"{}\"", #attr_name, rusti::Escaped(#val))?;
                            });
                        }
                        token_parser::AttributeValue::Dynamic(expr) => {
                            attr_code.extend(quote! {
                                write!(f, " {}=\"{}\"", #attr_name, rusti::Escaped(&(#expr)))?;
                            });
                        }
                        token_parser::AttributeValue::None => {
                            // Boolean attribute
                            attr_code.extend(quote! {
                                write!(f, " {}", #attr_name)?;
                            });
                        }
                    }
                }

                quote! {
                    write!(f, "<{}", #name)?;
                    #attr_code
                    write!(f, ">")?;
                    #children_code
                    write!(f, "</{}>", #name)?;
                }
            }
            token_parser::Node::Text(text) => {
                let content = &text.content;
                if content.is_empty() {
                    quote! {}
                } else {
                    // Strip outer quotes from string literals for cleaner rendering
                    let stripped = strip_outer_quotes(content);
                    quote! { write!(f, "{}", #stripped)?; }
                }
            }
            token_parser::Node::Expression(expr) => {
                let content = &expr.content;
                match context {
                    Context::Script => {
                        // In script tags, use rusti::js() to safely inject values (Debug formatting)
                        quote! { write!(f, "{}", rusti::js(&(#content)))?; }
                    }
                    Context::Style => {
                        // In style tags, use Display (raw text)
                        quote! { write!(f, "{}", #content)?; }
                    }
                    Context::Normal => {
                        // In normal HTML, use Escaped (HTML escaping)
                        quote! { write!(f, "{}", rusti::Escaped(&(#content)))?; }
                    }
                }
            }
            token_parser::Node::Comment(_) => {
                // Ignore comments in output
                quote! {}
            }
            token_parser::Node::Doctype(doctype) => {
                let content = &doctype.content;
                quote! { write!(f, "<!DOCTYPE {}>", #content)?; }
            }
            token_parser::Node::Fragment(frag) => {
                generate_body_with_context(&frag.children, context)
            }
            token_parser::Node::Block(block) => match block {
                token_parser::Block::If(if_block) => {
                    let condition = &if_block.condition;
                    let then_code = generate_body_with_context(&if_block.then_branch, context);
                    let else_code = if let Some(else_branch) = &if_block.else_branch {
                        let else_body = generate_body_with_context(else_branch, context);
                        quote! { else { #else_body } }
                    } else {
                        quote! {}
                    };
                    quote! {
                        if #condition {
                            #then_code
                        } #else_code
                    }
                }
                token_parser::Block::For(for_block) => {
                    let pattern = &for_block.pattern;
                    let iterator = &for_block.iterator;
                    let body_code = generate_body_with_context(&for_block.body, context);
                    quote! {
                        for #pattern in #iterator {
                            #body_code
                        }
                    }
                }
                token_parser::Block::Match(match_block) => {
                    let expr = &match_block.expr;
                    let arms = match_block.arms.iter().map(|arm| {
                        let pattern = &arm.pattern;
                        let body = generate_body_with_context(&arm.body, context);
                        quote! {
                            #pattern => { #body }
                        }
                    });
                    quote! {
                        match #expr {
                            #(#arms),*
                        }
                    }
                }
                token_parser::Block::Call(call_block) => {
                    let name = &call_block.name;
                    let args = &call_block.args;
                    let has_children = !call_block.children.is_empty();
                    let children_code = if has_children {
                        let children_body =
                            generate_body_with_context(&call_block.children, context);
                        quote! {
                            , rusti::from_fn(|f| {
                                #children_body
                                Ok(())
                            })
                        }
                    } else {
                        quote! {}
                    };

                    // Check if args are named or positional
                    let parser = syn::punctuated::Punctuated::<syn::Expr, syn::token::Comma>::parse_terminated;
                    let named_args =
                        if let Ok(exprs) = syn::parse::Parser::parse2(parser, args.clone()) {
                            if !exprs.is_empty()
                                && exprs.iter().all(|e| matches!(e, syn::Expr::Assign(_)))
                            {
                                Some(exprs)
                            } else {
                                None
                            }
                        } else {
                            None
                        };

                    if let Some(exprs) = named_args {
                        let setters = exprs.iter().map(|e| {
                            if let syn::Expr::Assign(assign) = e {
                                let key = &assign.left;
                                let value = &assign.right;
                                quote! { .#key(#value) }
                            } else {
                                unreachable!()
                            }
                        });
                        quote! {
                            rusti::Component::render(&#name::render(#name::Props::builder()
                                #(#setters)*
                                .build().expect("Failed to build props")
                                #children_code), f)?;
                        }
                    } else {
                        quote! {
                            rusti::Component::render(&#name(#args #children_code), f)?;
                        }
                    }
                }
                token_parser::Block::Component(comp_block) => {
                    let name = &comp_block.name;
                    quote! {
                        rusti::Component::render(&#name, f)?;
                    }
                }
                token_parser::Block::Let(let_block) => {
                    let pattern = &let_block.pattern;
                    let value = &let_block.value;
                    quote! {
                        let #pattern = #value;
                    }
                }
            },
        };
        stream.extend(chunk);
    }
    stream
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod attr_tests;

#[cfg(test)]
mod token_parser_tests;
