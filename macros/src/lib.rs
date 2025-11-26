// Force rebuild 3
mod component;
mod parser; // Keep for extern crate proc_macro;

mod css;
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
pub fn html(input: TokenStream) -> TokenStream {
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
        azumi::from_fn(move |f| {
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

#[derive(Clone, PartialEq, Debug)]
enum Context {
    Normal,
    Script,
    Style,
}

#[derive(Clone, Debug)]
struct GenerationContext {
    mode: Context,
    scope_id: Option<String>,
}

impl GenerationContext {
    fn normal() -> Self {
        Self {
            mode: Context::Normal,
            scope_id: None,
        }
    }

    fn with_scope(scope_id: String) -> Self {
        Self {
            mode: Context::Normal,
            scope_id: Some(scope_id),
        }
    }

    fn with_mode(&self, mode: Context) -> Self {
        Self {
            mode,
            scope_id: self.scope_id.clone(),
        }
    }
}

/// Recursively check if any <style> tags exist anywhere in the node tree
fn has_any_styles(nodes: &[token_parser::Node]) -> bool {
    nodes.iter().any(|node| match node {
        token_parser::Node::Element(elem) => elem.name == "style" || has_any_styles(&elem.children),
        token_parser::Node::Fragment(frag) => has_any_styles(&frag.children),
        token_parser::Node::Block(block) => match block {
            token_parser::Block::If(if_block) => {
                has_any_styles(&if_block.then_branch)
                    || if_block
                        .else_branch
                        .as_ref()
                        .map(|els| has_any_styles(els))
                        .unwrap_or(false)
            }
            token_parser::Block::For(for_block) => has_any_styles(&for_block.body),
            token_parser::Block::Match(match_block) => {
                match_block.arms.iter().any(|arm| has_any_styles(&arm.body))
            }
            token_parser::Block::Call(call_block) => has_any_styles(&call_block.children),
            _ => false,
        },
        _ => false,
    })
}

/// Collect ALL CSS content from all <style> tags in the component
fn collect_all_styles(nodes: &[token_parser::Node]) -> String {
    let mut css_content = String::new();
    collect_styles_recursive(nodes, &mut css_content);
    css_content
}

fn collect_styles_recursive(nodes: &[token_parser::Node], css_content: &mut String) {
    for node in nodes {
        match node {
            token_parser::Node::Element(elem) => {
                if elem.name == "style" {
                    // Extract CSS from this style tag
                    if let Some(src_attr) = elem.attrs.iter().find(|a| a.name == "src") {
                        if let token_parser::AttributeValue::Static(path) = &src_attr.value {
                            let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
                                .expect("CARGO_MANIFEST_DIR not set");
                            let file_path = if path.starts_with('/') {
                                std::path::Path::new(&manifest_dir).join(&path[1..])
                            } else {
                                std::path::Path::new(&manifest_dir).join(path)
                            };
                            if let Ok(content) = std::fs::read_to_string(&file_path) {
                                css_content.push_str(&content);
                                css_content.push('\n');
                            }
                        }
                    } else {
                        // Inline content
                        for child in &elem.children {
                            if let token_parser::Node::Text(text) = child {
                                css_content.push_str(&text.content);
                                css_content.push('\n');
                            }
                        }
                    }
                } else {
                    // Recurse into children
                    collect_styles_recursive(&elem.children, css_content);
                }
            }
            token_parser::Node::Fragment(frag) => {
                collect_styles_recursive(&frag.children, css_content);
            }
            token_parser::Node::Block(block) => match block {
                token_parser::Block::If(if_block) => {
                    collect_styles_recursive(&if_block.then_branch, css_content);
                    if let Some(else_branch) = &if_block.else_branch {
                        collect_styles_recursive(else_branch, css_content);
                    }
                }
                token_parser::Block::For(for_block) => {
                    collect_styles_recursive(&for_block.body, css_content);
                }
                token_parser::Block::Match(match_block) => {
                    for arm in &match_block.arms {
                        collect_styles_recursive(&arm.body, css_content);
                    }
                }
                token_parser::Block::Call(call_block) => {
                    collect_styles_recursive(&call_block.children, css_content);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

fn generate_body(nodes: &[token_parser::Node]) -> proc_macro2::TokenStream {
    // Pass 1: Check if component has any style tags
    if has_any_styles(nodes) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // Collect all CSS content
        let css_content = collect_all_styles(nodes);

        // Generate scope ID
        let mut hasher = DefaultHasher::new();
        css_content.hash(&mut hasher);
        let hash = hasher.finish();
        let scope_id = format!("s{:x}", hash);

        // Scope the CSS
        let scoped_css = crate::css::scope_css(&css_content, &scope_id);

        // Generate body with scope context
        let ctx = GenerationContext::with_scope(scope_id.clone());
        let body = generate_body_with_context(nodes, &ctx);

        // Inject scoped CSS at the beginning
        quote! {
            write!(f, "<style>{}</style>", #scoped_css)?;
            #body
        }
    } else {
        generate_body_with_context(nodes, &GenerationContext::normal())
    }
}

fn generate_body_with_context(
    nodes: &[token_parser::Node],
    ctx: &GenerationContext,
) -> proc_macro2::TokenStream {
    let mut stream = proc_macro2::TokenStream::new();
    for node in nodes {
        let chunk = match node {
            token_parser::Node::Element(elem) => {
                let name = &elem.name;

                // Skip <style> tags - they're already processed in generate_body
                if name == "style" {
                    continue;
                }

                // Determine context for children
                let child_context = if name == "script" {
                    ctx.with_mode(Context::Script)
                } else if name == "style" {
                    ctx.with_mode(Context::Style)
                } else {
                    ctx.clone()
                };

                // Generate children
                let children_code = generate_body_with_context(&elem.children, &child_context);

                // Generate attributes
                let mut attr_code = proc_macro2::TokenStream::new();
                for attr in &elem.attrs {
                    let attr_name = &attr.name;
                    match &attr.value {
                        token_parser::AttributeValue::Static(val) => {
                            attr_code.extend(quote! {
                                write!(f, " {}=\"{}\"", #attr_name, azumi::Escaped(#val))?;
                            });
                        }
                        token_parser::AttributeValue::Dynamic(expr) => {
                            attr_code.extend(quote! {
                                write!(f, " {}=\"{}\"", #attr_name, azumi::Escaped(&(#expr)))?;
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

                // Generate element with potential scope attribute from context
                if let Some(ref scope_id) = ctx.scope_id {
                    quote! {
                        write!(f, "<{}", #name)?;
                        write!(f, " data-{}", #scope_id)?;
                        #attr_code
                        write!(f, ">")?;
                        #children_code
                        write!(f, "</{}>", #name)?;
                    }
                } else {
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
                    content, ctx.mode
                );
                match ctx.mode {
                    Context::Script => {
                        // In script tags, use azumi::js() to safely inject values (Debug formatting)
                        println!("  -> Script context, using azumi::js");
                        quote! { write!(f, "{}", azumi::js(&(#content)))?; }
                    }
                    Context::Style => {
                        // In style tags, use Display (raw text)
                        quote! { write!(f, "{}", #content)?; }
                    }
                    Context::Normal => {
                        // In normal HTML, use Escaped (HTML escaping)
                        quote! { write!(f, "{}", azumi::Escaped(&(#content)))?; }
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
            token_parser::Node::Fragment(frag) => generate_body_with_context(&frag.children, ctx),
            token_parser::Node::Block(block) => match block {
                token_parser::Block::If(if_block) => {
                    let condition = &if_block.condition;
                    let then_code = generate_body_with_context(&if_block.then_branch, ctx);
                    let else_code = if let Some(else_branch) = &if_block.else_branch {
                        let else_body = generate_body_with_context(else_branch, ctx);
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
                    let body_code = generate_body_with_context(&for_block.body, ctx);
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
                        let body = generate_body_with_context(&arm.body, ctx);
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
                        let children_body = generate_body_with_context(&call_block.children, ctx);
                        quote! {
                            azumi::from_fn(|f| {
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
                            azumi::Component::render(&#name::render(#name::Props::builder()
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
                            azumi::Component::render(&#name(#args #args_separator #children_code), f)?;
                        }
                    }
                }
                token_parser::Block::Component(comp_block) => {
                    let name = &comp_block.name;
                    quote! {
                        azumi::Component::render(&#name, f)?;
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
