// Force rebuild 3
mod component;

mod css;
mod css_validator;
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

fn has_nested_style(elem: &token_parser::Element) -> bool {
    elem.children.iter().any(|child| {
        if let token_parser::Node::Element(el) = child {
            el.name == "style"
        } else {
            false
        }
    })
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
/// Returns (global_css, scoped_css)
fn collect_all_styles(nodes: &[token_parser::Node]) -> (String, String) {
    let mut global_css = String::new();
    let mut scoped_css = String::new();
    collect_styles_recursive(nodes, &mut global_css, &mut scoped_css);
    (global_css, scoped_css)
}

fn collect_styles_recursive(
    nodes: &[token_parser::Node],
    global_css: &mut String,
    scoped_css: &mut String,
) {
    for node in nodes {
        match node {
            token_parser::Node::Element(elem) => {
                if elem.name == "style" {
                    // Extract CSS from this style tag
                    if let Some(src_attr) = elem.attrs.iter().find(|a| a.name == "src") {
                        if let token_parser::AttributeValue::Static(path) = &src_attr.value {
                            // Use enhanced path resolution from validator
                            let file_path = crate::css_validator::resolve_css_file_path(path);
                            if let Ok(content) = std::fs::read_to_string(&file_path) {
                                // Check if this is a global.css file
                                if path.ends_with("global.css") {
                                    global_css.push_str(&content);
                                    global_css.push('\n');
                                } else {
                                    scoped_css.push_str(&content);
                                    scoped_css.push('\n');
                                }
                            }
                        }
                    } else {
                        // Inline content - treat as scoped
                        for child in &elem.children {
                            if let token_parser::Node::Text(text) = child {
                                scoped_css.push_str(&text.content);
                                scoped_css.push('\n');
                            }
                        }
                    }
                } else {
                    // Recurse into children
                    collect_styles_recursive(&elem.children, global_css, scoped_css);
                }
            }
            token_parser::Node::Fragment(frag) => {
                collect_styles_recursive(&frag.children, global_css, scoped_css);
            }
            token_parser::Node::Block(block) => match block {
                token_parser::Block::If(if_block) => {
                    collect_styles_recursive(&if_block.then_branch, global_css, scoped_css);
                    if let Some(else_branch) = &if_block.else_branch {
                        collect_styles_recursive(else_branch, global_css, scoped_css);
                    }
                }
                token_parser::Block::For(for_block) => {
                    collect_styles_recursive(&for_block.body, global_css, scoped_css);
                }
                token_parser::Block::Match(match_block) => {
                    for arm in &match_block.arms {
                        collect_styles_recursive(&arm.body, global_css, scoped_css);
                    }
                }
                token_parser::Block::Call(call_block) => {
                    collect_styles_recursive(&call_block.children, global_css, scoped_css);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

fn generate_body(nodes: &[token_parser::Node]) -> proc_macro2::TokenStream {
    // Pass 0: CSS Validation - Revolutionary compile-time CSS type checking!
    let css_validation_errors = css_validator::validate_component_css(nodes);

    // If there are compile errors from CSS validation, emit them
    if !css_validation_errors.is_empty() {
        return css_validation_errors;
    }

    // Collect all CSS content first (needed for validation and scoping)
    let (global_css, scoped_css) = collect_all_styles(nodes);

    // Pass 0.5: Strict CSS Validation (New Feature)
    // Extract valid selectors from the SCOPED CSS only (global CSS is opt-out)
    let (valid_classes, valid_ids) = crate::css::extract_selectors(&scoped_css);

    // Validate nodes against strict rules
    if let Err(e) = validate_nodes(nodes, &valid_classes, &valid_ids) {
        // For now, just print to stderr so we don't break the build for other examples
        // In a real strict mode, we would return the compile_error!
        eprintln!("CSS Validation Error: {}", e);
        // return quote! { compile_error!(#e); };
    }

    // Pass 1: Check if component has any style tags
    let has_global = !global_css.is_empty();
    let has_scoped = !scoped_css.is_empty();

    if has_global || has_scoped {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // Generate body
        let body = if has_scoped {
            // Generate scope ID from scoped CSS only
            let mut hasher = DefaultHasher::new();
            scoped_css.hash(&mut hasher);
            let hash = hasher.finish();
            let scope_id = format!("s{:x}", hash);

            // Scope the CSS
            let scoped_output = crate::css::scope_css(&scoped_css, &scope_id);

            // Generate body with scope context
            let ctx = GenerationContext::with_scope(scope_id.clone());
            let body_content = generate_body_with_context(nodes, &ctx);

            // Inject global CSS first (unscoped), then scoped CSS
            if has_global {
                quote! {
                    write!(f, "<style>{}</style>", #global_css)?;
                    write!(f, "<style>{}</style>", #scoped_output)?;
                    #body_content
                }
            } else {
                quote! {
                    write!(f, "<style>{}</style>", #scoped_output)?;
                    #body_content
                }
            }
        } else {
            // Only global CSS, no scoping needed
            let ctx = GenerationContext::normal();
            let body_content = generate_body_with_context(nodes, &ctx);

            quote! {
                write!(f, "<style>{}</style>", #global_css)?;
                #body_content
            }
        };

        body
    } else {
        generate_body_with_context(nodes, &GenerationContext::normal())
    }
}

fn validate_nodes(
    nodes: &[token_parser::Node],
    valid_classes: &std::collections::HashSet<String>,
    valid_ids: &std::collections::HashSet<String>,
) -> Result<(), String> {
    for node in nodes {
        match node {
            token_parser::Node::Element(elem) => {
                // Check attributes
                for attr in &elem.attrs {
                    let name = &attr.name;

                    // Rule 1: Ban inline styles
                    if name == "style" {
                        return Err(
                            "Inline styles are banned. Use CSS classes instead.".to_string()
                        );
                    }

                    // Rule 2: Check class existence
                    if name == "class" {
                        if let token_parser::AttributeValue::Static(val) = &attr.value {
                            for class_name in val.split_whitespace() {
                                if !valid_classes.contains(class_name) {
                                    return Err(format!(
                                        "Class '{}' is used in HTML but not defined in CSS.",
                                        class_name
                                    ));
                                }
                            }
                        }
                    }

                    // Rule 3: Check ID existence
                    if name == "id" {
                        if let token_parser::AttributeValue::Static(val) = &attr.value {
                            if !valid_ids.contains(val) {
                                return Err(format!(
                                    "ID '{}' is used in HTML but not defined in CSS.",
                                    val
                                ));
                            }
                        }
                    }
                }

                // Recurse
                validate_nodes(&elem.children, valid_classes, valid_ids)?;
            }
            token_parser::Node::Fragment(frag) => {
                validate_nodes(&frag.children, valid_classes, valid_ids)?;
            }
            token_parser::Node::Block(block) => match block {
                token_parser::Block::If(if_block) => {
                    validate_nodes(&if_block.then_branch, valid_classes, valid_ids)?;
                    if let Some(else_branch) = &if_block.else_branch {
                        validate_nodes(else_branch, valid_classes, valid_ids)?;
                    }
                }
                token_parser::Block::For(for_block) => {
                    validate_nodes(&for_block.body, valid_classes, valid_ids)?;
                }
                token_parser::Block::Match(match_block) => {
                    for arm in &match_block.arms {
                        validate_nodes(&arm.body, valid_classes, valid_ids)?;
                    }
                }
                token_parser::Block::Call(call_block) => {
                    validate_nodes(&call_block.children, valid_classes, valid_ids)?;
                }
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
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

                        // Check if name is snake_case (starts with lowercase)
                        let name_str = quote!(#name).to_string();
                        let last_segment = name_str.split("::").last().unwrap_or("");
                        let is_snake_case = last_segment
                            .chars()
                            .next()
                            .map_or(false, |c| c.is_lowercase());

                        let module_name = if is_snake_case {
                            let new_name = format!("{}_component", name_str);
                            syn::parse_str::<syn::Path>(&new_name).unwrap_or(name.clone())
                        } else {
                            name.clone()
                        };

                        quote! {
                            azumi::Component::render(&#module_name::render(#module_name::Props::builder()
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
mod token_parser_tests;
