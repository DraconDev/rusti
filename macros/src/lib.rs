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
pub fn azumi
(input: TokenStream) -> TokenStream {
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
        azumi
::from_fn(move |f| {
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

#[derive(Copy, Clone, PartialEq, Debug)]
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

                // Check if this element has <style> tags as DIRECT children only
                let has_style_children = elem.children.iter().any(|child| {
                    if let token_parser::Node::Element(el) = child {
                        el.name == "style"
                    } else {
                        false
                    }
                });

                // If this element has style children, it becomes a scoped container
                if has_style_children {
                    // Generate ONE scope ID for this container using library function
                    let scope_id_gen = quote! { azumi
::generate_scope_id() };

                    // Generate code for styled children with scope
                    let mut children_code = proc_macro2::TokenStream::new();
                    for child in &elem.children {
                        match child {
                            token_parser::Node::Element(child_elem)
                                if child_elem.name == "style" =>
                            {
                                // Handle <style> tag - transform CSS
                                let mut style_content = String::new();
                                for style_child in &child_elem.children {
                                    if let token_parser::Node::Text(text) = style_child {
                                        style_content.push_str(&text.content);
                                    }
                                }

                                if !style_content.is_empty() {
                                    children_code.extend(quote! {
                                        {
                                            let css_content = #style_content;
                                            let scoped_css = azumi
::scope_css(css_content, &scope_id);
                                            write!(f, "<style data-scope=\"{}\">", scope_id)?;
                                            write!(f, "{}", scoped_css)?;
                                            write!(f, "</style>")?;
                                        }
                                    });
                                }
                            }
                            token_parser::Node::Element(child_elem) => {
                                // Regular element - add data-scope attribute
                                let child_name = &child_elem.name;

                                // Generate child attributes
                                let mut child_attr_code = proc_macro2::TokenStream::new();
                                for attr in &child_elem.attrs {
                                    let attr_name = &attr.name;
                                    match &attr.value {
                                        token_parser::AttributeValue::Static(val) => {
                                            child_attr_code.extend(quote! {
                                                write!(f, " {}=\"{}\"", #attr_name, azumi
::Escaped(#val))?;
                                            });
                                        }
                                        token_parser::AttributeValue::Dynamic(expr) => {
                                            child_attr_code.extend(quote! {
                                                write!(f, " {}=\"{}\"", #attr_name, azumi
::Escaped(&(#expr)))?;
                                            });
                                        }
                                        token_parser::AttributeValue::None => {
                                            child_attr_code.extend(quote! {
                                                write!(f, " {}", #attr_name)?;
                                            });
                                        }
                                    }
                                }

                                // Determine child context
                                let grandchild_context = if child_name == "script" {
                                    Context::Script
                                } else if child_name == "style" {
                                    Context::Style
                                } else {
                                    Context::Normal
                                };

                                let grandchildren_code = generate_body_with_context(
                                    &child_elem.children,
                                    grandchild_context,
                                );

                                // Add data-scope attribute
                                children_code.extend(quote! {
                                    write!(f, "<{}", #child_name)?;
                                    write!(f, " data-{}", scope_id)?;
                                    #child_attr_code
                                    write!(f, ">")?;
                                    #grandchildren_code
                                    write!(f, "</{}>", #child_name)?;
                                });
                            }
                            token_parser::Node::Text(text) => {
                                let content = &text.content;
                                if !content.is_empty() {
                                    let stripped = strip_outer_quotes(content);
                                    children_code.extend(quote! {
                                        write!(f, "{}", #stripped)?;
                                    });
                                }
                            }
                            token_parser::Node::Expression(expr) => {
                                let content = &expr.content;
                                children_code.extend(quote! {
                                    write!(f, "{}", azumi
::Escaped(&(#content)))?;
                                });
                            }
                            _ => {
                                // Handle other node types (ForBlock, IfBlock, MatchBlock, etc)
                                // These need recursive handling which we'll skip for now in scoped containers
                                // Just generate them without scope attributes
                                let fallback_nodes = vec![child.clone()];
                                let fallback_code =
                                    generate_body_with_context(&fallback_nodes, context);
                                children_code.extend(fallback_code);
                            }
                        }
                    }

                    // Generate parent element with scoped children
                    let mut attr_code = proc_macro2::TokenStream::new();
                    for attr in &elem.attrs {
                        let attr_name = &attr.name;
                        match &attr.value {
                            token_parser::AttributeValue::Static(val) => {
                                attr_code.extend(quote! {
                                    write!(f, " {}=\"{}\"", #attr_name, azumi
::Escaped(#val))?;
                                });
                            }
                            token_parser::AttributeValue::Dynamic(expr) => {
                                attr_code.extend(quote! {
                                    write!(f, " {}=\"{}\"", #attr_name, azumi
::Escaped(&(#expr)))?;
                                });
                            }
                            token_parser::AttributeValue::None => {
                                attr_code.extend(quote! {
                                    write!(f, " {}", #attr_name)?;
                                });
                            }
                        }
                    }

                    // Don't return early! We need to process sibling elements too
                    quote! {
                        {
                            let scope_id = #scope_id_gen;
                            write!(f, "<{}", #name)?;
                            #attr_code
                            write!(f, ">")?;
                            #children_code
                            write!(f, "</{}>", #name)?;
                        }
                    }
                } else {
                    // Regular element (no style children) - original logic
                    let children_code = generate_body_with_context(&elem.children, child_context);

                    let mut attr_code = proc_macro2::TokenStream::new();
                    for attr in &elem.attrs {
                        let attr_name = &attr.name;
                        match &attr.value {
                            token_parser::AttributeValue::Static(val) => {
                                attr_code.extend(quote! {
                                    write!(f, " {}=\"{}\"", #attr_name, azumi
::Escaped(#val))?;
                                });
                            }
                            token_parser::AttributeValue::Dynamic(expr) => {
                                attr_code.extend(quote! {
                                    write!(f, " {}=\"{}\"", #attr_name, azumi
::Escaped(&(#expr)))?;
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
                println!(
                    "Generating expression: {:?} in context: {:?}",
                    content, context
                );
                match context {
                    Context::Script => {
                        // In script tags, use azumi
::js() to safely inject values (Debug formatting)
                        println!("  -> Script context, using azumi
::js");
                        quote! { write!(f, "{}", azumi
::js(&(#content)))?; }
                    }
                    Context::Style => {
                        // In style tags, use Display (raw text)
                        quote! { write!(f, "{}", #content)?; }
                    }
                    Context::Normal => {
                        // In normal HTML, use Escaped (HTML escaping)
                        quote! { write!(f, "{}", azumi
::Escaped(&(#content)))?; }
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
                            azumi
::from_fn(|f| {
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
                            } else if exprs.is_empty() {
                                // Empty args: check if name starts with Uppercase (Component convention)
                                // If so, treat as named args (Builder pattern) to support defaults
                                let name_str = quote!(#name).to_string();
                                let last_segment = name_str.split("::").last().unwrap_or("");
                                if last_segment
                                    .chars()
                                    .next()
                                    .map_or(false, |c| c.is_uppercase())
                                {
                                    Some(exprs)
                                } else {
                                    None
                                }
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

                        let children_arg = if has_children {
                            quote! { , #children_code }
                        } else {
                            quote! {}
                        };

                        quote! {
                            azumi
::Component::render(&#name::render(#name::Props::builder()
                                #(#setters)*
                                .build().expect("Failed to build props")
                                #children_arg), f)?;
                        }
                    } else {
                        let args_separator = if !args.is_empty() && has_children {
                            quote! { , }
                        } else {
                            quote! {}
                        };

                        quote! {
                            azumi
::Component::render(&#name(#args #args_separator #children_code), f)?;
                        }
                    }
                }
                token_parser::Block::Component(comp_block) => {
                    let name = &comp_block.name;
                    quote! {
                        azumi
::Component::render(&#name, f)?;
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
